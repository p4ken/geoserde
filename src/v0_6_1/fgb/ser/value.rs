use crate::v0_6_1::ser::prop::value::FlatValue;

pub fn from_flat_value(source: FlatValue<'_>) -> flatgeobuf::geozero::ColumnValue<'_> {
    match source {
        FlatValue::Bool(v) => flatgeobuf::geozero::ColumnValue::Bool(v),
        FlatValue::I8(v) => flatgeobuf::geozero::ColumnValue::Byte(v),
        FlatValue::I16(v) => flatgeobuf::geozero::ColumnValue::Short(v),
        FlatValue::I32(v) => flatgeobuf::geozero::ColumnValue::Int(v),
        FlatValue::I64(v) => flatgeobuf::geozero::ColumnValue::Long(v),
        FlatValue::U8(v) => flatgeobuf::geozero::ColumnValue::UByte(v),
        FlatValue::U16(v) => flatgeobuf::geozero::ColumnValue::UShort(v),
        FlatValue::U32(v) => flatgeobuf::geozero::ColumnValue::UInt(v),
        FlatValue::U64(v) => flatgeobuf::geozero::ColumnValue::ULong(v),
        FlatValue::F32(v) => flatgeobuf::geozero::ColumnValue::Float(v),
        FlatValue::F64(v) => flatgeobuf::geozero::ColumnValue::Double(v),
        FlatValue::Str(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FlatValue::Bytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
    }
}
