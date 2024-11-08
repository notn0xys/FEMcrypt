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
struct Decryptdata{
    file1:PathBuf,
    file2:PathBuf,
    nonce_string: String,
    error:String,
    private_key:String
}
#[derive(Default)]
struct Maindata{
    logs:String,
    combined_key:Content,
    pub_key:String,
    file:PathBuf,
    nonce_string: String,
    reminder:String,
    decrpt:Decryptdata

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
    Encryptdata,
    UpdateWithnoinput(String)
}
#[derive(Debug,Clone)]
enum DecryptPage{
    ChooseFile1,
    ChooseFile2,
    UpdateKey(String),
    UpdateNounce(String),
    DecryptData,
}
#[derive(Debug, Clone)]
enum MainPageMessage{
    Encrypt(EncryptPage),
    Decrypt(DecryptPage),
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
impl Decryptdata{
    fn new() -> Self{
        Decryptdata{
            file1:PathBuf::new(),
            file2:PathBuf::new(),
            error: "".to_string(),
            nonce_string: "".to_string(),
            private_key: "".to_string()
        }
    }
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
            nonce_string: "".to_string(),
            reminder: "".to_string(),
            decrpt:Decryptdata::new()


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
        .padding(50)
        .size(30)
        .align_x(Center);
        let nounce_txt = text_input("Nounce Output", &self.maindata.nonce_string).id("text_input")
        .on_input(|input: String| Message::MainPage(MainPageMessage::Encrypt(EncryptPage::UpdateWithnoinput(input))))                    
        .padding(50)
        .size(30)
        .align_x(Center);
        let reminder = text(&self.maindata.reminder).size(12).align_x(Center).width(Fill).size(14).color(Color::from_rgba(255.0, 0.0, 30.0, 0.5));
        let swap_btn = container(button("Get key!").on_press(Message::SwitchPage(Page::GenKeyPage))).padding(30).align_x(Center);
        let row_1 = row![file_btn,horizontal_space(),encrypt_btn].padding(30);
        column![
            row_1,
            input_key,
            swap_btn,
            nounce_txt,
            reminder,
        ]
        .padding(30)
        .spacing(20)
        .into()
    }

    fn view_decrypt_tab(&self) -> Element<Message> {
        let encrypted_btn = button("Choose Encrypted Data").on_press(Message::MainPage(MainPageMessage::Decrypt(DecryptPage::ChooseFile1)));
        let encrypted_key = button("Choose Encrypted Key").on_press(Message::MainPage(MainPageMessage::Decrypt(DecryptPage::ChooseFile2)));
        let row = row![encrypted_btn,horizontal_space(),encrypted_key].padding(30);
        let decrypt_btn = container(button("Decrypt").on_press(Message::MainPage(MainPageMessage::Decrypt(DecryptPage::DecryptData)))).padding(30);
        let nounce = text_input("Nounce Input", &self.maindata.decrpt.nonce_string).id("Nounce input")
        .on_input(|input: String| Message::MainPage(MainPageMessage::Decrypt(DecryptPage::UpdateNounce(input))))               
        .padding(50)
        .size(30)
        .align_x(Center);
        let key = text_input("Key Input", &self.maindata.decrpt.private_key).id("Key input")
        .on_input(|input: String| Message::MainPage(MainPageMessage::Decrypt(DecryptPage::UpdateKey(input))))               
        .padding(50)
        .size(30)
        .align_x(Center);
        let warning = text(&self.maindata.decrpt.error).size(12).align_x(Center).width(Fill).size(14).color(Color::from_rgba(255.0, 0.0, 30.0, 0.5));
        column![
            row,
            key,
            Space::new(30,30),
            nounce,
            decrypt_btn,
            warning
        ]
        .padding(35)
        .spacing(30)
        .into()
    }

