use crate::de::GeometryTypeMismatch;
use crate::fgb::de::FeatureError;

#[derive(Debug)]
pub enum Error {
    /// FlatGeobuf 形式エラー（破損ファイル、ヘッダ不正、index 不在など）。
    Fgb(flatgeobuf::Error),
    /// `geozero::GeomProcessor` 駆動中のエラー（書き込み時）。
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    /// ユーザー型の `DeserializeGeometry` がジオメトリ種別の不一致を返した。
    GeometryType(GeometryTypeMismatch),
    /// feature にジオメトリが含まれていなかった。
    MissingGeometry,
    /// FlatGeobuf の properties に未対応の `ColumnType` が現れた。
    UnsupportedColumnType(u8),
    /// properties の serde 経由エラー。
    Feature(FeatureError),
    /// ユーザー側 `Serialize` / `Deserialize` 由来の汎用エラー
    /// （`serde::ser::Error::custom` / `TableError::Root` / `TableError::Key` 等）。
    Source(String),
}

impl From<flatgeobuf::Error> for Error {
    fn from(e: flatgeobuf::Error) -> Self {
        Self::Fgb(e)
    }
}

impl From<flatgeobuf::geozero::error::GeozeroError> for Error {
    fn from(e: flatgeobuf::geozero::error::GeozeroError) -> Self {
        Self::Geozero(e)
    }
}

impl From<GeometryTypeMismatch> for Error {
    fn from(e: GeometryTypeMismatch) -> Self {
        Self::GeometryType(e)
    }
}

impl From<FeatureError> for Error {
    fn from(e: FeatureError) -> Self {
        Self::Feature(e)
    }
}

impl From<crate::ser::TableError<Error>> for Error {
    fn from(e: crate::ser::TableError<Error>) -> Self {
        match e {
            crate::ser::TableError::Sink(inner) => inner,
            other => Self::Source(other.to_string()),
        }
    }
}

impl From<crate::ser::TableError<crate::ser::SourceError>> for Error {
    fn from(e: crate::ser::TableError<crate::ser::SourceError>) -> Self {
        Self::Source(e.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fgb(_) => f.write_str("flatgeobuf format error"),
            Self::Geozero(_) => f.write_str("geozero processing failed"),
            Self::GeometryType(_) => f.write_str("geometry type mismatch"),
            Self::MissingGeometry => f.write_str("feature has no geometry"),
            Self::UnsupportedColumnType(c) => write!(f, "unsupported column type: {c}"),
            Self::Feature(_) => f.write_str("feature properties deserialization failed"),
            Self::Source(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fgb(e) => Some(e),
            Self::Geozero(e) => Some(e),
            Self::GeometryType(e) => Some(e),
            Self::Feature(e) => Some(e),
            Self::MissingGeometry | Self::UnsupportedColumnType(_) | Self::Source(_) => None,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Source(msg.to_string())
    }
}
