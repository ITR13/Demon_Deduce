use crate::brute_force_solve;
use crate::roles::*;
use crate::validate_candidate;
use arboard::Clipboard;
use colored::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_clipboard_loop() {
    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Failed to initialize clipboard"),
    ));

    let mut last_content = String::new();

    loop {
        let current_content = {
            let mut cb = clipboard.lock().unwrap();
            cb.get_text().unwrap_or_default()
        };

        if current_content != last_content {
            last_content = current_content.clone();
            parse_clipboard(&current_content);
        }

        thread::sleep(Duration::from_millis(200));
    }
}

pub fn run_from_clipboard() {
    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Failed to initialize clipboard"),
    ));

    let current_content = {
        let mut cb = clipboard.lock().unwrap();
        cb.get_text().unwrap_or_default()
    };
    parse_clipboard(&current_content);
}

fn parse_clipboard(content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        eprintln!("Clipboard content too short - expected at least 2 lines (deck and counts)");
        return;
    }

    let deck = match parse_roles(lines[0]) {
        Ok(deck) => deck,
        Err(e) => {
            eprintln!("Failed to parse deck '{}': {}", lines[0], e);
            return;
        }
    };

    let count_parts: Vec<&str> = lines[1].split_whitespace().collect();
    if count_parts.len() != 4 {
        eprintln!(
            "Expected 4 counts on the second line (villagers outcasts minions demons), found {}: '{}'",
            count_parts.len(),
            lines[1]
        );
        return;
    }

    let villagers = parse_count(count_parts[0], "villagers", 1);
    let outcasts = parse_count(count_parts[1], "outcasts", 1);
    let minions = parse_count(count_parts[2], "minions", 1);
    let demons = parse_count(count_parts[3], "demons", 1);
    let num_seats = villagers + outcasts + minions + demons;

    let mut visible = vec![None; num_seats];
    let mut confirmed: Vec<Option<Role>> = vec![None; num_seats];
    let mut observed = vec![RoleStatement::NoStatement; num_seats];

    let mut has_errors = false;

    for line in &lines[2..] {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 2 {
            continue;
        }

        let index = match parts[0].trim().parse::<usize>() {
            Ok(idx) if idx > 0 && idx <= num_seats => idx - 1,
            Ok(idx) => {
                eprintln!(
                    "Error: Index {} out of bounds (must be 1-{}) in line: {}",
                    idx, num_seats, line
                );
                has_errors = true;
                continue;
            }
            Err(e) => {
                eprintln!(
                    "Error: Invalid index '{}' in line: {} ({})",
                    parts[0].trim(),
                    line,
                    e
                );
                has_errors = true;
                continue;
            }
        };

        let vis_role = match parse_role(parts[1]) {
            Ok(role) => role,
            Err(e) => {
                eprintln!(
                    "Error: Invalid visible role '{}' in line: {} ({})",
                    parts[1], line, e
                );
                has_errors = true;
                None
            }
        };
        visible[index] = vis_role;

        if parts.len() >= 3 {
            match parse_role(parts[2]) {
                Ok(role) => {
                    confirmed[index] = role;
                }
                Err(e) => {
                    eprintln!(
                        "Error: Invalid confirmed role '{}' in line: {} ({})",
                        parts[2], line, e
                    );
                    has_errors = true;
                }
            }
        }

        if parts.len() >= 4 && parts[3] != "" {
            if let Some(role) = vis_role {
                match role.parse_natural_statement(parts[3]) {
                    Ok(statement) => {
                        observed[index] = statement;
                    }
                    Err(e) => {
                        eprintln!(
                            "Error: Invalid statement '{}' for {:?} in line: {} ({})",
                            parts[3], role, line, e
                        );
                        has_errors = true;
                    }
                }
            }
        }
    }

    if has_errors {
        eprintln!("\nErrors were encountered in input. Exiting.");
        std::process::exit(1);
    }

    run_solver_and_print(
        &deck, &visible, &confirmed, &observed, villagers, outcasts, minions, demons, true,
    );
}

fn parse_count(s: &str, name: &str, line_num: usize) -> usize {
    s.parse().unwrap_or_else(|_| {
        eprintln!(
            "Invalid {} count on line {}: '{}' is not a valid number",
            name, line_num, s
        );
        0
    })
}

fn parse_role(s: &str) -> Result<Option<Role>, String> {
    let lower = s.trim().to_lowercase();
    match lower.as_str() {
        "?" => Ok(None),
        "" => Ok(None),
        "baker" => Ok(None),
        role_str => Role::from_str(role_str)
            .map(Some)
            .map_err(|e| format!("Failed to parse role '{}': {}", lower, e)),
    }
}

