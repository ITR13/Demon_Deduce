// src/roles.rs
use std::any::Any;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Confessor,
    Minion,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Confessor => write!(f, "Confessor"),
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

/// Example of a more general claim statement (useful later):
/// e.g. "Player 1 is Evil" or "Player 2 is Good".
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

/// Produce the typed statement a card would make given:
/// - `true_role`: the role it really is from the deck
/// - `visible_role`: what role is shown (may be a disguise)
///
/// v0.1 minimal rules implemented:
/// - True Confessor -> ConfessorStatement::IAmGood
/// - True Minion disguised as Confessor -> ConfessorStatement::IAmDizzy (lying counterpart)
/// - Other combinations produce a simple ClaimStatement or silent behavior (kept minimal here).
pub fn produce_statement(true_role: Role, visible_role: Role, _position: usize) -> Box<dyn RoleStatement> {
    match true_role {
        Role::Confessor => Box::new(ConfessorStatement::IAmGood),
        Role::Minion => match visible_role {
            Role::Confessor => Box::new(ConfessorStatement::IAmDizzy),
            _ => Box::new(ClaimStatement {
                target_index: 0,
                claims_evil: true,
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confessor_says_good() {
        let s = produce_statement(Role::Confessor, Role::Confessor, 0);
        assert!(s.equals(&ConfessorStatement::IAmGood));
    }

    #[test]
    fn minion_disguised_says_dizzy() {
        let s = produce_statement(Role::Minion, Role::Confessor, 0);
        assert!(s.equals(&ConfessorStatement::IAmDizzy));
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
