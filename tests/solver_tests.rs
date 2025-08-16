use demon_deduce::roles::*;
use demon_deduce::{brute_force_solve, Role};

#[test]
fn finds_minion_with_typed_statements() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];
    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        Some(Role::Confessor),
    ];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<RoleStatement> = vec![
        ConfessorStatement::IAmGood.into(),
        ConfessorStatement::IAmGood.into(),
        ConfessorStatement::IAmDizzy.into(),
    ];
    let sols = brute_force_solve(&deck, &visible, &confirmed, &observed, 2, 0, 1, 0);
    assert_eq!(sols.len(), 1);
    assert_eq!(
        sols[0],
        vec![Role::Confessor, Role::Confessor, Role::Minion]
    );
}

fn is_evil(role: &Role) -> bool {
    role.alignment() == Alignment::Evil
}

#[test]
fn example_minion_disguised_as_confessor() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];
    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        Some(Role::Confessor),
    ];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        ConfessorStatement::IAmGood.into(),
        ConfessorStatement::IAmGood.into(),
        ConfessorStatement::IAmDizzy.into(),
    ];

    let sols = brute_force_solve(&deck, &visible, &confirmed, &observed, 2, 0, 1, 0);
    assert_eq!(sols.len(), 1);
    let sol = &sols[0];
    assert_eq!(sol[0], Role::Confessor);
    assert_eq!(sol[1], Role::Confessor);
    assert_eq!(sol[2], Role::Minion);
}

#[test]
fn example_with_claim_statement() {
    let deck = vec![Role::Confessor, Role::Minion, Role::Confessor];
    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        Some(Role::Confessor),
    ];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        ConfessorStatement::IAmGood.into(),
        JudgeStatement {
            target_index: 0,
            is_lying: true,
        }
        .into(),
        ConfessorStatement::IAmDizzy.into(),
    ];

    let _ = brute_force_solve(&deck, &visible, &confirmed, &observed, 2, 0, 1, 0);
}

