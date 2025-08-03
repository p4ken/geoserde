#![cfg(feature="shapefile")]

use std::{io::Cursor, vec};

use geoserde::GeoDeserialize;
use serde::{Deserialize, Serialize};

#[test]
fn de() -> anyhow::Result<()> {
    // Create in-memory shapefile
    let mut shp_buf = vec![];
    let mut dbf_buf = vec![];
    {
        let shp_w = shapefile::ShapeWriter::new(Cursor::new(&mut shp_buf));
        let dbf_w = shapefile::dbase::TableWriterBuilder::new()
            .add_integer_field("number".try_into().unwrap())
            .add_character_field("text".try_into().unwrap(), 50)
            .build_with_dest(Cursor::new(&mut dbf_buf));
        let mut w = shapefile::Writer::new(shp_w, dbf_w);
        w.write_shape_and_record(&polyline(1), &MyProperty::one())?;
        w.write_shape_and_record(&polyline(2), &MyProperty::two())?;
    }

    // Initialize reader
    let shp_r = shapefile::ShapeReader::new(Cursor::new(shp_buf))?;
    let dbf_r = shapefile::dbase::Reader::new(Cursor::new(dbf_buf))?;
    let mut reader = shapefile::Reader::new(shp_r, dbf_r);

    // ジオメトリは1コピーとなる
    for res in reader.iter_shapes_and_records_as::<shapefile::Polyline, MyProperty>() {
        let (geom, _prop) = res.unwrap();
        let _geom = geo_types::MultiLineString::from(geom)
            .0
            .into_iter()
            .next()
            .unwrap();
    }
    Ok(())
}

fn polyline(i: i32) -> shapefile::Polyline {
    let org = i as f64;
    geo_types::LineString::from(vec![[org, org + 0.1], [org + 0.2, org + 0.3]]).into()
}

#[derive(Serialize, Deserialize)]
struct MyProperty {
    number: i32,
    text: String,
}
impl MyProperty {
    pub fn one() -> Self {
        Self {
            number: 1,
            text: "one".into(),
        }
    }
    pub fn two() -> Self {
        Self {
            number: 2,
            text: "two".into(),
        }
    }
}

#[derive(GeoDeserialize)]
struct MyFeature {
    _number: i32,
    _text: String,
    #[geometry]
    _shape: geo_types::LineString,
}
