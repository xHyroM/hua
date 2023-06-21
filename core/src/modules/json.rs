use mlua::prelude::*;
use mlua::{Error, Result};

pub fn create<'lua>(ctx: &Lua, loaded: &LuaTable) -> Result<()> {
    let hua_module = ctx.create_table()?;

    add_encode(ctx, &hua_module)?;

    loaded.set("hua:json", hua_module)?;
    Ok(())
}

fn add_encode(ctx: &Lua, modules: &LuaTable) -> Result<()> {
    let encode = ctx.create_function(|lua, value: LuaValue| {
        match serde_json::to_string(&value) {
            Ok(serialized) => return lua.create_string(&serialized),
            Err(_) => return Err(Error::SerializeError(String::from("jhe"))),
        };
    })?;

    modules.set("encode", encode)?;

    Ok(())
}
