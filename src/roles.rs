use std::any::Any;
use std::fmt;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    // Villager
    Confessor,
    Enlightened,
    Gemcrafter,
    Hunter,
    Judge,
    Lover,
    Empress,
    // Outcast
    Wretch,
    // Evil
    Minion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    Villager,
    Outcast,
    Minion,
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
            Confessor | Gemcrafter | Hunter | Lover | Empress | Judge | Enlightened => Group::Villager,
            Wretch => Group::Outcast,
            Minion => Group::Minion,
        }
    }
    pub const fn alignment(self) -> Alignment {
        use Role::*;
        match self {
            Confessor | Gemcrafter | Hunter | Lover | Empress | Judge | Enlightened | Wretch => Alignment::Good,
            Minion => Alignment::Evil,
        }
    }
    pub const fn lying(self) -> bool {
        use Role::*;
        match self {
            Confessor | Gemcrafter | Hunter | Lover | Empress | Judge | Enlightened | Wretch => false,
            Minion => true,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Confessor => write!(f, "Confessor"),
            Role::Enlightened => write!(f, "Enlightened"),
            Role::Gemcrafter => write!(f, "Gemcrafter"),
            Role::Hunter => write!(f, "Hunter"),
            Role::Judge => write!(f, "Judge"),
            Role::Lover => write!(f, "Lover"),
            Role::Empress => write!(f, "Empress"),
            Role::Wretch => write!(f, "Wretch"),
            Role::Minion => write!(f, "Minion"),
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
#[derive(Debug, Clone)]
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

    /// Always true - matches any statement
    fn equals(&self, _other: &dyn RoleStatement) -> bool {
        true
    }
}


/// Confessor's statement types.
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

// Gemcrafter & Judge statement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimType {
    Good,
    Evil,
    Truthy,
    Lying,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimStatement {
    pub target_index: usize,
    pub claim_type: ClaimType,
}
impl fmt::Display for ClaimStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let claim_str = match self.claim_type {
            ClaimType::Good => "Good",
            ClaimType::Evil => "Evil",
            ClaimType::Truthy => "Truthy",
            ClaimType::Lying => "Lying",
        };

        write!(f, "Player {} is {}", self.target_index, claim_str)
    }
}
impl RoleStatement for ClaimStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<ClaimStatement>()
            .map(|o| o == self)
            .unwrap_or(false)
    }
}

// Lover, Empress, and Hunter statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvilCountStatement {
    pub target_indexes: Vec<usize>,
    pub evil_count: usize, // How many evil are there among the above listed
    pub minimum: bool, // If the found evils are at minimum evil_count (if false it's the exact number)
    pub none_closer: bool, // If true, there are no closer evils to the claimer
}
impl EvilCountStatement {
    fn unordered_indexes_eq(&self, other: &Self) -> bool {
        let mut self_sorted = self.target_indexes.clone();
        let mut other_sorted = other.target_indexes.clone();

        self_sorted.sort_unstable();
        other_sorted.sort_unstable();

        self_sorted == other_sorted
    }
}
impl fmt::Display for EvilCountStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Among {:#?} there are {} Evil", self.target_indexes, self.evil_count)
    }
}
impl RoleStatement for EvilCountStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn equals(&self, other: &dyn RoleStatement) -> bool {
        other
            .as_any()
            .downcast_ref::<EvilCountStatement>()
            .map(|o| self.unordered_indexes_eq(o) && self.evil_count == o.evil_count && self.minimum == o.minimum && self.none_closer == o.none_closer)
            .unwrap_or(false)
    }
}

fn neighbor_indexes(len: usize, position: usize, offset: usize) -> Vec<usize> {
    vec![
        (position + len - offset) % len,
        (position + offset) % len,
    ]
}

fn count_neighbor_evil(true_roles: &[Role], position: usize, offset: usize) -> usize {
    neighbor_indexes(true_roles.len(), position, offset)
        .iter()
        .filter(|&&i| true_roles[i].alignment() == Alignment::Evil)
        .count()
}

// Enlightened statement
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


