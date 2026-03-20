pub mod camera;
pub mod core;
pub mod gui;
pub mod scene;
pub mod transform;

use bytes::Bytes;
use crate::{camera::Camera, transform::Transform};
use gui::camera_config::CameraConfigWidget;

use ndarray::Array3;
use scene::Scene;

use iced::widget::image::Handle;
use iced::{
    Element, Pixels,
    widget::{Id, button, column, image, slider, text},
};

struct LumosGui {
    cam_widget: CameraConfigWidget,
    scene: Scene,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RenderCall,
    CameraWidget(gui::camera_config::CameraConfigMessage),
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
            }
            Message::CameraWidget(camera_config_message) => {
                self.cam_widget.update(camera_config_message)
            }
        }
    }
    fn view(self: &Self) -> Element<'_, Message> {
        let (h, w) = (
            self.cam_widget.cam.height_resolution,
            self.cam_widget.cam.width_resolution,
        );
        let image_data = Array3::<u8>::ones((h as usize, w as usize, 4)) * 255; 
        let img: Bytes = image_data.into_raw_vec_and_offset().0.into();
        column![
            self.cam_widget.view().map(Message::CameraWidget),
            image(Handle::from_rgba(w, h, img))
        ]
        .into()
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
