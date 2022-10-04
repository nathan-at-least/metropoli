//! This crate provides commandline option parsing and commands for metropoli, and comes with a thin executable, `metropoli` which wraps [host_main]
mod exec;
mod hostmain;
pub mod options;

pub use self::exec::Executable;
pub use self::hostmain::host_main;
pub use self::options::Options;
