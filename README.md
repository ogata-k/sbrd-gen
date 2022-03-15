# About
[日本語](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/README-ja.md)

This is a crate (Schema-Based Random Data GENerator, i.e. SBRD GEN) that can generate random dummy data based on a schema. It is available both as a library and as a CLI tool.

スキーマとスキーマのジェネレーターは[About Schema](#About Schema)、ジェネレーターとそのビルダーは[List of generators that can be specified](#List of generators that can be specified)を参照してください。

なお、このプログラムは[serde](https://serde.rs/ )を利用して、スキーマのパースと生成結果のフォーマット行っています。


## When used as a library
ライブラリとして使用する場合、[How to generate with a single generator](#How to generate with a single generator)と[How to combine multiple generators with a schema](#How to combine multiple generators with a schema)があります。

### How to generate with a single generator
生成結果を組み合わせるほどでもないときに単一のジェネレータで生成する方法を利用することができます。
もちろん[How to combine multiple generators with a schema](#How to combine multiple generators with a schema)でも生成可能です。

利用方法は、次の通りです。
1. ```GeneratorBuilder```の```new_xx```（xxは可変）でビルダーを用意します。nullを生成できるようにしたい場合は、```nullable```の指定を追加します。
2. ビルダーを```build```してジェネレータに変換する
3. ジェネレータにシード種とコンテキストを渡してダミーデータを生成する

以下に、実際の記述例を記します。

```rust
use rand::thread_rng;
use sbrd_gen::builder::GeneratorBuilder;
use sbrd_gen::value::DataValueMap;

fn main() {
    let builder = GeneratorBuilder::new_int(Some((0..=100).into())).nullable();
    let generator = builder.build().unwrap();
    let generated_value = generator.generate(&mut thread_rng(), &DataValueMap::new()).unwrap();
    
    println!("generated: {}", generated_value);
}
```

### How to combine multiple generators with a schema
複数のジェネレータを利用したい場合は、こちらの方法を利用することができます。

利用方法は、次の通りです。
1. 利用したいジェネレータのリストとして```ParentGeneratorBuilder```のリストを用意する。
   このリストは上から順に生成に利用されるため、宣言する順番を間違えると生成した値を使ってキーを置き換えることのできる[Script](#Script)や[Format](#Format)が正しく機能しないことになるので注意してください。
2. 利用したいジェネレータの内、出力したいキーの一覧を用意する。
3. 出力したいキーの一覧と利用したいジェネレータの一覧を引数に指定して、```SchemaBuilder```を構築する。
4. 構築した```SchemaBuilder```を```build```して、```Schema```に変換する。
5. 変換した```Schema```で、```generate```してダミーデータを生成、または```GeneratedValueWriter```トレイトの```write_xx```（xxは可変）でWriterに書き込む。

実際の記述例は、[all_builder.rs](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/examples/schema/all_builder.rs )をご覧ください。


## When used as a CLI tool
CLIツールとして使用する場合、スキーマファイルのファイルパスを指定してダミーデータを生成することができます。
CLIではスキーマファイルのファイル形式や出力する個数、出力するフォーマットなどを指定することができます。詳しくはCLIのヘルプをご覧ください。

### How to install
インストールする方法はいくつかありますが、代表なのは[Install using Cargo](#Install using Cargo)と[Install from GitHub release page](#Install from GitHub release page)です。

#### Install using Cargo
```cargo```コマンドが利用できる状態で以下のコマンドを叩いてください。
```sbrd-gen --help```でヘルプメッセージが出力されればインストールは成功しています。

``` bash
cargo install sbrd-gen
sbrd-gen --help
```
#### Install from GitHub release page
GitHubのリリースページからインストールする場合は、[From here](https://github.com/ogata-k/sbrd-gen/releases )から希望のバージョンをダウンロードします。
ダウンロードしたフォルダを展開後、バイナリファイルのパスを通して利用できるようにしてください。
```sbrd-gen --help```でヘルプメッセージが出力されればインストールは成功しています。

### How to use CLI tool
```sbrd-gen.exe```にパスを通したうえで```sbrd-gen [OPTIONS] <SCHEMA_FILE_PATH>```という文法でコマンドを実行します。
以下で、指定可能な引数やオプションを説明しますが、```sbrd-gen --help```で表示されるヘルプメッセージでも閲覧可能です。

#### Arguments
* `<SCHEMA_FILE_PATH>`：生成に用いるスキーマを記したファイルのファイルパス。

#### Options
* パーサー
    * 指定１：`--parser <PARSER_TYPE>`
    * 指定２：`-p <PARSER_TYPE>`
    * 説明：利用したいパーサーの種類を指定するオプションです。`<PARSER_TYPE>`に利用したいパーサーの種類を指定します。
    * 利用可能オプション：yaml, json
    * デフォルト：yaml
* 出力タイプ
    * 指定１：`--type <OUTPUT_TYPE>`
    * 指定２：`-t <OUTPUT_TYPE>`
    * 説明：出力したいフォーマットを指定するオプションです。`<OUTPUT_TYPE>`に利用したいフォーマッターを指定します。
    * 利用可能オプション：yaml, json, csv, tsv
    * デフォルト：json
* 出力数
    * 指定１：`--num <COUNT>`
    * 指定２：`-n <COUNT>`
    * 説明：スキーマの`keys`で指定したダミーデータのセットの個数を指定するオプションです。`<COUNT>`に個数を指定します。
    * デフォルト：10
* キーヘッダー出力させないことを表すフラグ
    * 指定：`--no-header`
    * 説明：出力結果にキーを含めたくない場合に指定するオプションです。
* スキーマのパースのみの実行
    * 指定：`--dry-run`
    * 説明：ダミーデータの出力をせずにスキーマのパースだけを行って終了することを指定するオプションです。
* ヘルプ
    * 指定１：`--help`
    * 指定２：`-h`
    * 説明：ヘルプを確認したいときに指定するオプションです。
* バージョン
    * 指定１：`--version`
    * 指定２：`-V`
    * 説明：バージョンを確認したいときに指定するオプションです。

## About Schema
スキーマは、`keys`をキーとする出力したい[Key](#Key)のシークエンスと、`generators`をキーとする[Generator Builders](#List of options for parent generator)のシークエンスからなるMap(KVS)で指定します。
フォーマットは、YamlとJsonをサポートしています。

記述例については、[all.yaml](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/examples/schema/all.yaml )や[all.json](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/examples/schema/all.json )をご覧ください。

### Value Context
スキーマからダミーデータを生成するときは、スキーマに指定されたジェネレーターを上から順に実行します。
このとき生成された値はValue Contextと呼ばれるMap(KVS)のデータ構造に保存されます。
つまり、Value Contextで参照可能なペアは、参照時点で生成に成功したジェネレーターのキーと値のペアです。
このValue Contextは、出力したいキーからキーに紐づく値を取得するのに用いられたり、[Script](#Script)や[Format](#Format)として指定された"{key}"（括弧とキーの間にはスペース無し）という表記をその時点のコンテキストにあるkeyに紐づく値で置き換えてから評価したり、といった形などで利用されます。

### List of options for parent generator
親ジェネレーターはキーとビルダーのオプションからなるMap(KVS)で指定します。
構造体としては```ParentGeneratorBuilder```となります。
#### Key
ジェネレーターを特定するためのキーで、`key`をキーとした文字列として指定します。

[Script](#Script)や[Format](#Format)を評価する際の置換キーとしても利用されます。

#### Builder
[List of generators that can be specified](#List of generators that can be specified)で列挙されているジェネレーターのオプションを指定することができます。
生成されるジェネレータは[Type](#Type)によって決まり、ほかのオプションも同様に解釈されます。

### List of generators that can be specified
スキーマや単一のジェネレーターとして指定可能なジェネレーターは以下の通りです。
#### String constructor (build_string module)
他のジェネレーターの生成結果をもとに文字列を組み立てるジェネレーターの集まりからなるモジュールです。
* duplicate permutation generator
    * 説明：生成結果を組み合わせて文字列にするジェネレーターです。範囲指定で指定された回数だけ値を生成して[Separator](#Separator)で貼り付けて文字列を作成します。[Separator](#Separator)のデフォルトは空文字（""）です。
    * 備考：なし
    * 構造体：```DuplicatePermutationGenerator```
    * タイプ：duplicate-permutation
    * 必須オプション：[Type](#Type)、[Separator](#Separator)、括弧内一つ以上（[List of child generators](#List of child generators)、[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)）
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Range (Integer)](#Range)、[Separator](#Separator)、[List of child generators](#List of child generators)、[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)
    * 生成型：String
* format generator
    * 説明：指定されたフォーマットにコンテキストを適応して文字列を構築するジェネレーターです。
    * 備考：なし
    * 構造体：```FormatGenerator```
    * タイプ：format
    * 必須オプション：[Type](#Type)、[Format](#Format)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Format](#Format)
    * 生成型：String
#### Distribution system (distribution module)
分布関数をもとに乱数を生成するジェネレーターの集まりからなるモジュールです。
* normal generator
    * 説明：正規分布に従って乱数を生成するジェネレーターです。
    * 備考：[Parameters](#Parameters)で指定できるのは、Real-numberの平均（`mean`）とReal-numberの標準偏差（`std_dev`）です。デフォルトは、それぞれ0.0、1.0です。
    * 構造体：```NormalGenerator```
    * タイプ：dist-normal
    * 必須オプション：[Type](#Type)、[Parameters](#Parameters)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Parameters](#Parameters)
    * 生成型：Real-number
#### Evaluation system (eval module)
指定した式を評価して値を出力するジェネレーターの集まりからなるモジュールです。
* eval generator
    * 説明：指定した[Script](#Script)を評価した結果を出力するジェネレーターです。
    * 備考：なし
    * 構造体：```EvalGenerator```
    * タイプ：eval-int（Integer）、eval-real（Real-number）、eval-bool（Boolean）、eval-string（String）
    * 必須オプション：[Type](#Type)、[Script](#Script)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Script](#Script)
    * 生成型：Integer（eval-int）、Real-number（eval-real）、Boolean（eval-bool）、String（eval-string）
#### Sequential change system (incremental module)
実行するたびに一定量増加するといったように逐次的に変化するジェネレーターの集まりからなるモジュールです。
* increment id generator
    * 説明：生成するたびに指定された[Increment](#Increment)のステップ数を加算してから生成するジェネレーターです。初期値は指定された[Increment](#Increment)の初期値です。
    * 備考：[Increment](#Increment)のデフォルトは、1始まりの1増加となっています。
    * 構造体：```IncrementIdGenerator```
    * タイプ：increment-id
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Increment (Integer)](#Increment)
    * 生成型：Integer
#### Primitive (primitive Module)
基本的な値を生成するジェネレーターの集まりからなるモジュールです。
* int generator
    * 説明：指定された[Range](#Range)でIntegerを生成するジェネレーターです。デフォルトの範囲はi16の最小値（-32768）以上i16の最大値（32767）以下です。
    * 備考：なし
    * 構造体：```IntGenerator```
    * タイプ：int
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Range (Integer)](#Range)
    * 生成型：Integer
* real generator
    * 説明：指定された[Range](#Range)でReal-numberを生成するジェネレーターです。デフォルトの範囲はi16の最小値（-32768）以上i16の最大値（32767）以下です。
    * 備考：生成された値の絶対値が大きいほど小数点以下の字数が減り、絶対値が小さいほど小数点以下の字数が増えます。
    * 構造体：```RealGenerator```
    * タイプ：real
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Range (Real-number)](#Range)
    * 生成型：Real-number
* bool generator
    * 説明：50%の確率でtrueかfalseを生成するジェネレーターです。
    * 備考：なし
    * 構造体：```BoolGenerator```
    * タイプ：bool
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)
    * 生成型：Boolean
* date time generator
    * 説明：[Format](#Format)で指定したフォーマットで日時を生成するジェネレーターです。
    * 備考：[Range](#Range)で指定する日時のフォーマットは"%Y-%m-%d %H:%M:%S"です。[Format](#Format)のデフォルト値も同じフォーマットです。フォーマットについては[こちら](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers )をご覧ください。デフォルトの[Range](#Range)は1900-01-01 00:00:00以上2151-01-01 00:00:00未満で、未指定の境界はデフォルト値が指定されたものとします。
    * 構造体：```DateTimeGenerator```
    * タイプ：date-time
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Range (DateTime-String)](#Range)、[Format](#Format)
    * 生成型：String
* date generator
    * 説明：[Format](#Format)で指定したフォーマットで日付を生成するジェネレーターです。
    * 備考：[Range](#Range)で指定する日付のフォーマットは"%Y-%m-%d"です。[Format](#Format)のデフォルト値も同じフォーマットです。フォーマットについては[こちら](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers )をご覧ください。デフォルトの[Range](#Range)は1900-01-01以上2151-01-01未満です、未指定の境界はデフォルト値が指定されたものとします。
    * 構造体：```DateGenerator```
    * タイプ：date
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Range (Date-String)](#Range)、[Format](#Format)
    * 生成型：String
* time generator
    * 説明：[Format](#Format)で指定したフォーマットで時刻を生成するジェネレーターです。
    * 備考：[Range](#Range)で指定する日時のフォーマットは"%H:%M:%S"です。[Format](#Format)のデフォルト値も同じフォーマットです。フォーマットについては[Here](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers )をご覧ください。デフォルトの[Range](#Range)は00:00:00以上23:59:59以下です、未指定の境界はデフォルト値が指定されたものとします。
    * 構造体：```TimeGenerator```
    * タイプ：time
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Range (Time-String)](#Range)、[Format](#Format)
    * 生成型：String
* always null generator
    * 説明：常にnullを生成するジェネレーターです。
    * 備考：なし
    * 構造体：```AlwaysNullGenerator```
    * タイプ：always-null
    * 必須オプション：[Type](#Type)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)
    * 生成型：Null
#### Child Generator Random Selection System (random_children module)
[List of child generators](#List of child generators)を利用して値を生成するジェネレーターです。
* case when generator
    * 説明：宣言順に[Condition](#Condition)を評価してtrueになった子ジェネレーターを利用して生成するジェネレーターです。
    * 備考：[Condition](#Condition)に引っかからなかった場合のためにデフォルト条件（つまり、[Condition](#Condition)が未指定）の子ジェネレーターが必要です。
    * 構造体：```CaseWhenGenerator```
    * タイプ：case-when
    * 必須オプション：[Type](#Type)、[Condition](#Condition)の指定がある[List of child generators](#List of child generators)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Condition](#Condition)の指定がある[List of child generators](#List of child generators)
    * 生成型：生成に利用した子ジェネレーターの生成型
* random child generator
    * 説明：[Weight](#Weight)を考慮して乱択したジェネレーターを利用して生成するジェネレーターです。
    * 備考：なし
    * 構造体：```RandomChildGnerator```
    * タイプ：random-child
    * 必須オプション：[Type](#Type)、[Weight](#Weight)の指定がある[List of child generators](#List of child generators)
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Weight](#Weight)の指定がある[List of child generators](#List of child generators)
    * 生成型：生成に利用した子ジェネレーターの生成型
#### Value alternative system (random_values module)
[Character list](#Character list)や[List of Values](#List of Values)、[External file path](#External file path)を利用して値を生成するジェネレーターの集まりからなるモジュールです。
* select generator
    * 説明：[Character list](#Character list)や[List of Values](#List of Values)、[External file path](#External file path)で指定された値を乱択するジェネレーターです。
    * 備考：なし
    * 構造体：```SelectGenerator```
    * タイプ：select-int（Integer）、select-real（Real-number）、select-string（String）
    * 必須オプション：[Type](#Type)、括弧内一つ以上（[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)）
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)
    * 生成型：Integer（select-int）、Real-number（select-real）、String（select-string）
* get value at generator
    * 説明：[Script](#Script)を評価して取得したインデックスにある値を、入力された値の一覧から取得するジェネレーターです。
    * 備考：なし
    * 構造体：```GetValueAtGenerator```
    * タイプ：get-int-value-at（Integer）、get-real-value-at（Real-number）、get-string-value（String）
    * 必須オプション：[Type](#Type)、[Script](#Script)、括弧内一つ以上（[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)）
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Script](#Script)、[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)
    * 生成型：Integer（get-int-value-at）、Real-number（get-real-value-at）、String（get-string-value-at）
* get value index generator
    * 説明：入力された値の一覧から乱択可能な範囲のインデックスを取得するジェネレーターです。
    * 備考：なし
    * 構造体：```GetValueIndexGenerator```
    * タイプ：get-value-index
    * 必須オプション：[Type](#Type)、括弧内一つ以上（[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)）
    * 指定可能オプション：[Type](#Type)、[Nullable](#Nullable)、[Character list](#Character list)、[List of Values](#List of Values)、[External file path](#External file path)
    * 生成型：Integer(Not negative)

### List of generator options
ジェネレーターを構築するのに指定できるオプションは次の通りです。
指定可能なオプションはジェネレーターによって違いますが、指定可能なオプション以外は無視されます。
#### Type
* 説明：[List of generators that can be specified](#List of generators that can be specified)で列挙されているジェネレーターのタイプ。ジェネレーターの種類を特定するために利用される。
* 備考：なし
* 構造体：```GeneratorType```
* キー名：`type`
* 値型：String
#### Nullable
* 説明：ジェネレーターが生成する値に加えてnullを生成することができるかのフラグ。trueならnullを生成することができる。デフォルトはfalse。
* 備考：なし
* 構造体：```bool```
* キー名：`nulable`
* 値型：Boolean
#### Format
* 説明：このフォーマットは、[Value Context](#Value Context)内のキーと値のペア（仮にそのペアを(key, value)とする。）を順番にフォーマット内の{key}（括弧とキーの間にはスペース無し）という文字列をvalueで置き換えてから文字列として評価されます。
* 備考：なし
* 構造体：```String```
* キー名：`format`
* 値型：String
#### Script
* 説明：このスクリプトは、[Value Context](#Value Context)内のキーと値のペア（仮にそのペアを(key, value)とする。）を順番にスクリプト内の{key}（括弧とキーの間にはスペース無し）という文字列をvalueで置き換えてから式として評価されます。文法や式などについて詳しくはEvaluatorのAPIドキュメントを参照してください。
* 備考：なし
* 構造体：```String```
* キー名：`script`
* 値型：String
#### Separator
* 説明：文字列の構築などで区切りに使う文字列です。
* 備考：なし
* 構造体：```String```
* キー名：`separator`
* 値型：String
#### Range
* 説明：繰り返し数の範囲や生成する値の範囲の指定に利用されるオプションです。
* 備考：範囲指定時に利用できる値型として利用可能なのは、Integer、Real-number、String、DateTime-String、Date-String、Time-Stringの６つです。日時関係の値の指定はそれぞれの[Primitive generators](#Primitive-(primitive Module))を参照してください。
* 構造体：```ValueBound```
* キー名：`range`
* 値型：値型の値を値に持つキー`start`とキー`end`、`end`の値を含むことを表すフラグを値に持つキー`include_end`からなるMap(KVS)であり、それぞれ任意指定です。`include_end`のデフォルト値はtrueです。
#### Increment
* 説明：ジェネレーターを実行するたびに更新される値の初期値と変化量を指定するためのオプションです。
* 備考：指定時に利用できる値型として利用可能なのは、Integer、Real-number、String、DateTime-String、Date-String、Time-Stringの６つです。値の指定は[Range](#Range)の指定と同じです。
* 構造体：```ValueStep```
* キー名：`increment`
* 値型：初期値として値型の値を値に持つキー`initial`と、変化量を表す値型の値を値に持つキー`step`からなるMap(KVS)であり、`initial`は必須、`step`は任意指定です、
#### List of child generators
* 説明：[List of generator options](#List of generator options)で指定されるジェネレーターのシークエンスを指定するオプションです。ここで指定するのは子ジェネレーターと呼ばれ、親ジェネレーターとは違い追加で[List of options for child generator](#List of options for child generator)を指定することができます。
* 備考：なし
* 構造体：```Vec<ChildGeneratorBuilder>>```
* キー名：`children`
* 値型：子ジェネレーターのシークエンス
#### Character list
* 説明：乱択の対象とする文字を列挙するオプションです。
* 備考：なし
* 構造体：```String```
* キー名：`chars`
* 値型：String
#### List of Values
* 説明：乱択の対象とする値を列挙するオプションです。
* 備考：値型として利用可能なのは、Integer、Real-number、Stringです。
* 構造体：```Vec<DataValue>```
* キー名：`values`
* 値型：Integer、Real-number、Stringのどれかの型からなるシークエンス
#### External file path
* 説明：乱択の対象とする値を一行==一つの値として列挙するファイルのファイルパスを指定するオプションです。絶対パスのほかにスキーマーファイルからの相対パスで指定することができます。
* 備考：なし
* 構造体：```PathBuf```
* キー名：`filepath`
* 値型：String
#### Parameters
* 説明：分布関数を構築する際に必要なパラメーターを指定するためのオプションです。指定するキーと値については[Distribution system](#Distribution system-(distribution module))の各ジェネレーターを参照してください。
* 備考：なし
* 構造体：```DataValueMap<String>```
* キー名：`parameters`
* 値型：Map(KVS)

### List of options for child generator
子ジェネレーターは、ジェネレーターで指定可能なオプションに加えて次に列挙するオプションも指定することができます。
#### Condition
* 説明：どの子ジェネレーターを利用するかの条件分岐の条件を指定するためのオプション。指定された場合[Script](#Script)と同様に評価してtrue/falseを判定し、未指定の場合常にtrueを返します。
* 備考：なし
* 構造体：```String```
* キー名：`condition`
* 値型：String
#### Weight
* 説明：子ジェネレーターを乱択する際の重みを指定するためのオプション。重みが大きいほどよく選択される。デフォルトの重みは1。
* 備考：なし
* 構造体：```Weight```
* キー名：`weight`
* 値型：Integer(Not negative)
