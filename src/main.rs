use iced::widget::{button,svg, column, text, text_input, Column};
use latex::{DocumentClass, Document};
use std::process::Command;
use std::fs;


pub fn main() -> iced::Result {
    iced::run("LaTeX renderer", update, view)
}

#[derive(Default)]
struct State{
    content: String,
    counter: u8,
    file_path: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    Clear,
    Render,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) =>{
            state.content = content;
        }
        Message::Clear => {state.content="".to_string();}
        Message::Render => {
            let mut doc = Document::new(DocumentClass::Article);
            doc.push(state.content.as_str()); 
            let rendered = latex::print(&doc).unwrap();
            let _ = fs::remove_file(&state.file_path);
            let _ = fs::write("report.tex", rendered);
            let _ = Command::new("latexmk").arg("report.tex").status();
            state.file_path=format!("tmp{}.svg",state.counter);
            let _ = Command::new("dvisvgm").args([format!("-o {}",state.file_path),"report.dvi".to_string()]).status();
            state.counter+=1;
        }
    }
}

fn view(state: &State) -> Column<Message> {
    column![
        text(state.content.clone()),
        text_input("type smth...", &state.content).on_input(Message::ContentChanged),
        button("clear").on_press(Message::Clear),
        button("render").on_press(Message::Render),
        svg(&state.file_path), // Use svg::Handle instead of a string path
    ].into()
}

