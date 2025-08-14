use crate::roles::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn brute_force_solve(
    deck: &[Role],
    visible_roles: &[Option<Role>],
    confirmed_roles: &[Option<Role>],
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

    // Pre-generate all possible role group combinations based on counts requested
    let (villager_combos, outcast_combos, minion_combos) =
        generate_role_combinations(deck, villagers, outcasts, minions);

    // Wretch needs to be replaced with any minion from the deck — precompute choices
    let deck_minion_choices: Vec<Role> = deck
        .iter()
        .copied()
        .unique()
        .filter(|r| r.group() == Group::Minion)
        .collect();

    // Disguised minions can appear as any non-evil role — precompute choices
    let deck_non_evil_choices: Vec<Role> = deck
        .iter()
        .copied()
        .unique()
        .filter(|r| r.alignment() != Alignment::Evil)
        .collect();

    let mut valid = Vec::new();
    let mut perm_current: Vec<Role> = Vec::with_capacity(n);

    let mut wretch_assign: Vec<Role> = Vec::with_capacity(n);
    let mut disguise_assign: Vec<Role> = Vec::with_capacity(n);

    // Try every possible combination of villagers, minions, and outcasts
    for v_combo in &villager_combos {
        for m_combo in &minion_combos {
            for o_combo in &outcast_combos {
                let combined = collect_roles(v_combo, m_combo, o_combo);

                // Prepare role counts for multiset permutation generation
                let mut counts: HashMap<Role, usize> = HashMap::new();
                for &r in &combined {
                    *counts.entry(r).or_insert(0) += 1;
                }

                let keys: Vec<Role> = counts.keys().copied().collect();

                // Generate all seat permutations of this role multiset
                permute_multiset(
                    &mut counts,
                    &keys,
                    &mut perm_current,
                    n,
                    &mut |candidate: &[Role]| {
                        // Immediately discard if known confirmed roles don’t match
                        if !confirmed_roles_ok(candidate, confirmed_roles) {
                            return;
                        }

                        // Build possible Wretch replacements and minion disguises for each seat
                        let (wretch_choices, disguise_choices) = build_choices(
                            candidate,
                            &deck_minion_choices,
                            &deck_non_evil_choices,
                        );

                        // DFS through every possible Wretch assignment + disguise mapping
                        assign_disguises_and_check(
                            candidate,
                            &wretch_choices,
                            &disguise_choices,
                            visible_roles,
                            &mut wretch_assign,
                            &mut disguise_assign,
                            0,
                            &mut |full_wretch_assign: &[Role], full_disguise_assign: &[Role]| {
                                // If the resulting seating matches all observed statements, keep it
                                let success = statements_match(
                                    candidate,
                                    full_wretch_assign,
                                    full_disguise_assign,
                                    observed_statements,
                                );
                                if success {
                                    valid.push(candidate.to_vec());
                                }
                                success
                            },
                        );
                    },
                );
            }
        }
    }

    valid
}

fn build_choices(
    candidate: &[Role],
    deck_minion_choices: &[Role],
    deck_non_evil_choices: &[Role],
) -> (Vec<Vec<Role>>, Vec<Vec<Role>>) {
    let mut wretch_choices = Vec::with_capacity(candidate.len());
    let mut disguise_choices = Vec::with_capacity(candidate.len());

    for &r in candidate {
        wretch_choices.push(if r == Role::Wretch {
            // Wretch's "true role" is always some minion
            deck_minion_choices.to_vec()
        } else {
            vec![r]
        });

        disguise_choices.push(if r.group() == Group::Minion {
            // Minions can appear as any non-evil role
            deck_non_evil_choices.to_vec()
        } else {
            vec![r]
        });
    }

    (wretch_choices, disguise_choices)
}

