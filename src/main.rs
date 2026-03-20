pub mod camera;
pub mod core;
pub mod gui;
pub mod scene;
pub mod transform;

use gui::camera_config::CameraConfigWidget;
use crate::{camera::Camera, transform::Transform};

use scene::Scene;

use iced::{
    Element,
    widget::{button, column, slider, text},
};

struct LumosGui {
    cam_widget: CameraConfigWidget,
    scene: Scene
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RenderCall,
    CameraWidget(gui::camera_config::CameraConfigMessage)
}

impl Default for LumosGui {
    fn default() -> Self {
        LumosGui {
            cam_widget: CameraConfigWidget::default(),
            scene: Scene { objects: vec![] },
        }
    }
}

impl LumosGui {
    fn update(self: &mut Self, message: Message) {
        match message {
            Message::RenderCall => {
                println!("Rendering was called");
            },
            Message::CameraWidget(camera_config_message) => {
                self.cam_widget.update(camera_config_message)
            },
        }
    }
    fn view(self: &Self) -> Element<'_, Message> {
        column![self.cam_widget.view().map(Message::CameraWidget)].into()
    }

    fn render() {}
}

fn main() {
    let res = iced::run(LumosGui::update, LumosGui::view);

    match res {
        Ok(_) => {
            println!("GUI ran successfully");
        }
        Err(_) => {
            println!("Error during GUI run")
        }
    }
}
