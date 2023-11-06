use iced::Application;
use iced::{
    executor, theme::Theme, widget::button, widget::Button, widget::Column, widget::Container,
    widget::Text, Command, Element, Length,
};

use tinyfiledialogs::open_file_dialog;

use std::env;

use crate::handle_docker::launch_analysis;

pub struct VirusAnalyzer {
    upload_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileDropped,
}

impl Application for VirusAnalyzer {
    type Theme = Theme;
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    // If your version of Iced requires Theme, you can usually set it to the default theme like this:
    // type Theme = iced::theme::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            VirusAnalyzer {
                upload_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Virus Analyzer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FileDropped => {
                // Open the file dialog
                // Assuming you want to start in the documents directory
                let default_path =
                    env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()) + "\\Documents";
                // Title and filter can be directly applied in the function call
                let malware_path = open_file_dialog("Select a malware file", &default_path, None)
                    .expect("Failed to open the file dialog");

                // Launch the analysis
                let malware_path_clone = malware_path.clone();
                let _analysis_thread = std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(launch_analysis(malware_path_clone));
                });
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let upload_button = Button::new(Text::new("Upload")).on_press(Message::FileDropped);

        let content = Column::new()
            .push(Text::new("VIRUS ANALYZER").size(40))
            .push(upload_button); // Use the button here

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
