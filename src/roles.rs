use bitvec::prelude::*;
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};

type TargetIndexes = BitArray<[u8; 2], Lsb0>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    // Villager
    Bard,
    Confessor,
    Empress,
    Enlightened,
    Gemcrafter,
    Hunter,
    Jester,
    Judge,
    Knight,
    Lover,
    Medium,
    Scout,
    Slayer,
    // Outcast
    Bombardier,
    PlagueDoctor,
    Wretch,
    // Minion
    Minion,
    Poisoner,
    TwinMinion,
    Witch,
    // Demon
    Baa,
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
            Bard | Confessor | Empress | Enlightened | Gemcrafter | Hunter | Jester | Judge
            | Knight | Lover | Medium | Scout | Slayer => Group::Villager,
            Bombardier | PlagueDoctor | Wretch => Group::Outcast,
            Minion | Poisoner | TwinMinion | Witch => Group::Minion,
            Baa => Group::Demon,
        }
    }
    pub const fn alignment(self) -> Alignment {
        use Role::*;
        match self {
            Bard | Confessor | Empress | Enlightened | Gemcrafter | Hunter | Jester | Judge
            | Knight | Lover | Medium | Scout | Slayer | Bombardier | PlagueDoctor | Wretch => {
                Alignment::Good
            }
            Baa | Minion | Poisoner | TwinMinion | Witch => Alignment::Evil,
        }
    }
    pub const fn lying(self) -> bool {
        use Role::*;
        match self {
            Bard | Confessor | Empress | Enlightened | Gemcrafter | Hunter | Jester | Judge
            | Knight | Lover | Medium | Scout | Slayer | Bombardier | PlagueDoctor | Wretch => {
                false
            }
            Baa | Minion | Poisoner | TwinMinion | Witch => true,
        }
    }
    pub fn parse_statement(&self, s: &str) -> RoleStatement {
        fn parse_indexes(s: &str) -> TargetIndexes {
            let mut bits = TargetIndexes::default();

            for idx_str in s.split(',') {
                let idx: usize = idx_str.trim().parse().expect("Invalid index");
                bits.set(idx, true);
            }

            bits
        }

        match self {
            Role::Bard => {
                let distance = if s.trim() == "none" {
                    None
                } else {
                    Some(
                        s.trim()
                            .parse()
                            .expect("Invalid distance for BardStatement"),
                    )
                };
                RoleStatement::Bard(BardStatement { distance })
            }
            Role::Confessor => match s.trim() {
                "iamgood" => RoleStatement::Confessor(ConfessorStatement::IAmGood),
                "iamdizzy" => RoleStatement::Confessor(ConfessorStatement::IAmDizzy),
                _ => panic!("Invalid Confessor statement: {}", s),
            },
            Role::Empress => {
                let target_indexes = parse_indexes(s);
                RoleStatement::Empress(EmpressStatement { target_indexes })
            }
            Role::Enlightened => match s.trim() {
                "clockwise" => RoleStatement::Enlightened(EnlightenedStatement::Clockwise),
                "counterclockwise" => {
                    RoleStatement::Enlightened(EnlightenedStatement::CounterClockwise)
                }
                "equidistant" => RoleStatement::Enlightened(EnlightenedStatement::Equidistant),
                _ => panic!("Invalid Enlightened statement: {}", s),
            },
            Role::Gemcrafter => {
                let target_index = s.trim().parse().expect("Invalid target index");
                RoleStatement::Gemcrafter(GemcrafterStatement { target_index })
            }
            Role::Hunter => {
                let distance = s.trim().parse().expect("Invalid distance");
                RoleStatement::Hunter(HunterStatement { distance })
            }
            Role::Jester => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    panic!("Invalid Jester statement: {}", s);
                }
                let target_indexes = parse_indexes(parts[0]);
                let evil_count = parts[1].trim().parse().expect("Invalid evil count");
                RoleStatement::Jester(JesterStatement {
                    target_indexes,
                    evil_count,
                })
            }
            Role::Judge => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    panic!("Invalid Judge statement: {}", s);
                }
                let target_index = parts[0].trim().parse().expect("Invalid target index");
                let is_lying = match parts[1].trim() {
                    "truthy" => false,
                    "lying" => true,
                    _ => panic!("Unknown claim type: {}", parts[1]),
                };
                RoleStatement::Judge(JudgeStatement {
                    target_index,
                    is_lying,
                })
            }
            Role::Lover => {
                let evil_count = s.trim().parse().expect("Invalid evil count");
                RoleStatement::Lover(LoverStatement { evil_count })
            }
            Role::Medium => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    panic!("Invalid Medium statement: {}", s);
                }
                let target_index = parts[0].trim().parse().expect("Invalid target index");
                let role: Role = parts[1].trim().parse().expect("Invalid target role");
                RoleStatement::Medium(MediumStatement { target_index, role })
            }
            Role::Scout => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    panic!("Invalid Scout statement: {}", s);
                }
                let role: Role = parts[0].trim().parse().expect("Invalid target role");
                let distance = parts[1].trim().parse().expect("Invalid distance");
                RoleStatement::Scout(ScoutStatement { role, distance })
            }
            Role::Slayer => {
                let parts: Vec<&str> = s.split(';').collect();
                if parts.len() != 2 {
                    panic!("Invalid Slayer statement: {}", s);
                }
                let target_index = parts[0].trim().parse().expect("Invalid target index");
                let alignment = match parts[1].trim() {
                    "good" => Alignment::Good,
                    "evil" => Alignment::Evil,
                    _ => panic!("Unknown alignment: {}", parts[1]),
                };
                RoleStatement::Slayer(SlayerStatement {
                    target_index,
                    alignment,
                })
            }
            Role::Knight
            | Role::Bombardier
            | Role::PlagueDoctor
            | Role::Wretch
            | Role::Minion
            | Role::Poisoner
            | Role::TwinMinion
            | Role::Witch
            | Role::Baa => {
                panic!("No statement parsing implemented for {:?}", self)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoleStatement {
    Unrevealed,
    Bard(BardStatement),
    Confessor(ConfessorStatement),
    Empress(EmpressStatement),
    Enlightened(EnlightenedStatement),
    Gemcrafter(GemcrafterStatement),
    Hunter(HunterStatement),
    Jester(JesterStatement),
    Judge(JudgeStatement),
    Lover(LoverStatement),
    Medium(MediumStatement),
    Scout(ScoutStatement),
    Slayer(SlayerStatement),
}

impl From<BardStatement> for RoleStatement {
    fn from(statement: BardStatement) -> Self {
        RoleStatement::Bard(statement)
    }
}

impl From<ConfessorStatement> for RoleStatement {
    fn from(statement: ConfessorStatement) -> Self {
        RoleStatement::Confessor(statement)
    }
}

impl From<EmpressStatement> for RoleStatement {
    fn from(statement: EmpressStatement) -> Self {
        RoleStatement::Empress(statement)
    }
}

impl From<EnlightenedStatement> for RoleStatement {
    fn from(statement: EnlightenedStatement) -> Self {
        RoleStatement::Enlightened(statement)
    }
}

impl From<GemcrafterStatement> for RoleStatement {
    fn from(statement: GemcrafterStatement) -> Self {
        RoleStatement::Gemcrafter(statement)
    }
}

impl From<HunterStatement> for RoleStatement {
    fn from(statement: HunterStatement) -> Self {
        RoleStatement::Hunter(statement)
    }
}

impl From<JesterStatement> for RoleStatement {
    fn from(statement: JesterStatement) -> Self {
        RoleStatement::Jester(statement)
    }
}

impl From<JudgeStatement> for RoleStatement {
    fn from(statement: JudgeStatement) -> Self {
        RoleStatement::Judge(statement)
    }
}

impl From<LoverStatement> for RoleStatement {
    fn from(statement: LoverStatement) -> Self {
        RoleStatement::Lover(statement)
    }
}

impl From<MediumStatement> for RoleStatement {
    fn from(statement: MediumStatement) -> Self {
        RoleStatement::Medium(statement)
    }
}

impl From<ScoutStatement> for RoleStatement {
    fn from(statement: ScoutStatement) -> Self {
        RoleStatement::Scout(statement)
    }
}

impl From<SlayerStatement> for RoleStatement {
    fn from(statement: SlayerStatement) -> Self {
        RoleStatement::Slayer(statement)
    }
}

impl fmt::Display for RoleStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleStatement::Unrevealed => write!(f, "Unrevealed"),
            RoleStatement::Bard(stmt) => stmt.fmt(f),
            RoleStatement::Confessor(stmt) => stmt.fmt(f),
            RoleStatement::Empress(stmt) => stmt.fmt(f),
            RoleStatement::Enlightened(stmt) => stmt.fmt(f),
            RoleStatement::Gemcrafter(stmt) => stmt.fmt(f),
            RoleStatement::Hunter(stmt) => stmt.fmt(f),
            RoleStatement::Jester(stmt) => stmt.fmt(f),
            RoleStatement::Judge(stmt) => stmt.fmt(f),
            RoleStatement::Lover(stmt) => stmt.fmt(f),
            RoleStatement::Medium(stmt) => stmt.fmt(f),
            RoleStatement::Scout(stmt) => stmt.fmt(f),
            RoleStatement::Slayer(stmt) => stmt.fmt(f),
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
pub struct EmpressStatement {
    pub target_indexes: TargetIndexes,
}

impl fmt::Display for EmpressStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Among {:#?} there is 1 Evil", self.target_indexes)
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
        write!(f, "Closest Evil is {}", self)
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
            "I am {} card{} away from closest Corrupted character",
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
            "Among {:#?} there are {} Evil",
            self.target_indexes, self.evil_count
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
pub struct ScoutStatement {
    pub role: Role,
    pub distance: usize,
}

impl fmt::Display for ScoutStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} is {} card away from closest Evil",
            self.role, self.distance
        )
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
        .unwrap_or(1)
}

