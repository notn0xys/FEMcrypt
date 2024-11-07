use std::path::{Path, PathBuf};
use sqlite::{Connection,Value,State};
use std::fs;
use std::fs::File;
use iced::alignment::Vertical;
use iced::widget::{
    self, button, center, checkbox, column, container, horizontal_space, keyed_column, row, scrollable, text, text_editor, text_input, vertical_space, Space
};
use iced::widget::text_editor::{Content};
use iced_aw::{TabLabel,TabBar};
use iced::{self, Center, Color, Element, Fill, Font, Length, Renderer, Subscription, Task as Command, Theme};
use rfd::FileDialog;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, DecodeRsaPrivateKey,EncodeRsaPublicKey, DecodeRsaPublicKey};
use std::env;
use pem::parse;
use hex;
#[derive(Default)]
struct Signup{
    username: String,
    password: String,
    err_msg:String
}
#[derive(Default)]
struct Signin{
    username: String,
    password: String,
    err_msg:String

}
#[derive(Default)]
struct Maindata{
    logs:String,
    combined_key:Content,
    pub_key:String,
    file:PathBuf,
    nonce_string: String,

}
#[derive(Default)]
struct App{
    current_user: i32,
    connect: Option<Connection>,
    current_page: Page,
    signin: Signin,
    signup: Signup,
    active_tab: MainTab,
    maindata: Maindata
}
#[derive(Default,Debug, Clone)]
enum Page{
    SigninPage,
    #[default] SignupPage,
    MainPage,
    GenKeyPage,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MainTab {
    Encrypt,
    Decrypt,
    Logs,
}
#[derive(Debug,Clone)]
enum EncryptPage{
    ChooseFile,
    UpdateKey(String),
    Encryptdata
}
#[derive(Debug, Clone)]
enum MainPageMessage{
    Encrypt(EncryptPage),
    Decrypt,
    Logs,
    Logout,
}
#[derive(Debug, Clone)]
enum SigninPageMessage{
    UpdateUser(String),
    UpdatePassword(String),
    Info(String,String),
}
#[derive(Debug, Clone)]
enum SignupPageMessage{
    UpdateUser(String),
    UpdatePassword(String),
    Sign(String,String),
}
#[derive(Debug, Clone)]
enum Message{
    SwitchPage(Page),
    Signup(SignupPageMessage),
    Signin(SigninPageMessage),
    MainPage(MainPageMessage),
    MainPageTabSelected(MainTab),
    GenKeyMessage,
    Edit(text_editor::Action),

}
impl Signup {
    fn new() -> Self{
        Signup{
            username: "".to_string(),
            password:"".to_string(),
            err_msg: "".to_string()
        }
    }
}
impl Signin {
    fn new() -> Self{
        Signin{
            username: "".to_string(),
            password:"".to_string(),
            err_msg: "".to_string()
        }
    }
}
impl Maindata{
    fn new() -> Self{
        Maindata{
            pub_key: "".to_string(),
            logs: "".to_string(),
            combined_key: Content::new(),
            file: PathBuf::new(),
            nonce_string: "".to_string()

        }
    }
}
impl Default for MainTab{
    fn default() -> Self {
        MainTab::Encrypt
    }
}
impl App{
    fn new() -> (Self, Command<Message>){
        let connection = match sqlite::open("data/app.db") {
            Ok(conn) => {
                println!("Database connection successfully established.");
                Some(conn)
            }
            Err(e) => {
                eprintln!("Failed to open database: {:?}", e);
                None
            }
        };
        (App {
            current_user : -1,
            connect: connection,
            current_page: Page::SignupPage,
            signin: Signin::new(),
            signup: Signup::new(),
            active_tab: MainTab::Encrypt,
            maindata: Maindata::new()
        },
        Command::none())
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
                let err_msg = text(self.signin.err_msg.clone()).align_x(Center).width(Fill).size(14).color(Color::from_rgba(255.0, 0.0, 30.0, 0.5));       
                column![Space::new(30,30),title,username,password,submit_b,err_msg,track].spacing(50).padding(60).width(Fill).into()     
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
                .on_press(Message::Signup(SignupPageMessage::Sign(self.signup.username.clone(), self.signup.password.clone())))
                .padding(15))
                .align_x(Center)
                .width(Fill);
                let signin: widget::Button<'_, Message, Theme, Renderer> = button("Already have an account? Sign in").on_press(Message::SwitchPage(Page::SigninPage));
                let track = row![horizontal_space(),signin].align_y(Vertical::Bottom);
                let err_msg = text(self.signup.err_msg.clone()).align_x(Center).width(Fill).size(14).color(Color::from_rgba(255.0, 0.0, 30.0, 0.5));       
                column![Space::new(30,30),title,username,password,submit_b,err_msg,track].spacing(50).padding(60).width(Fill).into()     
            }
            Page::MainPage => {
                let tabs = TabBar::new(|tab| Message::MainPageTabSelected(tab))
                .push(MainTab::Encrypt, TabLabel::Text("Encrypt".to_string()))
                .push(MainTab::Decrypt, TabLabel::Text("Decrypt".to_string()))
                .push(MainTab::Logs, TabLabel::Text("Logs".to_string()));
                let content = match self.active_tab{
                    MainTab::Decrypt => {
                        self.view_decrypt_tab()
                    }
                    MainTab::Logs => {
                        self.view_logs_tab()
                    }
                    MainTab::Encrypt => {
                        self.view_encrypt_tab()
                    }
                };
                column![tabs, content].into()
            }
            Page::GenKeyPage => {
                let b1 = button("Switch to main").on_press(Message::SwitchPage(Page::MainPage));
                let b2 = button("Gen Key").on_press(Message::GenKeyMessage,);
                let pub_k = text_editor(&self.maindata.combined_key).placeholder("Press Generate key to get a key").on_action(Message::Edit);
                let bot_row = row![b1,horizontal_space(),b2];
                scrollable(column![pub_k,bot_row].padding(20).spacing(20)).into()
            }
        }
    }
    fn view_encrypt_tab(&self) -> Element<Message> {
        let file_btn: widget::Button<'_, Message> = button("Choose File").on_press(Message::MainPage(MainPageMessage::Encrypt(EncryptPage::ChooseFile)));
        let encrypt_btn = button("Encyrpt").on_press(Message::MainPage(MainPageMessage::Encrypt(EncryptPage::Encryptdata)));
        let input_key = text_input("Key", &self.maindata.pub_key).id("text_input")
        .on_input(|input: String| Message::MainPage(MainPageMessage::Encrypt(EncryptPage::UpdateKey(input))))                    
        .padding(15)
        .size(30)
        .align_x(Center);
        let swap_btn = container(button("Get key!").on_press(Message::SwitchPage(Page::GenKeyPage))).padding(30).align_x(Center);
        let row_1 = row![file_btn,horizontal_space(),encrypt_btn].padding(30);
        column![
            row_1,
            input_key,
            swap_btn,
        ]
        .padding(30)
        .spacing(10)
        .into()
    }

    fn view_decrypt_tab(&self) -> Element<Message> {
        column![
            button("Choose File").on_press(Message::MainPage(MainPageMessage::Decrypt)),
            button("Choose Folder").on_press(Message::MainPage(MainPageMessage::Decrypt)),
            text("Decryption Method").size(20),
            button("Master Key").on_press(Message::MainPage(MainPageMessage::Decrypt))
        ]
        .spacing(10)
        .into()
    }

    fn view_logs_tab(&self) -> Element<Message> {
        let log_out_btn = button("Log out").on_press(Message::MainPage(MainPageMessage::Logout)); 
        let get_logs = container(button("Get logs").on_press(Message::MainPage(MainPageMessage::Logs))).align_x(Center);
        let bot_row = row![horizontal_space(),get_logs,horizontal_space(),log_out_btn];
        let logs_text = text(self.maindata.logs.clone()).size(20);
        let logs = scrollable(column![logs_text].padding(30).spacing(20));
        column![logs,bot_row].padding(30).spacing(20).into()
    }
    fn update(&mut self, message:Message){
        match message {
            Message::Edit(action) => {
                self.maindata.combined_key.perform(action);
            }
            Message::GenKeyMessage =>{
                self.gen_key();
            }
            Message::MainPageTabSelected(tab) => {
                self.active_tab = tab;
            }
            Message::Signup(message) => {
                match message {
                    SignupPageMessage::UpdatePassword(val) => {
                        self.signup.password = val
                    }
                    SignupPageMessage::UpdateUser(val) => {
                        self.signup.username = val
                    }
                    SignupPageMessage::Sign(user, pass) => {
                        self.add_data(&user, &pass);
                        self.signup.username = "".to_string();
                        self.signup.password = "".to_string();             
                    }
                }
            }
            Message::SwitchPage(page) => {
                self.current_page = page;
                self.signin.err_msg = "".to_string();
                self.signup.err_msg = "".to_string();
            }
            Message::MainPage(message) => {
                match message{
                    MainPageMessage::Decrypt => {
                        return
                    }
                    MainPageMessage::Encrypt(msg) => {
                        match msg {
                            EncryptPage::ChooseFile => {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.maindata.file = file;
                                }
                            }
                            EncryptPage::Encryptdata => {
                                if !self.maindata.file.exists() {
                                    println!("Error: The file has not been selected or cannot be found.");
                                    return;  // Early exit if the file doesn't exist
                                }
                                let mut rng = rand::thread_rng();
                                let data = fs::read(&self.maindata.file).expect("Failed to read file");
                                let aes_key = Aes256Gcm::generate_key(OsRng);
                                let cipher = Aes256Gcm::new(&aes_key);
                                let nonce = Aes256Gcm::generate_nonce(OsRng);
                                let ciphertext = cipher.encrypt(&nonce, data.as_ref()).expect("Failed to encrypt data");
                                let public_key = match parse(self.maindata.pub_key.as_bytes()) {  
                                    Ok(pem) => {
                                        match RsaPublicKey::from_pkcs1_der(&pem.contents) {
                                            Ok(key) => key, // Successful parsing of RSAPublicKey
                                            Err(e) => {
                                                println!("Failed to parse public key: {:?}", e);
                                                return;  // Exit early if parsing fails
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Failed to parse PEM: {:?}", e);
                                        return;  // Exit early if PEM parsing fails
                                    }
                                };
                                let encrypted_key = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &aes_key).expect("Failed to encrypt AES key");
                                let downloads_dir = env::home_dir().unwrap().join("Downloads");
                                let encrypted_folder = downloads_dir.join("encrypted_folder");
                                fs::create_dir_all(&encrypted_folder).expect("Failed to create encrypted folder");

                                let encrypted_key_path = encrypted_folder.join("encrypted_key.bin");
                                let encrypted_data_path = encrypted_folder.join("encrypted_data.bin");

                                fs::write(&encrypted_key_path, &encrypted_key).expect("Failed to write encrypted key");
                                fs::write(&encrypted_data_path, &ciphertext).expect("Failed to write encrypted data");
                                self.maindata.nonce_string = hex::encode(&nonce);
                                println!("{}",self.maindata.nonce_string);

                                println!("Encrypted key and data have been saved to {:?}", encrypted_folder);
                            }
                            EncryptPage::UpdateKey(msg) => {
                                self.maindata.pub_key = msg;
                            }
                        }
                    }
                    MainPageMessage::Logs => {
                        self.get_logs();
                    }
                    MainPageMessage::Logout => {
                        self.current_user = -1;
                        self.active_tab = MainTab::Encrypt;
                        self.maindata = Maindata::new();
                        self.current_page = Page::SignupPage
                    }
                }
            } 
            Message::Signin(message) => {
                match message {
                    SigninPageMessage::UpdateUser(value) => {
                        self.signin.username = value

                    }
                    SigninPageMessage::UpdatePassword(val) => {
                        self.signin.password = val
                    }
                    SigninPageMessage::Info(user, pass) => {
                        self.check_data(&user, &pass);
                        self.signin.username = "".to_string();
                        self.signin.password = "".to_string()                     
                    }
                }  
            }
        }
    }
    fn get_logs(&mut self) {
        match &self.connect{
            Some(value) => {
                let mut statement = value.prepare("
                    SELECT userdata.username, logs.action 
                    FROM logs 
                    JOIN userdata ON logs.userid = userdata.id 
                    WHERE logs.userid = ?").unwrap();
                statement.bind((1, self.current_user as i64)).unwrap();
                while let sqlite::State::Row = statement.next().unwrap() {
                    let username: String = statement.read(0).unwrap();
                    let action: String = statement.read(1).unwrap(); 
                    self.maindata.logs.push_str(&format!("User: {}, Action: {}\n", username, action));
                }
            }
            None => {
                println!("Error establiching connection at check_data")
            }
        }
    }
    fn add_data(&mut self,user:&str,pass:&str) {
        if user.len() == 0 || pass.len() == 0{
            self.signup.err_msg = "Invalid username or password".to_string();
            return;
        }
        if let Some(conn) = &self.connect {
            let mut statement = conn.prepare("SELECT id FROM userdata WHERE username = ?")
            .expect("Failed to prepare select statement");
            statement.bind((1, user)).expect("Failed to bind username");
        if statement.next().expect("Failed to execute SELECT statement") == sqlite::State::Row {
            self.signup.err_msg = "Username already exists".to_string();
            return;
        }
            let mut insert_state = conn.prepare("INSERT INTO userdata (username, password) VALUES (?, ?)")
                .expect("Failed to prepare statement");
            insert_state.bind((1, user)).expect("Failed to bind username");
            insert_state.bind((2, pass)).expect("Failed to bind password");
            insert_state.next().expect("Failed to execute statement");
            println!("User '{}' added successfully!", user);
            self.signup.err_msg = "".to_string();
            self.current_page = Page::GenKeyPage
        }
        
    }
    fn check_data(&mut self,user:&str,pass:&str) {
        match &self.connect{
            Some(value) => {
                let mut statement = value.prepare("SELECT id FROM userdata WHERE username = ? AND password = ?").unwrap();
                statement.bind((1, user)).unwrap();
                statement.bind((2, pass)).unwrap();
                if let State::Row = statement.next().unwrap() {
                    let user_id = statement.read::<i64, _>(0).unwrap() as i32;
                    self.current_user = user_id;
                    println!("Logged in sucessfully as {}", user);
                    println!("User id to track now set as {}",self.current_user);
                    self.signin.err_msg = "".to_string();
                    self.current_page = Page::MainPage;
                }else {
                    self.current_user = -1;
                    self.signin.err_msg = "Wrong username or password".to_string()
                }
            }
            None => {
                println!("Error establiching connection at check_data")
            }
        }
    }
    fn gen_key(&mut self) {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a private key");
        let public_key = RsaPublicKey::from(&private_key);
        let s1 = private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::CRLF).expect("Failed to convert private key to PKCS1 PEM").to_string();
        let s2 = public_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::CRLF).expect("Failed to convert public key to PKCS1 PEM").to_string();
        let combined = format!("{}\n{}",s1,s2);
        self.maindata.combined_key = Content::with_text(&combined);
    }       
}    
    fn theme(app: &App) -> Theme {
    Theme::Dracula
    }

pub fn main() -> iced::Result {
    let folder_path = "data";
    let db_path = format!("{}/app.db", folder_path);
    if !Path::new(folder_path).exists() {
        fs::create_dir(folder_path).expect("Failed to create folder");
    }
    if !Path::new(&db_path).exists() {
        File::create(db_path).unwrap();
    }
    let path = sqlite::open("data/app.db").unwrap();
    path.execute(
"CREATE TABLE IF NOT EXISTS userdata (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );"
    ).unwrap();
    path.execute(
        "CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    userid INTEGER NOT NULL,
    action TEXT NOT NULL,
    FOREIGN KEY (userid) REFERENCES userdata(id)
        );"
    ).unwrap();
    iced::application("Femcrypt", App::update, App::view).theme(theme).run_with(App::new)
}