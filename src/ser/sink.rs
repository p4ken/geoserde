pub trait GeometrySink {
    type Err: std::error::Error;
    fn coord(&mut self, index: usize, x: f64, y: f64) -> Result<(), Self::Err>;
    fn point_start(&mut self, index: usize) -> Result<(), Self::Err>;
    fn point_end(&mut self, index: usize) -> Result<(), Self::Err>;
    fn linestring_start(
        &mut self,
        is_child: bool,
        index: usize,
        coord_len: usize,
    ) -> Result<(), Self::Err>;
    fn linestring_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Err>;
    fn polygon_start(&mut self, is_child: bool, index: usize) -> Result<(), Self::Err>;
    fn polygon_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Err>;
    fn geometry_start(&mut self) -> Result<(), Self::Err>;
    fn geometry_end(&mut self) -> Result<(), Self::Err>;
}

#[cfg(feature = "geozero")]
impl<Z: geozero::FeatureProcessor> GeometrySink for Z {
    type Err = geozero::error::GeozeroError;
    fn coord(&mut self, index: usize, x: f64, y: f64) -> Result<(), Self::Err> {
        self.xy(x, y, index)
    }
    fn point_start(&mut self, index: usize) -> Result<(), Self::Err> {
        self.point_begin(index)
    }
    fn point_end(&mut self, index: usize) -> Result<(), Self::Err> {
        self.point_end(index)
    }
    fn linestring_start(
        &mut self,
        is_child: bool,
        index: usize,
        coord_len: usize,
    ) -> Result<(), Self::Err> {
        self.linestring_begin(!is_child, coord_len, index)
    }
    fn linestring_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Err> {
        self.linestring_end(!is_child, index)
    }
    fn polygon_start(&mut self, is_child: bool, index: usize) -> Result<(), Self::Err> {
        self.polygon_begin(!is_child, 1, index)
    }
    fn polygon_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Err> {
        self.polygon_end(!is_child, index)
    }
    fn geometry_start(&mut self) -> Result<(), Self::Err> {
        self.geometry_begin()
    }
    fn geometry_end(&mut self) -> Result<(), Self::Err> {
        self.geometry_end()
    }
}

pub trait PropertySink {
    type Err: std::error::Error;
    fn bool(&mut self, index: usize, key: &str, value: bool) -> Result<(), Self::Err>;
    fn i8(&mut self, index: usize, key: &str, value: i8) -> Result<(), Self::Err>;
    fn i16(&mut self, index: usize, key: &str, value: i16) -> Result<(), Self::Err>;
    fn i32(&mut self, index: usize, key: &str, value: i32) -> Result<(), Self::Err>;
    fn i64(&mut self, index: usize, key: &str, value: i64) -> Result<(), Self::Err>;
    fn u8(&mut self, index: usize, key: &str, value: u8) -> Result<(), Self::Err>;
    fn u16(&mut self, index: usize, key: &str, value: u16) -> Result<(), Self::Err>;
    fn u32(&mut self, index: usize, key: &str, value: u32) -> Result<(), Self::Err>;
    fn u64(&mut self, index: usize, key: &str, value: u64) -> Result<(), Self::Err>;
    fn f32(&mut self, index: usize, key: &str, value: f32) -> Result<(), Self::Err>;
    fn f64(&mut self, index: usize, key: &str, value: f64) -> Result<(), Self::Err>;
    fn bytes(&mut self, index: usize, key: &str, value: &[u8]) -> Result<(), Self::Err>;
    fn str(&mut self, index: usize, key: &str, value: &str) -> Result<(), Self::Err>;
}

#[cfg(feature = "geozero")]
impl<Z: geozero::PropertyProcessor> PropertySink for Z {
    type Err = geozero::error::GeozeroError;
    fn bool(&mut self, index: usize, key: &str, value: bool) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Bool(value))?;
        Ok(())
    }
    fn i8(&mut self, index: usize, key: &str, value: i8) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Byte(value))?;
        Ok(())
    }
    fn i16(&mut self, index: usize, key: &str, value: i16) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Short(value))?;
        Ok(())
    }
    fn i32(&mut self, index: usize, key: &str, value: i32) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Int(value))?;
        Ok(())
    }
    fn i64(&mut self, index: usize, key: &str, value: i64) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Long(value))?;
        Ok(())
    }
    fn u8(&mut self, index: usize, key: &str, value: u8) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::UByte(value))?;
        Ok(())
    }
    fn u16(&mut self, index: usize, key: &str, value: u16) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::UShort(value))?;
        Ok(())
    }
    fn u32(&mut self, index: usize, key: &str, value: u32) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::UInt(value))?;
        Ok(())
    }
    fn u64(&mut self, index: usize, key: &str, value: u64) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::ULong(value))?;
        Ok(())
    }
    fn f32(&mut self, index: usize, key: &str, value: f32) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Float(value))?;
        Ok(())
    }
    fn f64(&mut self, index: usize, key: &str, value: f64) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Double(value))?;
        Ok(())
    }
    fn bytes(&mut self, index: usize, key: &str, value: &[u8]) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::Binary(value))?;
        Ok(())
    }
    fn str(&mut self, index: usize, key: &str, value: &str) -> Result<(), Self::Err> {
        let _ = self.property(index, key, &geozero::ColumnValue::String(value))?;
        Ok(())
    }
}

pub trait FeatureSink:
    GeometrySink<Err = Self::FeatErr> + PropertySink<Err = Self::FeatErr>
{
    type FeatErr: std::error::Error;
    fn properties_start(&mut self) -> Result<(), Self::FeatErr>;
    fn properties_end(&mut self) -> Result<(), Self::FeatErr>;
    fn feature_start(&mut self, index: usize) -> Result<(), Self::FeatErr>;
    fn feature_end(&mut self, index: usize) -> Result<(), Self::FeatErr>;
}

#[cfg(feature = "geozero")]
impl<Z: geozero::FeatureProcessor> FeatureSink for Z {
    type FeatErr = geozero::error::GeozeroError;
    fn properties_start(&mut self) -> Result<(), Self::FeatErr> {
        self.properties_begin()
    }
    fn properties_end(&mut self) -> Result<(), Self::FeatErr> {
        self.properties_end()
    }
    fn feature_start(&mut self, index: usize) -> Result<(), Self::FeatErr> {
        self.feature_begin(index.try_into().unwrap())
    }
    fn feature_end(&mut self, index: usize) -> Result<(), Self::FeatErr> {
        self.feature_end(index.try_into().unwrap())
    }
}
