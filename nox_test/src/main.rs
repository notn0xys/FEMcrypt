use std::clone;
use iced::Sandbox;
use iced::widget::{button, column, progress_bar, text, Column};
use iced::{Element, Settings};
struct Editor{
}
#[derive(Debug,Clone)]
enum Message{}
impl Sandbox for Editor{
    type Message = Message;
    fn new() -> Self{
        Editor{}
    }
    fn title(&self) -> String{
        String::from("First app")
    }
    fn update(&mut self,message: Message){
        match message{

        }
    }
    fn view(&self) -> Element<Message>{
        text("hello! word").into()
    }
    

}
fn main() -> iced::Result {
	Editor::run(Settings::default())
}
