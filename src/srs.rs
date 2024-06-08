use gdal::vector::{Layer, LayerAccess};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{block::*, Borders, Paragraph, Tabs, Widget, Wrap};

#[derive(Debug, Default)]
pub struct Srs {
    pub name: String,
    pub wkt: String,
    pub proj4: String,
}

impl Srs {
    pub fn new(layer: &Layer) -> Self {
        match layer.spatial_ref() {
            Some(s) => Self {
                name: s.name().unwrap(),
                wkt: s.to_pretty_wkt().unwrap(),
                proj4: s.to_proj4().unwrap(),
            },
            None => Srs::default(),
        }
    }
}

impl Widget for Srs {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        //let title = Title::from(" Spatial reference ".bold().yellow());
        let block = Block::default()
            .title(" Spatial reference ".bold().yellow())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        let para = Paragraph::new(format!(
            "Name: {}\n WKT: {}\n Proj4: {}",
            self.name, self.wkt, self.proj4
        ))
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
        para.block(block).render(area, buf);
    }
}
