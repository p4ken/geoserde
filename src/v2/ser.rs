pub trait FormatProperty {
    // geoserdeはRust構造体寄りだから、integerとかではなくi32
    // 同じキーで複数回呼ばれるかもしれない（入れ子で同名フィールド）
    fn format_i32(&self, key: &str, value: i32);
}
