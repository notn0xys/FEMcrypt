use iced::widget::{button, text_input, Button, Column, Container, Text, TextInput};
use iced::{executor, Element, Length, Settings, Subscription, Application};
use iced::command::Command;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

#[derive(Default)]
struct SignUpPage {
    username: String,
    password: String,
    email: String,
    sign_up_button: button::State,
    pool: Option<Pool<Sqlite>>,  // SQLite connection pool
}

#[derive(Debug, Clone)]
enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    EmailChanged(String),
    SignUpPressed,
    UserSignedUp(Result<(), sqlx::Error>),  // Handle database operation result
}

impl Application for SignUpPage {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instance = SignUpPage::default();
        let command = Command::perform(initialize_db(), Message::UserSignedUp);
        (instance, command)
    }

    fn title(&self) -> String {
        String::from("Sign Up Page")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UsernameChanged(new_value) => {
                self.username = new_value;
            }
            Message::PasswordChanged(new_value) => {
                self.password = new_value;
            }
            Message::EmailChanged(new_value) => {
                self.email = new_value;
            }
            Message::SignUpPressed => {
                if let Some(pool) = &self.pool {
                    let username = self.username.clone();
                    let password = self.password.clone();
                    let email = self.email.clone();
                    let pool = pool.clone();
                    return Command::perform(
                        async move {
                            sqlx::query("INSERT INTO users (username, password, email) VALUES (?, ?, ?)")
                                .bind(username)
                                .bind(password)
                                .bind(email)
                                .execute(&pool)
                                .await
                        },
                        Message::UserSignedUp,
                    );
                }
            }
            Message::UserSignedUp(result) => {
                match result {
                    Ok(_) => println!("User signed up successfully!"),
                    Err(err) => eprintln!("Error signing up: {}", err),
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let username_input = TextInput::new(
            &mut text_input::State::new(),
            "Username",
            &self.username,
            Message::UsernameChanged,
        )
        .padding(10);

        let password_input = TextInput::new(
            &mut text_input::State::new(),
            "Password",
            &self.password,
            Message::PasswordChanged,
        )
        .password()
        .padding(10);

        let email_input = TextInput::new(
            &mut text_input::State::new(),
            "Email",
            &self.email,
            Message::EmailChanged,
        )
        .padding(10);

        let sign_up_button = Button::new(&mut self.sign_up_button, Text::new("Sign Up"))
            .on_press(Message::SignUpPressed);

        let content = Column::new()
            .push(Text::new("Sign Up").size(30))
            .push(username_input)
            .push(password_input)
            .push(email_input)
            .push(sign_up_button)
            .spacing(20)
            .align_items(iced::Alignment::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

async fn initialize_db() -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create pool");
    
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, username TEXT, password TEXT, email TEXT)")
        .execute(&pool)
        .await
        .expect("Failed to create table");

    pool
}

fn main() -> iced::Result {
    SignUpPage::run(Settings::default())
}