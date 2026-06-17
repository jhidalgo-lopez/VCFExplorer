mod tui;
mod vcf;
use simplelog::*;
use std::fs::OpenOptions;

fn main() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("vcf_explorer.log")
            .unwrap(),
    )])
    .unwrap();

    let mut siv = tui::add_ui();
    siv.run();
}
