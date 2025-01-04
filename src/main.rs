use cursive::{event::Key, menu, traits::*, views::Dialog};
// use std::fs;
use std::path::Path;

fn main() {
    let mut siv = cursive::default();
    let cwd = "./";
    siv.menubar()
        .add_subtree(
            "File",
            menu::Tree::new()
                // .leaf("Open...", |s| s.add_layer(Dialog::info("Open file!")))
                .leaf("Open...", move |s| {
                    s.add_layer(Dialog::info(list_dir(&cwd)))
                })
                .leaf("Close...", |s| s.add_layer(Dialog::info("Close file!"))),
        )
        .add_subtree(
            "Filter",
            menu::Tree::new().subtree(
                "By...",
                menu::Tree::new().with(|tree| {
                    for i in 1..23 {
                        tree.add_item(menu::Item::leaf(format!("Chromosome {i}"), move |s| {
                            s.add_layer(Dialog::info(format!("Filtering by Chromosome {i}")))
                        }))
                    }
                }),
            ),
        );

    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(Dialog::info(
        "Welcome to my Rust project!\nPress q to exit or Esc to access the menus.\nEnjoy it!",
    ));
    siv.run();
    list_dir(cwd);
}

fn list_dir(path_str: &str) -> String {
    let actual_path = Path::new(&path_str);
    let dirfiles = actual_path.read_dir().unwrap();
    let mut files = Vec::new();
    for entry in dirfiles {
        match entry {
            Ok(entry) => {
                println!("Processing entry: {:?}", entry.path());
                files.push(entry.path().as_os_str().to_str().unwrap().to_string());
            }
            Err(e) => {
                println!("  entry error: {:?}", e);
            }
        }
    }
    files.join("\n")
}
