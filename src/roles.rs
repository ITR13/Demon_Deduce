// src/roles.rs
use std::any::Any;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    // Villager
    Confessor,
    Gemcrafter,
    // Evil
    Minion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Villager,
    Evil,
}

impl Role {
    pub const fn alignment(self) -> Alignment {
        use Role::*;
        match self {
            Confessor | Gemcrafter => Alignment::Villager,
            Minion => Alignment::Evil,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Confessor => write!(f, "Confessor"),
            Role::Gemcrafter => write!(f, "Gemcrafter"),
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

// Gemcrafter statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimStatement {
    pub target_index: usize,
    pub claims_evil: bool, // true => "X is evil", false => "X is good"
}
impl fmt::Display for ClaimStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.claims_evil {
            write!(f, "Player {} is Evil", self.target_index)
        } else {
            write!(f, "Player {} is Good", self.target_index)
        }
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

    match true_role {
        Role::Confessor => vec![Box::new(ConfessorStatement::IAmGood)],
        Role::Gemcrafter => {
            // Claim all villagers are good
            true_roles
                .iter()
                .enumerate()
                .filter(|(_, r)| r.alignment() == Alignment::Villager)
                .map(|(idx, _)| {
                    Box::new(ClaimStatement {
                        target_index: idx,
                        claims_evil: false,
                    }) as Box<dyn RoleStatement>
                })
                .collect()
        }

        Role::Minion => match visible_role.unwrap() {
            Role::Confessor => vec![Box::new(ConfessorStatement::IAmDizzy)],
            Role::Gemcrafter => {
                // Claim all evil players are good
                true_roles
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.alignment() == Alignment::Evil)
                    .map(|(idx, _)| {
                        Box::new(ClaimStatement {
                            target_index: idx,
                            claims_evil: false,
                        }) as Box<dyn RoleStatement>
                    })
                    .collect()
            }
            other => panic!(
                "produce_statements: unsupported role combination: true={:?}, visible={:?}",
                true_role, other
            ),
        },
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
        let a = ClaimStatement { target_index: 1, claims_evil: true };
        let b = ClaimStatement { target_index: 1, claims_evil: true };
        let c = ClaimStatement { target_index: 2, claims_evil: false };
        assert!(a.equals(&b));
        assert!(!a.equals(&c));
    }
}
