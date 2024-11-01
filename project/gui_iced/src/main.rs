use iced::{
    executor, Align, Application, Checkbox, Column, Command, Container,
    Element, Length, Settings, Subscription, Text,
};
use std::thread;
use tokio;
use tokio::time::delay_for;
use std::time::Duration;
use tokio::task;
use tokio::sync::mpsc;

pub fn main() -> iced::Result {
    Events::run(Settings::default())
}

#[derive(Debug, Default)]
struct Events {
    last: Vec<iced_native::Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Init,
}

async fn other_init() {
    // unbounded_channel可以用来在同步线程和异步线程通信
    let (mut tx, mut rx) = mpsc::unbounded_channel();     
    
    // 开启一个同步线程
    task::spawn_blocking(move || {
        println!("spawn_blocking begin thread: {:?}", thread::current().id());
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_secs(1));            
            if let Err(_) = tx.send(i) {
                println!("receiver dropped");
                return;
            }
            i += 1;
        }
        println!("spawn_blocking end thread: {:?}", thread::current().id());
    });

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }

    /*
    loop {
        println!("other_init begin thread: {:?}", thread::current().id());    
        delay_for(Duration::from_millis(5000)).await;
        println!("other_init end thread: {:?}", thread::current().id());
    }
    */               
}

async fn init() {
    // 创建新的future
    tokio::spawn(async {     
        tokio::join!(other_init());
        println!("nerver: {:?}", thread::current().id());
    });
    
    println!("init thread: {:?}", thread::current().id());
}

impl Application for Events {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (
            Events::default(),
            Command::perform(init(), |_| Message::Init),
        )
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        println!("update thread: {:?}", thread::current().id());
        match message {
            Message::EventOccurred(event) => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
            }
            Message::Toggled(enabled) => {                
                self.enabled = enabled;
            }
            Message::Init => {
                println!("message init thread: {:?}", thread::current().id());                          
            }
        };
        
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.enabled {
            iced_native::subscription::events().map(Message::EventOccurred)
        } else {
            Subscription::none()
        }
    }

    fn view(&mut self) -> Element<Message> {
        let events = self.last.iter().fold(
            Column::new().spacing(10),
            |column, event| {
                column.push(Text::new(format!("{:?}", event)).size(40))
            },
        );

        let toggle = Checkbox::new(
            self.enabled,
            "Listen to runtime events",
            Message::Toggled,
        );

        let content = Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(events)
            .push(toggle);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
