use colored::*;
use demon_deduce::roles::*;
use demon_deduce::{brute_force_solve, Role};
use std::panic::Location;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (deck, visible, confirmed, observed, villagers, minions, demons, outcasts) =
        match parse_input(&args) {
            Ok(parsed) => parsed,
            Err(e) => {
                println!("{}", e);
                return; // Stop execution if input is invalid
            }
        };

    // Run the solver with parsed input
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
        Vec<Box<dyn RoleStatement>>,
        usize,
        usize,
        usize,
        usize,
    ),
    String,
> {
    if args.len() < 6 {
        return Err(format!(
            "Usage: {} <deck> <villagers> <minions> <demons> <outcasts> [visible:confirmed:statement...]",
            args[0]
        ));
    }

    // Parsing comma-separated roles into structured Role enum
    let deck = parse_roles(&args[1]).expect_alt("Failed to parse deck");

    // Convert counts from string to integer, fail fast if invalid
    let villagers = args[2].parse().expect_alt("Invalid villagers count");
    let minions = args[3].parse().expect_alt("Invalid minions count");
    let demons = args[4].parse().expect_alt("Invalid demons count");
    let outcasts = args[5].parse().expect_alt("Invalid outcasts count");

    let mut visible = Vec::new();
    let mut confirmed = Vec::new();
    let mut observed = Vec::new();

    for card_arg in &args[6..] {
        let parts: Vec<&str> = card_arg.split(':').collect();

        // Use None if input is "?" to represent unknown roles
        visible.push(if parts[0].eq_ignore_ascii_case("?") {
            None
        } else {
            Some(Role::from_str(parts[0]).expect_alt("Failed to parse visible"))
        });

        // Confirmed roles might not be provided, handle gracefully
        confirmed.push(if parts.len() <= 1 || parts[1].eq_ignore_ascii_case("?") {
            None
        } else {
            Some(Role::from_str(parts[1]).expect_alt("Failed to parse confirmed"))
        });

        // Default to UnrevealedStatement if statement is missing or unknown
        observed.push(
            if parts.len() <= 2
                || parts[2].eq_ignore_ascii_case("?")
                || parts[2].eq_ignore_ascii_case("unrevealed")
            {
                Box::new(UnrevealedStatement) as Box<dyn RoleStatement>
            } else {
                parse_statement_case_insensitive(parts[2])
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
    observed: &[Box<dyn RoleStatement>],
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

fn parse_roles(s: &str) -> Result<Vec<Role>, strum::ParseError> {
    s.split(',')
        .map(|r| {
            let trimmed = r.trim();
            Role::from_str(trimmed)
        })
        .collect::<Result<Vec<_>, _>>() // stop at first error
}
fn parse_statement_case_insensitive(s: &str) -> Box<dyn RoleStatement> {
    let s_lower = s.to_ascii_lowercase();
    let s_clean = s_lower.trim();

    fn parse_target_indexes(s: &str) -> Vec<usize> {
        s.split(',')
            .map(|s| s.trim().parse().expect_alt("Invalid target index"))
            .collect()
    }

    // Determine type of statement based on prefix
    if s_clean.starts_with("clockwise") {
        Box::new(EnlightenedStatement::Clockwise)
    } else if s_clean.starts_with("counterclockwise") {
        Box::new(EnlightenedStatement::CounterClockwise)
    } else if s_clean.starts_with("equidistant") {
        Box::new(EnlightenedStatement::Equidistant)
    } else if s_clean.starts_with("iamgood") {
        Box::new(ConfessorStatement::IAmGood)
    } else if s_clean.starts_with("iamdizzy") {
        Box::new(ConfessorStatement::IAmDizzy)
    } else if s_clean.starts_with("bard[") {
        let content = s_clean.trim_start_matches("bard[").trim_end_matches(']');
        let distance = if content == "none" {
            None
        } else {
            Some(
                content
                    .parse()
                    .expect_alt("Invalid distance for BardStatement"),
            )
        };
        Box::new(BardStatement { distance })
    } else if s_clean.starts_with("empress[") {
        let content = s_clean.trim_start_matches("empress[").trim_end_matches(']');
        let target_indexes = parse_target_indexes(content);
        Box::new(EmpressStatement { target_indexes })
    } else if s_clean.starts_with("gemcrafter[") {
        let content = s_clean
            .trim_start_matches("gemcrafter[")
            .trim_end_matches(']');
        let target_index = content.parse().expect_alt("Invalid target index");
        Box::new(GemcrafterStatement { target_index })
    } else if s_clean.starts_with("hunter[") {
        let content = s_clean.trim_start_matches("hunter[").trim_end_matches(']');
        let distance = content.parse().expect_alt("Invalid distance");
        Box::new(HunterStatement { distance })
    } else if s_clean.starts_with("jester[") {
        let parts: Vec<&str> = s_clean
            .trim_start_matches("jester[")
            .trim_end_matches(']')
            .split(';')
            .collect();
        if parts.len() != 2 {
            panic!("Invalid Jester statement format: {}", s);
        }
        let target_indexes = parse_target_indexes(parts[0]);
        let evil_count = parts[1].trim().parse().expect_alt("Invalid evil count");
        Box::new(JesterStatement {
            target_indexes,
            evil_count,
        })
    } else if s_clean.starts_with("judge[") {
        let parts: Vec<&str> = s_clean
            .trim_start_matches("judge[")
            .trim_end_matches(']')
            .split(';')
            .collect();
        if parts.len() != 2 {
            panic!("Invalid Judge statement format: {}", s);
        }
        let target_index = parts[0].parse().expect_alt("Invalid target index");
        let is_lying = match parts[1].trim() {
            "truthy" => false,
            "lying" => true,
            _ => panic!("Unknown claim type: {}", parts[1]),
        };
        Box::new(JudgeStatement {
            target_index,
            is_lying,
        })
    } else if s_clean.starts_with("lover[") {
        let content = s_clean.trim_start_matches("lover[").trim_end_matches(']');
        let evil_count = content.parse().expect_alt("Invalid evil count");
        Box::new(LoverStatement { evil_count })
    } else if s_clean.starts_with("medium[") {
        let parts: Vec<&str> = s_clean
            .trim_start_matches("medium[")
            .trim_end_matches(']')
            .split(';')
            .collect();
        if parts.len() != 2 {
            panic!("Invalid Medium statement format: {}", s);
        }
        let target_index = parts[0].trim().parse().expect_alt("Invalid target index");
        let role: Role = parts[1].trim().parse().expect_alt("Invalid target role");
        Box::new(MediumStatement { target_index, role })
    } else if s_clean.starts_with("scout[") {
        let parts: Vec<&str> = s_clean
            .trim_start_matches("scout[")
            .trim_end_matches(']')
            .split(';')
            .collect();
        if parts.len() != 2 {
            panic!("Invalid Scout statement format: {}", s);
        }
        let role: Role = parts[0].trim().parse().expect_alt("Invalid target role");
        let distance = parts[1].trim().parse().expect_alt("Invalid distance");
        Box::new(ScoutStatement { role, distance })
    } else if s_clean.starts_with("slayer[") {
        let parts: Vec<&str> = s_clean
            .trim_start_matches("slayer[")
            .trim_end_matches(']')
            .split(';')
            .collect();
        if parts.len() != 2 {
            panic!("Invalid Slayer statement format: {}", s);
        }
        let target_index = parts[0].parse().expect_alt("Invalid target index");
        let alignment = match parts[1].trim() {
            "good" => Alignment::Good,
            "evil" => Alignment::Evil,
            _ => panic!("Unknown alignment: {}", parts[1]),
        };
        Box::new(SlayerStatement {
            target_index,
            alignment,
        })
    } else if s_clean == "unrevealed" {
        Box::new(UnrevealedStatement)
    } else {
        panic!("Unknown statement type: {}", s)
    }
}

trait ExpectAlt<T> {
    fn expect_alt(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Display> ExpectAlt<T> for Result<T, E> {
    #[track_caller]
    fn expect_alt(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                let location = Location::caller();
                panic!(
                    "{}: {}\nCalled from: {}:{}",
                    msg,
                    e,
                    location.file(),
                    location.line()
                );
            }
        }
    }
}
