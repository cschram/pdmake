use anyhow::Result;
use mlua::{IntoLua, Lua};
use std::fs::File;
use std::io::BufReader;

pub struct Preprocessor {
    ctx: Lua,
}

impl Preprocessor {
    pub fn new() -> Self {
        Preprocessor { ctx: Lua::new() }
    }

    pub fn set_global(&self, name: impl IntoLua, value: impl IntoLua) -> Result<()> {
        self.ctx.global().set(name, value)
    }

    pub fn process<'a>(&'a self, name: &str, reader: &BufReader<File>) -> Result<MetaProgram<'a>> {}
}

pub struct MetaProgram<'a> {}
