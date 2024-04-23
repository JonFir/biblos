use settings::Settings;

mod api;
mod settings;

pub async fn run() -> anyhow::Result<()> {
    let settings = Settings::load()?;
    api::server::run(&settings).await?;
    Ok(())
}
