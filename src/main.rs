use iced::widget::{text_editor, container, button,svg, column, Column};
use latex::{DocumentClass, Document};
use std::process::Command;
use std::fs;


pub fn main() -> iced::Result {
    iced::run("LaTeX renderer", update, view)
}

#[derive(Default)]
struct State{
    content: text_editor::Content,
    counter: u8,
    file_path: String,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Clear,
    Render,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) =>{
            state.content.perform(action);
        }
        Message::Clear => {state.content=text_editor::Content::new();}
        Message::Render => {
            let mut doc = Document::new(DocumentClass::Article);
            doc.push(text_editor::Content::text(&state.content).as_str()); 
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
    let input = text_editor(&state.content).placeholder("type smth").on_action(Message::Edit);
    column![
        container(input),
        button("clear").on_press(Message::Clear),
        button("render").on_press(Message::Render),
        svg(&state.file_path), // Use svg::Handle instead of a string path
    ].into()
}

