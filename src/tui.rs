use crate::vcf::{filter_vcf, FilterConfig, VcfRecord};
use cursive::align::HAlign;
use cursive::traits::Nameable;
use cursive::CursiveRunnable;
use cursive::{
    event::Key,
    menu,
    traits::*,
    views::{Dialog, EditView, LinearLayout, ScrollView, SelectView, TextView},
};
use std::path::Path;

// Holds the application's state.
#[derive(Default)]
struct AppState {
    // A list of paths to the VCF files that are currently open.
    file_paths: Vec<String>,
    // All VCF records from all opened files, before any filtering.
    all_records: Vec<VcfRecord>,
    // The VCF records that are currently being displayed after filtering.
    displayed_records: Vec<VcfRecord>,
    // The current filter configuration.
    filters: FilterConfig,
}

// UI Section

// Creates a dialog box that allows the user to select an opened file to close.
fn close_box(s: &mut cursive::Cursive) {
    // Create a SelectView to list the currently opened files.
    let mut select_view = SelectView::new().h_align(HAlign::Center);
    let app_state = s.user_data::<AppState>().unwrap();
    select_view.add_all_str(app_state.file_paths.clone());
    // When a file is selected, it's removed from the app state,
    // and the VCF records are reloaded from the remaining files.
    select_view.set_on_submit(move |s, selected: &str| {
        let app_state = s.user_data::<AppState>().unwrap();
        // Remove the selected file from the list of file paths.
        app_state.file_paths.retain(|element| element != selected);
        // Re-read all VCF records from the remaining open files.
        let mut vcf_data_all: Vec<VcfRecord> = Vec::new();
        for file in &app_state.file_paths {
            vcf_data_all.append(&mut crate::vcf::read_vcf(file));
        }
        app_state.all_records = vcf_data_all;
        app_state.displayed_records = app_state.all_records.clone();
        // Close the dialog and update the main VCF view.
        s.pop_layer();
        update_vcf_view(s);
    });
    // Add the dialog to the Cursive root.
    s.add_layer(
        Dialog::around(select_view)
            .title("Close file:")
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

// Creates a dialog for browsing the file system to open a VCF file.
fn create_dir_box(s: &mut cursive::Cursive, initial_path: &str) {
    let path = initial_path.to_string();
    let mut select_view = SelectView::new().h_align(HAlign::Center);
    // Populate the SelectView with the contents of the initial directory.
    select_view.add_all_str(list_dir(initial_path));

    // Define the action to take when a file or directory is selected.
    select_view.set_on_submit(move |s, selected: &str| {
        let app_state = s.user_data::<AppState>().unwrap();

        // Handle navigating up to the parent directory.
        if selected == ".." {
            if let Some(parent) =
                Path::new(&std::path::absolute(&path).expect("ERROR: Could not get Path!")).parent()
            {
                let new_path = parent.to_str().unwrap();
                s.pop_layer(); // Close the current directory view.
                create_dir_box(s, new_path); // Open the parent directory view.
            }
        } else {
            let selected_path = Path::new(&path).join(selected);
            // If a directory is selected, navigate into it.
            if selected_path.is_dir() {
                s.pop_layer();
                create_dir_box(s, selected_path.to_str().unwrap());
            } else {
                // If a file is selected, add it to the app state and read its VCF records.
                let text = format!("You opened file: {}", selected_path.display());
                app_state
                    .file_paths
                    .push(selected_path.to_str().unwrap().into());
                // Re-read VCF data from all currently opened files.
                let mut vcf_data_all: Vec<VcfRecord> = Vec::new();
                for file in &app_state.file_paths {
                    vcf_data_all.append(&mut crate::vcf::read_vcf(file));
                }
                app_state.all_records = vcf_data_all;
                app_state.displayed_records = app_state.all_records.clone();
                // Show a confirmation dialog.
                s.add_layer(Dialog::around(TextView::new(text)).button("OK", |s| {
                    s.pop_layer(); // Close confirmation dialog.
                    s.pop_layer(); // Close file browser dialog.
                    update_vcf_view(s); // Refresh the main VCF view.
                }));
            }
        }
    });

    // Add the file browser dialog to the Cursive root.
    s.add_layer(
        Dialog::around(select_view)
            .title(format!("Browsing: {}", initial_path))
            .button("Cancel", |s| {
                s.pop_layer();
            })
            .with_name("open_file_box"),
    );
}

// Refreshes the main view that displays the VCF data table.
fn update_vcf_view(s: &mut cursive::Cursive) {
    let app_state = s.user_data::<AppState>().unwrap();
    // Re-create the table with the currently displayed records.
    let table_view = create_table(&app_state.displayed_records);
    // Pop the old VCF view layer and add the new one.
    s.pop_layer();
    s.add_layer(
        Dialog::around(ScrollView::new(table_view))
            .title("VCF Viewer")
            .full_screen()
            .with_name("vcf_data"),
    );
}

// Lists the contents of a directory, including a ".." entry for navigation.
fn list_dir(path_str: &str) -> Vec<String> {
    let path = Path::new(path_str);
    let mut entries = vec!["..".to_string()]; // Always add parent directory option.
    match path.read_dir() {
        Ok(_read_dir) => {
            for entry in _read_dir.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    entries.push(name.to_string());
                }
            }
        }
        Err(_) => {
            // Log an error if the directory can't be read.
            log::error!("\nThe directory couldn't be read.");
        }
    };
    entries
}

