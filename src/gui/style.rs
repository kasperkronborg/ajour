use iced::{button, container, Background, Color};

enum ColorPalette {
    Primary,
    OnPrimary,
    Secondary,
    OnSecondary,
    Error,
    OnError,
    Surface,
    OnSurface,
    Background,
}

impl ColorPalette {
    fn rgb(&self) -> Color {
        match self {
            ColorPalette::Primary => Color::from_rgb(0.88, 0.74, 0.28),
            ColorPalette::OnPrimary => Color::from_rgb(0.32, 0.27, 0.10),
            ColorPalette::Secondary => Color::from_rgb(0.22, 0.17, 0.28),
            ColorPalette::OnSecondary => Color::from_rgb(0.73, 0.52, 0.99),
            ColorPalette::Surface => Color::from_rgb(0.12, 0.12, 0.12),
            ColorPalette::OnSurface => Color::from_rgb(0.88, 0.88, 0.88),
            ColorPalette::Background => Color::from_rgb(0.07, 0.07, 0.07),
            ColorPalette::Error => Color::from_rgb(0.81, 0.4, 0.47),
            ColorPalette::OnError => Color::from_rgb(0.68, 0.0, 0.13),
        }
    }
}

pub struct DefaultButton;
impl button::StyleSheet for DefaultButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::Primary.rgb())),
            text_color: ColorPalette::OnPrimary.rgb(),
            border_radius: 2,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::OnPrimary.rgb())),
            text_color: ColorPalette::Primary.rgb(),
            ..self.active()
        }
    }
}

pub struct SecondaryButton;
impl button::StyleSheet for SecondaryButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::Secondary.rgb())),
            text_color: ColorPalette::OnSecondary.rgb(),
            border_radius: 2,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::Secondary.rgb())),
            text_color: ColorPalette::OnSecondary.rgb(),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::Secondary.rgb())),
            text_color: ColorPalette::OnSecondary.rgb(),
            ..self.active()
        }
    }
}

pub struct DeleteButton;
impl button::StyleSheet for DeleteButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::Error.rgb())),
            text_color: ColorPalette::OnError.rgb(),
            border_radius: 2,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ColorPalette::OnError.rgb())),
            text_color: ColorPalette::Error.rgb(),
            ..self.active()
        }
    }
}

pub struct Content;
impl container::StyleSheet for Content {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ColorPalette::Background.rgb())),
            ..container::Style::default()
        }
    }
}

pub struct AddonTextContainer;
impl container::StyleSheet for AddonTextContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ColorPalette::Surface.rgb())),
            text_color: Some(ColorPalette::OnSurface.rgb()),
            ..container::Style::default()
        }
    }
}

pub struct StatusTextContainer;
impl container::StyleSheet for StatusTextContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ColorPalette::Background.rgb())),
            text_color: Some(ColorPalette::OnSurface.rgb()),
            ..container::Style::default()
        }
    }
}

pub struct AddonDescriptionContainer;
impl container::StyleSheet for AddonDescriptionContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ColorPalette::Surface.rgb())),
            text_color: Some(ColorPalette::OnSurface.rgb()),
            ..container::Style::default()
        }
    }
}

pub struct Cell;
impl container::StyleSheet for Cell {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(ColorPalette::Background.rgb())),
            ..container::Style::default()
        }
    }
}
