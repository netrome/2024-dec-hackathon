// TODO: Only if I have time

pub enum DataTheme {
    Light,
    Dark,
    Unset,
}

impl DataTheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "data-theme=light",
            Self::Dark => "data-theme=dark",
            Self::Unset => "",
        }
    }
}
