use rlua::{InitFlags, Lua, StdLib};

mod modules;

#[derive(Default, Debug)]
pub struct Hua {
    lua: Lua,
}

impl Hua {
    pub fn new() -> Self {
        let lua = unsafe {
            Lua::unsafe_new_with_flags(
                StdLib::ALL_NO_DEBUG, // Load all Lua standard libraries apart from debug
                InitFlags::DEFAULT - InitFlags::REMOVE_LOADLIB, // Allow loading binary libraries
            )
        };

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
