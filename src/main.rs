use cursive::align::HAlign;
use cursive::traits::Nameable;
use cursive::CursiveRunnable;
use cursive::{
    event::Key,
    menu,
    traits::*,
    views::{Dialog, LinearLayout, ScrollView, SelectView, TextView},
};
use rust_htslib::bcf::{Read, Reader};
use std::path::Path;

fn main() {
    let mut siv = add_ui();
    siv.run();
}

// UI Section
fn close_box(file_list: Vec<String>) -> (Dialog, Vec<String>) {
    let mut select_view = SelectView::new().h_align(HAlign::Center);
    let new_file_list: Vec<String> = file_list.clone();
    select_view.add_all_str(file_list);
    select_view.set_on_submit(move |s, selected: &str| {
        let mut new_file_list = s.take_user_data::<Vec<String>>().unwrap_or_default();
        new_file_list.retain(|element| element != selected);
        s.pop_layer();
        s.set_user_data(new_file_list);
    });
    (
        Dialog::around(select_view)
            .title("Close file:")
            .button("Cancel", |s| {
                s.pop_layer();
            }),
        new_file_list,
    )
}

fn create_dir_box(initial_path: &str, file_list: Vec<String>) -> (Dialog, Vec<String>) {
    let path = initial_path.to_string();
    let mut select_view = SelectView::new().h_align(HAlign::Center);
    let new_file_list = file_list;
    select_view.add_all_str(list_dir(initial_path));

    select_view.set_on_submit(move |s, selected: &str| {
        let mut new_file_list = s.take_user_data::<Vec<String>>().unwrap_or_default();

        if selected == ".." {
            if let Some(parent) =
                Path::new(&std::path::absolute(&path).expect("ERROR: Could not get Path!")).parent()
            {
                let new_path = parent.to_str().unwrap();
                s.pop_layer();
                let (new_layer, updated_file_list) = create_dir_box(new_path, new_file_list);
                s.set_user_data(updated_file_list);
                s.add_layer(new_layer.with_name("open_file_box"));
            }
        } else {
            let selected_path = Path::new(&path).join(selected);
            if selected_path.is_dir() {
                s.pop_layer();
                let (new_layer, updated_file_list) =
                    create_dir_box(selected_path.to_str().unwrap(), new_file_list);
                s.set_user_data(updated_file_list);
                s.add_layer(new_layer.with_name("open_file_box"));
            } else {
                let text = format!("You opened file: {}", selected_path.display());
                new_file_list.push(selected_path.to_str().unwrap().into());
                s.set_user_data(new_file_list);
                s.add_layer(Dialog::around(TextView::new(text)).button("OK", |s| {
                    let new_file_list = s.take_user_data::<Vec<String>>().unwrap_or_default();
                    s.pop_layer();
                    s.pop_layer();
                    // s.pop_layer();
                    let table_view = update_table(&new_file_list);
                    s.set_user_data(new_file_list);
                    s.add_layer(
                        Dialog::around(ScrollView::new(table_view))
                            .title("VCF Viewer")
                            .full_screen()
                            .with_name("vcf_data"),
                    );
                }));
            }
        }
    });

    (
        Dialog::around(select_view)
            .title(format!("Browsing: {}", initial_path))
            .button("Cancel", |s| {
                s.pop_layer();
            }),
        new_file_list,
    )
}
fn update_table(file_list: &Vec<String>) -> LinearLayout {
    // Table view
    let mut vcf_data_all: Vec<VcfRecord> = Vec::new();
    for file in file_list {
        vcf_data_all.append(&mut read_vcf(file));
    }
    create_table(vcf_data_all)
}
fn list_dir(path_str: &str) -> Vec<String> {
    let path = Path::new(path_str);
    let mut entries = vec!["..".to_string()];
    match path.read_dir() {
        Ok(_read_dir) => {
            for entry in _read_dir.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    entries.push(name.to_string());
                }
            }
        }
        Err(_) => {
            println!("Test");
        }
    };
    entries
}

fn add_ui() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.set_autorefresh(true);
    siv.set_user_data(Vec::<String>::new());
    // Table view
    let table_view = create_table(Vec::new());
    siv.add_layer(
        Dialog::around(ScrollView::new(table_view))
            .title("VCF Viewer")
            .full_screen(),
    );
    let cwd = ".";
    siv.menubar()
        .add_subtree(
            "File",
            menu::Tree::new()
                .leaf("Open...", move |s| {
                    let file_list = s.take_user_data::<Vec<String>>().unwrap_or_default();
                    let (new_layer, updated_file_list) = create_dir_box(cwd, file_list);
                    s.set_user_data(updated_file_list);
                    s.add_layer(new_layer);
                })
                .leaf("Close...", move |s: &mut cursive::Cursive| {
                    let file_list = s.take_user_data::<Vec<String>>().unwrap_or_default();
                    if file_list.is_empty() {
                        s.add_layer(Dialog::info("No file opened"));
                    } else {
                        let (new_layer, updated_file_list) = close_box(file_list);
                        s.set_user_data(updated_file_list);
                        s.add_layer(new_layer);
                    }
                }),
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
    siv
}

fn create_table(records: Vec<VcfRecord>) -> LinearLayout {
    let mut layout = LinearLayout::vertical();
    // Add the header
    layout.add_child(
        LinearLayout::horizontal()
            .child(
                TextView::new("Chromosome")
                    .h_align(HAlign::Center)
                    .fixed_width(15),
            )
            .child(
                TextView::new("Position")
                    .h_align(HAlign::Center)
                    .fixed_width(10),
            )
            .child(TextView::new("ID").h_align(HAlign::Center).fixed_width(20))
            .child(
                TextView::new("Ref Allele")
                    .h_align(HAlign::Center)
                    .fixed_width(10),
            )
            .child(
                TextView::new("Alt Allele")
                    .h_align(HAlign::Center)
                    .fixed_width(10),
            ),
    );

    layout.add_child(TextView::new("-".repeat(70)));

    // Add records to the table
    for record in records {
        layout.add_child(
            LinearLayout::horizontal()
                .child(TextView::new(record.chromosome).min_width(10))
                .child(TextView::new(record.position.to_string()).min_width(15))
                .child(TextView::new(record.id.to_string()).min_width(10))
                .child(TextView::new(record.ref_allele).min_width(30))
                .child(TextView::new(record.alt_allele).min_width(30)),
        );
    }

    layout
}

// VCF SECTION
// Struct for storing VCF record information
#[derive(Debug)]
struct VcfRecord {
    chromosome: String,
    position: i64,
    id: u32,
    ref_allele: String,
    alt_allele: String,
}

// Function to read VCF data
fn read_vcf(path_str: &str) -> Vec<VcfRecord> {
    let mut records = Vec::new();
    let mut bcf = Reader::from_path(path_str).expect("Error opening file: {path_str:?}.");
    for record in bcf.records().flatten() {
        let mut chromosome = String::new();
        if let Ok(chr) = record.header().rid2name(record.rid().unwrap()) {
            for c in chr {
                chromosome.push(*c as char);
            }
        }

        let position = record.pos();
        let id = record.rid().unwrap();
        let mut ref_allele = String::new();
        for allele in record.alleles()[0] {
            ref_allele.push(char::from(*allele))
        }
        let mut alt_allele = String::new();
        for allele in record.alleles()[1] {
            alt_allele.push(char::from(*allele))
        }
        let entry = VcfRecord {
            chromosome,
            position,
            id,
            ref_allele,
            alt_allele,
        };
        records.push(entry);
    }
    records
}
