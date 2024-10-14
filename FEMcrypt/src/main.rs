use iced::{
    button, text_input, Button, Column, Container, Element, Length, Sandbox, Settings, Text,
    TextInput,
};

#[derive(Default)]
struct SignUpPage {
    username: String,
    password: String,
    email: String,

    username_input: text_input::State,
    password_input: text_input::State,
    email_input: text_input::State,

    sign_up_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    EmailChanged(String),
    SignUpPressed,
}

impl Sandbox for SignUpPage {
    type Message = Message;

    fn new() -> Self {
        SignUpPage::default()
    }

    fn title(&self) -> String {
        String::from("Sign Up Page")
    }

    fn update(&mut self, message: Message) {
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
                // Handle sign-up action (e.g., send credentials to server)
                println!("Sign Up pressed!");
                println!("Username: {}", self.username);
                println!("Password: {}", self.password);
                println!("Email: {}", self.email);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Create the text inputs for username, password, and email
        let username_input = TextInput::new(
            &mut self.username_input,
            "Username",
            &self.username,
            Message::UsernameChanged,
        )
        .padding(10)
        .size(20);

        let password_input = TextInput::new(
            &mut self.password_input,
            "Password",
            &self.password,
            Message::PasswordChanged,
        )
        .padding(10)
        .size(20)
        .password(); // Password field hidden

        let email_input = TextInput::new(
            &mut self.email_input,
            "E-mail",
            &self.email,
            Message::EmailChanged,
        )
        .padding(10)
        .size(20);

        // Create the Sign Up button
        let sign_up_button = Button::new(&mut self.sign_up_button, Text::new("Sign Up"))
            .padding(10)
            .on_press(Message::SignUpPressed);

        // Layout all components in a column
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("Sign Up").size(30))
            .push(username_input)
            .push(password_input)
            .push(email_input)
            .push(sign_up_button);

        // Create the container for the layout and center it on the screen
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    SignUpPage::run(Settings::default())
}
