use mlua::prelude::*;
use mlua::Result;

mod dummy;
mod json;

pub fn populate(ctx: Lua) -> Result<()> {
    let globals = ctx.globals();

    let packages: LuaTable = globals.get("package")?;
    let loaded: LuaTable = packages.get("loaded")?;

    #[cfg(debug_assertions)]
    dummy::create(ctx, &loaded)?;
    json::create(ctx, &loaded)?;

    Ok(())
}
