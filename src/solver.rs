use crate::roles::*;
use itertools::izip;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
pub fn brute_force_solve(
    deck: &[Role],
    visible_roles: &[Option<Role>],
    confirmed_roles: &[Option<Role>],
    observed_statements: &[RoleStatement],
    villagers: usize,
    outcasts: usize,
    minions: usize,
    demons: usize,
    verbose: bool,
) -> Vec<Vec<Role>> {
    assert_eq!(
        visible_roles.len(),
        observed_statements.len(),
        "visible_roles and observed_statements must match"
    );
    let n = visible_roles.len();

    // Pre-generate all possible role group combinations based on counts requested
    let (villager_combos, outcast_combos, minion_combos, demon_combos) =
        generate_role_combinations(deck, villagers, outcasts, minions, demons);

    // Wretch needs to be replaced with any minion from the deck — precompute choices
    let deck_minions: Vec<Role> = deck
        .iter()
        .copied()
        .filter(|r| r.group() == Group::Minion)
        .collect();

    // Disguised minions can appear as any non-evil role — precompute choices
    let deck_non_evil: Vec<Role> = deck
        .iter()
        .copied()
        .filter(|r| r.alignment() != Alignment::Evil)
        .collect();

    // Try every possible combination of villagers, minions, and outcasts
    villager_combos.par_iter().flat_map(|v_combo| {
        let deck_villager_not_in_play: Vec<Role> = deck
            .iter()
            .copied()
            .filter(|r| r.group() == Group::Villager && !v_combo.contains(r))
            .collect();

        let mut local_valid = Vec::new();
        let mut perm_current: Vec<Role> = Vec::with_capacity(n);
        let mut wretch_assign: Vec<Role> = Vec::with_capacity(n);
        let mut disguise_assign: Vec<Role> = Vec::with_capacity(n);

        for o_combo in &outcast_combos {
            let outcasts_not_in_play: Vec<Role> = deck
                .iter()
                .copied()
                .filter(|r| r.group() == Group::Outcast && !o_combo.contains(r))
                .collect();
            for m_combo in &minion_combos {
                let has_counsellor = m_combo.iter().any(|&r| r == Role::Counsellor);
                for d_combo in &demon_combos {
                    let combined_variations = generate_role_variations(
                        v_combo,
                        o_combo,
                        m_combo,
                        d_combo,
                        &outcasts_not_in_play,
                        has_counsellor,
                    );

                    for combined in combined_variations {
                        let deck_villagers_in_play: Vec<_> = combined
                            .iter()
                            .copied()
                            .filter(|r| r.group() == Group::Villager && !v_combo.contains(r))
                            .collect();

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
                            has_counsellor,
                            &mut |candidate: &[Role]| {
                                // Immediately discard if known confirmed roles don’t match
                                if !confirmed_roles_ok(candidate, confirmed_roles) {
                                    return;
                                }

                                // Build possible Wretch replacements and minion disguises for each seat
                                let (wretch_choices, disguise_choices) = build_choices(
                                    candidate,
                                    &deck_minions,
                                    &deck_non_evil,
                                    &deck_villagers_in_play,
                                    &deck_villager_not_in_play,
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
                                        verbose
                                    );
                                    if success {
                                        local_valid.push(candidate.to_vec());
                                    }
                                    success
                                },
                            );
                            },
                        );
                    }
                }
            }
        }

        local_valid
    }).collect()
}

fn generate_role_variations(
    v_combo: &[Role],
    o_combo: &[Role],
    m_combo: &[Role],
    d_combo: &[Role],
    outcasts_not_in_play: &[Role],
    has_counsellor: bool,
) -> Vec<Vec<Role>> {
    let mut combinations = Vec::new();

    if !has_counsellor {
        let base_combination: Vec<Role> = v_combo
            .iter()
            .chain(o_combo.iter())
            .chain(m_combo.iter())
            .chain(d_combo.iter())
            .copied()
            .collect();
        combinations.push(base_combination);
        return combinations;
    }

    for (i, role) in v_combo.iter().enumerate() {
        if role.group() == Group::Villager {
            for outcast in outcasts_not_in_play {
                let mut modified_v_combo = v_combo.to_vec();
                modified_v_combo[i] = *outcast;

                let combination: Vec<Role> = modified_v_combo
                    .iter()
                    .chain(o_combo.iter())
                    .chain(m_combo.iter())
                    .chain(d_combo.iter())
                    .copied()
                    .collect();

                combinations.push(combination);
            }
        }
    }

    combinations
}

