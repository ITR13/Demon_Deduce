// src/solver.rs
use crate::roles::{produce_statement, Role, RoleStatement};
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
) -> Vec<Vec<Role>> {
    assert_eq!(
        visible_roles.len(),
        observed_statements.len(),
        "visible_roles and observed_statements must match"
    );
    let n = visible_roles.len();
    assert_eq!(deck.len(), n, "v0.1 assumes deck size == played size");

    let mut valid: Vec<Vec<Role>> = Vec::new();
    let mut seen = HashSet::new();

    for perm in deck.iter().permutations(n) {
        let candidate: Vec<Role> = perm.into_iter().copied().collect();

        if !seen.insert(candidate.clone()) {
            continue;
        }

        // For each true_role position, determine disguise choices:
        // - If true_role == Minion => disguise may be any role present in deck (unique).
        // - Else: disguise == true_role.
        let disguise_choices: Vec<Vec<Role>> = candidate
            .iter()
            .map(|&r| {
                if r == Role::Minion {
                    deck.iter().copied().unique().collect()
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

            // Simulate typed statements
            let simulated: Vec<Box<dyn RoleStatement>> = candidate
                .iter()
                .zip(disguise_assign.iter())
                .enumerate()
                .map(|(idx, (&true_role, &vis_role))| produce_statement(true_role, vis_role, idx))
                .collect();

            // Compare simulated -> observed using typed equals
            let mut all_eq = true;
            for (sim, obs) in simulated.iter().zip(observed_statements.iter()) {
                if !obs.equals(sim.as_ref()) {
                    all_eq = false;
                    break;
                }
            }
            if all_eq {
                valid.push(candidate.clone());
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
