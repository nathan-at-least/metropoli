use wasmi::{ImportsBuilder, Module, ModuleInstance, ModuleRef};

#[derive(Debug)]
pub struct Polis {
    #[allow(dead_code)]
    modref: ModuleRef,
}

impl Polis {
    pub fn instantiate<B>(poliswasm: B) -> anyhow::Result<Self>
    where
        B: AsRef<[u8]>,
    {
        let module = Module::from_buffer(poliswasm)?;
        let nsmr = ModuleInstance::new(&module, &ImportsBuilder::default())?;
        let modref = nsmr.run_start(&mut wasmi::NopExternals)?;
        Ok(Polis { modref })
    }
}
