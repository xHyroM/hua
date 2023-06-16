use rlua::{Context, Error, Table};

mod dummy;

pub fn populate(ctx: Context) -> Result<(), Error> {
    let globals = ctx.globals();

    let packages: Table = globals.get("package")?;
    let loaded: Table = packages.get("loaded")?;

    dummy::create(ctx, loaded)?;

    Ok(())
}
