// mod parser;

use anyhow::Result;
use mlua::{IntoLua, Lua};
use std::path::Path;

pub struct Plua {
    ctx: Lua,
}

impl Plua {
    pub fn new() -> Self {
        Self { ctx: Lua::new() }
    }

    pub fn set_global(&self, name: impl IntoLua, value: impl IntoLua) -> Result<()> {
        self.ctx.globals().set(name, value)?;
        Ok(())
    }

    pub fn preprocess<P: AsRef<Path>>(&self, source: P, destination: P) -> Result<PluaMetaProgram> {
        Ok(PluaMetaProgram::new())
    }
}

pub struct PluaMetaProgram {}

impl PluaMetaProgram {
    fn new() -> Self {
        Self {}
    }

    pub fn compile() -> Result<String> {
        unimplemented!()
    }
}