pub fn run_args(args: Vec<String>) {
    let (validate_mode, candidate, filtered_args) =
        if let Some(validate_pos) = args.iter().position(|x| x == "--validate") {
            if validate_pos + 1 >= args.len() {
                eprintln!("Error: --validate requires a candidate argument");
                std::process::exit(1);
            }

            let candidate_str = &args[validate_pos + 1];
            let candidate = parse_roles(candidate_str).unwrap_or_else(|e| {
                eprintln!("Failed to parse candidate roles: {}", e);
                std::process::exit(1);
            });

            let mut filtered_args = args.clone();
            filtered_args.drain(validate_pos..=validate_pos + 1);

            (true, Some(candidate), filtered_args)
        } else {
            (false, None, args)
        };

    let (deck, visible, confirmed, observed, villagers, outcasts, minions, demons) =
        match parse_input(&filtered_args) {
            Ok(parsed) => parsed,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

    if validate_mode {
        match candidate {
            Some(candidate) => {
                match validate_candidate(&candidate, &deck, &visible, &confirmed, &observed) {
                    Ok(_) => println!("{}", "Candidate is valid!".green()),
                    Err(reasons) => {
                        println!("{}", "Candidate is invalid:".red());
                        for reason in reasons {
                            println!("- {}", reason);
                        }
                    }
                }
            }
            None => {
                eprintln!("Error: No candidate provided for validation");
                std::process::exit(1);
            }
        }
    } else {
        run_solver_and_print(
            &deck, &visible, &confirmed, &observed, villagers, outcasts, minions, demons, false,
        );
    }
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
            "Usage: {} <deck> <villagers> <outcasts> <minions> <demons> [visible:confirmed:statement...]\nGot {} arguments",
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
    let outcasts = args[3].parse().map_err(|_| {
        format!(
            "Invalid minions count '{}': must be a positive integer",
            args[3]
        )
    })?;
    let minions = args[4].parse().map_err(|_| {
        format!(
            "Invalid demons count '{}': must be a positive integer",
            args[4]
        )
    })?;
    let demons = args[5].parse().map_err(|_| {
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
            parse_role(parts[0]).map_err(|e| {
                format!(
                    "Invalid visible role '{}' in argument {} ('{}'): {}",
                    parts[0], position, card_arg, e
                )
            })?
        };
        visible.push(role);

        // Parse confirmed role
        confirmed.push(if parts.len() <= 1 || parts[1].eq_ignore_ascii_case("?") {
            None
        } else {
            parse_role(parts[1]).map_err(|e| {
                format!(
                    "Invalid confirmed role '{}' in argument {} ('{}'): {}",
                    parts[1], position, card_arg, e
                )
            })?
        });

        // Parse statement
        observed.push(
            if parts.len() <= 2
                || parts[2].eq_ignore_ascii_case("?")
                || parts[2].eq_ignore_ascii_case("unrevealed")
            {
                RoleStatement::NoStatement
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
        deck, visible, confirmed, observed, villagers, outcasts, minions, demons,
    ))
}

fn run_solver_and_print(
    deck: &[Role],
    visible: &[Option<Role>],
    confirmed: &[Option<Role>],
    observed: &[RoleStatement],
    villagers: usize,
    outcasts: usize,
    minions: usize,
    demons: usize,
    print_statements: bool,
) {
    if print_statements {
        println!("Deck: {:?}", deck);
        println!(
            "Villagers: {}, Outcasts: {}, Minions: {}, Demons: {}",
            villagers, outcasts, minions, demons,
        );

        for i in 0..visible.len() {
            let vis = match visible[i] {
                Some(role) => {
                    let confirmed_part = match confirmed[i] {
                        Some(c_role) if c_role != role => format!(" ({:?})", c_role),
                        _ => String::new(),
                    };
                    format!("{:?}{}", role, confirmed_part)
                }
                None => "Unrevealed".to_string(),
            };

            println!("Player {}: {} - {}", i, vis, observed[i]);
        }
    }

    let sols = brute_force_solve(
        deck, visible, confirmed, observed, villagers, outcasts, minions, demons, false,
    );

    if sols.len() == 0 {
        println!("No solutions found.");
        _ = brute_force_solve(
            deck, visible, confirmed, observed, villagers, outcasts, minions, demons, true,
        );
        return;
    }

    println!("Found {} solution(s)", sols.len());

    if sols.len() < 25 {
        for s in &sols {
            let line: Vec<String> = s.iter().map(|role| color_by_alignment(*role)).collect();
            println!("{}", line.join(", "));
        }
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
        println!("{}: {}", i+1, line.join(", "));
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
