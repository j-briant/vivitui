use gdal::spatial_ref::SpatialRef;
use gdal::vector::{Geometry, Layer, LayerAccess};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::canvas::{Canvas, Map, MapResolution, Rectangle};
use ratatui::widgets::{block::*, Borders, Paragraph, Tabs, Widget};

#[derive(Debug, Default)]
pub struct PositionMap {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl PositionMap {
    pub fn new(layer: &Layer) -> Self {
        let dst_spatial_ref = SpatialRef::from_epsg(4326).unwrap();
        if let Ok(extent) = layer.get_extent() {
            let mut new_extent =
                Geometry::bbox(extent.MinX, extent.MinY, extent.MaxX, extent.MaxY).unwrap();
            let spatial_ref = layer.spatial_ref().unwrap();
            new_extent.set_spatial_ref(spatial_ref);
            new_extent.transform_to_inplace(&dst_spatial_ref).unwrap();
            let new_envelope = new_extent.envelope();
            Self {
                xmin: new_envelope.MinX,
                ymin: new_envelope.MinY,
                xmax: new_envelope.MaxX,
                ymax: new_envelope.MaxY,
            }
        } else {
            Self {
                xmin: 0.0,
                ymin: 0.0,
                xmax: 0.0,
                ymax: 0.0,
            }
        }
    }
}

impl Widget for PositionMap {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(
                format!(
                    "xmin: {} xmax: {} ymin: {} ymax: {}",
                    self.xmin, self.xmax, self.ymin, self.ymax
                )
                .bold()
                .yellow(),
            )
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        let map = Canvas::default()
            .block(block.title("Canvas".bold().yellow()))
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,
                    color: Color::White,
                });
                ctx.draw(&Rectangle {
                    x: self.ymin - 10.0,
                    y: self.xmin - 10.0,
                    width: self.ymax - self.ymin + 10.0,
                    height: self.xmax - self.xmin + 10.0,
                    color: Color::Red,
                });
            })
            .marker(Marker::Dot);
        map.render(area, buf)
    }
}
