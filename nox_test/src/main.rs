use iced::alignment::Vertical;
use iced::widget::{
    self, button, center, checkbox, column, container, horizontal_space, keyed_column, row, text, text_input,vertical_space,Space
};
use iced::{self, Length,Center, Element, Fill, Font, Subscription, Task as Command, Theme, Renderer};
#[derive(Default)]
struct Signup{
    username: String,
    password: String
}
#[derive(Default)]
struct Signin{
    username: String,
    password: String
}
#[derive(Default)]
struct App{
    current_page: Page,
    signin: Signin,
    signup: Signup,
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
    UpdateUser(String),
    UpdatePassword(String),
    Signup(String,String),
}
#[derive(Debug, Clone)]
enum Message{
    SwitchPage(Page),
    Signup(SignupPageMessage),
    Signin(SigninPageMessage),
    MainPage(MainPageMessage),
}
impl Signup {
    fn new() -> Self{
        Signup{
            username: "".to_string(),
            password:"".to_string()
        }
    }
}
impl Signin {
    fn new() -> Self{
        Signin{
            username: "".to_string(),
            password:"".to_string()
        }
    }
}
impl App{
    fn new() -> Self{
        App {
            current_page: Page::SignupPage,
            signin: Signin::new(),
            signup: Signup::new()
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
                let submit_b:widget::Container<'_, Message> = container(button(container(text("Sign in")).align_x(Center))
                .on_press(Message::Signin(SigninPageMessage::Info(self.signin.username.clone(), self.signin.password.clone())))
                .padding(15))
                .align_x(Center)
                .width(Fill);
                let signup: widget::Button<'_, Message, Theme, Renderer> = button("Create an Account").on_press(Message::SwitchPage(Page::SignupPage));
                let track = row![horizontal_space(),signup];        
                column![Space::new(30,30),title,username,password,submit_b,vertical_space(),track].spacing(50).padding(60).width(Fill).into()     
            }
            Page::SignupPage => {
                
                let title = text("Sign Up").align_x(Center).width(Fill).size(40);
                let username: widget::TextInput<'_, Message, Theme, Renderer> = text_input("username", &self.signup.username)
                .id("username_signup")
                .on_input(|input: String| Message::Signup(SignupPageMessage::UpdateUser(input)))                    
                .padding(15)
                .size(30)
                .align_x(Center);
                let password: widget::TextInput<'_, Message, Theme, Renderer> = text_input("password", &self.signup.password)
                .id("password_signup")
                .on_input(|input: String| Message::Signup(SignupPageMessage::UpdatePassword(input)))                    
                .padding(15)
                .size(30)
                .align_x(Center);
                let submit_b:widget::Container<'_, Message> = container(button(container(text("Sign up")).align_x(Center))
                .on_press(Message::Signup(SignupPageMessage::Signup(self.signup.username.clone(), self.signup.password.clone())))
                .padding(15))
                .align_x(Center)
                .width(Fill);
                let signin: widget::Button<'_, Message, Theme, Renderer> = button("Already have an account? Sign in").on_press(Message::SwitchPage(Page::SigninPage));
                let track = row![horizontal_space(),signin].align_y(Vertical::Bottom);        
                column![Space::new(30,30),title,username,password,submit_b,vertical_space(),track].spacing(50).padding(60).width(Fill).into()     
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
            Message::Signup(message) => {
                match message{
                    SignupPageMessage::Signup(user, pass) => {
                        if user.len() == 0 ||  pass.len() == 0{
                            println!("Unable to save data as it is empty");
                        }
                        self.add_to_db();
                        self.current_page = Page::GenKeyPage
                    }
                    SignupPageMessage::UpdatePassword(val) => {
                        self.signup.password = val
                    }
                    SignupPageMessage::UpdateUser(val) => {
                        self.signup.username = val
                    }
                }
            }
        }
    }
    fn add_to_db(&self) {

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