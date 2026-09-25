FlatGeobuf 対応を `fgb/`（クレート名 `geoserde-fgb`）として別クレートに分けました。`make` はビルド・テスト・ドキュメント生成まで通っていて、テストは分割前と同じ 74 件が実行されています。コミットはまだしていません。

**構成**
```
Cargo.toml     … geoserde（ルートのパッケージ）+ [workspace]
src/           … de, ser
tests/         … ser_error.rs
fgb/
  Cargo.toml   … geoserde-fgb 0.1.0-beta.1
  src/         … lib.rs(旧 mod.rs), de.rs, ser.rs, error.rs
  tests/       … fgb_ の接頭辞を外したファイル名 + testing/
```

- **ルートの [Cargo.toml](Cargo.toml)**
  - `fgb` feature、`flatgeobuf` 依存、使われなくなった dev-dependencies を削除しました。
  - `default-members = [".", "fgb"]` にしたので、何も付けずに `cargo test` を実行しても両方のクレートが対象になります。
  - edition、repository、license は `[workspace.package]` で共通にしました。
- **[fgb/Cargo.toml](fgb/Cargo.toml)**：geoserde への依存は `default-features = false` にし、`geo` feature（既定で有効）から本体の `geoserde/geo` を有効にします。`flatgeobuf` の re-export は残しています。
- **Makefile**：`cargo` の各コマンドに `--workspace` を付けました。
- **ドキュメント**
  - [src/lib.rs](src/lib.rs)：`fgb` モジュールと feature の記述を消し、geoserde-fgb への案内を入れました。
  - [CHANGELOG.md](CHANGELOG.md)：`geoserde-fgb` に移った経緯を書き足し、コード例を `geoserde_fgb::` に書き換えました。
- `cargo package --list` で確認したところ、`geoserde` 本体のパッケージには `fgb/` のファイルは含まれていません。

**分けたことで必要になった変更**
- **`#[non_exhaustive]` を付けた型の扱い**：`TableError` と `FieldValue` が外部クレートの型になったので、`geoserde-fgb` の中の `match` がコンパイルエラーになりました。次のように直しています。
  - [src/ser/prop/table.rs](src/ser/prop/table.rs)：本体側に `TableError::into_sink(self) -> Result<E, TableError<Infallible>>` を追加し、fgb の `From` 変換はこのメソッドを使うようにしました。本体に variant が増えても fgb 側は変えずに済みます。
  - [fgb/src/ser.rs](fgb/src/ser.rs)：`to_column_value` がエラーを返せるようにし、[fgb/src/error.rs](fgb/src/error.rs) に `Error::UnsupportedFieldValue(FieldValue<'static>)` を追加しました。本体に FlatGeobuf で表せない値が追加されたときは、panic せずにこのエラーを返します。
- **テストが実行されていなかった問題**：移したテスト5ファイルの先頭に `#![cfg(feature = "fgb")]` が残っていて、存在しない feature を指すためテストがコンパイル対象から外れていました。これを削除しています。

**確認してほしい点・未対応**
- **バージョン番号**：`geoserde-fgb` は仮に `0.1.0-beta.1` にしました。本体は `0.5.3-beta.1` のままです。`geoserde-fgb` が本体の新しいAPIに依存しているので、本体の番号を上げる件（A-1）とあわせて決めてください。
- **リリースワークフロー**：`.github/workflows/release.yml` は変えていません。タグ名を `make version`（本体のバージョン）から作っているので、`geoserde-fgb` を単独でリリースするにはタグの付け方から見直す必要があります。
- **LICENSE ファイル**：`geoserde-fgb` のパッケージには LICENSE.txt が入りません。MIT ではライセンス文も一緒に配布するのが通例なので、`fgb/` にもコピーしておくのがよいと思います。