// Initializes and configures the Cursive TUI.
pub fn add_ui() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.set_autorefresh(true); // Ensure the UI redraws automatically.
                               // Initialize the application state and store it in Cursive's user data.
    let app_state = AppState::default();
    let table_view = create_table(&app_state.displayed_records);
    siv.set_user_data(app_state);
    // Add the main VCF viewer dialog, which is initially empty.
    siv.add_layer(
        Dialog::around(ScrollView::new(table_view))
            .title("VCF Viewer")
            .full_screen()
            .with_name("vcf_data"),
    );
    // Set the initial directory for the file browser.
    let cwd = ".";
    // Create the main menu bar.
    siv.menubar()
        .add_subtree(
            "File",
            menu::Tree::new()
                // "Open" menu item to launch the file browser.
                .leaf("Open...", move |s| {
                    create_dir_box(s, cwd);
                })
                // "Close" menu item to close an opened file.
                .leaf("Close...", move |s: &mut cursive::Cursive| {
                    let app_state = s.user_data::<AppState>().unwrap();
                    if app_state.file_paths.is_empty() {
                        s.add_layer(Dialog::info("No file opened"));
                    } else {
                        close_box(s);
                    }
                }),
        )
        .add_subtree(
            "Filter",
            menu::Tree::new()
                // "Chromosome" filter menu item.
                .leaf("Chromosome", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Filter by Chromosome")
                            .content(
                                LinearLayout::vertical()
                                    .child(TextView::new("Enter chromosome:"))
                                    .child(
                                        EditView::new()
                                            .on_submit(move |s, chr| {
                                                // On submit, update the filter and refresh the view.
                                                let app_state = s.user_data::<AppState>().unwrap();
                                                app_state.filters.chr = Some(chr.to_string());
                                                let filtered_records = filter_vcf(
                                                    &app_state.all_records,
                                                    &app_state.filters,
                                                );
                                                app_state.displayed_records = filtered_records;
                                                s.pop_layer();
                                                update_vcf_view(s);
                                            })
                                            .with_name("filter_chr"),
                                    ),
                            )
                            .button("Cancel", |s| {
                                s.pop_layer();
                            }),
                    );
                })
                // "Position" filter menu item.
                .leaf("Position", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Filter by Position")
                            .content(
                                LinearLayout::vertical()
                                    .child(TextView::new("Enter start position:"))
                                    .child(EditView::new().with_name("pos_start"))
                                    .child(TextView::new("Enter end position:"))
                                    .child(EditView::new().with_name("pos_end")),
                            )
                            .button("Filter", |s| {
                                // Get start and end positions from the EditViews.
                                let pos_start = s
                                    .call_on_name("pos_start", |view: &mut EditView| {
                                        view.get_content().parse::<i64>()
                                    })
                                    .unwrap();
                                let pos_end = s
                                    .call_on_name("pos_end", |view: &mut EditView| {
                                        view.get_content().parse::<i64>()
                                    })
                                    .unwrap();

                                // Apply the filter if the input is valid.
                                if let (Ok(start), Ok(end)) = (pos_start, pos_end) {
                                    let app_state = s.user_data::<AppState>().unwrap();
                                    app_state.filters.pos = Some((start, end));
                                    let filtered_records =
                                        filter_vcf(&app_state.all_records, &app_state.filters);
                                    app_state.displayed_records = filtered_records;
                                    s.pop_layer();
                                    update_vcf_view(s);
                                } else {
                                    s.add_layer(Dialog::info("Invalid position format!"));
                                }
                            })
                            .button("Cancel", |s| {
                                s.pop_layer();
                            }),
                    );
                })
                // "Quality" filter menu item.
                .leaf("Quality", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Filter by Quality")
                            .content(
                                LinearLayout::vertical()
                                    .child(TextView::new("Enter minimum quality:"))
                                    .child(EditView::new().with_name("min_qual"))
                                    .child(TextView::new("Enter maximum quality (optional):"))
                                    .child(EditView::new().with_name("max_qual")),
                            )
                            .button("Filter", |s| {
                                // Get min and max quality values.
                                let min_qual = s
                                    .call_on_name("min_qual", |view: &mut EditView| {
                                        view.get_content().parse::<f32>()
                                    })
                                    .unwrap();

                                let max_qual = s
                                    .call_on_name("max_qual", |view: &mut EditView| {
                                        view.get_content().parse::<f32>()
                                    })
                                    .unwrap();

                                // Apply the filter if the minimum quality is valid.
                                if let Ok(min) = min_qual {
                                    let max = max_qual.ok(); // Max quality is optional.
                                    let app_state = s.user_data::<AppState>().unwrap();
                                    app_state.filters.qual = Some((min, max));
                                    let filtered_records =
                                        filter_vcf(&app_state.all_records, &app_state.filters);
                                    app_state.displayed_records = filtered_records;
                                    s.pop_layer();
                                    update_vcf_view(s);
                                } else {
                                    s.add_layer(Dialog::info("Invalid quality format!"));
                                }
                            })
                            .button("Cancel", |s| {
                                s.pop_layer();
                            }),
                    );
                })
                // "Genotype" filter menu item.
                .leaf("Genotype", |s| {
                    s.add_layer(
                        Dialog::new()
                            .title("Filter by Genotype")
                            .content(
                                LinearLayout::vertical()
                                    .child(TextView::new("Reference allele (optional):"))
                                    .child(EditView::new().with_name("ref_allele"))
                                    .child(TextView::new("Alternative allele (optional):"))
                                    .child(EditView::new().with_name("alt_allele")),
                            )
                            .button("Filter", |s| {
                                // Get ref_allele and alt_allele values.
                                let ref_allele = s
                                    .call_on_name("ref_allele", |view: &mut EditView| {
                                        view.get_content()
                                    })
                                    .unwrap();

                                let alt_allele = s
                                    .call_on_name("alt_allele", |view: &mut EditView| {
                                        view.get_content()
                                    })
                                    .unwrap_or_default();

                                // Apply the filter if reference or alternative allelle is provided
                                if !ref_allele.is_empty() || !alt_allele.is_empty() {
                                    let app_state = s.user_data::<AppState>().unwrap();
                                    if !ref_allele.is_empty() {
                                        app_state.filters.ref_allele = Some(ref_allele.to_string());
                                    } else {
                                        app_state.filters.ref_allele = None;
                                    }
                                    if !alt_allele.is_empty() {
                                        app_state.filters.alt_allele = Some(alt_allele.to_string());
                                    } else {
                                        app_state.filters.alt_allele = None;
                                    }
                                    let filtered_records =
                                        filter_vcf(&app_state.all_records, &app_state.filters);
                                    app_state.displayed_records = filtered_records;
                                    s.pop_layer();
                                    update_vcf_view(s);
                                } else {
                                    s.add_layer(Dialog::info("Invalid genotype format!"));
                                }
                            })
                            .button("Cancel", |s| {
                                s.pop_layer();
                            }),
                    );
                })
                // "Clear All" menu item to reset all filters.
                .leaf("Clear All", |s| {
                    let app_state = s.user_data::<AppState>().unwrap();
                    app_state.filters = FilterConfig::default();
                    app_state.displayed_records = app_state.all_records.clone();
                    update_vcf_view(s);
                }),
        );
    // Configure global keybindings.
    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Esc, |s| s.select_menubar()); // Esc to focus the menu.
    siv.add_global_callback('q', |s| s.quit()); // 'q' to quit the application.
                                                // Show a welcome message on startup.
    siv.add_layer(Dialog::info(
        "Welcome to my Rust project!\nPress q to exit or Esc to access the menus.\nEnjoy it!",
    ));
    siv
}

