# About
[日本語](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/README-ja.md)

This is a crate (Schema-Based Random Data GENerator, i.e. SBRD GEN) that can generate random dummy data based on a schema. It is available both as a library and as a CLI tool.

See [About Schema](#About-Schema) for schema and schema generators, and [List of generators that can be specified](#List-of-generators-that-can-be-specified) for generators and their builders.

This program uses [serde](https://serde.rs/ ) to parse the schema and format the generated results.


## When used as a library
If you want to use it as a library, there are two ways: [How to generate with a single generator](#How-to-generate-with-a-single-generator) and [How to combine multiple generators with a schema](#How-to-combine-multiple-generators-with-a-schema).

### How to generate with a single generator
A single generator method can be used when the generated results are not so good that they need to be combined.
Of course, it can also be generated by [How to combine multiple generators with a schema](#How-to-combine-multiple-generators-with-a-schema).

The usage is as follows
1. Prepare a builder with ```new_xx``` (where xx is variable) in ```GeneratorBuilder```; If you want to be able to generate nulls, add the ```nullable``` specification.
2. Convert the builder into a generator by ```build```.
3. Generate dummy data by passing the seed species and context to the generator.

The following is an example of an actual description.

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
If you want to use multiple generators, you can use this method.

The procedure is as follows
1. Prepare a list of ``ParentGeneratorBuilder`` as a list of generators you want to use. 
Note that this list is used for generation from the top to the bottom, so if you declare them in the wrong order, [Script](#Script) and [Format](#Format), which can replace keys with generated values, will not function properly.
2. Prepare a list of keys to be output out of the generators you wish to use.
3. Construct ```SchemaBuilder``` with the list of keys you want to output and the list of generators you want to use as arguments. 
4. Build the ```SchemaBuilder``` and convert it to ```Schema```.
5. Generate ```generate``` with the converted ```Schema``` to generate dummy data, or write it to the Writer with ```write_xx``` (where xx is variable) in the ```GeneratedValueWriter``` trace.

See [all_builder.rs](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/examples/schema/all_builder.rs ) for an actual writing example.


## When used as a CLI tool
When used as a CLI tool, dummy data can be generated by specifying the file path of the schema file.
The CLI allows you to specify the file format of the schema file, the number of files to output, and the format of the output.
For details, please see the CLI help.

### How to install
There are several ways to install, the most common being [Install using Cargo](#Install-using-Cargo) and [Install from GitHub release page](#Install-from-GitHub-release-page).

#### Install using Cargo
With the ```cargo``` command available, hit the following command.
If you get a help message with ```sbrd-gen --help```, the installation was successful.

``` bash
cargo install sbrd-gen
sbrd-gen --help
```
#### Install from GitHub release page
To install from the GitHub release page, download the desired version from [here](https://github.com/ogata-k/sbrd-gen/releases ).
After extracting the downloaded folder, make it available through the binary file path.
If you get a help message with ```sbrd-gen --help```, the installation was successful.

### How to use CLI tool
Run the command with the syntax ```sbrd-gen [OPTIONS] <SCHEMA_FILE_PATH>``` after passing ```sbrd-gen.exe```.
The following describes the arguments and options that can be specified, but can also be viewed in the help message displayed by ```sbrd-gen --help```.

#### Arguments
* `<SCHEMA_FILE_PATH>` : File path of the file containing the schema to be used for generation.

#### Options
* Parser
    * Specific1 : `--parser <PARSER_TYPE>`
    * Specific2 : `-p <PARSER_TYPE>`
    * Description : This option specifies the type of parser to be used. Specify the type of parser you want to use for `<PARSER_TYPE>`.
    * Available options : yaml, json
    * Default : yaml
* Output type
    * Specific1 : `--type <OUTPUT_TYPE>`
    * Specific2 : `-t <OUTPUT_TYPE>`
    * Description : Option to specify the format you want to output. Specify the formatter you want to use for `<OUTPUT_TYPE>`.
    * Available options : yaml, json, csv, tsv
    * Default : json
* Number of outputs
    * Specific1 : `--num <COUNT>`
    * Specific2 : `-n <COUNT>`
    * Description : Option to specify the number of dummy data sets specified by `keys` in the schema. Specify the number in `<COUNT>`.
    * Default : 10
* Flag indicating that the key header should not be output.
    * Specific : `--no-header`
    * Description : Option to specify if you do not want to include the key in the output result.
* Execute schema parsing only
    * Specific : `--dry-run`
    * Description : Option to specify that only schema parsing is performed without outputting dummy data and then exit.
* Help
    * Specific1 : `--help`
    * Specific2 : `-h`
    * Description : Option to specify when you want to check help.
* Version
    * Specific1 : `--version`
    * Specific2 : `-V`
    * Description : Option to specify when you want to check the version.

## About Schema
The schema is specified by a Map(KVS) consisting of a sequence of [Key](#Key) to be output with `keys` as key and a sequence of [Generator Builders](#List-of-options-for-parent-generator) with `generators` as key.
The formats supported are Yaml and Json.

For example descriptions, see [all.yaml](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/examples/schema/all.yaml ) and [all.json](https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/examples/schema/all.json ).

### Value Context
When generating dummy data from the schema, the generators specified in the schema are executed from the top.
The generated values are stored in a Map (KVS) data structure called a Value Context.
In other words, the pairs that can be referenced in the Value Context are the key/value pairs of the generators that were successfully generated at the time of reference.
This Value Context can be used to retrieve the value associated with a key from the key to be output, or to convert the notation "{key}" (no space between brackets and key) specified as [Script](#Script) or [Format](#Format) to the current The value of the key in the context is replaced by the value associated with the key in the context, and then evaluated, etc.

### List of options for parent generator
The parent generator is specified by a Map(KVS) consisting of keys and builder options.
The structure is ```ParentGeneratorBuilder```.
#### Key
A key to identify the generator, specified as a string with `key` as the key.

It is also used as a substitution key when evaluating [Script](#Script) or [Format](#Format).

#### Builder
You can specify the generator options listed in [List of generators that can be specified](#List-of-generators-that-can-be-specified).
The generator to be generated is determined by [Type](#Type), and other options are interpreted in the same way.

### List of generators that can be specified
Generators that can be specified as a schema or a single generator are as follows.
#### String constructor (build_string module)
This module consists of a collection of generators that assemble strings based on the results generated by other generators.
* duplicate permutation generator
    * Description : Generator that combines generated results into a string. Generate values as many times as specified in the range and paste them with [Separator](#Separator) to create a string. Default for [Separator](#Separator) is an empty string ("").
    * Remarks : None
    * Struct : ```DuplicatePermutationGenerator```
    * Type : duplicate-permutation
    * Required options : [Type](#Type), [Separator](#Separator), One or more in parentheses([List of child generators](#List-of-child-generators), [Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path))
    * Available options : [Type](#Type), [Nullable](#Nullable), [Range (Integer)](#Range), [Separator](#Separator), [List of child generators](#List-of-child-generators), [Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path)
    * Generate value type : String
* format generator
    * Description : Generator that constructs a string by adapting the context to the specified format.
    * Remarks : None
    * Struct : ```FormatGenerator```
    * Type : format
    * Required options : [Type](#Type), [Format](#Format)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Format](#Format)
    * Generate value type : String
#### Distribution system (distribution module)
This module consists of a collection of generators that generate random numbers based on a distribution function.
* normal generator
    * Description : Generator that generates random numbers according to a normal distribution.
    * Remarks : [Parameters](#Parameters) can be the mean of Real-number (`mean`) and the standard deviation of Real-number (`std_dev`). Default is 0.0 and 1.0, respectively.
    * Struct : ```NormalGenerator```
    * Type : dist-normal
    * Required options : [Type](#Type), [Parameters](#Parameters)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Parameters](#Parameters)
    * Generate value type : Real-number
#### Evaluation system (eval module)
This module consists of a collection of generators that evaluate a given expression and output a value.
* eval generator
    * Description : Generator that outputs the result of evaluating the specified [Script](#Script).
    * Remarks : None
    * Struct : ```EvalGenerator```
    * Type : eval-int(Integer), eval-real(Real-number), eval-bool(Boolean), eval-string(String)
    * Required options : [Type](#Type), [Script](#Script)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Script](#Script)
    * Generate value type : Integer(eval-int), Real-number(eval-real), Boolean(eval-bool), String(eval-string)
#### Sequential change system (incremental module)
This module consists of a collection of generators that change sequentially, such as increasing by a certain amount each time they are executed.
* increment id generator
    * Description : Generator that adds the number of steps of the specified [Increment](#Increment) before each generation. The initial value is the initial value of the specified [Increment](#Increment).
    * Remarks : Default for [Increment](#Increment) is 1 increase beginning 1.
    * Struct : ```IncrementIdGenerator```
    * Type : increment-id
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Increment (Integer)](#Increment)
    * Generate value type : Integer
#### Primitive (primitive Module)
This module consists of a collection of generators that generate basic values.
* int generator
    * Description : Generator that generates Integer with the specified [Range](#Range), where the Default range is between the minimum value of i16 (-32768) and the maximum value of i16 (32767).
    * Remarks : None
    * Struct : ```IntGenerator```
    * Type : int
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Range (Integer)](#Range)
    * Generate value type : Integer
* real generator
    * Description : Generator that generates a Real-number in the specified [Range](#Range), where the Default range is between the minimum value of i16 (-32768) and the maximum value of i16 (32767).
    * Remarks : The larger the absolute value of the generated value, the fewer the number of characters after the decimal point, and the smaller the absolute value, the more the number of characters after the decimal point.
    * Struct : ```RealGenerator```
    * Type : real
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Range (Real-number)](#Range)
    * Generate value type : Real-number
* bool generator
    * Description : Generator that generates true or false with 50% probability.
    * Remarks : None
    * Struct : ```BoolGenerator```
    * Type : bool
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable)
    * Generate value type : Boolean
* date time generator
    * Description : This generator generates date and time in the format specified by [Format](#Format).
    * Remarks : The format of date and time specified by [Range](#Range) is "%Y-%m-%d %H:%M:%S". Default value format of [Format](#Format) has the same format. See [here](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers ) for the format. Default value of [Range](#Range) is from 1900-01-01 00:00:00 less than 2151-01-01 00:00:00. An unspecified boundary is assumed to have a Default value.
    * Struct : ```DateTimeGenerator```
    * Type : date-time
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Range (DateTime-String)](#Range), [Format](#Format)
    * Generate value type : String
* date generator
    * Description : This generator generates date in the format specified by [Format](#Format).
    * Remarks : The format of date specified by [Range](#Range) is "%Y-%m-%d". Default value format of [Format](#Format) has the same format. See [here](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers ) for the format. Default value of [Range](#Range) is from 1900-01-01 less than 2151-01-01. An unspecified boundary is assumed to have a Default value.
    * Struct : ```DateGenerator```
    * Type : date
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Range (Date-String)](#Range), [Format](#Format)
    * Generate value type : String
* time generator
    * Description : This generator generates time in the format specified by [Format](#Format).
    * Remarks : The format of time specified by [Range](#Range) is "%H:%M:%S". Default value format of [Format](#Format) has the same format. See [here](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers ) for the format. Default value of [Range](#Range) is from 00:00:00 less than and equal 23:59:59. An unspecified boundary is assumed to have a Default value.
    * Struct : ```TimeGenerator```
    * Type : time
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable), [Range (Time-String)](#Range), [Format](#Format)
    * Generate value type : String
* always null generator
    * Description : Generator that always generates null.
    * Remarks : None
    * Struct : ```AlwaysNullGenerator```
    * Type : always-null
    * Required options : [Type](#Type)
    * Available options : [Type](#Type), [Nullable](#Nullable)
    * Generate value type : Null
#### Child Generator Random Selection System (random_children module)
[List of child generators](#List-of-child-generators) is a generator that generates values.
* case when generator
    * Description : Generator generated by evaluating [Condition](#Condition) in the order of declaration and using child generators that are true.
    * Remarks : A child generator for the Default condition (i.e., [Condition](#Condition) is not specified) is needed in case [Condition](#Condition) is not caught.
    * Struct : ```CaseWhenGenerator```
    * Type : case-when
    * Required options : [Type](#Type), [List of child generators](#List-of-child-generators) with [Condition](#Condition) specified
    * Available options : [Type](#Type), [Nullable](#Nullable), [List of child generators](#List-of-child-generators) with [Condition](#Condition) specified
    * Generate value type : Generate value type of the child generator used for generation
* random child generator
    * Description : This generator is generated using a randomly selected generator considering [Weight](#Weight).
    * Remarks : None
    * Struct : ```RandomChildGnerator```
    * Type : random-child
    * Required options : [Type](#Type), [List of child generators](#List-of-child-generators) with [Weight](#Weight) specified
    * Available options : [Type](#Type), [Nullable](#Nullable), [List of child generators](#List-of-child-generators) with [Weight](#Weight) specified
    * Generate value type : Generate value type of the child generator used for generation
#### Value alternative system (random_values module)
This module consists of a collection of generators that generate values using [Character list](#Character-list), [List of Values](#List-of-Values), and [External file path](#External-file-path).
* select generator
    * Description : Generator to randomly select values specified by [Character list](#Character-list), [List of Values](#List-of-Values), or [External file path](#External-file-path).
    * Remarks : None
    * Struct : ```SelectGenerator```
    * Type : select-int(Integer), select-real(Real-number), select-string(String)
    * Required options : [Type](#Type), One or more in parentheses([Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path))
    * Available options : [Type](#Type), [Nullable](#Nullable), [Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path)
    * Generate value type : Integer(select-int), Real-number(select-real), String(select-string)
* get value at generator
    * Description : Generator that retrieves the value at the index obtained by evaluating [Script](#Script) from a list of input values.
    * Remarks : None
    * Struct : ```GetValueAtGenerator```
    * Type : get-int-value-at(Integer), get-real-value-at(Real-number), get-string-value(String)
    * Required options : [Type](#Type), [Script](#Script), One or more in parentheses([Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path))
    * Available options : [Type](#Type), [Nullable](#Nullable), [Script](#Script), [Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path)
    * Generate value type : Integer(get-int-value-at), Real-number(get-real-value-at), String(get-string-value-at)
* get value index generator
    * Description : Generator to obtain an index of a random selectable range from a list of input values.
    * Remarks : None
    * Struct : ```GetValueIndexGenerator```
    * Type : get-value-index
    * Required options : [Type](#Type), One or more in parentheses([Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path))
    * Available options : [Type](#Type), [Nullable](#Nullable), [Character list](#Character-list), [List of Values](#List-of-Values), [External file path](#External-file-path)
    * Generate value type : Integer(Not negative)

### List of generator options
The following options can be specified to build the generator.
The available options vary from generator to generator, but all other options are ignored.
#### Type
* Description : Type of generators listed in [List of generators that can be specified](#List-of-generators-that-can-be-specified), used to identify the type of generator.
* Remarks : None
* Struct : ```GeneratorType```
* Key name : `type`
* Value type : String
#### Nullable
* Description : A flag indicating whether null can be generated in addition to the value generated by the generator; if true, null can be generated; Default is false.
* Remarks : None
* Struct : ```bool```
* Key name : `nulable`
* Value type : Boolean
#### Format
* Description : This format is used for key/value pairs in [Value Context](#Value-Context) (let's say the pair is (key, value)). in turn, the string {key} (no space between the parentheses and the key) in the format is replaced by value before being evaluated as a String.
* Remarks : None
* Struct : ```String```
* Key name : `format`
* Value type : String
#### Script
* Description : This script is a key/value pair (let's say the pair is (key, value)) in [Value Context](#Value-Context). in turn, the string {key} (no space between the parentheses and the key) in the script is replaced by value before being evaluated as an expression. Please refer to the Evaluator API documentation for more information on syntax, expressions, etc.
* Remarks : None
* Struct : ```String```
* Key name : `script`
* Value type : String
#### Separator
* Description : A string used for delimitation in string construction, etc.
* Remarks : None
* Struct : ```String```
* Key name : `separator`
* Value type : String
#### Range
* Description : This option is used to specify the range of the number of iterations and the range of values to be generated.
* Remarks : The six available value types for ranges are Integer, Real-number, String, DateTime-String, Date-String, and Time-String. Refer to the respective [Primitive generators](#primitive-primitive-module) for specifying date/time-related values.
* Struct : ```ValueBound```
* Key name : `range`
* Value type : Map (KVS) consisting of the key `start` with the value of value type, the key `end`, and the key `include_end` with the flag indicating that the value of `end` is included, each of which is optional. The default value of `include_end` is true.
#### Increment
* Description : Option to specify the initial value and the amount of change in the value that will be updated each time the generator is run.
* Remarks : The six available value types are Integer, Real-number, String, DateTime-String, Date-String, and Time-String. Specifying a value is the same as specifying [Range](#Range).
* Struct : ```ValueStep```
* Key name : `increment`
* Value type : Map(KVS) consisting of a key `initial` with a value of value type as an initial value and a key `step` with a value of value type representing the amount of change, where `initial` is required and `step` is optional.
#### List of child generators
* Description : This option specifies the sequence of generators specified in [List of generator options](#List-of-generator-options). The generator specified here is called a child generator, and unlike the parent generator, an additional [List of options for child generator](#List-of-options-for-child-generator) can be specified.
* Remarks : None
* Struct : ```Vec<ChildGeneratorBuilder>>```
* Key name : `children`
* Value type : Sequence of child generators
#### Character list
* Description : Option to enumerate characters for random selection.
* Remarks : None
* Struct : ```String```
* Key name : `chars`
* Value type : String
#### List of Values
* Description : Option to enumerate values for random selection.
* Remarks : Available value types are Integer, Real-number, and String.
* Struct : ```Vec<DataValue>```
* Key name : `values`
* Value type : Sequence consisting of Integer, Real-number, or String type
#### External file path
* Description : This option specifies the file path of a file that enumerates the values to be selected for random selection as a single line == one value. In addition to an absolute path, it can be specified relative to the schema file.
* Remarks : None
* Struct : ```PathBuf```
* Key name : `filepath`
* Value type : String
#### Parameters
* Description : This option is used to specify the parameters needed to construct the distribution function. See each generator in [Distribution system](#distribution-system-distribution-module) for the keys and values to specify.
* Remarks : None
* Struct : ```DataValueMap<String>```
* Key name : `parameters`
* Value type : Map(KVS)

### List of options for child generator
The child generator can specify the options listed below in addition to the options that can be specified by the generator.
#### Condition
* Description : Option to specify the conditional branching condition for which child generator to use. If specified, it is evaluated like [Script](#Script) to determine true/false, always returning true if not specified.
* Remarks : None
* Struct : ```String```
* Key name : `condition`
* Value type : String
#### Weight
* Description : Option to specify the weight for random selection of child generators. The higher the weight, the more often it is selected; Default weight is 1.
* Remarks : None
* Struct : ```Weight```
* Key name : `weight`
* Value type : Integer(Not negative)
