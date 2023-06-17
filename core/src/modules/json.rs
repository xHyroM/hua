use rlua::{Context, Table, Value};
use rlua_derive::serialize;

pub fn create<'lua>(ctx: Context<'lua>, loaded: &Table<'lua>) -> Result<(), rlua::Error> {
    let hua_module = ctx.create_table()?;

    add_encode(ctx, &hua_module)?;

    loaded.set("hua:json", hua_module)?;
    Ok(())
}

fn add_encode<'lua>(ctx: Context<'lua>, modules: &Table<'lua>) -> Result<(), rlua::Error> {
    let encode = ctx.create_function(|lua, value: Value| match serialize(value) {
        Ok(s) => lua.create_string(&s),
        Err(e) => Err(rlua::Error::RuntimeError(format!(
            "Failed to serialize value: {}",
            e
        ))),
    })?;

    modules.set("encode", encode)?;

    Ok(())
}
