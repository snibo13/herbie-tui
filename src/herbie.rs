use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::text::Text;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HerbieLogo {
    size: Size,
}

/// The size of the logo
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Size {
    /// A tiny logo
    ///
    /// The default size of the logo (5x32 characters)
    ///
    /// ```text
    /// █ █ █ █ █ ████ █ ████ ████ ████
    /// █ █ █ █ █ █    █ █  █ █    █
    /// ████ █ █████  █ ████ ████ ████
    /// █ █ █ █ █    █ █ █  █ █    █
    /// █ █ █ █ █    ████ █  █ ████ ████
    /// ```
    #[default]
    Tiny,
    /// A small logo
    ///
    /// A slightly larger version of the logo (5x51 characters)
    ///
    /// ```text
    /// █████   ███████   ███████   ██████    ███████   ███████
    /// █   █   █         █     █   █     █      █      █
    /// █████   ███████   ██████    ██████       █      ███████
    /// █   █   █         █  █      █     █      █      █
    /// █   █   ███████   █   █     ██████       █      ███████
    /// ```
    Small,
}

impl HerbieLogo {
    /// Create a new H.E.R.B.I.E logo widget
    pub const fn new(size: Size) -> Self {
        Self { size }
    }

    /// Set the size of the logo
    #[must_use]
    pub const fn size(self, size: Size) -> Self {
        let _ = self; // Acknowledges that `self` is consumed and a new Self is returned
        Self { size }
    }

    /// Create a new H.E.R.B.I.E logo widget with a tiny size
    pub const fn tiny() -> Self {
        Self::new(Size::Tiny)
    }

    /// Create a new H.E.R.B.I.E logo widget with a small size
    pub const fn small() -> Self {
        Self::new(Size::Small)
    }
}

impl Widget for HerbieLogo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let logo = self.size.as_str();
        Text::raw(logo).render(area, buf);
    }
}

impl Size {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Tiny => Self::tiny(),
            Self::Small => Self::small(),
        }
    }

    // This would typically use `indoc!` in a real project for cleaner multiline strings.
    const fn tiny() -> &'static str {
        "██  ██    ████    ████      ████     ████    ████\n\
         ██  ██    █       █   █     █   █     ██     █   \n\
         ██████    ████    ████      ████      ██     ████\n\
         ██  ██    █       █  ██     █   █     ██     █   \n\
         ██  ██ ██ ████ ██ █   ██ ██ ████  ██ ████ ██ ████"
    }

    // This would typically use `indoc!` in a real project for cleaner multiline strings.
    const fn small() -> &'static str {
        "█████   ███████   ███████   ██████    ███████   ███████\n\
         █   █   █         █     █   █     █      █      █\n\
         █████   ███████   ██████    ██████       █      ███████\n\
         █   █   █         █  █      █     █      █      █\n\
         █   █   ███████   █   █     ██████       █      ███████"
    }
}

#[cfg(test)]
mod tests {
    // Assuming `rstest` would be imported and used if available
    // use rstest::rstest;

    use super::*;

    // #[rstest]
    // #[case::tiny(Size::Tiny)]
    // #[case::small(Size::Small)]
    // fn new_size(#[case] size: Size) {
    //     let logo = HerbieLogo::new(size);
    //     assert_eq!(logo.size, size);
    // }

    #[test]
    fn default_logo_is_tiny() {
        let logo = HerbieLogo::default();
        assert_eq!(logo.size, Size::Tiny);
    }

    #[test]
    fn set_logo_size_to_small() {
        let logo = HerbieLogo::default().size(Size::Small);
        assert_eq!(logo.size, Size::Small);
    }

    #[test]
    fn tiny_logo_constant() {
        let logo = HerbieLogo::tiny();
        assert_eq!(logo.size, Size::Tiny);
    }

    #[test]
    fn small_logo_constant() {
        let logo = HerbieLogo::small();
        assert_eq!(logo.size, Size::Small);
    }

    #[test]
    // #[rustfmt::skip] // Skipped for rustfmt if necessary in a real project
    fn render_tiny() {
        // Tiny Herbie logo is 32 characters wide and 5 lines tall.
        let mut buf = Buffer::empty(Rect::new(0, 0, 32, 5));
        HerbieLogo::tiny().render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█ █ █ █ █ ████ █ ████ ████ ████",
                "█ █ █ █ █ █    █ █  █ █    █   ",
                "████ █ █████  █ ████ ████ ████",
                "█ █ █ █ █    █ █ █  █ █    █   ",
                "█ █ █ █ █    ████ █  █ ████ ████",
            ])
        );
    }

    #[test]
    // #[rustfmt::skip] // Skipped for rustfmt if necessary in a real project
    fn render_small() {
        // Small Herbie logo is 51 characters wide and 5 lines tall.
        let mut buf = Buffer::empty(Rect::new(0, 0, 51, 5));
        HerbieLogo::small().render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "█████   ███████   ███████   ██████    ███████   ███████",
                "█   █   █         █     █   █     █      █      █",
                "█████   ███████   ██████    ██████       █      ███████",
                "█   █   █         █  █      █     █      █      █",
                "█   █   ███████   █   █     ██████       █      ███████",
            ])
        );
    }
}