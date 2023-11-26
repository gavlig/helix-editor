use crate::compositor::{Component, Context, ContextExt, surface_by_id_mut};
use tui::buffer::{Buffer as Surface, SurfaceFlags};

use helix_view::graphics::Rect;

pub struct Text {
    pub(crate) contents: tui::text::Text<'static>,
    size: (u16, u16),
    viewport: (u16, u16),
}

impl Text {
    pub fn new(contents: String) -> Self {
        Self {
            contents: tui::text::Text::from(contents),
            size: (0, 0),
            viewport: (0, 0),
        }
    }
}

impl From<tui::text::Text<'static>> for Text {
    fn from(contents: tui::text::Text<'static>) -> Self {
        Self {
            contents,
            size: (0, 0),
            viewport: (0, 0),
        }
    }
}

impl Component for Text {
    fn render(&mut self, area: Rect, surface: &mut Surface, _cx: &mut Context) {
        use tui::widgets::{Paragraph, Widget, Wrap};

        let par = Paragraph::new(self.contents.clone()).wrap(Wrap { trim: false });
        // .scroll(x, y) offsets

        par.render(area, surface);
    }

    fn render_ext(&mut self, ctx: &mut ContextExt) {
        use tui::widgets::{Paragraph, Widget, Wrap};

        let par = Paragraph::new(self.contents.clone()).wrap(Wrap { trim: false });
        // .scroll(x, y) offsets

        let id = String::from(self.id().unwrap());
        let area = Rect { width: self.size.0, height: self.size.1, ..Default::default() };

        let surface = surface_by_id_mut(&id, area, SurfaceFlags::default(), ctx.surfaces);
        par.render(area, surface);
    }

    fn id(&self) -> Option<&'static str> {
        Some("text-component")
    }

    fn required_size(&mut self, viewport: (u16, u16)) -> Option<(u16, u16)> {
        if viewport != self.viewport {
            let width = std::cmp::min(self.contents.width() as u16, viewport.0);
            let height = std::cmp::min(self.contents.height() as u16, viewport.1);
            self.size = (width, height);
            self.viewport = viewport;
        }
        Some(self.size)
    }
}

pub fn required_size(text: &tui::text::Text, max_text_width: u16) -> (u16, u16) {
    let mut text_width = 0;
    let mut height = 0;
    for content in &text.lines {
        height += 1;
        let content_width = content.width() as u16;
        if content_width > max_text_width {
            text_width = max_text_width;
            height += content_width.checked_div(max_text_width).unwrap_or(0);
        } else if content_width > text_width {
            text_width = content_width;
        }
    }
    (text_width, height)
}