fn build_choices(
    candidate: &[Role],
    deck_minions: &[Role],
    deck_non_evil: &[Role],
    deck_villagers_in_play: &[Role],
    deck_villager_not_in_play: &[Role],
) -> (Vec<Vec<Role>>, Vec<Vec<Role>>) {
    let mut wretch_choices = Vec::with_capacity(candidate.len());
    let mut disguise_choices = Vec::with_capacity(candidate.len());

    for &r in candidate {
        // Wretch choices
        wretch_choices.push(if r == Role::Wretch {
            // Wretch's "true role" is always some minion
            deck_minions.to_vec()
        } else {
            vec![r]
        });

        // Disguise choices
        let group = r.group();
        let choices = if group == Group::Demon {
            deck_villager_not_in_play.to_vec()
        } else if group == Group::Minion {
            deck_non_evil.to_vec()
        } else if r == Role::DoppelGanger {
            deck_villagers_in_play.to_vec()
        } else {
            vec![r]
        };

        disguise_choices.push(choices);
    }

    (wretch_choices, disguise_choices)
}

fn generate_role_combinations(
    deck: &[Role],
    villagers: usize,
    outcasts: usize,
    minions: usize,
    demons: usize,
) -> (
    Vec<Vec<Role>>,
    Vec<Vec<Role>>,
    Vec<Vec<Role>>,
    Vec<Vec<Role>>,
) {
    // Partition deck by group
    let (villager_roles, others): (Vec<Role>, Vec<Role>) = deck
        .iter()
        .cloned()
        .partition(|r| r.group() == Group::Villager);

    let (outcast_roles, others): (Vec<Role>, Vec<Role>) = others
        .into_iter()
        .partition(|r| r.group() == Group::Outcast);

    let (minion_roles, demon_roles): (Vec<Role>, Vec<Role>) =
        others.into_iter().partition(|r| r.group() == Group::Minion);

    // Generate combinations for each group
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

    let demon_combos: Vec<Vec<Role>> = demon_roles
        .iter()
        .combinations(demons)
        .map(|combo| combo.into_iter().copied().collect())
        .collect();

    (villager_combos, outcast_combos, minion_combos, demon_combos)
}

