use crate::Options;

/// Run the main process which parses args then executes the host
pub fn host_main() -> anyhow::Result<()> {
    let opts = Options::parse();
    todo!("{:#?}", opts)
}
