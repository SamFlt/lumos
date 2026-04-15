#[derive(Debug, Clone)]
pub enum RenderType {
    Depth,
    Normals,
    Color,
}

impl std::fmt::Display for RenderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Depth => "Depth",
            Self::Normals => "Normals",
            Self::Color => "Color",
        })
    }
}

#[derive(Clone)]
pub struct RendererSettings {
    pub render_type: RenderType,
}

impl Default for RendererSettings {
    fn default() -> Self {
        Self {
            render_type: RenderType::Depth,
        }
    }
}
