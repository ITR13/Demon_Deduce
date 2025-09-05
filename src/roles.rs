use bitvec::prelude::*;
use std::fmt;
use std::str::FromStr;
use strum_macros::{Display, EnumIter, EnumString};

type TargetIndexes = BitArray<[u8; 2], Lsb0>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    // Villager
    Alchemist,
    Architect,
    Baker,
    #[strum(serialize = "bard", serialize = "athlete")]
    Bard,
    Bishop,
    Confessor,
    Dreamer,
    Druid,
    Empress,
    Enlightened,
    #[strum(
        serialize = "fortuneteller",
        serialize = "fortune teller",
        serialize = "fortune"
    )]
    FortuneTeller,
    #[strum(serialize = "gemcrafter", serialize = "archivist")]
    Gemcrafter,
    Hunter,
    Jester,
    Judge,
    Knight,
    Knitter,
    Lover,
    #[strum(serialize = "medium", serialize = "lookout")]
    Medium,
    Oracle,
    #[strum(serialize = "poet", serialize = "gossip")]
    Poet,
    Scout,
    #[strum(serialize = "slayer", serialize = "gambler")]
    Slayer,
    Witness,
    // Outcast
    Bombardier,
    #[strum(serialize = "doppelganger", serialize = "doppleganger")]
    DoppelGanger,
    Drunk,
    #[strum(
        serialize = "plaguedoctor",
        serialize = "plague doctor",
        serialize = "plague"
    )]
    PlagueDoctor,
    Wretch,
    // Minion
    #[strum(serialize = "counsellor", serialize = "baron")]
    Counsellor,
    Minion,
    Poisoner,
    #[strum(serialize = "puppet", serialize = "marionette")]
    Puppet,
    #[strum(serialize = "puppeteer", serialize = "mezepheles")]
    Puppeteer,
    #[strum(
        serialize = "twinminion",
        serialize = "twin minion",
        serialize = "twin"
    )]
    TwinMinion,
    Witch,
    // Demon
    #[strum(serialize = "baa", serialize = "imp")]
    Baa,
    #[strum(serialize = "lilis", serialize = "lillith")]
    Lilis,
    Pooka,
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    Villager,
    Outcast,
    Minion,
    Demon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Good,
    Evil,
}

