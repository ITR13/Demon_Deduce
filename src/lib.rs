pub mod roles;
pub mod runner;
pub mod solver;

pub use roles::{Role, RoleStatement};
pub use runner::{run_args, run_clipboard_loop, run_from_clipboard};
pub use solver::brute_force_solve;
