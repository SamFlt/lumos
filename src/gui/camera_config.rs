use crate::{core::camera::Camera, core::transform::Transform};

use iced::{
    Element,
    widget::{button, column, row, slider, text},
};

#[derive(Debug, Copy, Clone)]
pub enum CameraConfigMessage {
    FocalLengthChanged(f64),
    SensorWidthChanged(f64),
    SensorHeightChanged(f64),
    ResolutionWidthChanged(u32),
    ResolutionHeightChanged(u32),
}

pub struct CameraConfigWidget {
    pub cam: Camera,
}

impl CameraConfigWidget {
    pub fn view(self: &Self) -> Element<'_, CameraConfigMessage> {
        column![
            row![
                "Focal length",
                text(self.cam.focal_length.to_string()),
                slider(
                    0.0..=0.05,
                    self.cam.focal_length,
                    CameraConfigMessage::FocalLengthChanged
                )
                .step(0.001),
                "m"
            ],
            row![
                "Sensor width",
                slider(
                    0.0..=0.05,
                    self.cam.sensor_width,
                    CameraConfigMessage::SensorWidthChanged
                ),
                "m"
            ],
            row![
                "Sensor height",
                slider(
                    0.0..=0.05,
                    self.cam.sensor_height,
                    CameraConfigMessage::SensorHeightChanged
                ),
                "m"
            ],
            row![
                "Resolution width",
                slider(
                    128..=4096,
                    self.cam.width_resolution,
                    CameraConfigMessage::ResolutionWidthChanged
                ),
                "px"
            ],
            row![
                "Resolution height",
                slider(
                    128..=4096,
                    self.cam.height_resolution,
                    CameraConfigMessage::ResolutionHeightChanged
                ),
                "px"
            ],
        ]
        .into()
    }

    pub fn update(self: &mut Self, message: CameraConfigMessage) {
        match message {
            CameraConfigMessage::FocalLengthChanged(focal) => {

                self.cam.focal_length = focal
            }
            CameraConfigMessage::SensorWidthChanged(width) => self.cam.sensor_width = width,
            CameraConfigMessage::SensorHeightChanged(height) => self.cam.sensor_height = height,
            CameraConfigMessage::ResolutionWidthChanged(width) => self.cam.width_resolution = width,
            CameraConfigMessage::ResolutionHeightChanged(height) => {
                self.cam.height_resolution = height
            }
        }
    }
}

impl Default for CameraConfigWidget {
    fn default() -> Self {
        Self {
            cam: Camera {
                pose: Transform::new(),
                focal_length: 0.032,
                sensor_width: 0.05,
                sensor_height: 0.05,
                width_resolution: 1200,
                height_resolution: 800,
            },
        }
    }
}
