mod app;
mod config_loader;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = config_loader::get_config_from_env();

    let mut app = App::new(app_config)?;

    app.main_loop()?;

    Ok(())
}
