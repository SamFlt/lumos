use iced::{Element, widget::combo_box};

use crate::core::renderer_settings::{RenderType, RendererSettings};

#[derive(Debug, Clone)]
pub enum Message {
    RenderTypeSelected(RenderType),
}

pub struct RendererSettingsWidget {
    pub settings: RendererSettings,
    pub render_type_box_state: combo_box::State<RenderType>,
    pub st: Option<RenderType>
}

impl Default for RendererSettingsWidget {
    fn default() -> Self {
        Self {
            settings: RendererSettings::default(),
            render_type_box_state: combo_box::State::with_selection(vec![RenderType::Depth, RenderType::Normals, RenderType::Color], Some(&RenderType::Depth)),
            st: None
        }
    }
}

impl RendererSettingsWidget {
    pub fn update(self: &mut Self, message: Message) {
        println!("{message:?}");
        match message {
            Message::RenderTypeSelected(r) => {
                self.settings.render_type = r.clone();
                self.st = Some(r)
            }
        }
    }

    pub fn view(self: &Self) -> Element<'_, Message> {
        combo_box(
            &self.render_type_box_state,
            "Render Type",
            self.st.as_ref(),
            Message::RenderTypeSelected,
        )
        .into()
    }
}
