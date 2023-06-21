use mlua::prelude::*;
use mlua::Result;

pub fn create<'lua>(ctx: Lua, loaded: LuaTable) -> Result<()> {
    let hua_module = ctx.create_table()?;

    add_encode(ctx, &hua_module)?;

    loaded.set("hua:json", hua_module)?;
    Ok(())
}

fn add_encode<'lua>(ctx: Lua, modules: LuaTable) -> Result<()> {
    let encode = ctx.create_function(|lua, value: LuaValue| match serialize(value) {
        Ok(s) => lua.create_string(&s),
        Err(e) => Err(rlua::Error::RuntimeError(format!(
            "Failed to serialize value: {}",
            e
        ))),
    })?;

    modules.set("encode", encode)?;

    Ok(())
}
