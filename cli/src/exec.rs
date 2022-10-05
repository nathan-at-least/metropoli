use crate::options::{Command, Options, RunOptions};

/// Values which can be executed, implemented for all [options](crate::options) types
///
/// # Example
///
/// ```should_fail
/// # fn main() -> anyhow::Result<()> {
/// use std::path::PathBuf;
/// use metropoli_host_cli::Executable;
/// use metropoli_host_cli::options::RunOptions;
///
/// RunOptions {
///     path: PathBuf::from("example.polis"),
/// }.execute()
/// # }
/// ```
pub trait Executable {
    fn execute(self) -> anyhow::Result<()>;
}

impl Executable for Options {
    fn execute(self) -> anyhow::Result<()> {
        self.command.execute()
    }
}

impl Executable for Command {
    fn execute(self) -> anyhow::Result<()> {
        use Command::*;

        match self {
            Run(x) => x.execute(),
        }
    }
}

impl Executable for RunOptions {
    fn execute(self) -> anyhow::Result<()> {
        use metropoli_host::run_polis;

        run_polis(self.path)
    }
}
