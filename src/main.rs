mod tui;
mod vcf;

fn main() {
    let mut siv = tui::add_ui();
    siv.run();
}
