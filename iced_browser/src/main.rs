use iced::alignment::Horizontal;
use iced::widget::{column, container, row, text_input};
use iced::Element;
use url::Url;
mod url;

fn main() -> iced::Result {
    iced::application("Iced Browser", update, view)
        .run()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::UrlChanged(url) => {
            if !url.is_empty() {
                state.url = UrlView {
                    schema: "schema todo".to_string(),
                    host: "host todo".to_string(),
                    port: 0000,
                };
            }
        },
    }
}

fn view(state: &State) -> Element<Message> {
    let url_input = text_input("Type url here ...", &state.input_url.as_str())
            .on_input(|input| Message::UrlChanged(input));
    let parsed_url = container(
        column![
            row![
                "Schema",
                "Host",
                "Port",
            ].spacing(10),
        ]
    );
    column![
        url_input,
        parsed_url,
    ]
    .align_x(Horizontal::Center)
    .into()
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
}

#[derive(Default)]
struct UrlView {
    schema: String,
    host: String,
    port: u16,
}

#[derive(Default)]
struct State {
    input_url: String,
    url: UrlView,
}
