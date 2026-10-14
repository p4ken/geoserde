use serde::Serialize;

mod testing;

/// A root-level rejection keeps the `TableError` in the source chain.
#[test]
fn root_rejection_source_chain() {
    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::Point);

    let err = ser.serialize_feature(testing::p(0), 42_i32).unwrap_err();
    assert!(matches!(
        err,
        geoserde_fgb::ser::Error::Table(geoserde::ser::TableError::Root)
    ));
    let source = std::error::Error::source(&err).unwrap();
    assert!(source.is::<geoserde::ser::TableError<std::convert::Infallible>>());
}

/// A feature that fails midway leaves nothing behind for the next feature.
#[test]
fn failed_feature_discarded() -> anyhow::Result<()> {
    use flatgeobuf::FallibleStreamingIterator;

    struct Failing;

    impl Serialize for Failing {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("boom"))
        }
    }

    #[derive(Serialize)]
    struct Bad {
        a: i32,
        b: i32,
        c: Failing,
    }

    #[derive(Serialize)]
    struct Good {
        a: i32,
    }

    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::Point);

    let bad = Bad {
        a: 1,
        b: 1,
        c: Failing,
    };
    assert!(ser.serialize_feature(testing::p(0), &bad).is_err());
    ser.serialize_feature(testing::p(1), &Good { a: 2 })?;

    let mut fgb_buf = Vec::new();
    ser.write(&mut fgb_buf)?;

    let fgb_reader = flatgeobuf::FgbReader::open(std::io::Cursor::new(fgb_buf))?;
    let columns = fgb_reader.header().columns().unwrap();
    let names: Vec<_> = columns.iter().map(|c| c.name()).collect();
    assert_eq!(names, ["a"]);

    let mut fgb_iter = fgb_reader.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(
        props,
        std::collections::HashMap::from([("a".to_string(), "2".to_string())])
    );
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

/// A geometry of the wrong type leaves nothing behind for the next feature.
#[test]
fn mismatched_geometry_discarded() -> anyhow::Result<()> {
    use flatgeobuf::FallibleStreamingIterator;

    #[derive(Serialize)]
    struct Props {
        a: i32,
    }

    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::Point);

    assert!(
        ser.serialize_feature(testing::ls(0), &Props { a: 1 })
            .is_err()
    );
    ser.serialize_feature(testing::p(1), &Props { a: 2 })?;

    let mut fgb_buf = Vec::new();
    ser.write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(std::io::Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(
        props,
        std::collections::HashMap::from([("a".to_string(), "2".to_string())])
    );
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

/// Without promote_to_multi, a LineString cannot be written to a MultiLineString layer.
#[test]
fn not_promoted_rejected() {
    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::MultiLineString);

    #[derive(Serialize)]
    struct NoProps {}
    let result = ser.serialize_feature(testing::ls(0), NoProps {});
    assert!(matches!(result, Err(geoserde_fgb::ser::Error::Geozero(_))));
}

/// A GeometryCollection is rejected even by an Unknown layer.
#[test]
fn geometry_collection_rejected() {
    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::Unknown);

    #[derive(Serialize)]
    struct NoProps {}
    let gc = geo_types::GeometryCollection::new_from(vec![testing::p(0).into()]);
    let result = ser.serialize_feature(gc, NoProps {});
    assert!(matches!(
        result,
        Err(geoserde_fgb::ser::Error::UnsupportedGeometry(_))
    ));
}

/// A GeometryCollection layer rejects every geometry.
#[test]
fn geometry_collection_layer_rejected() {
    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::GeometryCollection);

    #[derive(Serialize)]
    struct NoProps {}
    let result = ser.serialize_feature(testing::p(0), NoProps {});
    assert!(matches!(result, Err(geoserde_fgb::ser::Error::Geozero(_))));
}

/// An empty point is rejected, since FlatGeobuf cannot represent it.
#[test]
fn empty_point_rejected() {
    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::Point);

    #[derive(Serialize)]
    struct NoProps {}
    let result = ser.serialize_feature(testing::MaybePoint(None), NoProps {});
    assert!(matches!(result, Err(geoserde_fgb::ser::Error::Geozero(_))));
}

/// An empty point is rejected also when promoted to a MultiPoint.
#[test]
fn empty_point_promoted_rejected() {
    let mut ser = geoserde_fgb::LayerSerializer::options()
        .geometry_type(geoserde_fgb::flatgeobuf::GeometryType::MultiPoint)
        .index(false)
        .promote_to_multi(true)
        .create()
        .unwrap();

    #[derive(Serialize)]
    struct NoProps {}
    let result = ser.serialize_feature(testing::MaybePoint(None), NoProps {});
    assert!(matches!(result, Err(geoserde_fgb::ser::Error::Geozero(_))));
}
