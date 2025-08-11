use demon_deduce::{brute_force_solve, Role};
use demon_deduce::roles::{ConfessorStatement};

fn main() {
    let deck = vec![Role::Confessor, Role::Confessor, Role::Minion];
    let visible = vec![Some(Role::Confessor), Some(Role::Confessor), Some(Role::Confessor)];
    let observed: Vec<Box<dyn demon_deduce::roles::RoleStatement>> = vec![
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmGood),
        Box::new(ConfessorStatement::IAmDizzy),
    ];

    let sols = brute_force_solve(&deck, &visible, &observed);
    println!("Found {} solution(s)", sols.len());
    for s in sols {
        println!("{:?}", s);
    }
}
