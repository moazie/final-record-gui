use iced::{
    Element, Font,
    Length::Fill,
    Size, Task,
    alignment::Horizontal::Center,
    application,
    font::{Family, load},
    widget::{column, text},
};

use crate::display_reviews;

pub const CJK_FONT: Font = Font {
    family: Family::Name("Noto Sans CJK Regular"),
    ..Font::DEFAULT
};

#[derive(Default)]
struct FinalRecord {}

#[derive(Debug)]
enum Message {
    FontLoaded(std::result::Result<(), iced::font::Error>),
}

fn update(_state: &mut FinalRecord, message: Message) -> Task<Message> {
    match message {
        Message::FontLoaded(Ok(())) => println!("Successfully loaded font!"),
        Message::FontLoaded(Err(e)) => println!("Font engine failed to parse bytes: {:?}", e),
    }
    Task::none()
}

fn view(_state: &FinalRecord) -> Element<'_, Message> {
    column![
        text("Test test")
            .font(CJK_FONT)
            .width(Fill)
            .align_x(Center)
            .shaping(text::Shaping::Advanced)
            .size(23),
        text(display_reviews())
    ]
    .into()
}

fn boot() -> (FinalRecord, Task<Message>) {
    const _CJK_BYTES: &[u8] = include_bytes!("../assets/NotoSansCJK.otf");
    let load_cjk = load(_CJK_BYTES).map(Message::FontLoaded);

    (FinalRecord::default(), load_cjk)
}

pub fn run_interface() -> iced::Result {
    application(boot, update, view)
        .title("The Final Record")
        .default_font(CJK_FONT)
        .window(iced::window::Settings {
            size: Size {
                width: 600.,
                height: 600.,
            },
            ..iced::window::Settings::default()
        })
        .run()
}
