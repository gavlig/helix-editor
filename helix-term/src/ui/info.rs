use crate::compositor::{Component, Context, ContextExt, surface_by_id_mut};
use helix_view::graphics::{Margin, Rect};
use helix_view::info::Info;
use tui::buffer::{Buffer as Surface, SurfaceFlags};
use tui::widgets::{Block, Borders, Paragraph, Widget};

impl Component for Info {
    fn render(&mut self, viewport: Rect, surface: &mut Surface, cx: &mut Context) {
        let text_style = cx.editor.theme.get("ui.text.info");
        let popup_style = cx.editor.theme.get("ui.popup.info");

        // Calculate the area of the terminal to modify. Because we want to
        // render at the bottom right, we use the viewport's width and height
        // which evaluate to the most bottom right coordinate.
        let width = self.width + 2 + 2; // +2 for border, +2 for margin
        let height = self.height + 2; // +2 for border
        let area = viewport.intersection(Rect::new(
            viewport.width.saturating_sub(width),
            viewport.height.saturating_sub(height + 2), // +2 for statusline
            width,
            height,
        ));
        surface.clear_with(area, popup_style);

        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_style(popup_style);

        let margin = Margin::horizontal(1);
        let inner = block.inner(area).inner(&margin);
        block.render(area, surface);

        Paragraph::new(self.text.as_str())
            .style(text_style)
            .render(inner, surface);
    }

    fn render_ext(&mut self, ctx: &mut ContextExt) {
        let id = String::from(self.id().unwrap());
		let info_area = self.area();
        let surface = surface_by_id_mut(&id, info_area, SurfaceFlags::default(), ctx.surfaces);

        let text_style = ctx.vanilla.editor.theme.get("ui.text.info");
        let popup_style = ctx.vanilla.editor.theme.get("ui.popup.info");


        surface.clear_with(info_area, popup_style);

        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_style(popup_style);

        let margin = Margin::horizontal(1);
        let inner = block.inner(info_area).inner(&margin);
        block.render(info_area, surface);

        Paragraph::new(self.text.as_str())
            .style(text_style)
            .render(inner, surface);
    }

    fn id(&self) -> Option<&'static str> {
        Some("info-component")
    }
}
