use cursive::align::HAlign;
use cursive::{
    event::Key,
    menu,
    traits::*,
    views::{Dialog, SelectView, TextView},
};
use std::path::Path;

fn main() {
    let mut siv = cursive::default();
    let cwd = ".";
    siv.menubar()
        .add_subtree(
            "File",
            menu::Tree::new()
                .leaf("Open...", move |s| {
                    s.add_layer(create_dir_box(cwd));
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
                        }));
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
}

fn create_dir_box(initial_path: &str) -> Dialog {
    let path = initial_path.to_string();
    let mut select_view = SelectView::new().h_align(HAlign::Center);
    select_view.add_all_str(list_dir(initial_path));
    select_view.set_on_submit(move |s, selected: &str| {
        if selected == ".." {
            if let Some(parent) = Path::new(&path).parent() {
                let new_path = parent.to_str().unwrap();
                s.pop_layer();
                s.add_layer(create_dir_box(new_path));
            }
        } else {
            let selected_path = Path::new(&path).join(selected);
            if selected_path.is_dir() {
                s.pop_layer();
                s.add_layer(create_dir_box(selected_path.to_str().unwrap()));
            } else {
                let text = format!("You opened file: {}", selected_path.display());
                s.add_layer(Dialog::around(TextView::new(text)).button("OK", |s| {
                    s.pop_layer();
                    s.pop_layer();
                }));
            }
        }
    });
    Dialog::around(select_view)
        .title(format!("Browsing: {}", initial_path))
        .button("Cancel", |s| {
            s.pop_layer();
        })
}

fn list_dir(path_str: &str) -> Vec<String> {
    let path = Path::new(path_str);
    let mut entries = vec!["..".to_string()];
    if let Ok(read_dir) = path.read_dir() {
        for entry in read_dir.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                entries.push(name.to_string());
            }
        }
    }
    entries
}
