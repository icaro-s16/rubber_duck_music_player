use std::io;
use msc_app;
    
fn main() -> io::Result<()>{
    _ = ratatui::run(|terminal| msc_app::app::App::default().run(terminal));
    Ok(())
}
