use ratatui::prelude::*;
use ratatui::widgets::{block::*, Paragraph, Widget};

use crate::data::Extent;

#[derive(Debug, Default)]
pub struct ExtentUi(Extent);

impl ExtentUi {
    pub fn new(extent: Extent) -> Self {
        Self(extent)
    }
}

impl Widget for ExtentUi {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        //let title = Title::from(" Spatial reference ".bold().yellow());
        let block = Block::default().title(" Extent ".bold().yellow());

        let text = Text::from(vec![
            Line::from(vec![
                Span::from("Xmin: ").bold().yellow(),
                Span::from(self.0.xmin.to_string()),
            ]),
            Line::from(vec![
                Span::from("Ymin: ").bold().yellow(),
                Span::from(self.0.ymin.to_string()),
            ]),
            Line::from(vec![
                Span::from("Xmax: ").bold().yellow(),
                Span::from(self.0.xmax.to_string()),
            ]),
            Line::from(vec![
                Span::from("Ymax: ").bold().yellow(),
                Span::from(self.0.ymax.to_string()),
            ]),
        ]);
        let para = Paragraph::new(text);
        para.block(block).render(area, buf);
    }
}
