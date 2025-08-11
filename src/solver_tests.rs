use demon_deduce::{brute_force_solve, Role};
use demon_deduce::roles::ConfessorStatement;

#[test]
fn finds_minion_with_typed_statements() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];
    let visible = vec![Role::Confessor, Role::Confessor, Role::Confessor];
    let observed: Vec<Box<dyn demon_deduce::roles::RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmDizzy),
    ];
    let sols = brute_force_solve(&deck, &visible, &observed);
    assert_eq!(sols.len(), 1);
    assert_eq!(sols[0], vec![Role::Confessor, Role::Confessor, Role::Minion]);
}
