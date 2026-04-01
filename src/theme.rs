use colored::Color;
use dark_light::Mode;

pub fn detect() -> Theme {
    match dark_light::detect() {
        Ok(Mode::Dark) => Theme::Dark,
        _ => Theme::Light,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn is_light(&self) -> bool {
        matches!(self, Theme::Light)
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, Theme::Dark)
    }

    pub fn cyan(&self) -> Color {
        if self.is_dark() { Color::BrightCyan } else { Color::Cyan }
    }

    pub fn blue(&self) -> Color {
        if self.is_dark() { Color::BrightBlue } else { Color::Blue }
    }

    pub fn magenta(&self) -> Color {
        if self.is_dark() { Color::BrightMagenta } else { Color::Magenta }
    }

    pub fn red(&self) -> Color {
        if self.is_dark() { Color::BrightRed } else { Color::Red }
    }

    pub fn green(&self) -> Color {
        if self.is_dark() { Color::BrightGreen } else { Color::Green }
    }

    pub fn yellow(&self) -> Color {
        if self.is_dark() { Color::BrightYellow } else { Color::Yellow }
    }
}
