use iced::{widget::{button, column, text, Column, container, text_input, row}, Settings, Sandbox, Element, Length, Padding};

pub fn main() -> iced::Result {

    let mut settings = Settings::default();

    settings.window.size = (200,300);

    SimpleInterest::run(settings)
}

pub struct SimpleInterest {

    pub principle: String,
    pub interest_rate: String,
    pub time: String,


}

#[derive(Debug, Clone)]
pub enum Message {
    PChanged(String),
    RChanged(String),
    TChanged(String),

}


impl Sandbox for SimpleInterest {

    type Message = Message;

    fn new() -> SimpleInterest {
        SimpleInterest {
            principle:     String::from("0"),
            interest_rate: String::from("0"),
            time :         String::from("0")
        }
    }

    fn title(&self) -> String {
        format!("Simple Interest")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::PChanged(args) => {
                self.principle = args;

                println!("Principle {}",self.time);

            },
            Message::RChanged(args) => {

                self.interest_rate = args;

                println!("Rate {}",self.time);


            },
            Message::TChanged(args) => {

                self.time = args;

                println!("Time {}",self.time);
            },
        }
    }

    fn view(&self) ->  Element<Message> {

        // I = P*R*T

        let principle = &self.principle;
        let rate = &self.interest_rate;
        let time = &self.time;

        let output: f32 = (principle.parse::<f32>().unwrap() * ( rate.parse::<f32>().unwrap()  / 100. ) * time.parse::<f32>().unwrap());
        let Aoutput: f32 = principle.parse::<f32>().unwrap() + output;


        let si_widget = column![ // simple interest widget
            text("Simple Interest").size(40),
            text_input("Principle?",principle.as_str())
            .on_input(Message::PChanged)
            .padding(15)
            .size(5),
            text_input("Interest Rate?",rate.as_str())
            .on_input(Message::RChanged)
            .padding(15)
            .size(5),
            text_input("Time?",time.as_str())
            .on_input(Message::TChanged)
            .padding(15)
            .size(5),
            text(format!("Interest: {}", output )).size(15),
            text(format!("Interest + Output: {}", output )).size(15),

        ];

        let ci_widget = column![ // simple interest widget
            // do this later
            // i am to lazy
            //
        ];

        let content = row![
            si_widget,
            ci_widget // if we add compound interest
        ];

        container(content)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(15)
        .into()
    
    }
}
