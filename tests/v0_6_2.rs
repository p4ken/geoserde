#[test]
fn ser_test() -> anyhow::Result<()> {
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    let fgb_ser = geoserde::v0_6_2::fgb::ser::FeatureSerializer::new(fgb_writer);

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;
    Ok(())
}