impl Role {
    pub const fn group(self) -> Group {
        use Role::*;
        match self {
            Alchemist | Architect | Baker | Bard | Bishop | Confessor | Dreamer | Druid
            | Empress | Enlightened | FortuneTeller | Gemcrafter | Hunter | Jester | Judge
            | Knight | Knitter | Lover | Medium | Oracle | Poet | Scout | Slayer | Witness => {
                Group::Villager
            }
            Bombardier | DoppelGanger | PlagueDoctor | Wretch | Drunk => Group::Outcast,
            Counsellor | Minion | Poisoner | Puppet | Puppeteer | TwinMinion | Witch => {
                Group::Minion
            }
            Baa | Lilis | Pooka => Group::Demon,
        }
    }
    pub const fn alignment(self) -> Alignment {
        use Role::*;
        match self {
            Alchemist | Architect | Baker | Bard | Bishop | Confessor | Dreamer | Druid | Drunk
            | Empress | Enlightened | FortuneTeller | Gemcrafter | Hunter | Jester | Judge
            | Knight | Knitter | Lover | Medium | Oracle | Poet | Scout | Slayer | Bombardier
            | DoppelGanger | PlagueDoctor | Witness | Wretch => Alignment::Good,
            Baa | Counsellor | Lilis | Minion | Poisoner | Pooka | Puppet | Puppeteer
            | TwinMinion | Witch => Alignment::Evil,
        }
    }
    pub const fn lying(self) -> bool {
        use Role::*;
        match self {
            Alchemist | Architect | Baker | Bard | Bishop | Confessor | Dreamer | Druid
            | Empress | Enlightened | FortuneTeller | Gemcrafter | Hunter | Jester | Judge
            | Knight | Knitter | Lover | Medium | Oracle | Poet | Puppet | Scout | Slayer
            | Bombardier | DoppelGanger | PlagueDoctor | Witness | Wretch => false,
            Baa | Counsellor | Drunk | Lilis | Minion | Poisoner | Pooka | Puppeteer
            | TwinMinion | Witch => true,
        }
    }
    pub fn parse_statement(&self, s: &str) -> Result<RoleStatement, String> {
        fn parse_indexes(s: &str) -> Result<TargetIndexes, String> {
            let mut bits = TargetIndexes::default();

            for (i, idx_str) in s.split(',').enumerate() {
                let idx_str = idx_str.trim();
                let idx: usize = idx_str.parse().map_err(|_| {
                    format!(
                        "Invalid index '{}' at position {} in '{}'",
                        idx_str,
                        i + 1,
                        s
                    )
                })?;
                bits.set(idx, true);
            }

            Ok(bits)
        }

        match self {
            Role::Alchemist => {
                let corrupt_count = s.trim().parse().map_err(|_| {
                    format!("Invalid corrupt count '{}' for Alchemist", s)
                })?;
                Ok(AlchemistStatement { corrupt_count }.into())
            }
            Role::Architect => match s.trim() {
                "right" => Ok(ArchitectStatement::Right.into()),
                "left" => Ok(ArchitectStatement::Left.into()),
                "equal" => Ok(ArchitectStatement::Equal.into()),
                _ => Err(format!(
                    "Invalid Architect statement '{}' - expected 'left', 'right', or 'equal'",
                    s
                )),
            }
            Role::Bard => {
                let distance = if s.trim() == "none" {
                    None
                } else {
                    Some(s.trim().parse().map_err(|_| {
                        format!("Invalid distance '{}' for Bard - expected 'none' or a number", s)
                    })?)
                };
                Ok(BardStatement { distance }.into())
            }
            Role::Bishop => {
                let target_indexes = parse_indexes(s)?;
                Ok(BishopStatement { target_indexes }.into())
            }
            Role::Confessor => match s.trim() {
                "iamgood" => Ok(ConfessorStatement::IAmGood.into()),
                "iamdizzy" => Ok(ConfessorStatement::IAmDizzy.into()),
                _ => Err(format!(
                    "Invalid Confessor statement '{}' - expected 'iamgood' or 'iamdizzy'",
                    s
                )),
            },
            Role::Dreamer => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Dreamer statement '{}' - expected format 'target_indexes;role'",
                        s
                    ));
                }
                let target_index = s.trim().parse().map_err(|_| {
                    format!("Invalid target index '{}' for Dreamer", s)
                })?;
                let role: Role = parts[1].trim().to_lowercase().parse().map_err(|e| {
                    format!(
                        "Invalid target role '{}' in Dreamer statement: {}",
                        parts[1], e
                    )
                })?;
                Ok(DreamerStatement { target_index, role }.into())
            }
            Role::Druid => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Druid statement '{}' - expected format 'target_indexes;role'",
                        s
                    ));
                }
                let target_indexes = parse_indexes(parts[0])?;
                let role: Role = parts[1].trim().to_lowercase().parse().map_err(|e| {
                    format!(
                        "Invalid target role '{}' in Druid statement: {}",
                        parts[1], e
                    )
                })?;
                Ok(DruidStatement { target_indexes, role: Some(role) }.into())
            }
            Role::Empress => {
                let target_indexes = parse_indexes(s)?;
                Ok(EmpressStatement { target_indexes }.into())
            }
            Role::Enlightened => match s.trim() {
                "clockwise" => Ok(EnlightenedStatement::Clockwise.into()),
                "counterclockwise" => Ok(EnlightenedStatement::CounterClockwise.into()),
                "equidistant" => Ok(EnlightenedStatement::Equidistant.into()),
                _ => Err(format!(
                    "Invalid Enlightened statement '{}' - expected 'clockwise', 'counterclockwise', or 'equidistant'",
                    s
                )),
            },
            Role::FortuneTeller => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Jester statement '{}' - expected format 'indexes;evil'",
                        s
                    ));
                }
                let target_indexes = parse_indexes(parts[0])?;
                let is_evil: bool = parts[1].trim().parse().map_err(|_|{
                    format!("Invalid bool '{}' for FortuneTeller", parts[1])
                })?;
                Ok(FortuneTellerStatement {
                    target_indexes,
                    is_evil,
                }.into())
            }
            Role::Gemcrafter => {
                let target_index = s.trim().parse().map_err(|_| {
                    format!("Invalid target index '{}' for Gemcrafter", s)
                })?;
                Ok(GemcrafterStatement { target_index }.into())
            }
            Role::Hunter => {
                let distance = s.trim().parse().map_err(|_| {
                    format!("Invalid distance '{}' for Hunter", s)
                })?;
                Ok(HunterStatement { distance }.into())
            }
            Role::Jester => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Jester statement '{}' - expected format 'indexes;evil_count'",
                        s
                    ));
                }
                let target_indexes = parse_indexes(parts[0])?;
                let evil_count = parts[1].trim().parse().map_err(|_| {
                    format!("Invalid evil count '{}' in Jester statement", parts[1])
                })?;
                Ok(JesterStatement {
                    target_indexes,
                    evil_count,
                }.into())
            }
            Role::Judge => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Judge statement '{}' - expected format 'target_index;truthy|lying'",
                        s
                    ));
                }
                let target_index = parts[0].trim().parse().map_err(|_| {
                    format!("Invalid target index '{}' in Judge statement", parts[0])
                })?;
                let is_lying = match parts[1].trim() {
                    "truthy" => false,
                    "lying" => true,
                    _ => {
                        return Err(format!(
                            "Invalid claim type '{}' in Judge statement - expected 'truthy' or 'lying'",
                            parts[1]
                        ))
                    }
                };
                Ok(JudgeStatement {
                    target_index,
                    is_lying,
                }.into())
            }
            Role::Knitter => {
                let adjacent_count = s.trim().parse().map_err(|_| {
                    format!("Invalid adjacent count '{}' for Knitter", s)
                })?;
                Ok(KnitterStatement { adjacent_count }.into())
            }
            Role::Lover => {
                let evil_count = s.trim().parse().map_err(|_| {
                    format!("Invalid evil count '{}' for Lover", s)
                })?;
                Ok(LoverStatement { evil_count }.into())
            }
            Role::Medium => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Medium statement '{}' - expected format 'target_index;role'",
                        s
                    ));
                }
                let target_index = parts[0].trim().parse().map_err(|_| {
                    format!("Invalid target index '{}' in Medium statement", parts[0])
                })?;
                let role: Role = parts[1].trim().parse().map_err(|e| {
                    format!(
                        "Invalid target role '{}' in Medium statement: {}",
                        parts[1], e
                    )
                })?;
                Ok(MediumStatement { target_index, role }.into())
            }
            Role::Oracle => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Oracle statement '{}' - expected format 'target_indexes;role'",
                        s
                    ));
                }
                let target_indexes = parse_indexes(parts[0])?;
                let role: Role = parts[1].trim().to_lowercase().parse().map_err(|e| {
                    format!(
                        "Invalid target role '{}' in Oracle statement: {}",
                        parts[1], e
                    )
                })?;
                Ok(OracleStatement { target_indexes, role }.into())
            }
            Role::Poet => Ok(RoleStatement::NoStatement),
            Role::Scout => {
                if s.to_lowercase() == "none" {
                    return Ok(ScoutStatement {role: None, distance:0}.into());
                }

                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Scout statement '{}' - expected format 'role;distance'",
                        s
                    ));
                }
                let role: Role = parts[0].trim().to_lowercase().parse().map_err(|e| {
                    format!("Invalid role '{}' in Scout statement: {}", parts[0], e)
                })?;
                let distance = parts[1].trim().parse().map_err(|_| {
                    format!("Invalid distance '{}' in Scout statement", parts[1])
                })?;
                Ok(ScoutStatement { role: Some(role), distance }.into())
            }
            Role::Slayer => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "Invalid Slayer statement '{}' - expected format 'target_index;good|evil'",
                        s
                    ));
                }
                let target_index = parts[0].trim().parse().map_err(|_| {
                    format!("Invalid target index '{}' in Slayer statement", parts[0])
                })?;
                let alignment = match parts[1].trim() {
                    "good" => Alignment::Good,
                    "evil" => Alignment::Evil,
                    _ => {
                        return Err(format!(
                            "Invalid alignment '{}' in Slayer statement - expected 'good' or 'evil'",
                            parts[1]
                        ))
                    }
                };
                Ok(SlayerStatement {
                    target_index,
                    alignment,
                }.into())
            }
            Role::PlagueDoctor => {
                let target_indexes: Vec<usize> = s.split(';').map(|sv| sv.parse().map_err(|_| {
                    format!("Invalid target index in PlagueDoctor statement")
                })).collect::<Result<Vec<_>, _>>()?;
                match target_indexes.len() {
                    1 => Ok(PlagueDoctorStatement {
                        corruption_index: target_indexes[0],
                        evil_index: None,
                    }.into()),
                    2 => Ok(PlagueDoctorStatement {
                        corruption_index: target_indexes[1],
                        evil_index: Some(target_indexes[0]),
                    }.into()),
                    _ => Err(format!(
                        "PlagueDoctor must have 1 or 2 target indexes, got {} in '{}'",
                        target_indexes.len(),
                        s
                    )),
                }
            }
            Role::Baker
            | Role::Bombardier
            | Role::Knight
            | Role::DoppelGanger
            | Role::Drunk
            | Role::Wretch
            | Role::Baa
            | Role::Lilis
            | Role::Minion
            | Role::Poisoner
            | Role::Pooka
            | Role::Puppet
            | Role::Puppeteer
            | Role::TwinMinion
            | Role::Witch
            | Role::Counsellor
            | Role::Witness => Err(format!(
                "No statement parsing implemented for {:?}",
                self
            )),
        }
    }
    pub fn parse_natural_statement(&self, s: &str) -> Result<RoleStatement, String> {
        match self {
            Role::Alchemist => {
                if let Some(caps) = regex::Regex::new(r"I cured (\d+) Corruptions?")
                    .unwrap()
                    .captures(s)
                {
                    let corrupt_count = caps[1].parse().map_err(|_| {
                        format!("Invalid corrupt count in Alchemist statement '{}'", s)
                    })?;
                    Ok(AlchemistStatement { corrupt_count }.into())
                } else {
                    Err(format!("Invalid Alchemist statement '{}'", s))
                }
            }
            Role::Architect => {
                let s = s.to_lowercase();
                if let Some(caps) = regex::Regex::new(r"(left|right|equal)")
                    .unwrap()
                    .captures(&s)
                {
                    match caps[1].trim() {
                        "right" => Ok(ArchitectStatement::Right.into()),
                        "left" => Ok(ArchitectStatement::Left.into()),
                        "equal" => Ok(ArchitectStatement::Equal.into()),
                        _ => Err(format!(
                            "Invalid Architect statement '{}' - expected 'left', 'right', or 'equal'",
                            s
                        )),
                    }
                } else {
                    Err(format!("invalid bard statement '{}'", s))
                }
            }
            Role::Baker => Ok(RoleStatement::NoStatement),
            Role::Bard => {
                let s = s.to_lowercase();
                if let Some(caps) = regex::Regex::new(r"i am (\d+) cards?")
                    .unwrap()
                    .captures(&s)
                {
                    let distance = caps[1]
                        .parse()
                        .map_err(|_| format!("invalid distance in bard statement '{}'", s))?;
                    Ok(BardStatement {
                        distance: Some(distance),
                    }
                    .into())
                } else if s.trim() == "there are no corrupted characters" {
                    Ok(BardStatement { distance: None }.into())
                } else {
                    Err(format!("invalid bard statement '{}' - expected format like 'i am 2 cards away from corrupted' or 'none'", s))
                }
            }
            Role::Bishop => {
                if let Some(caps) = regex::Regex::new(r"#(\d+)[^#]+#(\d+)[^#]+(\d+)?")
                    .unwrap()
                    .captures(s)
                {
                    let mut target_indexes = Vec::new();
                    for i in 1..=3 {
                        if let Some(m) = caps.get(i) {
                            let idx = m.as_str().parse::<usize>().map_err(|_| {
                                format!("Invalid index in Bishop statement '{}'", s)
                            })?;
                            target_indexes.push(idx - 1);
                        }
                    }
                    if target_indexes.is_empty() {
                        return Err(format!(
                            "No valid indexes found in Bishop statement '{}'",
                            s
                        ));
                    }
                    let mut bits = TargetIndexes::default();
                    for idx in target_indexes {
                        bits.set(idx, true);
                    }

                    Ok(BishopStatement {
                        target_indexes: bits,
                    }
                    .into())
                } else {
                    Err(format!(
                        "Invalid Bishop statement '{}' - expected format like #8 #1 #7'",
                        s
                    ))
                }
            }
            Role::Confessor => {
                let s = s.trim().to_lowercase();
                match s.as_str() {
                "i am dizzy" | "i'm dizzy" | "iamdizzy" => Ok(ConfessorStatement::IAmDizzy.into()),
                "i am good" | "i'm good" | "iamgood" => Ok(ConfessorStatement::IAmGood.into()),
                _ => Err(format!(
                    "Invalid Confessor statement '{}' - expected something like 'I am dizzy' or 'I am good'",
                    s
                )),
            }
            }
            Role::Medium => {
                if let Some(caps) = regex::Regex::new(r"#(\d+)\s+is\s+a\s+real\s+(\w+)")
                    .unwrap()
                    .captures(s)
                {
                    let target_index = caps[1]
                        .parse::<usize>()
                        .map_err(|_| format!("Invalid target index in Medium statement '{}'", s))?;
                    let role = Role::from_str(&caps[2].to_lowercase())
                        .map_err(|_| format!("Invalid role '{}' in Medium statement", &caps[2]))?;
                    Ok(MediumStatement {
                        target_index: target_index - 1,
                        role,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Medium statement '{}' - expected format like '#4 is a real Hunter'", s))
                }
            }
            Role::Gemcrafter => {
                if let Some(caps) = regex::Regex::new(r"#(\d+)\s+is\s+Good")
                    .unwrap()
                    .captures(s)
                {
                    let target_index = caps[1].parse::<usize>().map_err(|_| {
                        format!("Invalid target index in Gemcrafter statement '{}'", s)
                    })?;
                    Ok(GemcrafterStatement {
                        target_index: target_index - 1,
                    }
                    .into())
                } else {
                    Err(format!(
                        "Invalid Gemcrafter statement '{}' - expected format like '#5 is Good'",
                        s
                    ))
                }
            }
            Role::Hunter => {
                if let Some(caps) =
                    regex::Regex::new(r"I am (\d+)\s*(?:cards?)?\s*away from closest Evil")
                        .unwrap()
                        .captures(s)
                {
                    let distance = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid distance in Hunter statement '{}'", s))?;
                    Ok(HunterStatement { distance }.into())
                } else {
                    Err(format!("Invalid Hunter statement '{}' - expe|ted format like 'I am 2 cards away from closest Evil'", s))
                }
            }
            Role::Enlightened => {
                if let Some(caps) = regex::Regex::new(
                    r"Closest Evil is:?\s*(Clockwise|Counter-clockwise|equidistant)",
                )
                .unwrap()
                .captures(s)
                {
                    match &caps[1] {
                        "Clockwise" => Ok(EnlightenedStatement::Clockwise.into()),
                        "Counter-clockwise" => Ok(EnlightenedStatement::CounterClockwise.into()),
                        "equidistant" => Ok(EnlightenedStatement::Equidistant.into()),
                        _ => Err(format!("Invalid Enlightened statement '{}'", s)),
                    }
                } else {
                    Err(format!("Invalid Enlightened statement '{}' - expected format like 'Closest Evil is: Clockwise'", s))
                }
            }
            Role::Judge => {
                if let Some(caps) = regex::Regex::new(r"#(\d+).*(Truth|Lying)")
                    .unwrap()
                    .captures(s)
                {
                    let target_index = caps[1]
                        .parse::<usize>()
                        .map_err(|_| format!("Invalid target index in Judge statement '{}'", s))?;
                    let is_lying = match &caps[2] {
                        "Truth" => false,
                        "Lying" => true,
                        _ => return Err(format!("Invalid claim type in Judge statement '{}'", s)),
                    };
                    Ok(JudgeStatement {
                        target_index: target_index - 1,
                        is_lying,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Judge statement '{}'", s))
                }
            }
            Role::Empress => {
                if let Some(caps) =
                    regex::Regex::new(r"One is Evil:\s*#(\d+)(?:\s*,\s*#(\d+))?(?:\s*or\s*#(\d+))?")
                        .unwrap()
                        .captures(s)
                {
                    let mut indexes = Vec::new();
                    for i in 1..=3 {
                        if let Some(m) = caps.get(i) {
                            let idx = m.as_str().parse::<usize>().map_err(|_| {
                                format!("Invalid index in Empress statement '{}'", s)
                            })?;
                            indexes.push(idx - 1);
                        }
                    }
                    if indexes.is_empty() {
                        return Err(format!(
                            "No valid indexes found in Empress statement '{}'",
                            s
                        ));
                    }
                    let mut bits = TargetIndexes::default();
                    for idx in indexes {
                        bits.set(idx, true);
                    }
                    Ok(EmpressStatement {
                        target_indexes: bits,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Empress statement '{}' - expected format like 'One is Evil: #8, #1 or #7'", s))
                }
            }
            Role::Lover => {
                if s.trim().eq_ignore_ascii_case("NO Evils adjacent to me") {
                    Ok(LoverStatement { evil_count: 0 }.into())
                } else if let Some(caps) = regex::Regex::new(r"(\d+)\s*Evils?\s*adjacent to me")
                    .unwrap()
                    .captures(s)
                {
                    let evil_count = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid evil count in Lover statement '{}'", s))?;
                    Ok(LoverStatement { evil_count }.into())
                } else {
                    Err(format!("Invalid Lover statement '{}' - expected format like 'NO Evils adjacent to me' or '2 Evils adjacent to me'", s))
                }
            }
            Role::Scout => {
                if s == "There is only 1 Evil" {
                    Ok(ScoutStatement {
                        role: None,
                        distance: 0,
                    }
                    .into())
                } else if let Some(caps) = regex::Regex::new(r"(\w+).*(\d+)").unwrap().captures(s) {
                    let role = Role::from_str(&caps[1].to_lowercase())
                        .map_err(|_| format!("Invalid role '{}' in Scout statement", &caps[1]))?;
                    let distance = caps[2]
                        .parse()
                        .map_err(|_| format!("Invalid distance in Scout statement '{}'", s))?;
                    Ok(ScoutStatement {
                        role: Some(role),
                        distance,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Scout statement '{}' - expected format like 'Minion is 1 card away from closest Evil'", s))
                }
            }
            Role::Knitter => {
                if s.trim()
                    .eq_ignore_ascii_case("Evils are not adjacent to eachother")
                {
                    Ok(KnitterStatement { adjacent_count: 0 }.into())
                } else if let Some(caps) = regex::Regex::new(r"(\d+)").unwrap().captures(s) {
                    let adjacent_count = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid evil count in Knitter statement '{}'", s))?;
                    Ok(KnitterStatement { adjacent_count }.into())
                } else {
                    Err(format!(
                        "Invalid Knitter statement '{}' - expected format",
                        s
                    ))
                }
            }
            Role::PlagueDoctor => {
                let s = s.to_lowercase();
                if let Some(caps) = regex::Regex::new(r"#(\d+)[^#]+(?:#(\d+))?")
                    .unwrap()
                    .captures(&s)
                {
                    let (corruption_index, evil_index) = match caps.get(2) {
                        Some(_) => {
                            // If there's a second part, then:
                            // - corruption_index is the second part
                            // - evil_index is the first part
                            let corruption_index: usize = caps[2].parse().map_err(|_| {
                                format!("invalid second index in plague doctor statement '{}'", s)
                            })?;
                            let corruption_index = corruption_index - 1;

                            let evil_index: usize = caps[1]
                                .parse::<usize>()
                                .map_err(|_| {
                                    format!(
                                        "invalid first index in plague doctor statement '{}'",
                                        s
                                    )
                                })
                                .map(|x| x - 1)?;

                            (corruption_index, Some(evil_index))
                        }
                        None => {
                            // If there's only one part, it's corruption_index and evil_index is None
                            let corruption_index: usize = caps[1].parse().map_err(|_| {
                                format!("invalid index in plague doctor statement '{}'", s)
                            })?;
                            let corruption_index = corruption_index - 1;

                            (corruption_index, None)
                        }
                    };
                    Ok(PlagueDoctorStatement {
                        corruption_index,
                        evil_index,
                    }
                    .into())
                } else {
                    Err(format!("invalid plague doctor statement '{}'", s))
                }
            }
            Role::FortuneTeller => {
                if let Some(caps) = regex::Regex::new(r"#(\d+).*#(\d+).*(True|False)")
                    .unwrap()
                    .captures(s)
                {
                    let first: usize = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid index in FortuneTeller statement '{}'", s))?;
                    let second: usize = caps[2]
                        .parse()
                        .map_err(|_| format!("Invalid index in FortuneTeller statement '{}'", s))?;
                    let is_evil = caps[3] == *"True";
                    let target_indexes = to_bitvec(vec![first - 1, second - 1]);
                    Ok(FortuneTellerStatement {
                        target_indexes,
                        is_evil,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Scout statement '{}'", s))
                }
            }
            Role::Jester => {
                if let Some(caps) = regex::Regex::new(r"#(\d+).*#(\d+).*#(\d+).*(\d+) Evils?")
                    .unwrap()
                    .captures(s)
                {
                    let mut indexes = Vec::new();
                    for i in 1..=3 {
                        if let Some(m) = caps.get(i) {
                            let idx: usize = m.as_str().parse().map_err(|_| {
                                format!("Invalid index in Empress statement '{}'", s)
                            })?;
                            indexes.push(idx - 1);
                        }
                    }
                    let target_indexes = to_bitvec(indexes);
                    let evil_count: usize = caps[4]
                        .parse()
                        .map_err(|_| format!("Invalid index in Empress statement '{}'", s))?;
                    Ok(JesterStatement {
                        target_indexes,
                        evil_count,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Empress statement '{}' - expected format like 'One is Evil: #8, #1 or #7'", s))
                }
            }
            Role::Oracle => {
                if let Some(caps) = regex::Regex::new(r"#(\d+) or #(\d+) is a (\w+)")
                    .unwrap()
                    .captures(s)
                {
                    let mut indexes = Vec::new();
                    for i in 1..=2 {
                        if let Some(m) = caps.get(i) {
                            let idx: usize = m.as_str().parse().map_err(|_| {
                                format!("Invalid index in Oracle statement '{}'", s)
                            })?;
                            indexes.push(idx - 1);
                        }
                    }
                    let target_indexes = to_bitvec(indexes);
                    let role: Role = caps[3].trim().to_lowercase().parse().map_err(|e| {
                        format!(
                            "Invalid target role '{}' in Oracle statement: {}",
                            &caps[3], e
                        )
                    })?;

                    Ok(OracleStatement {
                        target_indexes,
                        role,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Oracle statement '{}'", s))
                }
            }
            Role::Druid => {
                if let Some(caps) =
                    regex::Regex::new(r"Among #(\d+), #(\d+), #(\d+) there is: (\w+)")
                        .unwrap()
                        .captures(s)
                {
                    let mut indexes = Vec::new();
                    for i in 1..=3 {
                        if let Some(m) = caps.get(i) {
                            let idx: usize = m
                                .as_str()
                                .parse()
                                .map_err(|_| format!("Invalid index in Druid statement '{}'", s))?;
                            indexes.push(idx - 1);
                        }
                    }
                    let target_indexes = to_bitvec(indexes);
                    let role: Role = caps[4].trim().to_lowercase().parse().map_err(|e| {
                        format!(
                            "Invalid target role '{}' in Druid statement: {}",
                            &caps[4], e
                        )
                    })?;

                    Ok(DruidStatement {
                        target_indexes,
                        role: Some(role),
                    }
                    .into())
                } else if let Some(caps) =
                    regex::Regex::new(r"Among #(\d+), #(\d+), #(\d+) there are NO Outcasts")
                        .unwrap()
                        .captures(s)
                {
                    let mut indexes = Vec::new();
                    for i in 1..=3 {
                        if let Some(m) = caps.get(i) {
                            let idx: usize = m
                                .as_str()
                                .parse()
                                .map_err(|_| format!("Invalid index in Druid statement '{}'", s))?;
                            indexes.push(idx - 1);
                        }
                    }
                    let target_indexes = to_bitvec(indexes);

                    Ok(DruidStatement {
                        target_indexes,
                        role: None,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Druid statement '{}'", s))
                }
            }
            Role::Slayer => {
                if let Some(caps) = regex::Regex::new(r"I killed Evil.*#(\d+)")
                    .unwrap()
                    .captures(s)
                {
                    let target_index: usize = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid index in Slauer statement '{}'", s))?;
                    Ok(SlayerStatement {
                        target_index,
                        alignment: Alignment::Evil,
                    }
                    .into())
                } else if let Some(caps) = regex::Regex::new(r"I couldn't kill.*#(\d+)")
                    .unwrap()
                    .captures(s)
                {
                    let target_index: usize = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid index in Slayer statement '{}'", s))?;
                    Ok(SlayerStatement {
                        target_index,
                        alignment: Alignment::Good,
                    }
                    .into())
                } else {
                    Err(format!("Invalid Slayer statement '{}'", s))
                }
            }
            Role::Dreamer => {
                if let Some(caps) = regex::Regex::new(r"#(\d+) could be: (\w+)")
                    .unwrap()
                    .captures(s)
                {
                    let target_index: usize = caps[1]
                        .parse()
                        .map_err(|_| format!("Invalid index in Dreamer statement '{}'", s))?;
                    let role: Role = caps[2].trim().to_lowercase().parse().map_err(|e| {
                        format!(
                            "Invalid target role '{}' in Dreamer statement: {}",
                            &caps[2], e
                        )
                    })?;

                    Ok(DreamerStatement { target_index: target_index - 1, role }.into())
                } else {
                    Err(format!("Invalid Oracle statement '{}'", s))
                }
            }
            Role::Poet => Ok(RoleStatement::NoStatement),
            _ => Err(format!(
                "No natural statement parsing implemented for {:?}",
                self
            )),
        }
    }
}

macro_rules! role_statements {
    (
        $(
            $role:ident($stmt:ident)
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum RoleStatement {
            NoStatement,
            $(
                $role($stmt),
            )*
        }

        $(
            impl From<$stmt> for RoleStatement {
                fn from(statement: $stmt) -> Self {
                    RoleStatement::$role(statement)
                }
            }
        )*

        impl fmt::Display for RoleStatement {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    RoleStatement::NoStatement => write!(f, "No Statement"),
                    $(
                        RoleStatement::$role(stmt) => stmt.fmt(f),
                    )*
                }
            }
        }
    }
}

role_statements! {
    Alchemist(AlchemistStatement),
    Architect(ArchitectStatement),
    Bard(BardStatement),
    Bishop(BishopStatement),
    Confessor(ConfessorStatement),
    Druid(DruidStatement),
    Dreamer(DreamerStatement),
    Empress(EmpressStatement),
    Enlightened(EnlightenedStatement),
    FortuneTeller(FortuneTellerStatement),
    Gemcrafter(GemcrafterStatement),
    Hunter(HunterStatement),
    Jester(JesterStatement),
    Judge(JudgeStatement),
    Knitter(KnitterStatement),
    Lover(LoverStatement),
    Medium(MediumStatement),
    Oracle(OracleStatement),
    Scout(ScoutStatement),
    Slayer(SlayerStatement),
    PlagueDoctor(PlagueDoctorStatement),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlchemistStatement {
    pub corrupt_count: usize,
}

impl fmt::Display for AlchemistStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I cured {} corruptions", self.corrupt_count)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchitectStatement {
    Right,
    Left,
    Equal,
}

impl fmt::Display for ArchitectStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchitectStatement::Right => write!(f, "Right side is more Evil"),
            ArchitectStatement::Left => write!(f, "Left side is more Evil"),
            ArchitectStatement::Equal => write!(f, "Both sides are Equally evil"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BardStatement {
    pub distance: Option<usize>,
}

impl fmt::Display for BardStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.distance {
            Some(distance) => write!(
                f,
                "I am {} card{} away from closest Corrupted character",
                distance,
                if distance == 1 { "" } else { "s" }
            ),
            None => write!(f, "There are no Corrupted characters"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BishopStatement {
    pub target_indexes: TargetIndexes,
}

impl fmt::Display for BishopStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Among {} there is 1 villager, 1 minion, and maybe 1 outcast",
            self.target_indexes
                .iter_ones()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfessorStatement {
    IAmGood,
    IAmDizzy,
}

impl fmt::Display for ConfessorStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfessorStatement::IAmGood => write!(f, "I am Good"),
            ConfessorStatement::IAmDizzy => write!(f, "I am Dizzy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DreamerStatement {
    pub target_index: usize,
    pub role: Role,
}

impl fmt::Display for DreamerStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} could be {}", self.target_index, self.role)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DruidStatement {
    pub target_indexes: TargetIndexes,
    pub role: Option<Role>,
}

impl fmt::Display for DruidStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(role) = self.role {
            write!(
                f,
                "Among {} there is a {}",
                self.target_indexes
                    .iter_ones()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                role
            )
        } else {
            write!(
                f,
                "Among {} there are NO Outcasts",
                self.target_indexes
                    .iter_ones()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmpressStatement {
    pub target_indexes: TargetIndexes,
}

impl fmt::Display for EmpressStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Among {} there is 1 Evil",
            self.target_indexes
                .iter_ones()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnlightenedStatement {
    Clockwise,
    CounterClockwise,
    Equidistant,
}

impl fmt::Display for EnlightenedStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closest Evil is {:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FortuneTellerStatement {
    pub target_indexes: TargetIndexes,
    pub is_evil: bool,
}

impl fmt::Display for FortuneTellerStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Is {:?} Evil?: {}",
            self.target_indexes.iter_ones().collect::<Vec<_>>(),
            self.is_evil
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GemcrafterStatement {
    pub target_index: usize,
}

impl fmt::Display for GemcrafterStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} is good", self.target_index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HunterStatement {
    pub distance: usize,
}

impl fmt::Display for HunterStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "I am {} card{} away from closest Evil",
            self.distance,
            if self.distance == 1 { "" } else { "s" },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JesterStatement {
    pub target_indexes: TargetIndexes,
    pub evil_count: usize,
}

impl fmt::Display for JesterStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Among {} there are {} Evil",
            self.target_indexes
                .iter_ones()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.evil_count
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudgeStatement {
    pub target_index: usize,
    pub is_lying: bool,
}

impl fmt::Display for JudgeStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let claim_str = if self.is_lying { "Lying" } else { "Truthful" };
        write!(f, "#{} is {}", self.target_index, claim_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnitterStatement {
    pub adjacent_count: usize,
}

impl fmt::Display for KnitterStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There are {} Evil pairs", self.adjacent_count)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoverStatement {
    pub evil_count: usize,
}

impl fmt::Display for LoverStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There are {} Evil adjacent to me", self.evil_count)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediumStatement {
    pub target_index: usize,
    pub role: Role,
}

impl fmt::Display for MediumStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} is {}", self.target_index, self.role)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleStatement {
    pub target_indexes: TargetIndexes,
    pub role: Role,
}

impl fmt::Display for OracleStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Among {} there is a {}",
            self.target_indexes
                .iter_ones()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.role
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoutStatement {
    pub role: Option<Role>,
    pub distance: usize,
}

impl fmt::Display for ScoutStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(role) = self.role {
            write!(
                f,
                "{} is {} card(s) away from closest Evil",
                role, self.distance
            )
        } else {
            write!(f, "There is only 1 Evil")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlayerStatement {
    pub target_index: usize,
    pub alignment: Alignment,
}

impl fmt::Display for SlayerStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} is {:?}", self.target_index, self.alignment)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlagueDoctorStatement {
    pub corruption_index: usize,
    pub evil_index: Option<usize>,
}

impl fmt::Display for PlagueDoctorStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.evil_index {
            None => write!(f, "{} is not corrupt", self.corruption_index),
            Some(evil_idx) => write!(
                f,
                "{} is evil. {} is corrupt",
                evil_idx, self.corruption_index,
            ),
        }
    }
}

pub fn neighbor_indexes(len: usize, position: usize, offset: usize) -> Vec<usize> {
    vec![(position + len - offset) % len, (position + offset) % len]
}

pub fn to_bitvec(indices: Vec<usize>) -> TargetIndexes {
    let mut bits = TargetIndexes::default();
    for i in indices {
        bits.set(i, true);
    }
    bits
}

fn count_evil<'a>(roles: impl IntoIterator<Item = &'a Role>) -> usize {
    roles
        .into_iter()
        .filter(|role| role.alignment() == Alignment::Evil)
        .count()
}

fn count_neighbor_evil(true_roles: &[Role], position: usize, offset: usize) -> usize {
    count_evil(
        neighbor_indexes(true_roles.len(), position, offset)
            .iter()
            .map(|&i| &true_roles[i]),
    )
}

pub fn closest_evil_direction(true_roles: &[Role], position: usize) -> EnlightenedStatement {
    let len = true_roles.len();
    let max_offset = (len + 1) / 2;

    for offset in 1..=max_offset {
        let neighbors = neighbor_indexes(len, position, offset);
        let left = true_roles[neighbors[0]];
        let right = true_roles[neighbors[1]];

        let left_evil = left.alignment() == Alignment::Evil;
        let right_evil = right.alignment() == Alignment::Evil;

        match (left_evil, right_evil) {
            (true, true) => return EnlightenedStatement::Equidistant,
            (true, false) => return EnlightenedStatement::CounterClockwise,
            (false, true) => return EnlightenedStatement::Clockwise,
            _ => (), // keep searching
        }
    }

    EnlightenedStatement::Equidistant
}

pub fn closest_evil_distance(true_roles: &[Role], position: usize) -> usize {
    let max_index = (true_roles.len() + 1) / 2;
    (1..=max_index)
        .find(|&i| count_neighbor_evil(true_roles, position, i) > 0)
        .unwrap_or(true_roles.len())
}

pub fn closest_corrupt_distance(corruptions: &[bool], position: usize) -> Option<usize> {
    let max_distance = corruptions.len() / 2 + 1;

    (1..=max_distance).find(|&distance| {
        neighbor_indexes(corruptions.len(), position, distance)
            .iter()
            .any(|&i| corruptions[i])
    })
}

pub fn count_evil_pairs(true_roles: &[Role]) -> usize {
    true_roles
        .windows(2)
        .filter(|w| w[0].alignment() == Alignment::Evil && w[1].alignment() == Alignment::Evil)
        .count()
}

fn count_side_evils(true_roles: &[Role]) -> ArchitectStatement {
    let len = true_roles.len();
    let half = len / 2;

    let second_half_start = if len % 2 == 0 { half } else { half + 1 };

    let right_evil_count = true_roles[..half]
        .iter()
        .filter(|r| r.alignment() == Alignment::Evil)
        .count();

    let left_evil_count = true_roles[second_half_start..len - 1]
        .iter()
        .filter(|r| r.alignment() == Alignment::Evil)
        .count();

    if left_evil_count > right_evil_count {
        ArchitectStatement::Left
    } else if left_evil_count < right_evil_count {
        ArchitectStatement::Right
    } else {
        ArchitectStatement::Equal
    }
}

/// Check if a card can produce a specific statement given:
/// - `visible_role`: what role is shown (may be a disguise)
/// - `is_lying`: if the character should lie
/// - `true_roles`: the true roles of all the cards in play
/// - `position`: the index of the speaking card
/// - `statement`: the statement to check
pub fn can_produce_statement(
    visible_role: Role,
    is_lying: bool,
    true_roles: &[Role],
    disguised_roles: &[Role],
    corruptions: &[bool],
    drunk_uncorruptions: &[usize],
    position: usize,
    statement: &RoleStatement,
) -> bool {
    if is_lying {
        match visible_role {
            Role::Alchemist => {
                if let RoleStatement::Alchemist(AlchemistStatement { corrupt_count }) = statement {
                    *corrupt_count != 0
                } else {
                    false
                }
            }
            Role::Architect => *statement != RoleStatement::Architect(count_side_evils(true_roles)),
            Role::Bard => {
                let closest_distance = closest_corrupt_distance(corruptions, position);
                if let RoleStatement::Bard(BardStatement { distance }) = statement {
                    if let Some(stmt_dist) = distance {
                        *stmt_dist != closest_distance.unwrap_or(*stmt_dist + 1)
                            && *stmt_dist <= (true_roles.len() + 1) / 2
                    } else {
                        closest_distance.is_some()
                    }
                } else {
                    false
                }
            }
            Role::Bishop => {
                if let RoleStatement::Bishop(BishopStatement { target_indexes }) = statement {
                    target_indexes
                        .iter_ones()
                        .all(|i| true_roles[i].group() == Group::Villager)
                } else {
                    false
                }
            }
            Role::Confessor => *statement == RoleStatement::Confessor(ConfessorStatement::IAmDizzy),
            Role::Dreamer => {
                if let RoleStatement::Dreamer(DreamerStatement { target_index, role }) = statement {
                    let found_role = true_roles[*target_index];
                    found_role.alignment() != Alignment::Evil || found_role != *role
                } else {
                    false
                }
            }
            Role::Druid => {
                if let RoleStatement::Druid(DruidStatement {
                    target_indexes,
                    role: role_option,
                }) = statement
                {
                    if let Some(role) = role_option {
                        target_indexes
                            .iter_ones()
                            .all(|idx| true_roles[idx] != *role)
                    } else {
                        target_indexes
                            .iter_ones()
                            .any(|idx| true_roles[idx].group() == Group::Outcast)
                    }
                } else {
                    false
                }
            }
            Role::Empress => {
                if let RoleStatement::Empress(EmpressStatement { target_indexes }) = statement {
                    target_indexes
                        .iter_ones()
                        .all(|i| true_roles[i].alignment() != Alignment::Evil)
                } else {
                    false
                }
            }
            Role::Enlightened => {
                let true_response = closest_evil_direction(true_roles, position);
                if let RoleStatement::Enlightened(stmt) = statement {
                    stmt != &true_response
                } else {
                    false
                }
            }
            Role::FortuneTeller => {
                if let RoleStatement::FortuneTeller(FortuneTellerStatement {
                    target_indexes,
                    is_evil,
                }) = statement
                {
                    let any_evil = target_indexes
                        .iter_ones()
                        .any(|i| true_roles[i].alignment() == Alignment::Evil);
                    return any_evil != *is_evil;
                } else {
                    false
                }
            }
            Role::Gemcrafter => {
                if let RoleStatement::Gemcrafter(GemcrafterStatement { target_index }) = statement {
                    *target_index < true_roles.len()
                        && true_roles[*target_index].alignment() == Alignment::Evil
                } else {
                    false
                }
            }
            Role::Hunter => {
                let index = closest_evil_distance(true_roles, position);
                if let RoleStatement::Hunter(HunterStatement { distance }) = statement {
                    *distance != index
                } else {
                    false
                }
            }
            Role::Jester => {
                if let RoleStatement::Jester(JesterStatement {
                    target_indexes,
                    evil_count,
                }) = statement
                {
                    *evil_count != count_evil(target_indexes.iter_ones().map(|i| &true_roles[i]))
                } else {
                    false
                }
            }
            Role::Judge => {
                if let RoleStatement::Judge(JudgeStatement {
                    target_index,
                    is_lying: stmt_lying,
                }) = statement
                {
                    *stmt_lying
                        != ((true_roles[*target_index].lying() || corruptions[*target_index])
                            && disguised_roles[*target_index] != Role::Confessor)
                } else {
                    false
                }
            }
            Role::Knitter => {
                if let RoleStatement::Knitter(KnitterStatement { adjacent_count }) = statement {
                    let true_adjacent_count = count_evil_pairs(true_roles);

                    *adjacent_count != true_adjacent_count
                } else {
                    false
                }
            }
            Role::Lover => {
                let neighbors = neighbor_indexes(true_roles.len(), position, 1);
                let real_evil_count = neighbors
                    .iter()
                    .filter(|&&idx| true_roles[idx].alignment() == Alignment::Evil)
                    .count();

                if let RoleStatement::Lover(LoverStatement { evil_count }) = statement {
                    *evil_count != real_evil_count && *evil_count <= 2
                } else {
                    false
                }
            }
            Role::Medium => {
                if let RoleStatement::Medium(MediumStatement { target_index, role }) = statement {
                    *target_index < true_roles.len()
                        && *target_index < disguised_roles.len()
                        && true_roles[*target_index] != disguised_roles[*target_index]
                        && *role == disguised_roles[*target_index]
                } else {
                    false
                }
            }
            Role::Oracle => {
                if let RoleStatement::Oracle(OracleStatement {
                    target_indexes,
                    role: _,
                }) = statement
                {
                    target_indexes
                        .iter_ones()
                        .all(|i| true_roles[i].alignment() != Alignment::Evil)
                } else {
                    false
                }
            }
            Role::Scout => {
                if let RoleStatement::Scout(ScoutStatement {
                    role: role_option,
                    distance,
                }) = statement
                {
                    let evil_count = true_roles
                        .iter()
                        .filter(|r| r.alignment() == Alignment::Evil)
                        .count();

                    if let Some(role) = role_option {
                        evil_count == 1
                            || !true_roles.iter().enumerate().any(|(idx, r)| {
                                r == role
                                    && *distance == closest_evil_distance(true_roles, idx)
                                    && true_roles[idx].alignment() == Alignment::Evil
                            })
                    } else {
                        evil_count != 1
                    }
                } else {
                    false
                }
            }
            Role::Slayer => {
                if let RoleStatement::Slayer(SlayerStatement {
                    target_index,
                    alignment,
                }) = statement
                {
                    *target_index < true_roles.len() && *alignment == Alignment::Good
                } else {
                    false
                }
            }
            Role::PlagueDoctor => {
                if let RoleStatement::PlagueDoctor(PlagueDoctorStatement {
                    corruption_index,
                    evil_index,
                }) = statement
                {
                    let is_corrupt = corruptions[*corruption_index];

                    match evil_index {
                        None => is_corrupt || *corruption_index == position,
                        Some(evil_idx) => {
                            !is_corrupt && true_roles[*evil_idx].alignment() == Alignment::Good
                        }
                    }
                } else {
                    false
                }
            }
            Role::Bombardier | Role::Wretch | Role::Knight => {
                *statement == RoleStatement::NoStatement
            }
            other => panic!(
                "can_produce_statement: unsupported role combination: visible={:?}, lying={:?}",
                other, is_lying
            ),
        }
    } else {
        match visible_role {
            Role::Alchemist => {
                if let RoleStatement::Alchemist(AlchemistStatement { corrupt_count }) = statement {
                    *corrupt_count == drunk_uncorruptions[position]
                } else {
                    false
                }
            }
            Role::Architect => *statement == RoleStatement::Architect(count_side_evils(true_roles)),
            Role::Bard => {
                let closest_distance = closest_corrupt_distance(corruptions, position);
                if let RoleStatement::Bard(BardStatement { distance }) = statement {
                    *distance == closest_distance
                } else {
                    false
                }
            }
            Role::Bishop => {
                if let RoleStatement::Bishop(BishopStatement { target_indexes }) = statement {
                    let mut villager_count = 0;
                    let mut minion_count = 0;
                    let mut outcast_count = 0;
                    let mut other_count = 0;

                    for i in target_indexes.iter_ones() {
                        match true_roles[i].group() {
                            Group::Villager => villager_count += 1,
                            Group::Minion => minion_count += 1,
                            Group::Outcast => outcast_count += 1,
                            _ => other_count += 1,
                        }
                    }

                    villager_count == 1
                        && minion_count == 1
                        && outcast_count <= 1
                        && other_count == 0
                } else {
                    false
                }
            }
            Role::Confessor => *statement == RoleStatement::Confessor(ConfessorStatement::IAmGood),
            Role::Druid => {
                if let RoleStatement::Druid(DruidStatement {
                    target_indexes,
                    role: role_option,
                }) = statement
                {
                    if let Some(role) = role_option {
                        target_indexes
                            .iter_ones()
                            .any(|idx| true_roles[idx] == *role)
                    } else {
                        !target_indexes
                            .iter_ones()
                            .any(|idx| true_roles[idx].group() == Group::Outcast)
                    }
                } else {
                    false
                }
            }
            Role::Enlightened => {
                *statement
                    == RoleStatement::Enlightened(closest_evil_direction(true_roles, position))
            }
            Role::Empress => {
                if let RoleStatement::Empress(EmpressStatement { target_indexes }) = statement {
                    let (evil_count, good_count) =
                        target_indexes
                            .iter_ones()
                            .fold((0, 0), |(evil, good), i| match true_roles[i].alignment() {
                                Alignment::Evil => (evil + 1, good),
                                Alignment::Good => (evil, good + 1),
                            });
                    evil_count == 1 && good_count == 2
                } else {
                    false
                }
            }
            Role::Dreamer => {
                if let RoleStatement::Dreamer(DreamerStatement { target_index, role }) = statement {
                    let found_role = true_roles[*target_index];
                    found_role.alignment() != Alignment::Evil || found_role == *role
                } else {
                    false
                }
            }
            Role::FortuneTeller => {
                if let RoleStatement::FortuneTeller(FortuneTellerStatement {
                    target_indexes,
                    is_evil,
                }) = statement
                {
                    let any_evil = target_indexes
                        .iter_ones()
                        .any(|i| true_roles[i].alignment() == Alignment::Evil);
                    return any_evil == *is_evil;
                } else {
                    false
                }
            }
            Role::Gemcrafter => {
                if let RoleStatement::Gemcrafter(GemcrafterStatement { target_index }) = statement {
                    *target_index < true_roles.len()
                        && true_roles[*target_index].alignment() == Alignment::Good
                } else {
                    false
                }
            }
            Role::Hunter => {
                let index = closest_evil_distance(true_roles, position);
                if let RoleStatement::Hunter(HunterStatement { distance }) = statement {
                    *distance == index
                } else {
                    false
                }
            }
            Role::Jester => {
                if let RoleStatement::Jester(JesterStatement {
                    target_indexes,
                    evil_count,
                }) = statement
                {
                    *evil_count == count_evil(target_indexes.iter_ones().map(|i| &true_roles[i]))
                } else {
                    false
                }
            }
            Role::Judge => {
                if let RoleStatement::Judge(JudgeStatement {
                    target_index,
                    is_lying: stmt_lying,
                }) = statement
                {
                    *stmt_lying
                        == ((true_roles[*target_index].lying() || corruptions[*target_index])
                            && disguised_roles[*target_index] != Role::Confessor)
                } else {
                    false
                }
            }
            Role::Knitter => {
                if let RoleStatement::Knitter(KnitterStatement { adjacent_count }) = statement {
                    let true_adjacent_count = count_evil_pairs(true_roles);

                    *adjacent_count == true_adjacent_count
                } else {
                    false
                }
            }
            Role::Lover => {
                let evil_count = count_neighbor_evil(true_roles, position, 1);
                if let RoleStatement::Lover(LoverStatement { evil_count: c }) = statement {
                    *c == evil_count
                } else {
                    false
                }
            }
            Role::Medium => {
                if let RoleStatement::Medium(MediumStatement { target_index, role }) = statement {
                    *target_index != position
                        && true_roles[*target_index].alignment() == Alignment::Good
                        && *role == true_roles[*target_index]
                } else {
                    false
                }
            }
            Role::Oracle => {
                if let RoleStatement::Oracle(OracleStatement {
                    target_indexes,
                    role,
                }) = statement
                {
                    let mut targets = target_indexes.iter_ones();
                    let first = targets.next().unwrap();
                    let second = targets.next().unwrap();

                    (true_roles[first].alignment() == Alignment::Good
                        && true_roles[second] == *role)
                        || (true_roles[second].alignment() == Alignment::Good
                            && true_roles[first] == *role)
                } else {
                    false
                }
            }
            Role::Scout => {
                if let RoleStatement::Scout(ScoutStatement {
                    role: role_option,
                    distance,
                }) = statement
                {
                    let evil_count = true_roles
                        .iter()
                        .filter(|r| r.alignment() == Alignment::Evil)
                        .count();

                    if let Some(role) = role_option {
                        evil_count != 1
                            && true_roles.iter().enumerate().any(|(idx, r)| {
                                r == role
                                    && *distance == closest_evil_distance(true_roles, idx)
                                    && true_roles[idx].alignment() == Alignment::Evil
                            })
                    } else {
                        evil_count == 1
                    }
                } else {
                    false
                }
            }
            Role::Slayer => {
                if let RoleStatement::Slayer(SlayerStatement {
                    target_index,
                    alignment,
                }) = statement
                {
                    *target_index < true_roles.len()
                        && *alignment == true_roles[*target_index].alignment()
                } else {
                    false
                }
            }
            Role::PlagueDoctor => {
                if let RoleStatement::PlagueDoctor(PlagueDoctorStatement {
                    corruption_index,
                    evil_index,
                }) = statement
                {
                    let is_corrupt = corruptions[*corruption_index];

                    match evil_index {
                        None => !is_corrupt,
                        Some(evil_idx) => {
                            is_corrupt && true_roles[*evil_idx].alignment() == Alignment::Evil
                        }
                    }
                } else {
                    false
                }
            }
            Role::Wretch | Role::Bombardier | Role::Knight => {
                *statement == RoleStatement::NoStatement
            }
            other => panic!(
                "can_produce_statement: unsupported role combination: true={:?}, visible={:?}",
                visible_role, other
            ),
        }
    }
}
