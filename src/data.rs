use gdal::{errors::GdalError, Dataset, DatasetOptions, DriverManager, GdalOpenFlags};
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
