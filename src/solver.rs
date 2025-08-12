use crate::roles::*;
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
    outcasts: usize,
    minions: usize,
) -> Vec<Vec<Role>> {
    assert_eq!(
        visible_roles.len(),
        observed_statements.len(),
        "visible_roles and observed_statements must match"
    );
    let n = visible_roles.len();

    let mut valid: Vec<Vec<Role>> = Vec::new();
    let mut seen = HashSet::new();

    let (villager_roles, non_villagers): (Vec<Role>, Vec<Role>) = deck
        .iter()
        .partition(|role| role.group() == Group::Villager);

    let (outcast_roles, minion_roles): (Vec<Role>, Vec<Role>) = non_villagers
        .into_iter()
        .partition(|role| role.group() == Group::Outcast);
    let villager_combos: Vec<_> = villager_roles.iter().combinations(villagers).collect();
    let outcast_combos: Vec<_> = outcast_roles.iter().combinations(outcasts).collect();
    let minion_combos: Vec<_> = minion_roles.iter().combinations(minions).collect();

    for v_combo in &villager_combos {
        for m_combo in &minion_combos {
            for o_combo in &outcast_combos {
                let combined: Vec<Role> = v_combo
                    .iter()
                    .chain(m_combo.iter())
                    .chain(o_combo.iter())
                    .map(|&&r| r)
                    .collect();

                for perm in combined.iter().permutations(n) {
                    let candidate: Vec<Role> = perm.into_iter().copied().collect();

                    if !seen.insert(candidate.clone()) {
                        continue;
                    }

                    // Wretch can fool other roles with their disguise, so it needs to be handled separately from disguises
                    // - If true_role == Wretch => disguise may be any minion role present in deck.
                    // - Else: disguise == true_role.
                    let wretch_disguise_choices: Vec<Vec<Role>> = candidate
                        .iter()
                        .map(|&r| {
                            if r == Role::Wretch {
                                deck
                                    .iter()
                                    .copied()
                                    .unique()
                                    .filter(|role| role.group() == Group::Minion)
                                    .collect()
                            } else {
                                vec![r]
                            }
                        })
                        .collect();


                    // For each true_role position, determine disguise choices:
                    // - If true_role == Minion => disguise may be any non-evil role present in deck.
                    // - Else: disguise == true_role.
                    let disguise_choices: Vec<Vec<Role>> = candidate
                        .iter()
                        .map(|&r| {
                            if r.group() == Group::Minion {
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
                    'disguise_loop: for wretch_disguise_assign in cartesian(&wretch_disguise_choices) {
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
                                // NB: Using wretch_disguise_assign as true role here, and not above bc we want the card itself to know it's true role, but not other cards
                                let possible_statements = produce_statements(true_role, Some(vis_role), &wretch_disguise_assign, &disguise_assign, idx);

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
                                break 'disguise_loop;
                            }
                        }
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
