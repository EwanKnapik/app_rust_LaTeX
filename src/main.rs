use iced::widget::{row,text_editor, text, container, button,svg, column, Column};
use latex::{DocumentClass, Document};
use std::process::Command;
use std::fs;
use iced::Background;
use iced::Color;
use iced::Theme;


pub fn main() -> iced::Result {
    iced::run("LaTeX renderer", update, view)
}

enum Tabs{
    Texteditor,
    Graph,
}


struct State{
    content: text_editor::Content,
    counter: u8,
    file_path: String,
    tab:Tabs,
}

impl Default for State{
    fn default() -> Self{
        Self{
            content : text_editor::Content::new(),
            counter : 0,
            file_path : String::new(),
            tab : Tabs::Texteditor

        }
    }

}



#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Clear,
    Render,
    ToTextEditor,
    ToGraph,
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
            println!("{}",rendered);
            let _ = fs::remove_file(&state.file_path);
            let _ = fs::write("report.tex", rendered);
            let _ = Command::new("latexmk").arg("report.tex").status();
            state.file_path=format!("tmp{}.svg",state.counter);
            let _ = Command::new("dvisvgm").args([format!("-o {}",state.file_path),"report.dvi".to_string()]).status();
            state.counter+=1;}
        Message::ToTextEditor =>{
            state.tab=Tabs::Texteditor;
        }
        Message::ToGraph =>{
            state.tab=Tabs::Graph;
        
        }
    }
}

fn view(state: &State) -> Column<Message> {
    let input = text_editor(&state.content).placeholder("type smth").on_action(Message::Edit).height(500);
    column![
        row![button("Text Editor").on_press(Message::ToTextEditor),
        button("Graph").on_press(Message::ToGraph)
        ],
    match state.tab{
        Tabs::Texteditor => {
    column![
        row![
            button("clear").on_press(Message::Clear),
            button("render").on_press(Message::Render)],
        row![
            container(input).style(container::bordered_box).style(|_|{container::bordered_box(&Theme::Dark)}),
            container(svg(&state.file_path).style(|_,_|{svg::Style{color:Some(Color::BLACK)}})).style(|_|{container::background(Background::Color(Color::WHITE))}).height(500),
        ]
    ]
        }
        Tabs::Graph => {
            column![
            text("work in progress")
            ]
        }
        }
    ].into()
    }
