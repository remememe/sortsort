use ratatui::{TerminalOptions, Viewport};
use clap::Parser;
use sortsort::app::App;
use sortsort::cfg::{SortArgs};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = SortArgs::parse();
    
    let viewport = Viewport::Fullscreen;
    let mut terminal = ratatui::init_with_options(TerminalOptions { viewport });
    terminal.clear()?;

    let mut app = App::new(&args.app_conf,&args.bar_conf);
    let res = app.run(&mut terminal);

    if let Err(e) = res {
		eprintln!("{e}");
	}
    
    ratatui::restore();

    terminal.clear()?;

    Ok(())
}