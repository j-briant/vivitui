use gdal::{
    errors::GdalError,
    spatial_ref::SpatialRef,
    vector::{Geometry, Layer, LayerAccess},
    Dataset, DatasetOptions, DriverManager, GdalOpenFlags,
};
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref DRIVERS: Vec<String> = {
        DriverManager::register_all();
        let count = DriverManager::count();
        let mut list: Vec<String> = vec![];
        for i in 0..count {
            if let Ok(d) = DriverManager::get_driver(i) {
            list.push(d.short_name())
            }
        }
        list
    };
}

lazy_static::lazy_static! {
    static ref DRIVERS_STR: Vec<&'static str> = {
        let v: Vec<&str> = DRIVERS.iter().map(|s| s.as_str()).collect();
        v
    };
}

fn get_dataset_options() -> DatasetOptions<'static> {
    DatasetOptions {
        open_flags: GdalOpenFlags::GDAL_OF_VECTOR,
        allowed_drivers: Some(&DRIVERS_STR),
        open_options: None,
        sibling_files: None,
    }
}

pub fn dataset(p: PathBuf) -> Result<Dataset, GdalError> {
    Dataset::open_ex(p, get_dataset_options())
}

#[derive(Debug, Default, Clone)]
pub struct LayerInfo {
    pub name: String,
    pub extent: Extent,
    pub position_map: PositionMap,
    pub srs: Srs,
    pub feature_number: u64,
}

impl From<&Layer<'_>> for LayerInfo {
    fn from(layer: &Layer) -> Self {
        Self {
            name: layer.name(),
            extent: Extent::from(layer),
            position_map: PositionMap::from(layer),
            srs: Srs::from(layer),
            feature_number: layer.feature_count(),
        }
    }
}

impl LayerInfo {
    pub fn from_dataset(dataset: &Dataset) -> Vec<Self> {
        dataset.layers().map(|l| LayerInfo::from(&l)).collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Extent {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

impl From<&Layer<'_>> for Extent {
    fn from(layer: &Layer) -> Self {
        if let Ok(extent) = layer.get_extent() {
            Self {
                xmin: extent.MinX,
                ymin: extent.MinY,
                xmax: extent.MaxX,
                ymax: extent.MaxY,
            }
        } else {
            Self::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Srs {
    pub name: String,
    pub wkt: String,
    pub proj4: String,
}

impl From<&Layer<'_>> for Srs {
    fn from(layer: &Layer<'_>) -> Self {
        match layer.spatial_ref() {
            Some(srs) => Self {
                name: srs.name().unwrap_or_default(),
                wkt: srs.to_pretty_wkt().unwrap_or_default(),
                proj4: srs.to_proj4().unwrap_or_default(),
            },
            None => Self::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PositionMap {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

impl From<&Layer<'_>> for PositionMap {
    fn from(layer: &Layer<'_>) -> Self {
        let dst_spatial_ref = SpatialRef::from_epsg(4326).unwrap();
        let extent = Extent::from(layer);
        let mut new_extent =
            Geometry::bbox(extent.xmin, extent.ymin, extent.xmax, extent.ymax).unwrap();
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
    }
}
