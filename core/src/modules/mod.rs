use rlua::{Context, Error, Table};

mod dummy;
mod json;

pub fn populate(ctx: Context) -> Result<(), Error> {
    let globals = ctx.globals();

    let packages: Table = globals.get("package")?;
    let loaded: Table = packages.get("loaded")?;

    #[cfg(debug_assertions)]
    dummy::create(ctx, &loaded)?;
    json::create(ctx, &loaded)?;

    Ok(())
}
