use iced::widget::{column, text_input};
use iced::{Element, Length, Task, Theme};

#[derive(Debug)]
struct App {
    value: i64,
    content: String,
}

#[derive(Clone, Debug)]
enum Message {
    SearchUpdated(String),
    Search,
}

impl Default for App{
    fn default() -> Self {
        Self { value: 0, content: String::from("hello") }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchUpdated(s) => {
                self.content = s;
            }
            Message::Search => {
                println!("{}", self.content);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let query = text_input("placeholder", &self.content)
            .on_input(Message::SearchUpdated)
            .on_submit(Message::Search)
            .width(Length::Fixed(220.0));

        column![query].into()
    }

    fn theme(&self) -> Theme{
        Theme::Dark
    }
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("title")
        .theme(App::theme)
        .run()
}