fn generate_role_combinations(
    deck: &[Role],
    villagers: usize,
    outcasts: usize,
    minions: usize,
) -> (Vec<Vec<Role>>, Vec<Vec<Role>>, Vec<Vec<Role>>) {
    // Partition deck into groups for combination generation
    let (villager_roles, others): (Vec<Role>, Vec<Role>) =
        deck.iter().cloned().partition(|r| r.group() == Group::Villager);

    let (outcast_roles, minion_roles): (Vec<Role>, Vec<Role>) =
        others.into_iter().partition(|r| r.group() == Group::Outcast);

    // Generate all ways to pick the exact number of each group type
    let villager_combos: Vec<Vec<Role>> = villager_roles
        .iter()
        .combinations(villagers)
        .map(|combo| combo.into_iter().copied().collect())
        .collect();

    let outcast_combos: Vec<Vec<Role>> = outcast_roles
        .iter()
        .combinations(outcasts)
        .map(|combo| combo.into_iter().copied().collect())
        .collect();

    let minion_combos: Vec<Vec<Role>> = minion_roles
        .iter()
        .combinations(minions)
        .map(|combo| combo.into_iter().copied().collect())
        .collect();

    (villager_combos, outcast_combos, minion_combos)
}

fn collect_roles(v_combo: &[Role], m_combo: &[Role], o_combo: &[Role]) -> Vec<Role> {
    v_combo
        .iter()
        .chain(m_combo.iter())
        .chain(o_combo.iter())
        .copied()
        .collect()
}

fn permute_multiset<F>(
    counts: &mut HashMap<Role, usize>,
    keys: &[Role],
    current: &mut Vec<Role>,
    target_len: usize,
    process: &mut F,
) where
    F: FnMut(&[Role]),
{
    if current.len() == target_len {
        process(current.as_slice());
        return;
    }

    for &k in keys {
        let cnt = counts.get(&k).copied().unwrap_or(0);
        if cnt == 0 {
            continue;
        }

        // Temporarily consume one of this role before recursing
        counts.insert(k, cnt - 1);
        current.push(k);

        permute_multiset(counts, keys, current, target_len, process);
        // Restore state after exploring this branch
        current.pop();
        counts.insert(k, cnt);
    }
}

fn confirmed_roles_ok(candidate: &[Role], confirmed_roles: &[Option<Role>]) -> bool {
    // A seat is only invalid if a confirmed role exists and doesn't match the candidate
    candidate
        .iter()
        .zip(confirmed_roles.iter())
        .all(|(r, c)| c.is_none() || c.as_ref() == Some(r))
}

fn assign_disguises_and_check<F>(
    candidate: &[Role],
    wretch_choices: &[Vec<Role>],
    disguise_choices: &[Vec<Role>],
    visible_roles: &[Option<Role>],
    wretch_assign: &mut Vec<Role>,
    disguise_assign: &mut Vec<Role>,
    pos: usize,
    on_complete: &mut F,
) -> bool where
    F: FnMut(&[Role], &[Role]) -> bool,
{
    let n = candidate.len();
    if pos == n {
        return on_complete(wretch_assign.as_slice(), disguise_assign.as_slice());
    }

    for &w_choice in &wretch_choices[pos] {
        wretch_assign.push(w_choice);

        for &d_choice in &disguise_choices[pos] {
            // If visible role is fixed for this seat, skip mismatches
            if let Some(req_vis) = visible_roles[pos] {
                if req_vis != d_choice {
                    continue;
                }
            }

            disguise_assign.push(d_choice);
            let success = assign_disguises_and_check(
                candidate,
                wretch_choices,
                disguise_choices,
                visible_roles,
                wretch_assign,
                disguise_assign,
                pos + 1,
                on_complete,
            );
            disguise_assign.pop();
            if success {
                wretch_assign.pop();
                return true;
            }
        }

        wretch_assign.pop();
    }
    return false;
}

fn statements_match(
    candidate: &[Role],
    wretch_assign: &[Role],
    disguise_assign: &[Role],
    observed_statements: &[Box<dyn RoleStatement>],
) -> bool {
    for (idx, (&true_role, &vis_role)) in candidate.iter().zip(disguise_assign.iter()).enumerate() {
        // Generate every statement that could be made by this seat
        let possible_statements =
            produce_statements(true_role, Some(vis_role), wretch_assign, disguise_assign, idx);

        // If none of the possible statements match the observed one, reject candidate
        let obs = &observed_statements[idx];
        if !possible_statements.iter().any(|ps| obs.equals(ps.as_ref())) {
            return false;
        }
    }
    true
}
