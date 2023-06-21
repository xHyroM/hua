use mlua::prelude::*;
use mlua::Result;

mod modules;

#[derive(Debug)]
pub struct Hua {
    lua: Lua,
}

impl Hua {
    pub fn new() -> Self {
        let lua = Lua::new();

        Hua { lua }
    }

    pub fn evaluate(&mut self, code: &str) -> Result<()> {
        self.lua.load(code).exec()
    }

    pub fn populate(&mut self) -> Result<()> {
        modules::populate(&self.lua)
    }
}