#[test]
fn test_iam_good_iam_dizzy_unrevealed() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];

    let visible = vec![Some(Role::Confessor), Some(Role::Confessor), None];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        ConfessorStatement::IAmGood.into(),
        ConfessorStatement::IAmDizzy.into(),
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 2, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[1]),
            "No matching solution found. Solutions: {:#?}",
            solutions
        );
    }
    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_iam_good_iam_good_unrevealed() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];

    let visible = vec![Some(Role::Confessor), Some(Role::Confessor), None];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        ConfessorStatement::IAmGood.into(),
        ConfessorStatement::IAmGood.into(),
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 2, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[2]),
            "No matching solution found. Solutions: {:#?}",
            solutions
        );
    }
    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_iam_good_claim_1_is_good_unrevealed() {
    let deck = vec![Role::Confessor, Role::Gemcrafter, Role::Minion];

    let visible = vec![Some(Role::Confessor), Some(Role::Gemcrafter), None];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        ConfessorStatement::IAmGood.into(),
        GemcrafterStatement { target_index: 0 }.into(),
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 2, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[2]),
            "No matching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_lover_lover_unrevealed_minion_unrevealed() {
    let deck = vec![
        Role::Lover,
        Role::Lover,
        Role::Confessor,
        Role::Confessor,
        Role::Minion,
    ];

    let visible = vec![Some(Role::Lover), Some(Role::Lover), None, None, None];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        LoverStatement { evil_count: 0 }.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[3]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_lover_lover_unrevealed_unrevealed_minion() {
    let deck = vec![
        Role::Lover,
        Role::Lover,
        Role::Confessor,
        Role::Confessor,
        Role::Minion,
    ];

    let visible = vec![Some(Role::Lover), Some(Role::Lover), None, None, None];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        LoverStatement { evil_count: 1 }.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[4]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_loverminion_lover_unrevealed_unrevealed() {
    let deck = vec![Role::Lover, Role::Lover, Role::Lover, Role::Minion];

    let visible = vec![
        Some(Role::Lover),
        Some(Role::Lover),
        Some(Role::Lover),
        None,
    ];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        LoverStatement { evil_count: 1 }.into(),
        LoverStatement { evil_count: 1 }.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 3, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[0]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_empress_empress_empress() {
    let deck = vec![
        Role::Empress,
        Role::Empress,
        Role::Empress,
        Role::Empress,
        Role::Minion,
    ];

    let visible = vec![
        Some(Role::Empress),
        Some(Role::Empress),
        Some(Role::Empress),
        Some(Role::Empress),
        Some(Role::Empress),
    ];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        EmpressStatement {
            target_indexes: to_bitvec(vec![1, 2, 3]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 3, 4]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 3, 4]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 1, 4]),
        }
        .into(),
        EmpressStatement {
            target_indexes: to_bitvec(vec![0, 1, 2]),
        }
        .into(),
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[0]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_hunter_lover() {
    let deck = vec![
        Role::Hunter,
        Role::Lover,
        Role::Confessor,
        Role::Confessor,
        Role::Confessor,
        Role::Minion,
    ];

    let visible = vec![
        Some(Role::Hunter),
        Some(Role::Lover),
        None,
        None,
        None,
        None,
    ];
    let confirmed = vec![None; visible.len()];

    let observed: Vec<RoleStatement> = vec![
        HunterStatement { distance: 3 }.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[3]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_enlightened() {
    let deck = vec![
        Role::Judge,
        Role::Hunter,
        Role::Confessor,
        Role::Lover,
        Role::Gemcrafter,
        Role::Enlightened,
        Role::Minion,
    ];
    let visible = vec![
        Some(Role::Gemcrafter),
        Some(Role::Enlightened),
        Some(Role::Lover),
        None,
        None,
        None,
    ];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<RoleStatement> = vec![
        GemcrafterStatement { target_index: 2 }.into(),
        EnlightenedStatement::Equidistant.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[4]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_wretch() {
    let deck = vec![
        Role::Hunter,
        Role::Empress,
        Role::Lover,
        Role::Gemcrafter,
        Role::Confessor,
        Role::Wretch,
        Role::Minion,
    ];
    let visible = vec![
        Some(Role::Empress),
        Some(Role::Lover),
        Some(Role::Confessor),
        None,
        Some(Role::Lover),
        None,
        Some(Role::Hunter),
    ];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<RoleStatement> = vec![
        EmpressStatement {
            target_indexes: to_bitvec(vec![5, 2, 3]),
        }
        .into(),
        LoverStatement { evil_count: 0 }.into(),
        ConfessorStatement::IAmGood.into(),
        RoleStatement::Unrevealed,
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
        HunterStatement { distance: 2 }.into(),
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 1, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[4]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_twin_and_medium() {
    use Role::*;
    let deck = vec![
        Judge,
        Lover,
        Gemcrafter,
        Enlightened,
        Medium,
        Wretch,
        Minion,
        TwinMinion,
    ];
    let visible = vec![
        Some(Medium),
        Some(Judge),
        Some(Gemcrafter),
        Some(Lover),
        Some(Gemcrafter),
        None,
        None,
    ];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<RoleStatement> = vec![
        MediumStatement {
            target_index: 2,
            role: Gemcrafter,
        }
        .into(),
        JudgeStatement {
            target_index: 0,
            is_lying: true,
        }
        .into(),
        GemcrafterStatement { target_index: 0 }.into(),
        LoverStatement { evil_count: 1 }.into(),
        GemcrafterStatement { target_index: 3 }.into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 1, 2, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[0]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
        assert!(
            is_evil(&solution[2]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_jester() {
    use Role::*;
    let deck = vec![
        Gemcrafter, Jester, Empress, Hunter, Lover, Wretch, Minion, TwinMinion,
    ];
    let visible = vec![
        Some(Jester),
        None,
        Some(Lover),
        None,
        None,
        Some(Hunter),
        Some(Lover),
        None,
    ];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<RoleStatement> = vec![
        JesterStatement {
            target_indexes: to_bitvec(vec![0, 2, 5]),
            evil_count: 1,
        }
        .into(),
        RoleStatement::Unrevealed,
        LoverStatement { evil_count: 1 }.into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        HunterStatement { distance: 4 }.into(),
        LoverStatement { evil_count: 0 }.into(),
        RoleStatement::Unrevealed,
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 1, 2, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[5]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
        assert!(
            is_evil(&solution[6]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_confirmed() {
    use Role::*;
    let deck = vec![Knight, Minion];
    let visible = vec![Some(Knight), Some(Knight)];
    let confirmed = vec![Some(Knight), None];
    let observed: Vec<RoleStatement> = vec![RoleStatement::Unrevealed, RoleStatement::Unrevealed];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 1, 0, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[1]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}

#[test]
fn test_scout() {
    use Role::*;
    let deck = vec![Scout, Empress, Judge, Enlightened, Jester, Wretch, Witch];
    let visible = vec![
        Some(Wretch),
        Some(Empress),
        None,
        Some(Jester),
        Some(Scout),
        Some(Enlightened),
    ];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<RoleStatement> = vec![
        RoleStatement::Unrevealed,
        EmpressStatement {
            target_indexes: to_bitvec(vec![2, 3, 4]),
        }
        .into(),
        RoleStatement::Unrevealed,
        RoleStatement::Unrevealed,
        ScoutStatement {
            role: Witch,
            distance: 3,
        }
        .into(),
        EnlightenedStatement::Clockwise.into(),
    ];

    let solutions = brute_force_solve(&deck, &visible, &confirmed, &observed, 4, 1, 1, 0);
    for solution in &solutions {
        assert!(
            is_evil(&solution[3]),
            "Unmatching solution found. Solutions: {:#?}",
            solutions
        );
    }

    assert!(
        !solutions.is_empty(),
        "No matching solution found. Solutions: {:#?}",
        solutions
    );
}