pub fn closest_corrupt_distance(corruptions: &[bool], position: usize) -> Option<usize> {
    let max_distance = corruptions.len() / 2 + 1;

    (1..=max_distance).find(|&distance| {
        neighbor_indexes(corruptions.len(), position, distance)
            .iter()
            .any(|&i| corruptions[i])
    })
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
    position: usize,
    statement: &RoleStatement,
) -> bool {
    if is_lying {
        match visible_role {
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
            Role::Confessor => *statement == RoleStatement::Confessor(ConfessorStatement::IAmDizzy),
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
                    *distance != index && *distance <= (true_roles.len() + 1) / 2
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
                    *target_index < true_roles.len()
                        && *stmt_lying == true_roles[*target_index].lying()
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
            Role::Scout => {
                if let RoleStatement::Scout(ScoutStatement { role, distance }) = statement {
                    true_roles
                        .iter()
                        .any(|r| r == role && r.alignment() == Alignment::Evil)
                        && *distance
                            != closest_evil_distance(
                                true_roles,
                                true_roles.iter().position(|r| r == role).unwrap(),
                            )
                        && *distance <= (true_roles.len() + 1) / 2
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
            Role::Bombardier | Role::Wretch | Role::PlagueDoctor | Role::Knight => {
                *statement == RoleStatement::Unrevealed
            }
            other => panic!(
                "can_produce_statement: unsupported role combination: visible={:?}, lying={:?}",
                other, is_lying
            ),
        }
    } else {
        match visible_role {
            Role::Bard => {
                let closest_distance = closest_corrupt_distance(corruptions, position);
                if let RoleStatement::Bard(BardStatement { distance }) = statement {
                    *distance == closest_distance
                } else {
                    false
                }
            }
            Role::Confessor => *statement == RoleStatement::Confessor(ConfessorStatement::IAmGood),
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
                    *target_index < true_roles.len()
                        && *stmt_lying == true_roles[*target_index].lying()
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
                    *target_index < true_roles.len()
                        && true_roles[*target_index].alignment() == Alignment::Good
                        && *role == true_roles[*target_index]
                } else {
                    false
                }
            }
            Role::Scout => {
                if let RoleStatement::Scout(ScoutStatement { role, distance }) = statement {
                    if let Some(idx) = true_roles.iter().position(|r| r == role) {
                        *distance == closest_evil_distance(true_roles, idx)
                            && true_roles[idx].alignment() == Alignment::Evil
                    } else {
                        false
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
            Role::Wretch | Role::PlagueDoctor | Role::Bombardier | Role::Knight => {
                *statement == RoleStatement::Unrevealed
            }
            other => panic!(
                "can_produce_statement: unsupported role combination: true={:?}, visible={:?}",
                visible_role, other
            ),
        }
    }
}
