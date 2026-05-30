use clap::{Parser};
use crate::app::SortingAlgorithm;
#[derive(Parser, Debug)]
#[command(version, about, long_about = "CLI sorting algorithm visualizer")]
pub struct SortArgs {
	#[command(flatten)]
	pub app_conf: AppOptions,

	#[command(flatten)]
	pub bar_conf: BarOptions,
}
#[derive(Debug, Clone, Parser)]
pub struct AppOptions {
    /// Number of elements to sort
    #[arg(short, long, default_value_t = 4)]
    pub amount: usize,

    /// Enable border
    #[arg(short, long, default_value_t = false)]
    pub border: bool,

    /// Border color in RGB format R,G,B
    #[arg(
        short = 'c',
        long = "color",
        value_parser = parse_rgb,
        default_value = "185,99,100"
    )]
    pub border_color: [u8; 3],

    /// Loop the program
    #[arg(short, long, default_value_t = false)]
    pub looped: bool,

    /// Display sorting statistics
    #[arg(short, long, default_value_t = false)]
    pub info: bool,

    /// Sorting method
    #[arg(short = 's', long = "algorithm", default_value_t = SortingAlgorithm::Random, ignore_case = true)]
	pub sorting_algorithm: SortingAlgorithm,
}
#[derive(Debug, Clone, Parser)]
pub struct BarOptions {
    /// Width of each bar
    #[arg(long="width")]
    pub bar_width: Option<u16>,
}
fn parse_rgb(s: &str) -> Result<[u8; 3], String> {
    let parts: Vec<_> = s.split(',').collect();

    if parts.len() != 3 {
        return Err("Format: R,G,B".into());
    }

    let r = parts[0].parse::<u8>().map_err(|_| "invalid red")?;
    let g = parts[1].parse::<u8>().map_err(|_| "invalid green")?;
    let b = parts[2].parse::<u8>().map_err(|_| "invalid blue")?;

    Ok([r, g, b])
}