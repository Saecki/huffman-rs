use huffman_lib::Tree;
use iced::{executor, text_input, Application, Column, Command, Container, Settings, TextInput};

fn main() -> iced::Result {
    Huffman::run(Settings::default())
}

#[derive(Clone, Debug)]
struct Huffman {
    tree: Tree,
    state: State,
}

#[derive(Clone, Debug)]
struct State {
    input: text_input::State,
}

#[derive(Clone, Debug)]
enum Message {
    Input(String),
}

impl Application for Huffman {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                tree: Tree::from(String::new()),
                state: State {
                    input: text_input::State::new(),
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Huffman")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::Input(s) => self.tree = Tree::from(s),
        }

        println!("{}", self.tree.input());

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let text_input = TextInput::new(
            &mut self.state.input,
            "input",
            self.tree.input(),
            Message::Input,
        );
        Container::new(Column::new().push(text_input)).into()
    }
}
