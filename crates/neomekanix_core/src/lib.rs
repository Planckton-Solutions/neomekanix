pub mod events;

pub fn init() -> anyhow::Result<()> {
    log::info!("Initalized Log");
    Ok(())
}