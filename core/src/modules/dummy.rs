use std::env;

use rlua::{Context, Table};

pub fn create<'lua>(ctx: Context<'lua>, loaded: Table<'lua>) -> Result<(), rlua::Error> {
    let hua_module = ctx.create_table()?;

    add_version(ctx, &hua_module)?;

    loaded.set("hua", hua_module)?;
    Ok(())
}

fn add_version<'lua>(ctx: Context<'lua>, modules: &Table<'lua>) -> Result<(), rlua::Error> {
    let version = ctx.create_function(|_, ()| {
        let version = env!("CARGO_PKG_VERSION");
        Ok(version.to_string())
    })?;

    modules.set("version", version)?;

    Ok(())
}
