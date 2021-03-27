#![windows_subsystem = "windows"]

pub mod hashing_algorithm;

use iui::{
    controls::{Combobox, Entry, Label, Button, VerticalBox, HorizontalBox},
    prelude::*,
};
use std::{cell::RefCell, fs, io, path::Path, path::PathBuf, rc::Rc};

use crate::hashing_algorithm::HashingAlgorithm;

fn main() {
    // Initialize the basics
    let ui = UI::init().expect("cannot create ui");
    let mut window =
        Window::new(&ui, "Indiana Hash", 200, 200, WindowType::NoMenubar);
    let mut main_view = VerticalBox::new(&ui);
    main_view.set_padded(&ui, true);

    let mut filename_view = HorizontalBox::new(&ui);
    filename_view.set_padded(&ui, true);

    let filename_value = Rc::new(RefCell::new(PathBuf::new()));
    let hashing_algorithm_value = Rc::new(RefCell::new(None));

    let mut filename = Entry::new(&ui);
    let mut filename_selection = Button::new(&ui, " ... ");

    let mut hashing_algorithm = Combobox::new(&ui);
    hashing_algorithm.append(&ui, "None");
    for algorithm in &HashingAlgorithm::ALL {
        hashing_algorithm.append(&ui, algorithm.name());
    }
    // Select "None" as the default algorithm
    hashing_algorithm.set_selected(&ui, 0);

    let status = Label::new(&ui, "Enter a filename and a hashing algorithm");

    let mut calculate_hash = {
        let ui = ui.clone();
        let filename = filename_value.clone();
        let hashing_algorithm = hashing_algorithm_value.clone();
        let mut status = status.clone();
        move || {
            let hashing_algorithm = *hashing_algorithm.borrow();
            status.set_text(
                &ui,
                &match on_calculate_hash(&filename.borrow(), hashing_algorithm)
                {
                    Ok(hash) => hash,
                    Err(err_message) => err_message,
                },
            );
        }
    };

    let mut change_filename = {
        let ui = ui.clone();
        let mut calculate_hash = calculate_hash.clone();
        let mut window = window.clone();
        move |new_filename: PathBuf| {
            // Set window title to contain the file's name to make it easier to
            // see
            match new_filename.file_name() {
                None => window.set_title(&ui, "Indiana Hash"),
                Some(filename) => window.set_title(&ui, &{
                    filename.to_string_lossy().to_string()
                        + " \u{2012} Indiana Hash"
                }),
            }

            *filename_value.borrow_mut() = new_filename;
            calculate_hash();
        }
    };

    filename_selection.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let mut filename = filename.clone();
        let mut change_filename = change_filename.clone();
        move |_| if let Some(path) = window.open_file(&ui) {
            let visible_path = path.to_string_lossy();
            filename.set_value(&ui, &visible_path);
            change_filename(path);
        }
    });

    filename.on_changed(&ui,
        move |new_filename| change_filename(PathBuf::from(new_filename)));

    hashing_algorithm.on_selected(&ui, move |new_algorithm_id| {
        *hashing_algorithm_value.borrow_mut() = match new_algorithm_id {
            0 => None,
            id => Some(HashingAlgorithm::ALL[id as usize - 1]),
        };
        calculate_hash();
    });

    // Add controls to the UI
    filename_view.append(&ui, filename, LayoutStrategy::Stretchy);
    filename_view.append(&ui, filename_selection, LayoutStrategy::Compact);
    main_view.append(&ui, filename_view, LayoutStrategy::Compact);
    main_view.append(&ui, hashing_algorithm, LayoutStrategy::Compact);
    main_view.append(&ui, status, LayoutStrategy::Compact);

    // Show the window
    window.set_child(&ui, main_view);
    window.show(&ui);
    ui.main();
}

fn on_calculate_hash(
    filename: &Path,
    hashing_algorithm: Option<HashingAlgorithm>,
) -> Result<String, String> {
    let mut file = io::BufReader::with_capacity(
        page_size::get(),
        match fs::File::open(filename) {
            Ok(x) => x,
            Err(err) => {
                return Err(format!("Cannot open file: {:?}", err));
            },
        },
    );

    let algorithm = match hashing_algorithm {
        Some(x) => x,
        None => {
            return Err("No hashing algorithm selected".into());
        },
    };

    match algorithm.calculate(&mut file) {
        Ok(hash) => Ok(format!("{}: {}", algorithm, hash)),
        Err(err) => Err(format!("Cannot read file: {:?}", err)),
    }
}
