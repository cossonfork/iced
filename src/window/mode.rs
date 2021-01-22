/// The mode of a window-based application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The application appears in its own window.
    Windowed,

    /// The application takes the whole screen of its current monitor.
    Fullscreen,

    /// The application takes minimum
    Minimum,
}

impl Into<iced_winit::Mode> for Mode {
    fn into(self) -> iced_winit::Mode {
        match self {
            Self::Windowed => iced_winit::Mode::Windowed,
            Self::Fullscreen => iced_winit::Mode::Fullscreen,
            Self::Minimum => iced_winit::Mode::Minimum,
        }
    }
}
