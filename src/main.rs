use demon_deduce::{brute_force_solve, Role};
use demon_deduce::roles::*;

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
    run_solver_and_print(&deck, &visible, &confirmed, &observed, villagers, minions, demons, outcasts);
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
    let deck = parse_roles(&args[1]).map_err(|_| "Failed to parse deck".to_string())?;

    // Convert counts from string to integer, fail fast if invalid
    let villagers = args[2].parse().map_err(|_| "Invalid villagers count".to_string())?;
    let minions = args[3].parse().map_err(|_| "Invalid minions count".to_string())?;
    let demons = args[4].parse().map_err(|_| "Invalid demons count".to_string())?;
    let outcasts = args[5].parse().map_err(|_| "Invalid outcasts count".to_string())?;

    let mut visible = Vec::new();
    let mut confirmed = Vec::new();
    let mut observed = Vec::new();

    for card_arg in &args[6..] {
        let parts: Vec<&str> = card_arg.split(':').collect();

        // Use None if input is "?" to represent unknown roles
        visible.push(if parts[0].eq_ignore_ascii_case("?") {
            None
        } else {
            Some(Role::from_str_case_insensitive(parts[0])?)
        });

        // Confirmed roles might not be provided, handle gracefully
        confirmed.push(if parts.len() == 1 || parts[1].eq_ignore_ascii_case("?") {
            None
        } else {
            Some(Role::from_str_case_insensitive(parts[1])?)
        });

        // Default to UnrevealedStatement if statement is missing or unknown
        observed.push(if parts.len() == 2
            || parts[2].eq_ignore_ascii_case("?")
            || parts[2].eq_ignore_ascii_case("unrevealed")
        {
            Box::new(UnrevealedStatement) as Box<dyn RoleStatement>
        } else {
            parse_statement_case_insensitive(parts[2])
        });
    }

    Ok((deck, visible, confirmed, observed, villagers, minions, demons, outcasts))
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
    let sols = brute_force_solve(deck, visible, confirmed, observed, villagers, minions, demons, outcasts);

    println!("Found {} solution(s)", sols.len());
    for s in sols {
        println!("{:?}", s);
    }
}

fn parse_roles(s: &str) -> Result<Vec<Role>, String> {
    s.split(',')
        .map(|r| {
            let trimmed = r.trim();
            Role::from_str_case_insensitive(trimmed)
        })
        .collect::<Result<Vec<_>, _>>() // stop at first error
}

fn parse_statement_case_insensitive(s: &str) -> Box<dyn RoleStatement> {
    let s_lower = s.to_ascii_lowercase();
    let s_clean = s_lower.trim();

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
    } else if s_clean.starts_with("claim[") {
        // Split claim statement into target and type
        let parts: Vec<&str> = s_clean.trim_start_matches("claim[").trim_end_matches(']').split(';').collect();
        if parts.len() != 2 {
            panic!("Invalid Claim statement format: {}", s);
        }
        let target_index = parts[0].parse().expect("Invalid target index");
        let claim_type = match parts[1].trim() {
            "good" => ClaimType::Good,
            "evil" => ClaimType::Evil,
            "truthy" => ClaimType::Truthy,
            "lying" => ClaimType::Lying,
            _ => panic!("Unknown claim type: {}", parts[1]),
        };
        Box::new(ClaimStatement { target_index, claim_type })
    } else if s_clean.starts_with("evilcount[") {
        let parts: Vec<&str> = s_clean.trim_start_matches("evilcount[").trim_end_matches(']').split(';').collect();
        if parts.len() < 2 {
            panic!("Invalid EvilCount statement format: {}", s);
        }

        // Allow multiple targets, parse optional fields with defaults
        let target_indexes = parts[0].split(',')
            .map(|s| s.trim().parse().expect("Invalid target index"))
            .collect();
        let evil_count = parts[1].trim().parse().expect("Invalid evil count");
        let minimum = if parts.len() > 2 { parts[2].trim().parse().unwrap_or(false) } else { false };
        let none_closer = if parts.len() > 3 { parts[3].trim().parse().unwrap_or(false) } else { false };

        Box::new(EvilCountStatement { target_indexes, evil_count, minimum, none_closer })
    } else if s_clean.starts_with("roleclaim[") {
        let parts: Vec<&str> = s_clean.trim_start_matches("roleclaim[").trim_end_matches(']').split(';').collect();
        if parts.len() != 2 {
            panic!("Invalid RoleClaim statement format: {}", s);
        }
        let target_index = parts[0].trim().parse().expect("Invalid target index");
        let role = Role::from_str_case_insensitive(parts[1].trim()).expect("Invalid target role");
        Box::new(RoleClaimStatement { target_index, role })
    } else if s_clean.starts_with("roledistance[") {
        let parts: Vec<&str> = s_clean.trim_start_matches("roledistance[").trim_end_matches(']').split(';').collect();
        if parts.len() != 2 {
            panic!("Invalid RoleDistance statement format: {}", s);
        }
        let role = Role::from_str_case_insensitive(parts[0].trim()).expect("Invalid target role");
        let distance = parts[1].trim().parse().expect("Invalid distance");
        Box::new(RoleDistanceStatement { role, distance })
    } else {
        panic!("Unknown statement type: {}", s)
    }
}

trait CaseInsensitiveFromStr {
    fn from_str_case_insensitive(s: &str) -> Result<Self, String> where Self: Sized;
}

impl CaseInsensitiveFromStr for Role {
    fn from_str_case_insensitive(s: &str) -> Result<Self, String> {
        let s_lower = s.to_ascii_lowercase();
        match s_lower.as_str() {
            // Map string to enum, fail if unknown
            "confessor" => Ok(Role::Confessor),
            "empress" => Ok(Role::Empress),
            "enlightened" => Ok(Role::Enlightened),
            "gemcrafter" => Ok(Role::Gemcrafter),
            "hunter" => Ok(Role::Hunter),
            "jester" => Ok(Role::Jester),
            "judge" => Ok(Role::Judge),
            "knight" => Ok(Role::Knight),
            "lover" => Ok(Role::Lover),
            "medium" => Ok(Role::Medium),
            "scout" => Ok(Role::Scout),
            "wretch" => Ok(Role::Wretch),
            "bombardier" => Ok(Role::Bombardier),
            "baa" => Ok(Role::Baa),
            "minion" => Ok(Role::Minion),
            "twinminion" => Ok(Role::TwinMinion),
            "witch" => Ok(Role::Witch),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}