    fn view_logs_tab(&self) -> Element<Message> {
        let log_out_btn = button("Log out").on_press(Message::MainPage(MainPageMessage::Logout)); 
        let get_logs = container(button("Get logs").on_press(Message::MainPage(MainPageMessage::Logs))).align_x(Center);
        let bot_row = row![horizontal_space(),get_logs,horizontal_space(),log_out_btn];
        let logs_text = text(self.maindata.logs.clone()).size(20);
        let logs = container(
            scrollable(column![logs_text].padding(30).spacing(20))
        ).height(Length::Fixed(599.0)).width(Length::Fill).align_x(Center);
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
                    MainPageMessage::Decrypt(msg) => {
                        match msg{
                            DecryptPage::ChooseFile1 => {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.maindata.decrpt.file1 = file;
                                }

                            }
                            DecryptPage::ChooseFile2 => {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.maindata.decrpt.file2 = file;
                                }

                            }
                            DecryptPage::DecryptData => {
                                if !self.maindata.decrpt.file1.exists() || !self.maindata.decrpt.file2.exists() {
                                    println!("Error: One or both files have not been selected or cannot be found.");
                                    return;
                                }
                                let private_key = match parse(self.maindata.decrpt.private_key.as_bytes()) {
                                    Ok(pem) => {
                                        match RsaPrivateKey::from_pkcs1_der(&pem.contents) {
                                            Ok(key) => key, 
                                            Err(e) => {
                                                println!("Failed to parse private key: {:?}", e);
                                                return;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Failed to parse PEM: {:?}", e);
                                        return;
                                    }
                                };
                                let encrypted_key = fs::read(&self.maindata.decrpt.file2).expect("Failed to read encrypted key");
                                let encrypted_data = fs::read(&self.maindata.decrpt.file1).expect("Failed to read encrypted data");
                                let nonce_slice = match hex::decode(&self.maindata.decrpt.nonce_string) {
                                    Ok(nonce) => nonce,
                                    Err(e) => {
                                        println!("Failed to decode nonce: {:?}", e);
                                        return;  
                                    }
                                };
                                let nonce = Nonce::from_slice(&nonce_slice);

                                let aes_key = match private_key.decrypt(Pkcs1v15Encrypt, &encrypted_key) {
                                    Ok(key) => key, 
                                    Err(e) => {
                                        println!("Failed to decrypt AES key: {:?}", e);
                                        return; 
                                    }
                                };
                                let cipher = Aes256Gcm::new_from_slice(&aes_key).unwrap();
                                let decrypted_data = cipher.decrypt(&nonce, encrypted_data.as_ref()).expect("Failed to decrypt data");
                                let downloads_dir = env::home_dir().unwrap().join("Documents");
                                let decrypted_folder = downloads_dir.join("decrypted_folder");
                                fs::create_dir_all(&decrypted_folder).expect("Failed to create decrypted folder");
                                let decrypted_path = decrypted_folder.join("decrypted_data.txt");
                                fs::write(&decrypted_path, &decrypted_data).expect("Failed to write decrypted data");
                                self.log_action(self.current_user.into(), "Decrypt");
                                println!("Decrypted data has been saved to {:?}", decrypted_folder);
                                
                            }
                            DecryptPage::UpdateNounce(msg) => {
                                self.maindata.decrpt.nonce_string = msg
                            }
                            DecryptPage::UpdateKey(msg) => {
                                self.maindata.decrpt.private_key = msg
                            }
                        }
                    }
                    MainPageMessage::Encrypt(msg) => {
                        match msg {
                            EncryptPage::UpdateWithnoinput(msg) => {}
                            EncryptPage::ChooseFile => {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.maindata.file = file;
                                }
                            }
                            EncryptPage::Encryptdata => {
                                if !self.maindata.file.exists() {
                                    println!("Error: The file has not been selected or cannot be found.");
                                    return;  
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
                                            Ok(key) => key, 
                                            Err(e) => {
                                                println!("Failed to parse public key: {:?}", e);
                                                return;  
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Failed to parse PEM: {:?}", e);
                                        return;
                                    }
                                };
                                let encrypted_key = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &aes_key).expect("Failed to encrypt AES key");
                                let downloads_dir = env::home_dir().unwrap().join("Documents");
                                let encrypted_folder = downloads_dir.join("encrypted_folder");
                                fs::create_dir_all(&encrypted_folder).expect("Failed to create encrypted folder");

                                let encrypted_key_path = encrypted_folder.join("encrypted_key.bin");
                                let encrypted_data_path = encrypted_folder.join("encrypted_data.bin");

                                fs::write(&encrypted_key_path, &encrypted_key).expect("Failed to write encrypted key");
                                fs::write(&encrypted_data_path, &ciphertext).expect("Failed to write encrypted data");
                                self.maindata.nonce_string = hex::encode(&nonce);
                                self.maindata.reminder = "Dont forget to copy and send your nonce! it is used for decryption".to_string();
                                self.log_action(self.current_user.into(), "Encrypt");
                                println!("{}",self.maindata.nonce_string);
                                println!("Encrypted key and data have been saved to {:?}", encrypted_folder);
                            }
                            EncryptPage::UpdateKey(msg) => {
                                self.maindata.pub_key = msg;
                            }
                        }
                    }
                    MainPageMessage::Logs => {
                        self.log_action(self.current_user.into(), "Getlogs");
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
        self.maindata.logs.clear();
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
            let mut get_id = conn.prepare("SELECT id FROM userdata WHERE username = ? AND password = ?").unwrap();
            get_id.bind((1, user)).unwrap();
            get_id.bind((2, pass)).unwrap();
            if let State::Row = get_id.next().unwrap() {
                let user_id = get_id.read::<i64, _>(0).unwrap() as i32;
                self.current_user = user_id;
                println!("User id to track now set as {}",self.current_user);
            }else {
                self.current_user = -1;
                self.signup.err_msg = "Failed to track".to_string()
            }
            
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
    fn log_action(&self, user_id: i64, action: &str) {
        match &self.connect {
            Some(val) => {
                let mut statement = val.prepare("INSERT INTO logs (userid, action) VALUES (?, ?)").unwrap();
                statement.bind((1, user_id)).unwrap();
                statement.bind((2, action)).unwrap();
                statement.next().expect("Failed to execute statement");
                println!("Testing logs added")
            }

            None => {
                return
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
        self.log_action(self.current_user.into(), "Generate Key");
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