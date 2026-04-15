use iced::Length::Fill;
use iced::widget::{row, rule};

pub mod camera_config;
pub mod renderer_settings;

use bytes::Bytes;
use camera_config::CameraConfigWidget;

use iced::widget::button;
use ndarray::Array3;

use crate::core::renderer::LumosRenderer;
use crate::gui::renderer_settings::RendererSettingsWidget;

use iced::widget::image::Handle;
use iced::{
    Element,
    widget::{column, image},
};

pub struct LumosGui {
    cam_widget: CameraConfigWidget,
    settings_widget: RendererSettingsWidget,
    renderer: LumosRenderer,
    image: Array3<u8>,
}

#[derive(Debug, Clone)]
pub enum Message {
    RenderCall,
    CameraWidget(camera_config::CameraConfigMessage),
    RenderSettingsWidget(renderer_settings::Message)
}

impl Default for LumosGui {
    fn default() -> Self {
        LumosGui {
            cam_widget: CameraConfigWidget::default(),
            settings_widget: RendererSettingsWidget::default(),
            renderer: LumosRenderer::default(),
            image: Array3::<u8>::default((800, 400, 4)),
        }
    }
}

impl LumosGui {
    pub fn update(self: &mut Self, message: Message) {
        match message {
            Message::RenderCall => {
                println!("Rendering was called");
                self.renderer.camera = self.cam_widget.cam.clone();
                self.renderer.settings = self.settings_widget.settings.clone();
                self.image = self.renderer.render();
            }
            Message::CameraWidget(camera_config_message) => {
                self.cam_widget.update(camera_config_message)
            }
            Message::RenderSettingsWidget(message) => {
                self.settings_widget.update(message)
            },
        }
    }
    pub fn view(self: &Self) -> Element<'_, Message> {
        let (h, w) = (self.image.shape()[0] as u32, self.image.shape()[1] as u32);
        let img: Bytes = self.image.clone().into_raw_vec_and_offset().0.into();

        let config_col = column![
            self.cam_widget.view().map(Message::CameraWidget),
            self.settings_widget.view().map(Message::RenderSettingsWidget),
            button("Render").on_press(Message::RenderCall),
        ]
        .max_width(200);

        row![
            config_col,
            rule::vertical(5),
            image(Handle::from_rgba(w, h, img)).width(Fill)
        ]
        .into()
    }
}
