mod tui;
mod vcf;
use simplelog::*;
use std::fs::File;

fn main() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("vcf_explorer.log").unwrap(),
    )])
    .unwrap();

    let mut siv = tui::add_ui();
    siv.run();
}
