
use crate::core::transform::Transform;

pub trait Body {
    fn pose(&self) -> Transform;

    fn in_other_body_frame(&self, other: &Self) -> Self;
}