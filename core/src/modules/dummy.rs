use rlua::{Context, Table};

pub fn create<'lua>(ctx: Context<'lua>, loaded: Table<'lua>) -> Result<(), rlua::Error> {
    let hua_module = ctx.create_table()?;

    hua_module.set("value1", 42).unwrap();
    hua_module.set("value2", "hello").unwrap();

    loaded.set("hua", hua_module)?;

    Ok(())
}
