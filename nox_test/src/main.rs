use iced;
use iced::futures::io::Sink;
use iced::widget::{
    self, button, center, checkbox, column, container, keyed_column, row,
    scrollable, text, text_input, Text,
};
use iced::{Center, Element, Fill, Font, Subscription, Task as Command, Theme, Renderer};
#[derive(Default)]
struct Signin{
    username: String,
    password: String
}
#[derive(Default)]
struct App{
    current_page: Page,
    signin: Signin,
}
#[derive(Default,Debug, Clone)]
enum Page{
    SigninPage,
    #[default] SignupPage,
    MainPage,
    GenKeyPage,
}
#[derive(Debug, Clone)]
enum MainPageMessage{
    Encrypt,
    Decrypt,
    Logs
}
#[derive(Debug, Clone)]
enum SigninPageMessage{
    UpdateUser(String),
    UpdatePassword(String),
    Info(String,String),
    Login,
}
#[derive(Debug, Clone)]
enum SignupPageMessage{
    Signup
}
#[derive(Debug, Clone)]
enum Message{
    SwitchPage(Page),
    Signup(SignupPageMessage),
    Signin(SigninPageMessage),
    MainPage(MainPageMessage),
}
impl Signin {
    fn new() -> Self{
        Signin{
            username: "test".to_string(),
            password:"123".to_string()
        }
    }
}
impl App{
    fn new() -> Self{
        App {
            current_page: Page::SignupPage,
            signin: Signin::new()
        }
    }
    fn view(&self) -> Element<Message>{
        match &self.current_page{
            Page::SigninPage => {
            let title = text("Sign in").align_x(Center).width(Fill).size(40);
            let username: widget::TextInput<'_, Message, Theme, Renderer> = text_input("username", &self.signin.username)
            .id("username_signin")
            .on_input(|input: String| Message::Signin(SigninPageMessage::UpdateUser(input)))                    
            .padding(15)
            .size(30)
            .align_x(Center);
            let password: widget::TextInput<'_, Message, Theme, Renderer> = text_input("password", &self.signin.password)
            .id("password_signin")
            .on_input(|input: String| Message::Signin(SigninPageMessage::UpdatePassword(input)))                    
            .padding(15)
            .size(30)
            .align_x(Center);
            let submit_b: widget::Button<'_, Message, Theme, Renderer> = button(container(text("Sign in")).align_x(Center))
            .on_press(Message::Signin(SigninPageMessage::Info(self.signin.username.clone(), self.signin.password.clone())))
            .padding(15);        
            let content = column![title,username,password,submit_b]
            .spacing(20);
            

            scrollable(container(content).center_x(Fill).padding(60)).into()
                
            }
            Page::SignupPage => {
                
                container(
                    column![
                    button("Signin?").on_press(Message::SwitchPage(Page::SigninPage)),
                    button("Continue?").on_press(Message::SwitchPage(Page::GenKeyPage)),
                ]
                ).padding(10).into()
            }
            Page::MainPage => {
                button("Log out").on_press(Message::SwitchPage(Page::SignupPage)).into()
            }
            Page::GenKeyPage => {
                button("Switch to main").on_press(Message::SwitchPage(Page::MainPage),).into()
            }
        }
    }
    fn update(&mut self, message:Message){
        match message {
            Message::SwitchPage(Page) => {
                self.current_page = Page
            }
            Message::MainPage(message) => {
                match message{
                    MainPageMessage::Decrypt => {
                        return
                    }
                    MainPageMessage::Encrypt => {
                        return
                    }
                    MainPageMessage::Logs => {
                        return
                    }
                }
            } 
            Message::Signin(message) => {
                match message {
                    SigninPageMessage::Login => {

                    }
                    SigninPageMessage::UpdateUser(value) => {
                        self.signin.username = value

                    }
                    SigninPageMessage::UpdatePassword(val) => {
                        self.signin.password = val
                    }
                    SigninPageMessage::Info(user, pass) => {
                        if self.check_data(){
                            self.current_page = Page::MainPage;
                        }
                    }
                 }  
            }
            Message::Signup( message) => {
                
            }
        }
    }
    fn check_data(&self) -> bool {
        true
    }
}
fn theme(app: &App) -> Theme {
    Theme::TokyoNight
}
pub fn main() -> iced::Result {
    iced::application("Page switch", App::update, App::view).theme(theme).run()
}