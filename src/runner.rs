use crate::brute_force_solve;
use crate::roles::*;
use arboard::Clipboard;
use colored::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn parse_count(s: &str, name: &str, line_num: usize) -> usize {
    s.parse().unwrap_or_else(|_| {
        eprintln!(
            "Invalid {} count on line {}: '{}' is not a valid number",
            name, line_num, s
        );
        0
    })
}

fn parse_role(s: &str) -> Option<Role> {
    match s.trim() {
        "?" => None,
        role_str => Role::from_str(role_str).ok(),
    }
}

pub fn run_args(args: Vec<String>) {
    let (deck, visible, confirmed, observed, villagers, minions, demons, outcasts) =
        match parse_input(&args) {
            Ok(parsed) => parsed,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

    run_solver_and_print(
        &deck, &visible, &confirmed, &observed, villagers, minions, demons, outcasts,
    );
}
fn parse_input(
    args: &[String],
) -> Result<
    (
        Vec<Role>,
        Vec<Option<Role>>,
        Vec<Option<Role>>,
        Vec<RoleStatement>,
        usize,
        usize,
        usize,
        usize,
    ),
    String,
> {
    if args.len() < 6 {
        return Err(format!(
            "Usage: {} <deck> <villagers> <minions> <demons> <outcasts> [visible:confirmed:statement...]\nGot {} arguments",
            args[0],
            args.len() - 1
        ));
    }

    let deck = parse_roles(&args[1]).map_err(|e| {
        format!(
            "Failed to parse deck '{}': {}\nExpected comma-separated roles like 'bard,confessor,empress'",
            args[1], e
        )
    })?;

    let villagers = args[2].parse().map_err(|_| {
        format!(
            "Invalid villagers count '{}': must be a positive integer",
            args[2]
        )
    })?;
    let minions = args[3].parse().map_err(|_| {
        format!(
            "Invalid minions count '{}': must be a positive integer",
            args[3]
        )
    })?;
    let demons = args[4].parse().map_err(|_| {
        format!(
            "Invalid demons count '{}': must be a positive integer",
            args[4]
        )
    })?;
    let outcasts = args[5].parse().map_err(|_| {
        format!(
            "Invalid outcasts count '{}': must be a positive integer",
            args[5]
        )
    })?;

    let mut visible = Vec::new();
    let mut confirmed = Vec::new();
    let mut observed = Vec::new();

    for (arg_idx, card_arg) in args[6..].iter().enumerate() {
        let parts: Vec<&str> = card_arg.split(':').collect();
        let position = 6 + arg_idx;

        // Parse visible role
        let role = if parts[0].eq_ignore_ascii_case("?") {
            None
        } else {
            Some(Role::from_str(parts[0]).map_err(|e| {
                format!(
                    "Invalid visible role '{}' in argument {} ('{}'): {}",
                    parts[0], position, card_arg, e
                )
            })?)
        };
        visible.push(role);

        // Parse confirmed role
        confirmed.push(if parts.len() <= 1 || parts[1].eq_ignore_ascii_case("?") {
            None
        } else {
            Some(Role::from_str(parts[1]).map_err(|e| {
                format!(
                    "Invalid confirmed role '{}' in argument {} ('{}'): {}",
                    parts[1], position, card_arg, e
                )
            })?)
        });

        // Parse statement
        observed.push(
            if parts.len() <= 2
                || parts[2].eq_ignore_ascii_case("?")
                || parts[2].eq_ignore_ascii_case("unrevealed")
            {
                RoleStatement::Unrevealed
            } else {
                let role = role.ok_or_else(|| {
                    format!(
                        "Cannot provide statement for unrevealed role in argument {} ('{}')",
                        position, card_arg
                    )
                })?;
                role.parse_statement(parts[2]).map_err(|e| {
                    format!(
                        "Invalid statement '{}' for role {:?} in argument {} ('{}'): {}",
                        parts[2], role, position, card_arg, e
                    )
                })?
            },
        );
    }

    Ok((
        deck, visible, confirmed, observed, villagers, minions, demons, outcasts,
    ))
}

fn run_solver_and_print(
    deck: &[Role],
    visible: &[Option<Role>],
    confirmed: &[Option<Role>],
    observed: &[RoleStatement],
    villagers: usize,
    minions: usize,
    demons: usize,
    outcasts: usize,
) {
    // Solve all valid assignments and collect solutions
    let sols = brute_force_solve(
        deck, visible, confirmed, observed, villagers, minions, demons, outcasts,
    );

    if sols.len() == 0 {
        println!("No solutions found.");
        return;
    }

    println!("Found {} solution(s)", sols.len());

    for s in &sols {
        let line: Vec<String> = s.iter().map(|role| color_by_alignment(*role)).collect();
        println!("{}", line.join(", "));
    }

    println!("\nPossible roles per position:");
    for (i, _) in sols[0].iter().enumerate() {
        // Collect all roles that appear at this position across all solutions
        let mut possible_roles: Vec<Role> = sols.iter().map(|sol| sol[i]).collect();
        possible_roles.sort();
        possible_roles.dedup();
        let line: Vec<String> = possible_roles
            .into_iter()
            .map(|role| color_by_group(role))
            .collect();
        println!("Position {}: {}", i, line.join(", "));
    }
}

fn color_by_alignment(role: Role) -> String {
    match role.alignment() {
        Alignment::Good => format!("{}", format!("{:?}", role).green()),
        Alignment::Evil => format!("{}", format!("{:?}", role).red()),
    }
}

fn color_by_group(role: Role) -> String {
    match role.group() {
        Group::Villager => format!("{}", format!("{:?}", role).green()),
        Group::Outcast => format!("{}", format!("{:?}", role).yellow()),
        Group::Minion => format!("{}", format!("{:?}", role).red()),
        Group::Demon => format!("{}", format!("{:?}", role).bright_red()),
    }
}

fn parse_roles(s: &str) -> Result<Vec<Role>, String> {
    s.to_lowercase()
        .split(',')
        .enumerate()
        .map(|(i, r)| {
            let trimmed = r.trim();
            Role::from_str(trimmed).map_err(|e| {
                format!(
                    "Failed to parse role '{}' at position {}: {}",
                    trimmed,
                    i + 1,
                    e
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()
}
