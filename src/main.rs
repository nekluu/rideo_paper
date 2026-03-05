use iced::{Color, Element, Task};
use iced_layershell::application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;
use iced_video_player::{Video, VideoPlayer};

pub fn main() -> Result<(), iced_layershell::Error> {
    application(App::new, App::namespace, App::update, App::view)
        .style(|app: &App, theme| app.style(theme))
        //.subscription(|app: &App| app.subscription())
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: None,
                exclusive_zone: -1,
                anchor: Anchor::all(),
                start_mode: StartMode::Active,
                layer: Layer::Background,
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}

struct App {
    video: Video,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {}


impl App {
    fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let video_path = current_dir.join(".media").join("173656-849839042.mp4");
        
        let mut video = Video::new(
            &url::Url::from_file_path(&video_path).unwrap(),
        )
            .unwrap();
        video.set_looping(true);
        Self { video }
    }
    fn namespace() -> String {
        String::from("Counter - Iced")
    }
    /*
        fn subscription(&self) -> iced::Subscription<Message> {

        }
    */
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        VideoPlayer::new(&self.video)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .content_fit(iced::ContentFit::Cover)
            .into()
    }

    fn style(&self, theme: &iced::Theme) -> iced::theme::Style {
        use iced::theme::Style;
        Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}