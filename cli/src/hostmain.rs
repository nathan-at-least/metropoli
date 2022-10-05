use crate::{Executable, Options};

/// Run the main process which parses args then executes the host
pub fn host_main() -> anyhow::Result<()> {
    Options::parse().execute()
}
