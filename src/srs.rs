use ratatui::prelude::*;
use ratatui::widgets::{block::*, Paragraph, Widget};

use crate::data::Srs;

/* #[derive(Debug, Default)]
pub struct Srs {
    pub name: String,
    pub wkt: String,
    pub proj4: String,
} */

#[derive(Debug, Default, Clone)]
pub struct SrsUi(Srs);

impl SrsUi {
    pub fn new(srs: Srs) -> Self {
        Self(srs)
    }
}

impl Widget for SrsUi {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        //let title = Title::from(" Spatial reference ".bold().yellow());
        let block = Block::default().title(" Spatial reference ".bold().yellow());
        let para = Paragraph::new(format!(
            "Name: {}\n WKT: {}\n Proj4: {}",
            self.0.name, self.0.wkt, self.0.proj4
        ));
        para.block(block).render(area, buf);
    }
}
