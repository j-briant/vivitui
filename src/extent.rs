use gdal::vector::{Layer, LayerAccess};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{block::*, Borders, Paragraph, Tabs, Widget};

#[derive(Debug, Default)]
pub struct Extent {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl Extent {
    pub fn new(layer: &Layer) -> Self {
        let extent = layer.get_extent();
        match extent {
            Ok(ext) => Self {
                xmin: ext.MinX,
                ymin: ext.MinY,
                xmax: ext.MaxX,
                ymax: ext.MaxY,
            },
            Err(_) => Self::default(),
        }
    }
}

impl Widget for Extent {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        //let title = Title::from(" Spatial reference ".bold().yellow());
        let block = Block::default()
            .title(" Extent ".bold().yellow())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        let text = Text::from(vec![
            Line::from(vec![
                Span::from("Xmin: ").bold().yellow(),
                Span::from(self.xmin.to_string()),
            ]),
            Line::from(vec![
                Span::from("Ymin: ").bold().yellow(),
                Span::from(self.ymin.to_string()),
            ]),
            Line::from(vec![
                Span::from("Xmax: ").bold().yellow(),
                Span::from(self.xmax.to_string()),
            ]),
            Line::from(vec![
                Span::from("Ymax: ").bold().yellow(),
                Span::from(self.ymax.to_string()),
            ]),
        ]);
        let para = Paragraph::new(text);
        para.block(block).render(area, buf);
    }
}
