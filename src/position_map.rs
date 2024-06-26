use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Map, MapResolution, Rectangle};
use ratatui::widgets::{block::*, Widget};

use crate::data::PositionMap;

/* #[derive(Debug, Default)]
pub struct PositionMap {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
} */

pub struct PositionMapUi(PositionMap);

impl PositionMapUi {
    pub fn new(position_map: PositionMap) -> Self {
        Self(position_map)
    }
}

/* impl PositionMap {
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
 */

impl Widget for PositionMapUi {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default().title("Canvas".bold().yellow());

        let map = Canvas::default()
            .block(block)
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,
                    color: Color::White,
                });
                ctx.draw(&Rectangle {
                    x: self.0.ymin,
                    y: self.0.xmin,
                    width: self.0.ymax - self.0.ymin,
                    height: self.0.xmax - self.0.xmin,
                    color: Color::Red,
                });
            })
            .marker(Marker::Braille);
        map.render(area, buf)
    }
}
