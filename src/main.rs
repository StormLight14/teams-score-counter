use iced::widget::{column, container, text, button, Radio};
use iced::{
    Alignment, Sandbox, Theme, Element, Length, Settings, Renderer
};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

enum State{
    Counter,
    Settings,
    NotFound,
}

struct App {
    theme: Theme,
    state: State,
    team_one_score: u32,
    team_two_score: u32,
    page: u8,
    pages: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ThemeType {
    Light,
    Dark,
} 

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(ThemeType),
    IncrementPressed(Team),
    DecrementPressed(Team),
    PageBack,
    PageForward,
}

#[derive(Debug, Clone)]
enum Team {
    One,
    Two,
  }  

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            state: State::Counter,
            team_one_score: 0,
            team_two_score: 0,
            page: 1,
            pages: 2,
        }
    }

    fn title(&self) -> String {
        let subtitle = match self.state {
            State::Counter => "Teams",
            State::Settings => "Settings",
            State::NotFound => "Page not found"
        };

        format!("{subtitle} - Score Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme_type) => {
                match theme_type {
                    ThemeType::Light => {
                        self.theme = Theme::Light
                    }
                    ThemeType::Dark => {
                        self.theme = Theme::Dark
                    }
                }
            }
            Message::IncrementPressed(team) => {
                match team {
                    Team::One => self.team_one_score += 1,
                    Team::Two => self.team_two_score += 1
                }
            }
            Message::DecrementPressed(team) => {
                match team {
                    Team::One => {
                        if self.team_one_score > 0 {
                            self.team_one_score -= 1
                        }
                    }
                        
                    Team::Two => {
                        if self.team_two_score > 0 {
                            self.team_two_score -= 1
                        }
                    }
                }
            }
            Message::PageForward => {
                if self.page < self.pages {
                    self.page += 1;
                } else {
                    self.page = 1;
                }
            }
            Message::PageBack => {
                if self.page > 1 {
                    self.page -= 1;
                } else {
                    self.page = self.pages;
                }
            }
        }

        match self.page {
            1 => {
                self.state = State::Counter
            }
            2 => {
                self.state = State::Settings
            }
            _ => {
                self.state = State::NotFound
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.state {
            State::Counter => { // Pattern match the variant and destructure the value
                column![
                    text(format!("Team 1: {}", self.team_one_score)).size(25),
                    text(format!("Team 2: {}", self.team_two_score)).size(25),
                    AppButton::view(&AppButton {button_type: ButtonType::TeamOneCounter(CountType::Increment)}, "Give Point to Team 1"),
                    AppButton::view(&AppButton {button_type: ButtonType::TeamOneCounter(CountType::Decrement)}, "Take Point from Team 1"),
                    AppButton::view(&AppButton {button_type: ButtonType::TeamTwoCounter(CountType::Increment)}, "Give Point to Team 2"),
                    AppButton::view(&AppButton {button_type: ButtonType::TeamTwoCounter(CountType::Decrement)}, "Take Point from Team 2"),
                    AppButton::view(&AppButton {button_type: ButtonType::Settings}, "")
                ]
                .padding(20)
                .align_items(Alignment::Center)
            }

            State::Settings => {
                let light_theme_button: Radio<Message, Renderer> = Radio::new(
                    "Light Theme",
                    ThemeType::Light,
                    Some(match self.theme {
                        Theme::Light => ThemeType::Light,
                        Theme::Dark => ThemeType::Dark,
                        _ => ThemeType::Dark,
                    }),
                    Message::ThemeChanged,
                );
                let dark_theme_button: Radio<Message, Renderer> = Radio::new(
                    "Dark Theme",
                    ThemeType::Dark,
                    Some(match self.theme {
                        Theme::Light => ThemeType::Light,
                        Theme::Dark => ThemeType::Dark,
                        _ => ThemeType::Dark,
                    }),
                    Message::ThemeChanged,
                );

                let content = column![
                    light_theme_button, 
                    dark_theme_button, 
                    AppButton::view(&AppButton {button_type: ButtonType::Counter}, ""),
                ].width(Length::Fill).align_items(Alignment::Center);
                content
            }
            
            State::NotFound => {
                column![].into()
            }
        };

        container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug, Clone, Copy)]
enum CountType {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy)]
enum ButtonType {
    Counter,
    Settings,
    TeamOneCounter(CountType),
    TeamTwoCounter(CountType),
}


#[derive(Debug, Clone)]
struct AppButton {
    button_type: ButtonType
}

impl AppButton {
    fn view(&self, label: &'static str) -> Element<Message> {
        match self.button_type {
            ButtonType::Settings => {
                column![
                    button("Settings").on_press(Message::PageForward)
                ]
                .padding(20)
                .align_items(Alignment::Center)
                .into()
            }
            ButtonType::Counter => {
                column![
                    button("Score Counter").on_press(Message::PageBack)
                ]
                .padding(20)
                .align_items(Alignment::Center)
                .into()
            }
            ButtonType::TeamOneCounter(count_type) => {
                match count_type {
                    CountType::Increment => {
                        column![
                            button(label).on_press(Message::IncrementPressed(Team::One)).width(500)
                        ]
                        .align_items(Alignment::Center)
                        .width(Length::Fill)
                        .padding(10)
                        .into()
                    }
                    CountType::Decrement => {
                        column![
                            button(label).on_press(Message::DecrementPressed(Team::One)).width(500)
                        ]
                        .align_items(Alignment::Center)
                        .width(Length::Fill)
                        .padding(10)
                        .into()
                    }
                }
                
            }
            
            
            ButtonType::TeamTwoCounter(count_type) => {
                match count_type {
                    CountType::Increment => {
                        column![
                            button(label).on_press(Message::IncrementPressed(Team::Two)).width(500)
                        ]
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .padding(10)
                        .into()
                    }
                    CountType::Decrement => {
                        column![
                            button(label).on_press(Message::DecrementPressed(Team::Two)).width(500)
                        ]
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .padding(10)
                        .into()
                    }
                }
            }
            }
        }
        
    }