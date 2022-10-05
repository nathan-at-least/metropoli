use std::path::Path;

pub fn run_polis<P>(poliswasm: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    use metropoli_polis::Polis;

    let pwbytes = std::fs::read(poliswasm)?;
    let polis = Polis::instantiate(pwbytes)?;
    todo!("{:#?}", polis);
}
