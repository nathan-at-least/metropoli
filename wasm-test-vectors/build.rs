const WASM_TARGET: &str = "wasm32-unknown-unknown";
const WASM_PAYLOADS: &str = "wasm-payloads";

fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use std::path::PathBuf;
    use std::process::Command;

    let mut cmd = Command::new("cargo");
    cmd.current_dir("wasm");
    cmd.args(["build", "--target", WASM_TARGET]);
    eprintln!("Executing: {:?}", &cmd);

    let status = cmd.status()?;
    assert!(status.success());

    let targetdir = PathBuf::from("wasm/target");
    let builddir = targetdir.join(WASM_TARGET).join("debug");
    let destdir = targetdir.join(WASM_PAYLOADS);

    std::fs::remove_dir_all(&destdir)
        .or_else(|e| {
            use std::io::ErrorKind::NotFound;

            // If it doesn't exist, that's ok:
            match e.kind() {
                NotFound => Ok(()),
                _ => Err(e),
            }
        })
        .with_context(|| format!("remove_dir_all {:?}", destdir.display()))?;
    std::fs::create_dir(&destdir).with_context(|| format!("create_dir {:?}", destdir.display()))?;

    for direntres in builddir
        .read_dir()
        .with_context(|| format!("read_dir {:?}", builddir.display()))?
    {
        let dirent = direntres?;
        let srcpath = dirent.path();
        if srcpath.to_str().unwrap_or("").ends_with(".wasm") {
            let dstpath = destdir.join(srcpath.file_name().expect("missing filename"));
            eprint!("Copying {:?} -> {:?}...", &srcpath, &dstpath);
            let bytes = std::fs::copy(&srcpath, &dstpath).with_context(|| {
                format!("cp {:?} -> {:?}", srcpath.display(), dstpath.display())
            })?;
            eprintln!(" {} bytes", bytes);
        }
    }

    Ok(())
}
