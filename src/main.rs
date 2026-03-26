pub mod core;
pub mod gui;

use bytes::Bytes;
use gui::camera_config::CameraConfigWidget;

use iced::widget::button;
use ndarray::Array3;

use crate::core::renderer::LumosRenderer;

use iced::widget::image::Handle;
use iced::{
    Element,
    widget::{column, image},
};

struct LumosGui {
    cam_widget: CameraConfigWidget,
    renderer: LumosRenderer,
    image: Array3<u8>,
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
            renderer: LumosRenderer::default(),
            image: Array3::<u8>::default((800, 400, 4)),
        }
    }
}

impl LumosGui {
    fn update(self: &mut Self, message: Message) {
        match message {
            Message::RenderCall => {
                println!("Rendering was called");
                self.renderer.camera = self.cam_widget.cam.clone();
                self.image = self.renderer.render();
            }
            Message::CameraWidget(camera_config_message) => {
                self.cam_widget.update(camera_config_message)
            }
        }
    }
    fn view(self: &Self) -> Element<'_, Message> {
        let (h, w) = (self.image.shape()[0] as u32, self.image.shape()[1] as u32);
        let img: Bytes = self.image.clone().into_raw_vec_and_offset().0.into();
        column![
            self.cam_widget.view().map(Message::CameraWidget),
            button("Render").on_press(Message::RenderCall),
            image(Handle::from_rgba(w, h, img))
        ]
        .into()
    }
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