/// Produce the typed statements a card can make given:
/// - `true_role`: the role it really is from the deck
/// - `visible_role`: what role is shown (may be a disguise, or unrevealed)
/// - `true_roles`: the true roles of all the cards in play
/// - `_position`: the index of the speaking card
pub fn produce_statements(
    true_role: Role,
    visible_role: Option<Role>,
    true_roles: &[Role],
    _position: usize
) -> Vec<Box<dyn RoleStatement>> {
    // If the card is unrevealed, we don't produce any info beyond UnrevealedStatement
    if visible_role.is_none() {
        return vec![Box::new(UnrevealedStatement)];
    }

    if true_role.lying() {
        return match visible_role.unwrap() {
            Role::Confessor => vec![Box::new(ConfessorStatement::IAmDizzy)],
            Role::Enlightened => {
                let true_response = closest_evil_direction(true_roles, _position);
                EnlightenedStatement::iterator()
                    .filter(|&stmt| stmt != true_response)
                    .map(|stmt| Box::new(stmt) as Box<dyn RoleStatement>)
                    .collect()
            }
            Role::Gemcrafter => {
                // Claim all evil players are good
                true_roles
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.alignment() == Alignment::Evil)
                    .map(|(idx, _)| {
                        Box::new(ClaimStatement {
                            target_index: idx,
                            claim_type: ClaimType::Good,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            },
            Role::Hunter => {
                let max_index = (true_roles.len() + 1) / 2;
                let index = (1..=max_index)
                    .find(|&i| count_neighbor_evil(true_roles, _position, i) > 0)
                    .unwrap_or(1);

                (1..=max_index)
                    .filter(|&i| i != index)
                    .map(|i| {
                        Box::new(EvilCountStatement {
                            target_indexes: neighbor_indexes(true_roles.len(), _position, i),
                            evil_count: 1,
                            minimum: true,
                            none_closer: true,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            },
            Role::Judge => {
                // Claim all lying players are truthy, and truthy players are lying
                let mut statements: Vec<Box<dyn RoleStatement>> = true_roles
                    .iter()
                    .enumerate()
                    .map(|(idx, r)| {
                        Box::new(ClaimStatement {
                            target_index: idx,
                            claim_type: if r.lying() { ClaimType::Truthy } else { ClaimType::Lying },
                        }) as Box<dyn RoleStatement>
                    })
                    .collect();
                statements.push(Box::new(UnrevealedStatement));

                statements
            },
            Role::Lover => {
                let neighbors = neighbor_indexes(true_roles.len(), _position, 1);

                let real_evil_count = neighbors
                    .iter()
                    .filter(|&&idx| true_roles[idx].alignment() == Alignment::Evil)
                    .count();

                (0..=2)
                    .filter(|&fake_count| fake_count != real_evil_count)
                    .map(|fake_count| {
                        Box::new(EvilCountStatement {
                            target_indexes: neighbors.clone(),
                            evil_count: fake_count,
                            minimum: false,
                            none_closer: false,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            },
            Role::Empress => {
                let good = true_roles
                    .iter()
                    .enumerate()
                    .filter(|(_, role)| role.alignment() != Alignment::Evil);

                good.combinations(3)
                    .map(move |pair| {
                        let target_indexes = vec![pair[0].0, pair[1].0, pair[2].0];
                        Box::new(EvilCountStatement {
                            target_indexes: target_indexes,
                            evil_count: 1,
                            minimum: false,
                            none_closer: false,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            },
            other => panic!(
                "produce_statements: unsupported role combination: true={:?}, visible={:?}",
                true_role, other
            ),
        };
    }

    return match visible_role.unwrap() {
        Role::Confessor => vec![Box::new(ConfessorStatement::IAmGood)],
        Role::Enlightened => vec![Box::new(closest_evil_direction(true_roles, _position)) as Box<dyn RoleStatement>],
        Role::Gemcrafter => {
            // Claim all villagers are good
            true_roles
                .iter()
                .enumerate()
                .filter(|(_, r)| r.alignment() == Alignment::Good)
                .map(|(idx, _)| {
                    Box::new(ClaimStatement {
                        target_index: idx,
                        claim_type: ClaimType::Good,
                    }) as Box<dyn RoleStatement>
                })
                .collect()
        },
        Role::Hunter => {
            let max_index = (true_roles.len() + 1) / 2;
            let index = (1..=max_index)
                .find(|&i| count_neighbor_evil(true_roles, _position, i) > 0)
                .unwrap_or(1);

            vec![Box::new(EvilCountStatement {
                target_indexes: neighbor_indexes(true_roles.len(), _position, index),
                evil_count: 1,
                minimum: true,
                none_closer: true,
            })]
        },
        Role::Judge => {
            // Claim all lying players are lying, and truthy players are truthy
            let mut statements: Vec<Box<dyn RoleStatement>> = true_roles
                .iter()
                .enumerate()
                .map(|(idx, r)| {
                    Box::new(ClaimStatement {
                        target_index: idx,
                        claim_type: if r.lying() { ClaimType::Lying } else { ClaimType::Truthy},
                    }) as Box<dyn RoleStatement>
                })
                .collect();
            statements.push(Box::new(UnrevealedStatement));

            statements
        },
        Role::Lover => {
            let evil_count = count_neighbor_evil(true_roles, _position, 1);
            vec![Box::new(EvilCountStatement {
                target_indexes: neighbor_indexes(true_roles.len(), _position, 1),
                evil_count: evil_count,
                minimum: false,
                none_closer: false,
            })]
        },
        Role::Empress => {
            let (evil, good): (Vec<_>, Vec<_>) = true_roles
                .iter()
                .enumerate()
                .partition(|(_, r)| r.alignment() == Alignment::Evil);

            evil.iter()
                .flat_map(|(ei, _)| {
                    good.iter()
                        .combinations(2)
                        .map(move |pair| {
                            let target_indexes = vec![*ei, pair[0].0, pair[1].0];
                            Box::new(EvilCountStatement {
                                target_indexes: target_indexes,
                                evil_count: 1,
                                minimum: false,
                                none_closer: false,
                            }) as Box<dyn RoleStatement>
                        })
                })
                .collect()
        },
        Role::Wretch => vec![Box::new(UnrevealedStatement)],
        other => panic!(
            "produce_statements: unsupported role combination: true={:?}, visible={:?}",
            true_role, other
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confessor_says_good() {
        let s = produce_statements(Role::Confessor, Some(Role::Confessor), &[], 0);
        assert!(s[0].equals(&ConfessorStatement::IAmGood));
    }

    #[test]
    fn minion_disguised_says_dizzy() {
        let s = produce_statements(Role::Minion, Some(Role::Confessor), &[], 0);
        assert!(s[0].equals(&ConfessorStatement::IAmDizzy));
    }

    #[test]
    fn claim_statement_equality() {
        let a = ClaimStatement { target_index: 1, claim_type: ClaimType::Evil };
        let b = ClaimStatement { target_index: 1, claim_type: ClaimType::Evil };
        let c = ClaimStatement { target_index: 2, claim_type: ClaimType::Good };
        assert!(a.equals(&b));
        assert!(!a.equals(&c));
    }
}
