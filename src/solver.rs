use crate::roles::{produce_statements, Role, Alignment, RoleStatement};
use itertools::Itertools;
use std::collections::HashSet;
use std::option::Option;

/// Brute-force solver that uses typed statements per role.
///
/// - `deck`: roles available in the deck (e.g., [Confessor, Confessor, Minion])
/// - `visible_roles`: what each played card shows (fixed)
/// - `observed_statements`: typed statements observed from each played card (fixed)
///
/// Returns: Vec of valid permutations (Vec<Role> true roles assigned to positions)
pub fn brute_force_solve(
    deck: &[Role],
    visible_roles: &[Option<Role>],
    observed_statements: &[Box<dyn RoleStatement>],
    villagers: usize,
    evil: usize,
) -> Vec<Vec<Role>> {
    assert_eq!(
        visible_roles.len(),
        observed_statements.len(),
        "visible_roles and observed_statements must match"
    );
    let n = visible_roles.len();

    let mut valid: Vec<Vec<Role>> = Vec::new();
    let mut seen = HashSet::new();

    let (villager_roles, evil_roles): (Vec<_>, Vec<_>) = deck
        .iter()
        .partition(|role| role.alignment() == Alignment::Villager);
    let villager_combos = villager_roles.iter().combinations(villagers);
    let evil_combos = evil_roles.iter().combinations(evil);

    for v_combo in villager_combos {
        for e_combo in evil_combos.clone() {
            let combined: Vec<Role> = v_combo
                .iter()
                .chain(e_combo.iter())
                .copied()
                .cloned()
                .collect();

            for perm in combined.iter().permutations(n) {
                let candidate: Vec<Role> = perm.into_iter().copied().collect();

                if !seen.insert(candidate.clone()) {
                    continue;
                }

                // For each true_role position, determine disguise choices:
                // - If true_role == Minion => disguise may be any non-evil role present in deck.
                // - Else: disguise == true_role.
                let disguise_choices: Vec<Vec<Role>> = candidate
                    .iter()
                    .map(|&r| {
                        if r == Role::Minion {
                            deck
                                .iter()
                                .copied()
                                .unique()
                                .filter(|role| role.alignment() != Alignment::Evil)
                                .collect()
                        } else {
                            vec![r]
                        }
                    })
                    .collect();

                // iterate cartesian product of disguise assignments
                for disguise_assign in cartesian(&disguise_choices) {
                    // Check visible role match
                    let visible_ok = disguise_assign
                        .iter()
                        .zip(visible_roles.iter())
                        .all(|(d, v)| v.is_none() || v.as_ref() == Some(d));
                    if !visible_ok {
                        continue;
                    }

                    let mut all_eq = true;
                    for (idx, (&true_role, &vis_role)) in candidate.iter().zip(disguise_assign.iter()).enumerate() {
                        // produce_statements likely returns Vec<Box<dyn RoleStatement>>
                        let possible_statements = produce_statements(true_role, Some(vis_role), &candidate, idx);

                        let obs = &observed_statements[idx];

                        // Check if any of the possible statements match the observed
                        if !possible_statements
                            .iter()
                            .any(|ps| obs.equals(ps.as_ref()))
                        {
                            all_eq = false;
                            break;
                        }
                    }
                    if all_eq {
                        valid.push(candidate.clone());
                    }
                }
            }
        }
    }

    valid
}

/// Small cartesian product collector (fine for v0.1 small sizes).
fn cartesian<T: Clone>(choices: &[Vec<T>]) -> Vec<Vec<T>> {
    if choices.is_empty() {
        return vec![vec![]];
    }
    let mut acc: Vec<Vec<T>> = vec![vec![]];
    for opts in choices {
        let mut next = Vec::new();
        for prefix in &acc {
            for item in opts {
                let mut p = prefix.clone();
                p.push(item.clone());
                next.push(p);
            }
        }
        acc = next;
    }
    acc
}
