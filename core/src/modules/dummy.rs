use mlua::prelude::*;
use mlua::Result;

use std::env;

pub fn create<'lua>(ctx: Lua, loaded: LuaTable) -> Result<()> {
    let hua_module = ctx.create_table()?;

    add_version(ctx, &hua_module)?;

    loaded.set("hua:dummy", hua_module)?;
    Ok(())
}

fn add_version(ctx: Lua, modules: &LuaTable) -> Result<()> {
    let version = ctx.create_function(|_, ()| {
        let version = env!("CARGO_PKG_VERSION");
        Ok(version.to_string())
    })?;

    modules.set("version", version)?;

    Ok(())
}