// Creates a LinearLayout that acts as a table to display VCF records.
pub fn create_table(records: &[VcfRecord]) -> LinearLayout {
    let mut layout = LinearLayout::vertical();
    // Add the table header with fixed-width columns.
    layout.add_child(
        LinearLayout::horizontal()
            .child(
                TextView::new("Chromosome")
                    .h_align(HAlign::Center)
                    .fixed_width(11),
            )
            .child(
                TextView::new("Position")
                    .h_align(HAlign::Center)
                    .fixed_width(11),
            )
            .child(TextView::new("ID").h_align(HAlign::Center).fixed_width(5))
            .child(
                TextView::new("Quality")
                    .h_align(HAlign::Center)
                    .fixed_width(10),
            )
            .child(
                TextView::new("Ref Allele")
                    .h_align(HAlign::Center)
                    .fixed_width(20),
            )
            .child(
                TextView::new("Alt Allele")
                    .h_align(HAlign::Center)
                    .fixed_width(20),
            ),
    );

    // Add a separator line below the header.
    layout.add_child(TextView::new("-".repeat(80)));

    // Add each VCF record as a new row in the layout.
    for record in records {
        layout.add_child(
            LinearLayout::horizontal()
                .child(
                    TextView::new(record.chromosome.clone())
                        .h_align(HAlign::Center)
                        .min_width(11),
                )
                .child(
                    TextView::new(record.position.to_string())
                        .h_align(HAlign::Center)
                        .min_width(11),
                )
                .child(
                    TextView::new(
                        String::from_utf8(record.id.clone()).expect("Could not read entry ID."),
                    )
                    .h_align(HAlign::Center)
                    .min_width(5),
                )
                .child(
                    TextView::new(record.quality.to_string())
                        .h_align(HAlign::Center)
                        .min_width(10),
                )
                .child(
                    TextView::new(record.ref_allele.clone())
                        .h_align(HAlign::Center)
                        .min_width(20),
                )
                .child(
                    TextView::new(record.alt_allele.clone())
                        .h_align(HAlign::Center)
                        .min_width(20),
                ),
        );
    }

    layout
}
