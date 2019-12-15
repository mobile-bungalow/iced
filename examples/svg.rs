use iced::{Column, Container, Element, Length, Sandbox, Settings, Svg};

pub fn main() {
    Tiger::run(Settings::default())
}

#[derive(Default)]
struct Tiger;

impl Sandbox for Tiger {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("SVG - Iced")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        let content =
            Column::new()
                .width(Length::Shrink)
                .padding(20)
                .push(Svg::new(format!(
                    "{}/examples/resources/tiger.svg",
                    env!("CARGO_MANIFEST_DIR")
                )));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
