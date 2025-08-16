use itertools::Itertools;
use std::any::Any;
use std::fmt;
use strum_macros::{Display, EnumIter, EnumString};

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
}

/// Trait type for typed statements coming from roles.
/// Implementors should be `Clone` + `Debug` + 'static so we can clone and downcast them.
pub trait RoleStatement: RoleStatementClone + fmt::Debug + Send + Sync {
    /// For downcasting
    fn as_any(&self) -> &dyn Any;

    /// Compare this statement with another RoleStatement.
    /// Implementations should attempt a downcast and return true only if types & payloads match.
    fn equals(&self, other: &dyn RoleStatement) -> bool;
}

/// Helper trait to allow cloning Box<dyn RoleStatement>
pub trait RoleStatementClone {
    fn clone_box(&self) -> Box<dyn RoleStatement>;
}
impl<T> RoleStatementClone for T
where
    T: 'static + RoleStatement + Clone,
{
    fn clone_box(&self) -> Box<dyn RoleStatement> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn RoleStatement> {
    fn clone(&self) -> Box<dyn RoleStatement> {
        self.clone_box()
    }
}

/// Cards not yet revealed have no statement
#[derive(Debug, Clone, PartialEq)]
pub struct UnrevealedStatement;

impl fmt::Display for UnrevealedStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unrevealed")
    }
}

impl RoleStatement for UnrevealedStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<UnrevealedStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for BardStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<BardStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for ConfessorStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<ConfessorStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmpressStatement {
    pub target_indexes: Vec<usize>,
}
impl EmpressStatement {
    fn unordered_indexes_eq(&self, other: &Self) -> bool {
        let mut self_sorted = self.target_indexes.clone();
        let mut other_sorted = other.target_indexes.clone();

        self_sorted.sort_unstable();
        other_sorted.sort_unstable();
        self_sorted.dedup();
        other_sorted.dedup();

        self_sorted == other_sorted
    }
}
impl fmt::Display for EmpressStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Among {:#?} there is 1 Evil", self.target_indexes)
    }
}
impl RoleStatement for EmpressStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<EmpressStatement>()
            .map(|o| self.unordered_indexes_eq(o))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnlightenedStatement {
    Clockwise,
    CounterClockwise,
    Equidistant,
}
impl EnlightenedStatement {
    fn iterator() -> impl Iterator<Item = EnlightenedStatement> {
        [
            EnlightenedStatement::Clockwise,
            EnlightenedStatement::CounterClockwise,
            EnlightenedStatement::Equidistant,
        ]
        .into_iter()
    }
}
impl fmt::Display for EnlightenedStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closest Evil is {}", self)
    }
}
impl RoleStatement for EnlightenedStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<EnlightenedStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for GemcrafterStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<GemcrafterStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for HunterStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<HunterStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JesterStatement {
    pub target_indexes: Vec<usize>,
    pub evil_count: usize,
}
impl JesterStatement {
    fn unordered_indexes_eq(&self, other: &Self) -> bool {
        let mut self_sorted = self.target_indexes.clone();
        let mut other_sorted = other.target_indexes.clone();

        self_sorted.sort_unstable();
        other_sorted.sort_unstable();
        self_sorted.dedup();
        other_sorted.dedup();

        self_sorted == other_sorted
    }
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
impl RoleStatement for JesterStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<JesterStatement>()
            .map(|o| self.unordered_indexes_eq(o) && self.evil_count == o.evil_count)
            .unwrap_or(false)
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
impl RoleStatement for JudgeStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<JudgeStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for LoverStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<LoverStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for MediumStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<MediumStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for ScoutStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<ScoutStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
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
impl RoleStatement for SlayerStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<SlayerStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
    }
}

