use demon_deduce::{brute_force_solve, Role};
use demon_deduce::roles::*;
use std::boxed::Box;

#[test]
fn finds_minion_with_typed_statements() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];
    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        Some(Role::Confessor),
    ];
    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmDizzy),
    ];
    let sols = brute_force_solve(&deck, &visible, &observed);
    assert_eq!(sols.len(), 1);
    assert_eq!(sols[0], vec![Role::Confessor, Role::Confessor, Role::Minion]);
}

fn is_evil(role: &Role) -> bool {
    matches!(role, Role::Minion)
}

#[test]
fn example_minion_disguised_as_confessor() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];
    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        Some(Role::Confessor),
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmDizzy),
    ];

    let sols = brute_force_solve(&deck, &visible, &observed);
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

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ClaimStatement { target_index: 0, claims_evil: true }),
        Box::new(ConfessorStatement::IAmDizzy),
    ];

    let _ = brute_force_solve(&deck, &visible, &observed);
}

#[test]
fn test_iam_good_iam_dizzy_unrevealed() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];

    // visible: None where observed is UnrevealedStatement (index 2)
    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        None,
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmDizzy),
        Box::new(UnrevealedStatement),
    ];

    let solutions = brute_force_solve(&deck, &visible, &observed);
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

    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Confessor),
        None,
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmGood),
        Box::new(UnrevealedStatement),
    ];

    let solutions = brute_force_solve(&deck, &visible, &observed);
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

    let visible = vec![
        Some(Role::Confessor),
        Some(Role::Gemcrafter),
        None,
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ClaimStatement { target_index: 0, claims_evil: false }),
        Box::new(UnrevealedStatement),
    ];

    let solutions = brute_force_solve(&deck, &visible, &observed);
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
    let deck = vec![Role::Lover, Role::Lover, Role::Confessor, Role::Confessor, Role::Minion];

    let visible = vec![
        Some(Role::Lover),
        Some(Role::Lover),
        None,
        None,
        None,
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(EvilCountStatement { target_indexes: vec![1, 4], evil_count: 0}),
        Box::new(EvilCountStatement { target_indexes: vec![0, 2], evil_count: 0}),
        Box::new(UnrevealedStatement),
        Box::new(UnrevealedStatement),
        Box::new(UnrevealedStatement),
    ];

    let solutions = brute_force_solve(&deck, &visible, &observed);
    for solution in &solutions {
        assert!(
            is_evil(&solution[3]),
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
fn test_lover_lover_unrevealed_unrevealed_minion() {
    let deck = vec![Role::Lover, Role::Lover, Role::Confessor, Role::Confessor, Role::Minion];

    let visible = vec![
        Some(Role::Lover),
        Some(Role::Lover),
        None,
        None,
        None,
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(EvilCountStatement { target_indexes: vec![1, 4], evil_count: 1}),
        Box::new(EvilCountStatement { target_indexes: vec![0, 2], evil_count: 0}),
        Box::new(UnrevealedStatement),
        Box::new(UnrevealedStatement),
        Box::new(UnrevealedStatement),
    ];

    let solutions = brute_force_solve(&deck, &visible, &observed);
    for solution in &solutions {
        assert!(
            is_evil(&solution[4]),
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
fn test_loverminion_lover_unrevealed_unrevealed() {
    let deck = vec![Role::Lover, Role::Lover, Role::Lover, Role::Minion];

    let visible = vec![
        Some(Role::Lover),
        Some(Role::Lover),
        Some(Role::Lover),
        None,
    ];

    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(EvilCountStatement { target_indexes: vec![1, 3], evil_count: 1}),
        Box::new(EvilCountStatement { target_indexes: vec![0, 2], evil_count: 1}),
        Box::new(EvilCountStatement { target_indexes: vec![1, 3], evil_count: 0}),
        Box::new(UnrevealedStatement),
    ];

    let solutions = brute_force_solve(&deck, &visible, &observed);
    for solution in &solutions {
        assert!(
            is_evil(&solution[0]),
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