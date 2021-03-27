use iced::pick_list;
use std::{fs, io, path::PathBuf};

use crate::{hashing_algorithm::HashingAlgorithm, message::Message};

#[derive(Default)]
pub struct App {
    hash: Option<Result<String, String>>,
    file_path: PathBuf,
    main_page: iced::scrollable::State,
    algorithm_list: pick_list::State<HashingAlgorithm>,
    file_path_input: iced::text_input::State,
    selected_algorithm: Option<HashingAlgorithm>,
}

impl App {
    fn on_calculate_hash(&mut self) {
        let mut file = io::BufReader::with_capacity(
            page_size::get(),
            match fs::File::open(&self.file_path) {
                Ok(x) => x,
                Err(err) => {
                    self.hash =
                        Some(Err(format!("Cannot open file: {:?}", err)));
                    return;
                },
            },
        );

        let algorithm = match self.selected_algorithm {
            Some(x) => x,
            None => {
                self.hash = Some(Err("No hashing algorithm selected".into()));
                return;
            },
        };

        match algorithm.calculate(&mut file) {
            Ok(hash) => self.hash = Some(Ok(hash)),
            Err(err) => {
                self.hash = Some(Err(format!("Cannot read file: {:?}", err)))
            },
        }
    }
}

impl iced::Sandbox for App {
    type Message = Message;

    fn new() -> Self { Self::default() }

    fn title(&self) -> String {
        let mut title = match self.file_path.file_name() {
            Some(short_name) => {
                short_name.to_string_lossy().to_string() + " - "
            },
            None => String::new(),
        };

        title.push_str("Indiana Hash");

        title
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::HashingAlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
                self.on_calculate_hash();
            },
            Message::FilePathChanged(file_path) => {
                self.file_path = PathBuf::from(file_path);
                self.on_calculate_hash();
            },
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let file_path = iced::TextInput::new(
            &mut self.file_path_input,
            "Path to File",
            &self.file_path.to_string_lossy(),
            Message::FilePathChanged,
        )
        .width(iced::Length::Fill);

        let algorithm_list = iced::PickList::new(
            &mut self.algorithm_list,
            &HashingAlgorithm::ALL[..],
            self.selected_algorithm,
            Message::HashingAlgorithmSelected,
        )
        .width(iced::Length::Fill);

        let mut main_page = iced::Scrollable::new(&mut self.main_page)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(13)
            .padding(17)
            .push(file_path)
            .push(algorithm_list);

        if let Some(ref hashing_result) = self.hash {
            main_page = main_page.push(match hashing_result {
                Ok(hash) => iced::Text::new(format!(
                    "{}: {}",
                    self.selected_algorithm
                        .map(|algorithm| algorithm.to_string())
                        .unwrap_or_else(|| "Hash".into()),
                    hash
                )),
                Err(error_message) => iced::Text::new(error_message)
                    .color(iced::Color::from_rgb8(255, 0, 0)),
            });
        }

        iced::Container::new(main_page)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
