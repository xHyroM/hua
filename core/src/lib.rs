use rlua::Lua;

mod modules;

#[derive(Default, Debug)]
pub struct Hua {
    lua: Lua,
}

impl Hua {
    pub fn new() -> Self {
        let lua = Lua::new();

        Hua { lua }
    }

    pub fn evaluate(&mut self, code: &str) -> Result<(), rlua::Error> {
        self.lua.context(|ctx| {
            ctx.load(code).exec()?;
            Ok(())
        })
    }

    pub fn populate(&mut self) -> Result<(), rlua::Error> {
        self.lua.context(modules::populate)
    }
}
