// 要件整理

// ソートしたいのは任意の HashMap. properties 全体ではない。
// ソートは serialze_feature の外で実施。

// ソートするには HashMap を key-value pair にする必要がある。
// key-value pair も serialize_feature の外で実施できる。
// とはいえ LayerSerializer は任意の構造を受け入れたい気持ちもある
// LayerSerializer の中でも key-value pair が作られる。