pub fn neighbor_indexes(len: usize, position: usize, offset: usize) -> Vec<usize> {
    vec![(position + len - offset) % len, (position + offset) % len]
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

/// Produce the typed statements a card can make given:
/// - `visible_role`: what role is shown (may be a disguise)
/// - `is_lying`: if the character should lie
/// - `true_roles`: the true roles of all the cards in play
/// - `position`: the index of the speaking card
pub fn produce_statements(
    visible_role: Role,
    is_lying: bool,
    true_roles: &[Role],
    disguised_roles: &[Role],
    corruptions: &[bool],
    position: usize,
) -> Vec<Box<dyn RoleStatement>> {
    if is_lying {
        return match visible_role {
            Role::Bard => {
                let max_index = (true_roles.len() + 1) / 2;
                let closest_distance = closest_corrupt_distance(corruptions, position);

                let mut statements: Vec<Box<dyn RoleStatement>> = (1..=max_index)
                    .filter(|&i| Some(i) != closest_distance)
                    .map(|i| {
                        Box::new(BardStatement { distance: Some(i) }) as Box<dyn RoleStatement>
                    })
                    .collect();

                // Add the "no corruption" statement if there actually is a closest corruption
                if closest_distance.is_some() {
                    statements.push(Box::new(BardStatement { distance: None }));
                }

                statements
            }
            Role::Confessor => vec![Box::new(ConfessorStatement::IAmDizzy)],
            Role::Empress => {
                let good = true_roles
                    .iter()
                    .enumerate()
                    .filter(|(_, role)| role.alignment() != Alignment::Evil);

                good.combinations(3)
                    .map(move |triplet| {
                        let target_indexes = vec![triplet[0].0, triplet[1].0, triplet[2].0];
                        Box::new(EmpressStatement {
                            target_indexes: target_indexes,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            }
            Role::Enlightened => {
                let true_response = closest_evil_direction(true_roles, position);
                EnlightenedStatement::iterator()
                    .filter(|&stmt| stmt != true_response)
                    .map(|stmt| Box::new(stmt) as Box<dyn RoleStatement>)
                    .collect()
            }
            Role::Gemcrafter => {
                // Claim all evil cards are good
                true_roles
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.alignment() == Alignment::Evil)
                    .map(|(idx, _)| {
                        Box::new(GemcrafterStatement { target_index: idx })
                            as Box<dyn RoleStatement>
                    })
                    .collect()
            }
            Role::Hunter => {
                let max_index = (true_roles.len() + 1) / 2;
                let index = closest_evil_distance(true_roles, position);

                (1..=max_index)
                    .filter(|&i| i != index)
                    .map(|i| Box::new(HunterStatement { distance: i }) as Box<dyn RoleStatement>)
                    .collect()
            }
            Role::Jester => true_roles
                .iter()
                .enumerate()
                .combinations(3)
                .flat_map(move |triplet| {
                    let target_indexes = vec![triplet[0].0, triplet[1].0, triplet[2].0];
                    let evil_count = count_evil(target_indexes.iter().map(|&i| &true_roles[i]));

                    (0..=3)
                        .filter(|&fake_count| fake_count != evil_count)
                        .map(|fake_count| {
                            Box::new(JesterStatement {
                                target_indexes: target_indexes.clone(),
                                evil_count: fake_count,
                            }) as Box<dyn RoleStatement>
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
            Role::Judge => {
                // Claim all lying cards are truthy, and truthy cards are lying
                let mut statements: Vec<Box<dyn RoleStatement>> = true_roles
                    .iter()
                    .enumerate()
                    .map(|(idx, r)| {
                        Box::new(JudgeStatement {
                            target_index: idx,
                            is_lying: r.lying(),
                        }) as Box<dyn RoleStatement>
                    })
                    .collect();
                statements.push(Box::new(UnrevealedStatement));
                statements
            }
            Role::Lover => {
                let neighbors = neighbor_indexes(true_roles.len(), position, 1);
                let real_evil_count = neighbors
                    .iter()
                    .filter(|&&idx| true_roles[idx].alignment() == Alignment::Evil)
                    .count();

                (0..=2)
                    .filter(|&fake_count| fake_count != real_evil_count)
                    .map(|fake_count| {
                        Box::new(LoverStatement {
                            evil_count: fake_count,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            }
            Role::Medium => {
                // Claim all disguised cards as their disguise
                true_roles
                    .iter()
                    .zip(disguised_roles.iter())
                    .enumerate()
                    .filter(|(_, (r, d))| r != d)
                    .map(|(idx, (_, d))| {
                        Box::new(MediumStatement {
                            target_index: idx,
                            role: *d,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            }
            Role::Scout => {
                // Claim all evil roles as the wrong distance from their closest evil
                let max_index = (true_roles.len() + 1) / 2;
                true_roles
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.alignment() == Alignment::Evil)
                    .flat_map(|(idx, r)| {
                        let distance = closest_evil_distance(true_roles, idx);
                        (1..=max_index)
                            .filter(|&i| i != distance)
                            .map(|i| {
                                Box::new(ScoutStatement {
                                    role: *r,
                                    distance: i,
                                }) as Box<dyn RoleStatement>
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect()
            }
            Role::Slayer => {
                // Claim all cards as good no matter what
                let mut statements: Vec<Box<dyn RoleStatement>> = true_roles
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| {
                        Box::new(SlayerStatement {
                            target_index: idx,
                            alignment: Alignment::Good,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect();
                statements.push(Box::new(UnrevealedStatement));
                statements
            }
            // TODO: PlagueDoctor
            Role::Bombardier | Role::Wretch | Role::PlagueDoctor | Role::Knight => {
                vec![Box::new(UnrevealedStatement)]
            }
            other => panic!(
                "produce_statements: unsupported role combination: visible={:?}, lying={:?}",
                other, is_lying
            ),
        };
    }

    return match visible_role {
        Role::Bard => {
            let closest_distance = closest_corrupt_distance(corruptions, position);
            vec![Box::new(BardStatement {
                distance: closest_distance,
            })]
        }
        Role::Confessor => vec![Box::new(ConfessorStatement::IAmGood)],
        Role::Enlightened => vec![Box::new(closest_evil_direction(true_roles, position))],
        Role::Empress => {
            let (evil, good): (Vec<_>, Vec<_>) = true_roles
                .iter()
                .enumerate()
                .partition(|(_, r)| r.alignment() == Alignment::Evil);

            evil.iter()
                .flat_map(|(ei, _)| {
                    good.iter().combinations(2).map(move |pair| {
                        let target_indexes = vec![*ei, pair[0].0, pair[1].0];
                        Box::new(EmpressStatement {
                            target_indexes: target_indexes,
                        }) as Box<dyn RoleStatement>
                    })
                })
                .collect()
        }
        Role::Gemcrafter => {
            // Claim all villagers are good
            true_roles
                .iter()
                .enumerate()
                .filter(|(_, r)| r.alignment() == Alignment::Good)
                .map(|(idx, _)| {
                    Box::new(GemcrafterStatement { target_index: idx }) as Box<dyn RoleStatement>
                })
                .collect()
        }
        Role::Hunter => {
            let index = closest_evil_distance(true_roles, position);
            vec![Box::new(HunterStatement { distance: index })]
        }
        Role::Jester => true_roles
            .iter()
            .enumerate()
            .combinations(3)
            .map(move |triplet| {
                let target_indexes = vec![triplet[0].0, triplet[1].0, triplet[2].0];
                let evil_count = count_evil(target_indexes.iter().map(|&i| &true_roles[i]));
                Box::new(JesterStatement {
                    target_indexes: target_indexes.clone(),
                    evil_count: evil_count,
                }) as Box<dyn RoleStatement>
            })
            .collect::<Vec<_>>(),
        Role::Judge => {
            // Claim all lying cards are lying, and truthy cards are truthy
            let mut statements: Vec<Box<dyn RoleStatement>> = true_roles
                .iter()
                .enumerate()
                .map(|(idx, r)| {
                    Box::new(JudgeStatement {
                        target_index: idx,
                        is_lying: r.lying(),
                    }) as Box<dyn RoleStatement>
                })
                .collect();
            statements.push(Box::new(UnrevealedStatement));
            statements
        }
        Role::Lover => {
            let evil_count = count_neighbor_evil(true_roles, position, 1);
            vec![Box::new(LoverStatement {
                evil_count: evil_count,
            })]
        }
        Role::Medium => {
            // Claim all good cards as their role
            true_roles
                .iter()
                .enumerate()
                .filter(|(_, r)| r.alignment() == Alignment::Good)
                .map(|(idx, r)| {
                    Box::new(MediumStatement {
                        target_index: idx,
                        role: *r,
                    }) as Box<dyn RoleStatement>
                })
                .collect()
        }
        Role::Scout => {
            // Claim all evil roles as the distance from their closest evil
            true_roles
                .iter()
                .enumerate()
                .filter(|(_, r)| r.alignment() == Alignment::Evil)
                .map(|(idx, r)| {
                    let distance = closest_evil_distance(true_roles, idx);
                    Box::new(ScoutStatement {
                        role: *r,
                        distance: distance,
                    }) as Box<dyn RoleStatement>
                })
                .collect()
        }
        Role::Slayer => {
            // Claim all good cards or evil cards as their alignment
            let mut statements: Vec<Box<dyn RoleStatement>> = true_roles
                .iter()
                .enumerate()
                .map(|(idx, r)| {
                    Box::new(SlayerStatement {
                        target_index: idx,
                        alignment: r.alignment(),
                    }) as Box<dyn RoleStatement>
                })
                .collect();
            statements.push(Box::new(UnrevealedStatement));
            statements
        }
        // TODO: PlagueDoctor
        Role::Wretch | Role::PlagueDoctor | Role::Bombardier | Role::Knight => {
            vec![Box::new(UnrevealedStatement)]
        }
        other => panic!(
            "produce_statements: unsupported role combination: true={:?}, visible={:?}",
            visible_role, other
        ),
    };
}
