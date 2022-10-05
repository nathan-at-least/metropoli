use crate::Polis;

#[test]
fn instantiate_empty_wasm() -> anyhow::Result<()> {
    let wasm: Vec<u8> = wabt::wat2wasm(
        r#"
            (module)
            "#,
    )?;

    let _polis = Polis::instantiate(wasm)?;
    Ok(())
}
