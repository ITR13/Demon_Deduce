use demon_deduce::{brute_force_solve, Role};
use demon_deduce::roles::*;

fn main() {
    let deck = vec![Role::Judge, Role::Hunter, Role::Confessor, Role::Lover, Role::Gemcrafter, Role::Enlightened, Role::Minion];
    let visible = vec![Some(Role::Gemcrafter), Some(Role::Enlightened), Some(Role::Lover), None, None, None];
    let confirmed = vec![None; visible.len()];
    let observed: Vec<Box<dyn RoleStatement>> = vec![
        Box::new(ClaimStatement{target_index: 2, claim_type: ClaimType::Good}),
        Box::new(EnlightenedStatement::Equidistant),
        Box::new(EvilCountStatement { target_indexes: vec![1, 3], evil_count: 0, minimum: false, none_closer: false}),
        Box::new(UnrevealedStatement),
        Box::new(UnrevealedStatement),
        Box::new(UnrevealedStatement),
    ];

    let sols = brute_force_solve(&deck, &visible, &confirmed, &observed, 5, 0, 1);
    println!("Found {} solution(s)", sols.len());
    for s in sols {
        println!("{:?}", s);
    }
}