fn permute_multiset<F>(
    counts: &mut HashMap<Role, usize>,
    keys: &[Role],
    current: &mut Vec<Role>,
    target_len: usize,
    has_counsellor: bool,
    process: &mut F,
) where
    F: FnMut(&[Role]),
{
    if current.len() == target_len {
        // This can be optimized by checking for it earlier in the run
        if let Some(counsellor_pos) = current.iter().position(|&r| r == Role::Counsellor) {
            let len = current.len();
            let left_pos = if counsellor_pos == 0 {
                len - 1
            } else {
                counsellor_pos - 1
            };
            let right_pos = if counsellor_pos == len - 1 {
                0
            } else {
                counsellor_pos + 1
            };

            let has_adjacent_outcast = current[left_pos].group() == Group::Outcast
                || current[right_pos].group() == Group::Outcast;

            if !has_adjacent_outcast {
                return;
            }
        }

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

        permute_multiset(counts, keys, current, target_len, has_counsellor, process);
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
) -> bool
where
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
    observed_statements: &[RoleStatement],
    verbose: bool,
) -> bool {
    // NB: This makes us lose corruption data! A proper solution would consider the corruptions separately
    let corrupt_permutations = execute_corruption(candidate, wretch_assign);

    'corruption_loop: for pre_corruption in corrupt_permutations {
        let (corruption, uncorruptions) =
            execute_uncorruption(candidate, disguise_assign, &pre_corruption);

        for (idx, (&true_role, &vis_role, is_corrupt)) in
            izip!(candidate.iter(), disguise_assign.iter(), corruption.iter()).enumerate()
        {
            let obs = &observed_statements[idx];
            if *obs == RoleStatement::NoStatement {
                continue;
            }

            let lying = true_role.lying() || *is_corrupt;

            let is_valid = can_produce_statement(
                vis_role,
                lying,
                wretch_assign,
                disguise_assign,
                corruption.as_slice(),
                uncorruptions.as_slice(),
                idx,
                obs,
            );

            // If not valid, reject candidate
            if !is_valid {
                if verbose {
                    let candidate_str = candidate
                        .iter()
                        .zip(corruption.iter())
                        .map(|(role, corrupted)| {
                            if *corrupted {
                                format!("{}*", role)
                            } else {
                                role.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    eprintln!("Invalid candidate: [{}]", candidate_str);
                    eprintln!(
                        "Statement {} didn't match for role {} (visible as {})",
                        obs, true_role, vis_role
                    );
                }
                continue 'corruption_loop;
            }
        }
        // All statements matched
        return true;
    }
    // All corruption permutationed had some statement that didn't match
    return false;
}

fn execute_corruption(true_roles: &[Role], wretch_assign: &[Role]) -> Vec<Vec<bool>> {
    let len = true_roles.len();
    let mut poison_options: Vec<Vec<usize>> = Vec::new();

    // Sort by role priority
    let mut roles_with_indices: Vec<(usize, Role)> = true_roles.iter().enumerate().map(|(i, &r)| (i, r)).collect();
    roles_with_indices.sort_by(|a, b| {
        let priority = |role: Role| match role {
            Role::Pooka => 0,
            Role::Poisoner => 1,
            Role::PlagueDoctor => 2,
            _ => 3,
        };
        priority(a.1).cmp(&priority(b.1)).then_with(|| a.0.cmp(&b.0))
    });

    // Collect lists of lists to permute over
    for (i, role) in roles_with_indices {
        match role {
            Role::Pooka => {
                // All neighbouring villagers
                let neighbors = neighbor_indexes(len, i, 1);
                for n in neighbors {
                    if wretch_assign[n].group() == Group::Villager {
                        poison_options.push(vec![n]); // We pretend it's actually two separate roles making choices
                    }
                }
            }
            Role::Poisoner => {
                // One neighbouring villager
                let neighbors = neighbor_indexes(len, i, 1);
                let eligible: Vec<usize> = neighbors
                    .into_iter()
                    .filter(|&n| wretch_assign[n].group() == Group::Villager)
                    .collect();

                if !eligible.is_empty() {
                    poison_options.push(eligible);
                }
            }
            Role::PlagueDoctor => {
                // Any random villager
                let eligible: Vec<usize> = wretch_assign
                    .iter()
                    .enumerate()
                    .filter(|(_, &r)| r.group() == Group::Villager)
                    .map(|(index, _)| index)
                    .collect();

                if !eligible.is_empty() {
                    poison_options.push(eligible);
                }
            }
            _ => {}
        }
    }

    fn combine(
        poison_options: &[Vec<usize>],
        idx: usize,
        current: &mut Vec<bool>,
        result: &mut Vec<Vec<bool>>,
    ) {
        if idx == poison_options.len() {
            result.push(current.clone());
            return;
        }

        let mut any_valid = false;
        for &target in &poison_options[idx] {
            if !current[target] {
                let mut next = current.clone();
                next[target] = true;
                combine(poison_options, idx + 1, &mut next, result);
                any_valid = true;
            }
        }
        if !any_valid {
            combine(poison_options, idx + 1, current, result);
        }
    }

    let mut result = Vec::new();
    combine(&poison_options, 0, &mut vec![false; len], &mut result);

    result
}

fn execute_uncorruption(
    true_roles: &[Role],
    disguised_roles: &[Role],
    corruption: &[bool],
) -> (Vec<bool>, Vec<usize>) {
    let len = corruption.len();
    let mut mut_corruption = corruption.to_vec();
    let mut cleared_counts = vec![0_usize; len];

    for i in 0..len {
        if disguised_roles[i] == Role::Alchemist && !mut_corruption[i] && !true_roles[i].lying() {
            let mut cleared = 0;

            for &offset in &[1, 2] {
                for &neighbor in &neighbor_indexes(len, i, offset) {
                    if mut_corruption[neighbor] {
                        mut_corruption[neighbor] = false;
                        cleared += 1;
                    }
                }
            }

            cleared_counts[i] = cleared;
        }
    }

    (mut_corruption, cleared_counts)
}
