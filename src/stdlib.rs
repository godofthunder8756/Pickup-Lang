use crate::compiler::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{Write, Read};

/// Initialize standard library modules
pub fn create_stdlib() -> HashMap<String, Value> {
    let mut stdlib = HashMap::new();

    // JSON module
    stdlib.insert("json".to_string(), create_json_module());

    // FS module
    stdlib.insert("fs".to_string(), create_fs_module());

    // String module
    stdlib.insert("string".to_string(), create_string_module());

    // Math module
    stdlib.insert("math".to_string(), create_math_module());

    // Array module
    stdlib.insert("array".to_string(), create_array_module());

    // Type module
    stdlib.insert("type".to_string(), create_type_module());

    // OS module
    stdlib.insert("os".to_string(), create_os_module());

    // Table module (for dictionary operations)
    stdlib.insert("table".to_string(), create_table_module());

    // Base64 module
    stdlib.insert("base64".to_string(), create_base64_module());

    // DateTime module
    stdlib.insert("datetime".to_string(), create_datetime_module());

    // Regex module
    stdlib.insert("regex".to_string(), create_regex_module());

    // Crypto module
    stdlib.insert("crypto".to_string(), create_crypto_module());

    // HTTP module
    stdlib.insert("http".to_string(), create_http_module());

    // CSV module
    stdlib.insert("csv".to_string(), create_csv_module());

    // Path module
    stdlib.insert("path".to_string(), create_path_module());

    // Assert module
    stdlib.insert("assert".to_string(), create_assert_module());

    // Set module
    stdlib.insert("set".to_string(), create_set_module());

    // IO module
    stdlib.insert("io".to_string(), create_io_module());

    // Log module
    stdlib.insert("log".to_string(), create_log_module());

    // URL module
    stdlib.insert("url".to_string(), create_url_module());

    stdlib
}

fn create_json_module() -> Value {
    let mut json_funcs = HashMap::new();

    json_funcs.insert("parse".to_string(), Value::NativeFunction("json".to_string(), "parse".to_string()));
    json_funcs.insert("stringify".to_string(), Value::NativeFunction("json".to_string(), "stringify".to_string()));
    json_funcs.insert("pretty".to_string(), Value::NativeFunction("json".to_string(), "pretty".to_string()));
    json_funcs.insert("valid".to_string(), Value::NativeFunction("json".to_string(), "valid".to_string()));

    Value::Module(json_funcs)
}

fn create_fs_module() -> Value {
    let mut fs_funcs = HashMap::new();

    fs_funcs.insert("read".to_string(), Value::NativeFunction("fs".to_string(), "read".to_string()));
    fs_funcs.insert("write".to_string(), Value::NativeFunction("fs".to_string(), "write".to_string()));
    fs_funcs.insert("exists".to_string(), Value::NativeFunction("fs".to_string(), "exists".to_string()));
    fs_funcs.insert("readdir".to_string(), Value::NativeFunction("fs".to_string(), "readdir".to_string()));
    fs_funcs.insert("mkdir".to_string(), Value::NativeFunction("fs".to_string(), "mkdir".to_string()));
    fs_funcs.insert("remove".to_string(), Value::NativeFunction("fs".to_string(), "remove".to_string()));
    fs_funcs.insert("isdir".to_string(), Value::NativeFunction("fs".to_string(), "isdir".to_string()));
    fs_funcs.insert("isfile".to_string(), Value::NativeFunction("fs".to_string(), "isfile".to_string()));
    fs_funcs.insert("append".to_string(), Value::NativeFunction("fs".to_string(), "append".to_string()));

    // New fs functions
    fs_funcs.insert("copy".to_string(), Value::NativeFunction("fs".to_string(), "copy".to_string()));
    fs_funcs.insert("rename".to_string(), Value::NativeFunction("fs".to_string(), "rename".to_string()));
    fs_funcs.insert("basename".to_string(), Value::NativeFunction("fs".to_string(), "basename".to_string()));
    fs_funcs.insert("dirname".to_string(), Value::NativeFunction("fs".to_string(), "dirname".to_string()));
    fs_funcs.insert("extension".to_string(), Value::NativeFunction("fs".to_string(), "extension".to_string()));
    fs_funcs.insert("join_path".to_string(), Value::NativeFunction("fs".to_string(), "join_path".to_string()));
    fs_funcs.insert("absolute".to_string(), Value::NativeFunction("fs".to_string(), "absolute".to_string()));
    fs_funcs.insert("filesize".to_string(), Value::NativeFunction("fs".to_string(), "filesize".to_string()));
    fs_funcs.insert("modified".to_string(), Value::NativeFunction("fs".to_string(), "modified".to_string()));
    fs_funcs.insert("created".to_string(), Value::NativeFunction("fs".to_string(), "created".to_string()));
    fs_funcs.insert("is_symlink".to_string(), Value::NativeFunction("fs".to_string(), "is_symlink".to_string()));
    fs_funcs.insert("read_bytes".to_string(), Value::NativeFunction("fs".to_string(), "read_bytes".to_string()));
    fs_funcs.insert("write_bytes".to_string(), Value::NativeFunction("fs".to_string(), "write_bytes".to_string()));
    fs_funcs.insert("glob".to_string(), Value::NativeFunction("fs".to_string(), "glob".to_string()));
    fs_funcs.insert("walk".to_string(), Value::NativeFunction("fs".to_string(), "walk".to_string()));
    fs_funcs.insert("stat".to_string(), Value::NativeFunction("fs".to_string(), "stat".to_string()));
    fs_funcs.insert("temp_file".to_string(), Value::NativeFunction("fs".to_string(), "temp_file".to_string()));
    fs_funcs.insert("temp_dir".to_string(), Value::NativeFunction("fs".to_string(), "temp_dir".to_string()));
    fs_funcs.insert("read_lines".to_string(), Value::NativeFunction("fs".to_string(), "read_lines".to_string()));
    fs_funcs.insert("touch".to_string(), Value::NativeFunction("fs".to_string(), "touch".to_string()));

    Value::Module(fs_funcs)
}

fn create_string_module() -> Value {
    let mut string_funcs = HashMap::new();

    string_funcs.insert("length".to_string(), Value::NativeFunction("string".to_string(), "length".to_string()));
    string_funcs.insert("upper".to_string(), Value::NativeFunction("string".to_string(), "upper".to_string()));
    string_funcs.insert("lower".to_string(), Value::NativeFunction("string".to_string(), "lower".to_string()));
    string_funcs.insert("substring".to_string(), Value::NativeFunction("string".to_string(), "substring".to_string()));
    string_funcs.insert("split".to_string(), Value::NativeFunction("string".to_string(), "split".to_string()));
    string_funcs.insert("trim".to_string(), Value::NativeFunction("string".to_string(), "trim".to_string()));
    string_funcs.insert("replace".to_string(), Value::NativeFunction("string".to_string(), "replace".to_string()));
    string_funcs.insert("contains".to_string(), Value::NativeFunction("string".to_string(), "contains".to_string()));
    string_funcs.insert("starts_with".to_string(), Value::NativeFunction("string".to_string(), "starts_with".to_string()));
    string_funcs.insert("ends_with".to_string(), Value::NativeFunction("string".to_string(), "ends_with".to_string()));
    string_funcs.insert("find".to_string(), Value::NativeFunction("string".to_string(), "find".to_string()));
    string_funcs.insert("reverse".to_string(), Value::NativeFunction("string".to_string(), "reverse".to_string()));
    string_funcs.insert("char".to_string(), Value::NativeFunction("string".to_string(), "char".to_string()));
    string_funcs.insert("byte".to_string(), Value::NativeFunction("string".to_string(), "byte".to_string()));
    string_funcs.insert("format".to_string(), Value::NativeFunction("string".to_string(), "format".to_string()));
    string_funcs.insert("repeat".to_string(), Value::NativeFunction("string".to_string(), "repeat".to_string()));

    // New string functions
    string_funcs.insert("ltrim".to_string(), Value::NativeFunction("string".to_string(), "ltrim".to_string()));
    string_funcs.insert("rtrim".to_string(), Value::NativeFunction("string".to_string(), "rtrim".to_string()));
    string_funcs.insert("pad_left".to_string(), Value::NativeFunction("string".to_string(), "pad_left".to_string()));
    string_funcs.insert("pad_right".to_string(), Value::NativeFunction("string".to_string(), "pad_right".to_string()));
    string_funcs.insert("count".to_string(), Value::NativeFunction("string".to_string(), "count".to_string()));
    string_funcs.insert("is_empty".to_string(), Value::NativeFunction("string".to_string(), "is_empty".to_string()));
    string_funcs.insert("is_numeric".to_string(), Value::NativeFunction("string".to_string(), "is_numeric".to_string()));
    string_funcs.insert("is_alpha".to_string(), Value::NativeFunction("string".to_string(), "is_alpha".to_string()));
    string_funcs.insert("is_alphanumeric".to_string(), Value::NativeFunction("string".to_string(), "is_alphanumeric".to_string()));
    string_funcs.insert("capitalize".to_string(), Value::NativeFunction("string".to_string(), "capitalize".to_string()));
    string_funcs.insert("title".to_string(), Value::NativeFunction("string".to_string(), "title".to_string()));
    string_funcs.insert("lines".to_string(), Value::NativeFunction("string".to_string(), "lines".to_string()));
    string_funcs.insert("chars".to_string(), Value::NativeFunction("string".to_string(), "chars".to_string()));
    string_funcs.insert("match".to_string(), Value::NativeFunction("string".to_string(), "match".to_string()));
    string_funcs.insert("replace_first".to_string(), Value::NativeFunction("string".to_string(), "replace_first".to_string()));
    string_funcs.insert("insert".to_string(), Value::NativeFunction("string".to_string(), "insert".to_string()));
    string_funcs.insert("remove".to_string(), Value::NativeFunction("string".to_string(), "remove".to_string()));
    string_funcs.insert("slug".to_string(), Value::NativeFunction("string".to_string(), "slug".to_string()));
    string_funcs.insert("truncate".to_string(), Value::NativeFunction("string".to_string(), "truncate".to_string()));
    string_funcs.insert("word_wrap".to_string(), Value::NativeFunction("string".to_string(), "word_wrap".to_string()));
    string_funcs.insert("center".to_string(), Value::NativeFunction("string".to_string(), "center".to_string()));
    string_funcs.insert("escape_html".to_string(), Value::NativeFunction("string".to_string(), "escape_html".to_string()));
    string_funcs.insert("unescape_html".to_string(), Value::NativeFunction("string".to_string(), "unescape_html".to_string()));

    Value::Module(string_funcs)
}

fn create_math_module() -> Value {
    let mut math_funcs = HashMap::new();

    // Math constants
    math_funcs.insert("pi".to_string(), Value::Number(std::f64::consts::PI));
    math_funcs.insert("e".to_string(), Value::Number(std::f64::consts::E));
    math_funcs.insert("inf".to_string(), Value::Number(f64::INFINITY));
    math_funcs.insert("nan".to_string(), Value::Number(f64::NAN));

    // Math functions
    math_funcs.insert("floor".to_string(), Value::NativeFunction("math".to_string(), "floor".to_string()));
    math_funcs.insert("ceil".to_string(), Value::NativeFunction("math".to_string(), "ceil".to_string()));
    math_funcs.insert("round".to_string(), Value::NativeFunction("math".to_string(), "round".to_string()));
    math_funcs.insert("abs".to_string(), Value::NativeFunction("math".to_string(), "abs".to_string()));
    math_funcs.insert("min".to_string(), Value::NativeFunction("math".to_string(), "min".to_string()));
    math_funcs.insert("max".to_string(), Value::NativeFunction("math".to_string(), "max".to_string()));
    math_funcs.insert("sqrt".to_string(), Value::NativeFunction("math".to_string(), "sqrt".to_string()));
    math_funcs.insert("pow".to_string(), Value::NativeFunction("math".to_string(), "pow".to_string()));
    math_funcs.insert("sin".to_string(), Value::NativeFunction("math".to_string(), "sin".to_string()));
    math_funcs.insert("cos".to_string(), Value::NativeFunction("math".to_string(), "cos".to_string()));
    math_funcs.insert("tan".to_string(), Value::NativeFunction("math".to_string(), "tan".to_string()));
    math_funcs.insert("asin".to_string(), Value::NativeFunction("math".to_string(), "asin".to_string()));
    math_funcs.insert("acos".to_string(), Value::NativeFunction("math".to_string(), "acos".to_string()));
    math_funcs.insert("atan".to_string(), Value::NativeFunction("math".to_string(), "atan".to_string()));
    math_funcs.insert("atan2".to_string(), Value::NativeFunction("math".to_string(), "atan2".to_string()));
    math_funcs.insert("log".to_string(), Value::NativeFunction("math".to_string(), "log".to_string()));
    math_funcs.insert("log10".to_string(), Value::NativeFunction("math".to_string(), "log10".to_string()));
    math_funcs.insert("exp".to_string(), Value::NativeFunction("math".to_string(), "exp".to_string()));
    math_funcs.insert("random".to_string(), Value::NativeFunction("math".to_string(), "random".to_string()));
    math_funcs.insert("randomint".to_string(), Value::NativeFunction("math".to_string(), "randomint".to_string()));
    math_funcs.insert("sign".to_string(), Value::NativeFunction("math".to_string(), "sign".to_string()));
    math_funcs.insert("clamp".to_string(), Value::NativeFunction("math".to_string(), "clamp".to_string()));

    // New math functions
    math_funcs.insert("log2".to_string(), Value::NativeFunction("math".to_string(), "log2".to_string()));
    math_funcs.insert("hypot".to_string(), Value::NativeFunction("math".to_string(), "hypot".to_string()));
    math_funcs.insert("sinh".to_string(), Value::NativeFunction("math".to_string(), "sinh".to_string()));
    math_funcs.insert("cosh".to_string(), Value::NativeFunction("math".to_string(), "cosh".to_string()));
    math_funcs.insert("tanh".to_string(), Value::NativeFunction("math".to_string(), "tanh".to_string()));
    math_funcs.insert("deg".to_string(), Value::NativeFunction("math".to_string(), "deg".to_string()));
    math_funcs.insert("rad".to_string(), Value::NativeFunction("math".to_string(), "rad".to_string()));
    math_funcs.insert("lerp".to_string(), Value::NativeFunction("math".to_string(), "lerp".to_string()));
    math_funcs.insert("isnan".to_string(), Value::NativeFunction("math".to_string(), "isnan".to_string()));
    math_funcs.insert("isinf".to_string(), Value::NativeFunction("math".to_string(), "isinf".to_string()));
    math_funcs.insert("fmod".to_string(), Value::NativeFunction("math".to_string(), "fmod".to_string()));
    math_funcs.insert("trunc".to_string(), Value::NativeFunction("math".to_string(), "trunc".to_string()));
    math_funcs.insert("fract".to_string(), Value::NativeFunction("math".to_string(), "fract".to_string()));
    math_funcs.insert("cbrt".to_string(), Value::NativeFunction("math".to_string(), "cbrt".to_string()));
    math_funcs.insert("gcd".to_string(), Value::NativeFunction("math".to_string(), "gcd".to_string()));
    math_funcs.insert("lcm".to_string(), Value::NativeFunction("math".to_string(), "lcm".to_string()));
    math_funcs.insert("factorial".to_string(), Value::NativeFunction("math".to_string(), "factorial".to_string()));
    math_funcs.insert("sum".to_string(), Value::NativeFunction("math".to_string(), "sum".to_string()));
    math_funcs.insert("product".to_string(), Value::NativeFunction("math".to_string(), "product".to_string()));
    math_funcs.insert("mean".to_string(), Value::NativeFunction("math".to_string(), "mean".to_string()));
    math_funcs.insert("median".to_string(), Value::NativeFunction("math".to_string(), "median".to_string()));
    math_funcs.insert("variance".to_string(), Value::NativeFunction("math".to_string(), "variance".to_string()));
    math_funcs.insert("stddev".to_string(), Value::NativeFunction("math".to_string(), "stddev".to_string()));
    math_funcs.insert("prime".to_string(), Value::NativeFunction("math".to_string(), "prime".to_string()));
    math_funcs.insert("fibonacci".to_string(), Value::NativeFunction("math".to_string(), "fibonacci".to_string()));
    math_funcs.insert("map".to_string(), Value::NativeFunction("math".to_string(), "map".to_string()));

    Value::Module(math_funcs)
}

fn create_array_module() -> Value {
    let mut array_funcs = HashMap::new();

    array_funcs.insert("length".to_string(), Value::NativeFunction("array".to_string(), "length".to_string()));
    array_funcs.insert("push".to_string(), Value::NativeFunction("array".to_string(), "push".to_string()));
    array_funcs.insert("pop".to_string(), Value::NativeFunction("array".to_string(), "pop".to_string()));
    array_funcs.insert("shift".to_string(), Value::NativeFunction("array".to_string(), "shift".to_string()));
    array_funcs.insert("unshift".to_string(), Value::NativeFunction("array".to_string(), "unshift".to_string()));
    array_funcs.insert("slice".to_string(), Value::NativeFunction("array".to_string(), "slice".to_string()));
    array_funcs.insert("concat".to_string(), Value::NativeFunction("array".to_string(), "concat".to_string()));
    array_funcs.insert("join".to_string(), Value::NativeFunction("array".to_string(), "join".to_string()));
    array_funcs.insert("reverse".to_string(), Value::NativeFunction("array".to_string(), "reverse".to_string()));
    array_funcs.insert("sort".to_string(), Value::NativeFunction("array".to_string(), "sort".to_string()));
    array_funcs.insert("contains".to_string(), Value::NativeFunction("array".to_string(), "contains".to_string()));
    array_funcs.insert("find".to_string(), Value::NativeFunction("array".to_string(), "find".to_string()));
    array_funcs.insert("range".to_string(), Value::NativeFunction("array".to_string(), "range".to_string()));

    // New array functions
    array_funcs.insert("first".to_string(), Value::NativeFunction("array".to_string(), "first".to_string()));
    array_funcs.insert("last".to_string(), Value::NativeFunction("array".to_string(), "last".to_string()));
    array_funcs.insert("flatten".to_string(), Value::NativeFunction("array".to_string(), "flatten".to_string()));
    array_funcs.insert("unique".to_string(), Value::NativeFunction("array".to_string(), "unique".to_string()));
    array_funcs.insert("filter_nil".to_string(), Value::NativeFunction("array".to_string(), "filter_nil".to_string()));
    array_funcs.insert("fill".to_string(), Value::NativeFunction("array".to_string(), "fill".to_string()));
    array_funcs.insert("insert".to_string(), Value::NativeFunction("array".to_string(), "insert".to_string()));
    array_funcs.insert("remove_at".to_string(), Value::NativeFunction("array".to_string(), "remove_at".to_string()));
    array_funcs.insert("min".to_string(), Value::NativeFunction("array".to_string(), "min".to_string()));
    array_funcs.insert("max".to_string(), Value::NativeFunction("array".to_string(), "max".to_string()));
    array_funcs.insert("sum".to_string(), Value::NativeFunction("array".to_string(), "sum".to_string()));
    array_funcs.insert("avg".to_string(), Value::NativeFunction("array".to_string(), "avg".to_string()));
    array_funcs.insert("zip".to_string(), Value::NativeFunction("array".to_string(), "zip".to_string()));
    array_funcs.insert("count".to_string(), Value::NativeFunction("array".to_string(), "count".to_string()));
    array_funcs.insert("copy".to_string(), Value::NativeFunction("array".to_string(), "copy".to_string()));
    array_funcs.insert("clear".to_string(), Value::NativeFunction("array".to_string(), "clear".to_string()));
    array_funcs.insert("swap".to_string(), Value::NativeFunction("array".to_string(), "swap".to_string()));
    array_funcs.insert("shuffle".to_string(), Value::NativeFunction("array".to_string(), "shuffle".to_string()));
    array_funcs.insert("sample".to_string(), Value::NativeFunction("array".to_string(), "sample".to_string()));
    array_funcs.insert("chunk".to_string(), Value::NativeFunction("array".to_string(), "chunk".to_string()));
    array_funcs.insert("partition".to_string(), Value::NativeFunction("array".to_string(), "partition".to_string()));
    array_funcs.insert("rotate".to_string(), Value::NativeFunction("array".to_string(), "rotate".to_string()));
    array_funcs.insert("take".to_string(), Value::NativeFunction("array".to_string(), "take".to_string()));
    array_funcs.insert("drop".to_string(), Value::NativeFunction("array".to_string(), "drop".to_string()));
    array_funcs.insert("repeat".to_string(), Value::NativeFunction("array".to_string(), "repeat".to_string()));

    Value::Module(array_funcs)
}

fn create_type_module() -> Value {
    let mut type_funcs = HashMap::new();

    type_funcs.insert("typeof".to_string(), Value::NativeFunction("type".to_string(), "typeof".to_string()));
    type_funcs.insert("tonumber".to_string(), Value::NativeFunction("type".to_string(), "tonumber".to_string()));
    type_funcs.insert("tostring".to_string(), Value::NativeFunction("type".to_string(), "tostring".to_string()));
    type_funcs.insert("tobool".to_string(), Value::NativeFunction("type".to_string(), "tobool".to_string()));
    type_funcs.insert("isnil".to_string(), Value::NativeFunction("type".to_string(), "isnil".to_string()));
    type_funcs.insert("isnumber".to_string(), Value::NativeFunction("type".to_string(), "isnumber".to_string()));
    type_funcs.insert("isstring".to_string(), Value::NativeFunction("type".to_string(), "isstring".to_string()));
    type_funcs.insert("isbool".to_string(), Value::NativeFunction("type".to_string(), "isbool".to_string()));
    type_funcs.insert("istable".to_string(), Value::NativeFunction("type".to_string(), "istable".to_string()));
    type_funcs.insert("isfunction".to_string(), Value::NativeFunction("type".to_string(), "isfunction".to_string()));

    // Additional type functions
    type_funcs.insert("isdict".to_string(), Value::NativeFunction("type".to_string(), "isdict".to_string()));
    type_funcs.insert("isdictionary".to_string(), Value::NativeFunction("type".to_string(), "isdictionary".to_string()));
    type_funcs.insert("isarray".to_string(), Value::NativeFunction("type".to_string(), "isarray".to_string()));
    type_funcs.insert("ismodule".to_string(), Value::NativeFunction("type".to_string(), "ismodule".to_string()));
    type_funcs.insert("isempty".to_string(), Value::NativeFunction("type".to_string(), "isempty".to_string()));
    type_funcs.insert("default".to_string(), Value::NativeFunction("type".to_string(), "default".to_string()));

    Value::Module(type_funcs)
}

fn create_os_module() -> Value {
    let mut os_funcs = HashMap::new();

    os_funcs.insert("time".to_string(), Value::NativeFunction("os".to_string(), "time".to_string()));
    os_funcs.insert("clock".to_string(), Value::NativeFunction("os".to_string(), "clock".to_string()));
    os_funcs.insert("exit".to_string(), Value::NativeFunction("os".to_string(), "exit".to_string()));
    os_funcs.insert("getenv".to_string(), Value::NativeFunction("os".to_string(), "getenv".to_string()));
    os_funcs.insert("execute".to_string(), Value::NativeFunction("os".to_string(), "execute".to_string()));
    os_funcs.insert("sleep".to_string(), Value::NativeFunction("os".to_string(), "sleep".to_string()));

    // New os functions
    os_funcs.insert("platform".to_string(), Value::NativeFunction("os".to_string(), "platform".to_string()));
    os_funcs.insert("arch".to_string(), Value::NativeFunction("os".to_string(), "arch".to_string()));
    os_funcs.insert("hostname".to_string(), Value::NativeFunction("os".to_string(), "hostname".to_string()));
    os_funcs.insert("cwd".to_string(), Value::NativeFunction("os".to_string(), "cwd".to_string()));
    os_funcs.insert("chdir".to_string(), Value::NativeFunction("os".to_string(), "chdir".to_string()));
    os_funcs.insert("home".to_string(), Value::NativeFunction("os".to_string(), "home".to_string()));
    os_funcs.insert("setenv".to_string(), Value::NativeFunction("os".to_string(), "setenv".to_string()));
    os_funcs.insert("unsetenv".to_string(), Value::NativeFunction("os".to_string(), "unsetenv".to_string()));
    os_funcs.insert("envvars".to_string(), Value::NativeFunction("os".to_string(), "envvars".to_string()));
    os_funcs.insert("tmpdir".to_string(), Value::NativeFunction("os".to_string(), "tmpdir".to_string()));
    os_funcs.insert("pid".to_string(), Value::NativeFunction("os".to_string(), "pid".to_string()));
    os_funcs.insert("args".to_string(), Value::NativeFunction("os".to_string(), "args".to_string()));
    os_funcs.insert("user".to_string(), Value::NativeFunction("os".to_string(), "user".to_string()));
    os_funcs.insert("cpus".to_string(), Value::NativeFunction("os".to_string(), "cpus".to_string()));
    os_funcs.insert("version".to_string(), Value::NativeFunction("os".to_string(), "version".to_string()));
    os_funcs.insert("family".to_string(), Value::NativeFunction("os".to_string(), "family".to_string()));
    os_funcs.insert("shell".to_string(), Value::NativeFunction("os".to_string(), "shell".to_string()));
    os_funcs.insert("spawn".to_string(), Value::NativeFunction("os".to_string(), "spawn".to_string()));
    os_funcs.insert("which".to_string(), Value::NativeFunction("os".to_string(), "which".to_string()));

    Value::Module(os_funcs)
}

fn create_table_module() -> Value {
    let mut table_funcs = HashMap::new();

    table_funcs.insert("keys".to_string(), Value::NativeFunction("table".to_string(), "keys".to_string()));
    table_funcs.insert("values".to_string(), Value::NativeFunction("table".to_string(), "values".to_string()));
    table_funcs.insert("entries".to_string(), Value::NativeFunction("table".to_string(), "entries".to_string()));
    table_funcs.insert("has".to_string(), Value::NativeFunction("table".to_string(), "has".to_string()));
    table_funcs.insert("get".to_string(), Value::NativeFunction("table".to_string(), "get".to_string()));
    table_funcs.insert("set".to_string(), Value::NativeFunction("table".to_string(), "set".to_string()));
    table_funcs.insert("delete".to_string(), Value::NativeFunction("table".to_string(), "delete".to_string()));
    table_funcs.insert("merge".to_string(), Value::NativeFunction("table".to_string(), "merge".to_string()));
    table_funcs.insert("size".to_string(), Value::NativeFunction("table".to_string(), "size".to_string()));
    table_funcs.insert("copy".to_string(), Value::NativeFunction("table".to_string(), "copy".to_string()));
    table_funcs.insert("clear".to_string(), Value::NativeFunction("table".to_string(), "clear".to_string()));
    table_funcs.insert("from_entries".to_string(), Value::NativeFunction("table".to_string(), "from_entries".to_string()));
    table_funcs.insert("invert".to_string(), Value::NativeFunction("table".to_string(), "invert".to_string()));

    Value::Module(table_funcs)
}

fn create_base64_module() -> Value {
    let mut base64_funcs = HashMap::new();

    base64_funcs.insert("encode".to_string(), Value::NativeFunction("base64".to_string(), "encode".to_string()));
    base64_funcs.insert("decode".to_string(), Value::NativeFunction("base64".to_string(), "decode".to_string()));

    Value::Module(base64_funcs)
}

fn create_datetime_module() -> Value {
    let mut datetime_funcs = HashMap::new();

    datetime_funcs.insert("now".to_string(), Value::NativeFunction("datetime".to_string(), "now".to_string()));
    datetime_funcs.insert("timestamp".to_string(), Value::NativeFunction("datetime".to_string(), "timestamp".to_string()));
    datetime_funcs.insert("format".to_string(), Value::NativeFunction("datetime".to_string(), "format".to_string()));
    datetime_funcs.insert("parse".to_string(), Value::NativeFunction("datetime".to_string(), "parse".to_string()));
    datetime_funcs.insert("year".to_string(), Value::NativeFunction("datetime".to_string(), "year".to_string()));
    datetime_funcs.insert("month".to_string(), Value::NativeFunction("datetime".to_string(), "month".to_string()));
    datetime_funcs.insert("day".to_string(), Value::NativeFunction("datetime".to_string(), "day".to_string()));
    datetime_funcs.insert("hour".to_string(), Value::NativeFunction("datetime".to_string(), "hour".to_string()));
    datetime_funcs.insert("minute".to_string(), Value::NativeFunction("datetime".to_string(), "minute".to_string()));
    datetime_funcs.insert("second".to_string(), Value::NativeFunction("datetime".to_string(), "second".to_string()));
    datetime_funcs.insert("weekday".to_string(), Value::NativeFunction("datetime".to_string(), "weekday".to_string()));
    datetime_funcs.insert("day_of_year".to_string(), Value::NativeFunction("datetime".to_string(), "day_of_year".to_string()));
    datetime_funcs.insert("is_leap_year".to_string(), Value::NativeFunction("datetime".to_string(), "is_leap_year".to_string()));
    datetime_funcs.insert("add_days".to_string(), Value::NativeFunction("datetime".to_string(), "add_days".to_string()));
    datetime_funcs.insert("add_hours".to_string(), Value::NativeFunction("datetime".to_string(), "add_hours".to_string()));
    datetime_funcs.insert("add_minutes".to_string(), Value::NativeFunction("datetime".to_string(), "add_minutes".to_string()));
    datetime_funcs.insert("add_seconds".to_string(), Value::NativeFunction("datetime".to_string(), "add_seconds".to_string()));
    datetime_funcs.insert("diff".to_string(), Value::NativeFunction("datetime".to_string(), "diff".to_string()));
    datetime_funcs.insert("from_timestamp".to_string(), Value::NativeFunction("datetime".to_string(), "from_timestamp".to_string()));

    Value::Module(datetime_funcs)
}

fn create_regex_module() -> Value {
    let mut regex_funcs = HashMap::new();

    regex_funcs.insert("match".to_string(), Value::NativeFunction("regex".to_string(), "match".to_string()));
    regex_funcs.insert("find".to_string(), Value::NativeFunction("regex".to_string(), "find".to_string()));
    regex_funcs.insert("find_all".to_string(), Value::NativeFunction("regex".to_string(), "find_all".to_string()));
    regex_funcs.insert("replace".to_string(), Value::NativeFunction("regex".to_string(), "replace".to_string()));
    regex_funcs.insert("replace_all".to_string(), Value::NativeFunction("regex".to_string(), "replace_all".to_string()));
    regex_funcs.insert("split".to_string(), Value::NativeFunction("regex".to_string(), "split".to_string()));
    regex_funcs.insert("escape".to_string(), Value::NativeFunction("regex".to_string(), "escape".to_string()));
    regex_funcs.insert("is_valid".to_string(), Value::NativeFunction("regex".to_string(), "is_valid".to_string()));
    regex_funcs.insert("captures".to_string(), Value::NativeFunction("regex".to_string(), "captures".to_string()));

    Value::Module(regex_funcs)
}

fn create_crypto_module() -> Value {
    let mut crypto_funcs = HashMap::new();

    crypto_funcs.insert("md5".to_string(), Value::NativeFunction("crypto".to_string(), "md5".to_string()));
    crypto_funcs.insert("sha1".to_string(), Value::NativeFunction("crypto".to_string(), "sha1".to_string()));
    crypto_funcs.insert("sha256".to_string(), Value::NativeFunction("crypto".to_string(), "sha256".to_string()));
    crypto_funcs.insert("sha512".to_string(), Value::NativeFunction("crypto".to_string(), "sha512".to_string()));
    crypto_funcs.insert("hmac_sha256".to_string(), Value::NativeFunction("crypto".to_string(), "hmac_sha256".to_string()));
    crypto_funcs.insert("uuid".to_string(), Value::NativeFunction("crypto".to_string(), "uuid".to_string()));
    crypto_funcs.insert("uuid_v4".to_string(), Value::NativeFunction("crypto".to_string(), "uuid_v4".to_string()));
    crypto_funcs.insert("random_bytes".to_string(), Value::NativeFunction("crypto".to_string(), "random_bytes".to_string()));
    crypto_funcs.insert("random_hex".to_string(), Value::NativeFunction("crypto".to_string(), "random_hex".to_string()));
    crypto_funcs.insert("hash".to_string(), Value::NativeFunction("crypto".to_string(), "hash".to_string()));

    Value::Module(crypto_funcs)
}

fn create_http_module() -> Value {
    let mut http_funcs = HashMap::new();

    http_funcs.insert("get".to_string(), Value::NativeFunction("http".to_string(), "get".to_string()));
    http_funcs.insert("post".to_string(), Value::NativeFunction("http".to_string(), "post".to_string()));
    http_funcs.insert("put".to_string(), Value::NativeFunction("http".to_string(), "put".to_string()));
    http_funcs.insert("delete".to_string(), Value::NativeFunction("http".to_string(), "delete".to_string()));
    http_funcs.insert("patch".to_string(), Value::NativeFunction("http".to_string(), "patch".to_string()));
    http_funcs.insert("head".to_string(), Value::NativeFunction("http".to_string(), "head".to_string()));
    http_funcs.insert("request".to_string(), Value::NativeFunction("http".to_string(), "request".to_string()));
    http_funcs.insert("download".to_string(), Value::NativeFunction("http".to_string(), "download".to_string()));

    Value::Module(http_funcs)
}

fn create_csv_module() -> Value {
    let mut csv_funcs = HashMap::new();

    csv_funcs.insert("parse".to_string(), Value::NativeFunction("csv".to_string(), "parse".to_string()));
    csv_funcs.insert("stringify".to_string(), Value::NativeFunction("csv".to_string(), "stringify".to_string()));
    csv_funcs.insert("read".to_string(), Value::NativeFunction("csv".to_string(), "read".to_string()));
    csv_funcs.insert("write".to_string(), Value::NativeFunction("csv".to_string(), "write".to_string()));
    csv_funcs.insert("parse_row".to_string(), Value::NativeFunction("csv".to_string(), "parse_row".to_string()));
    csv_funcs.insert("stringify_row".to_string(), Value::NativeFunction("csv".to_string(), "stringify_row".to_string()));

    Value::Module(csv_funcs)
}

fn create_path_module() -> Value {
    let mut path_funcs = HashMap::new();

    path_funcs.insert("join".to_string(), Value::NativeFunction("path".to_string(), "join".to_string()));
    path_funcs.insert("basename".to_string(), Value::NativeFunction("path".to_string(), "basename".to_string()));
    path_funcs.insert("dirname".to_string(), Value::NativeFunction("path".to_string(), "dirname".to_string()));
    path_funcs.insert("extname".to_string(), Value::NativeFunction("path".to_string(), "extname".to_string()));
    path_funcs.insert("stem".to_string(), Value::NativeFunction("path".to_string(), "stem".to_string()));
    path_funcs.insert("normalize".to_string(), Value::NativeFunction("path".to_string(), "normalize".to_string()));
    path_funcs.insert("is_absolute".to_string(), Value::NativeFunction("path".to_string(), "is_absolute".to_string()));
    path_funcs.insert("is_relative".to_string(), Value::NativeFunction("path".to_string(), "is_relative".to_string()));
    path_funcs.insert("resolve".to_string(), Value::NativeFunction("path".to_string(), "resolve".to_string()));
    path_funcs.insert("relative".to_string(), Value::NativeFunction("path".to_string(), "relative".to_string()));
    path_funcs.insert("split".to_string(), Value::NativeFunction("path".to_string(), "split".to_string()));
    path_funcs.insert("with_extension".to_string(), Value::NativeFunction("path".to_string(), "with_extension".to_string()));
    path_funcs.insert("separator".to_string(), Value::NativeFunction("path".to_string(), "separator".to_string()));

    Value::Module(path_funcs)
}

fn create_assert_module() -> Value {
    let mut assert_funcs = HashMap::new();

    assert_funcs.insert("equal".to_string(), Value::NativeFunction("assert".to_string(), "equal".to_string()));
    assert_funcs.insert("not_equal".to_string(), Value::NativeFunction("assert".to_string(), "not_equal".to_string()));
    assert_funcs.insert("true".to_string(), Value::NativeFunction("assert".to_string(), "true".to_string()));
    assert_funcs.insert("false".to_string(), Value::NativeFunction("assert".to_string(), "false".to_string()));
    assert_funcs.insert("nil".to_string(), Value::NativeFunction("assert".to_string(), "nil".to_string()));
    assert_funcs.insert("not_nil".to_string(), Value::NativeFunction("assert".to_string(), "not_nil".to_string()));
    assert_funcs.insert("type".to_string(), Value::NativeFunction("assert".to_string(), "type".to_string()));
    assert_funcs.insert("fail".to_string(), Value::NativeFunction("assert".to_string(), "fail".to_string()));
    assert_funcs.insert("greater".to_string(), Value::NativeFunction("assert".to_string(), "greater".to_string()));
    assert_funcs.insert("less".to_string(), Value::NativeFunction("assert".to_string(), "less".to_string()));
    assert_funcs.insert("contains".to_string(), Value::NativeFunction("assert".to_string(), "contains".to_string()));
    assert_funcs.insert("matches".to_string(), Value::NativeFunction("assert".to_string(), "matches".to_string()));

    Value::Module(assert_funcs)
}

fn create_set_module() -> Value {
    let mut set_funcs = HashMap::new();

    set_funcs.insert("new".to_string(), Value::NativeFunction("set".to_string(), "new".to_string()));
    set_funcs.insert("from_array".to_string(), Value::NativeFunction("set".to_string(), "from_array".to_string()));
    set_funcs.insert("add".to_string(), Value::NativeFunction("set".to_string(), "add".to_string()));
    set_funcs.insert("remove".to_string(), Value::NativeFunction("set".to_string(), "remove".to_string()));
    set_funcs.insert("has".to_string(), Value::NativeFunction("set".to_string(), "has".to_string()));
    set_funcs.insert("size".to_string(), Value::NativeFunction("set".to_string(), "size".to_string()));
    set_funcs.insert("union".to_string(), Value::NativeFunction("set".to_string(), "union".to_string()));
    set_funcs.insert("intersection".to_string(), Value::NativeFunction("set".to_string(), "intersection".to_string()));
    set_funcs.insert("difference".to_string(), Value::NativeFunction("set".to_string(), "difference".to_string()));
    set_funcs.insert("symmetric_difference".to_string(), Value::NativeFunction("set".to_string(), "symmetric_difference".to_string()));
    set_funcs.insert("is_subset".to_string(), Value::NativeFunction("set".to_string(), "is_subset".to_string()));
    set_funcs.insert("is_superset".to_string(), Value::NativeFunction("set".to_string(), "is_superset".to_string()));
    set_funcs.insert("to_array".to_string(), Value::NativeFunction("set".to_string(), "to_array".to_string()));
    set_funcs.insert("clear".to_string(), Value::NativeFunction("set".to_string(), "clear".to_string()));

    Value::Module(set_funcs)
}

fn create_io_module() -> Value {
    let mut io_funcs = HashMap::new();

    io_funcs.insert("read_line".to_string(), Value::NativeFunction("io".to_string(), "read_line".to_string()));
    io_funcs.insert("read_all".to_string(), Value::NativeFunction("io".to_string(), "read_all".to_string()));
    io_funcs.insert("print".to_string(), Value::NativeFunction("io".to_string(), "print".to_string()));
    io_funcs.insert("println".to_string(), Value::NativeFunction("io".to_string(), "println".to_string()));
    io_funcs.insert("eprint".to_string(), Value::NativeFunction("io".to_string(), "eprint".to_string()));
    io_funcs.insert("eprintln".to_string(), Value::NativeFunction("io".to_string(), "eprintln".to_string()));
    io_funcs.insert("flush".to_string(), Value::NativeFunction("io".to_string(), "flush".to_string()));
    io_funcs.insert("input".to_string(), Value::NativeFunction("io".to_string(), "input".to_string()));

    Value::Module(io_funcs)
}

fn create_log_module() -> Value {
    let mut log_funcs = HashMap::new();

    log_funcs.insert("debug".to_string(), Value::NativeFunction("log".to_string(), "debug".to_string()));
    log_funcs.insert("info".to_string(), Value::NativeFunction("log".to_string(), "info".to_string()));
    log_funcs.insert("warn".to_string(), Value::NativeFunction("log".to_string(), "warn".to_string()));
    log_funcs.insert("error".to_string(), Value::NativeFunction("log".to_string(), "error".to_string()));
    log_funcs.insert("fatal".to_string(), Value::NativeFunction("log".to_string(), "fatal".to_string()));
    log_funcs.insert("trace".to_string(), Value::NativeFunction("log".to_string(), "trace".to_string()));
    log_funcs.insert("level".to_string(), Value::NativeFunction("log".to_string(), "level".to_string()));

    Value::Module(log_funcs)
}

fn create_url_module() -> Value {
    let mut url_funcs = HashMap::new();

    url_funcs.insert("parse".to_string(), Value::NativeFunction("url".to_string(), "parse".to_string()));
    url_funcs.insert("format".to_string(), Value::NativeFunction("url".to_string(), "format".to_string()));
    url_funcs.insert("encode".to_string(), Value::NativeFunction("url".to_string(), "encode".to_string()));
    url_funcs.insert("decode".to_string(), Value::NativeFunction("url".to_string(), "decode".to_string()));
    url_funcs.insert("encode_component".to_string(), Value::NativeFunction("url".to_string(), "encode_component".to_string()));
    url_funcs.insert("decode_component".to_string(), Value::NativeFunction("url".to_string(), "decode_component".to_string()));
    url_funcs.insert("query_parse".to_string(), Value::NativeFunction("url".to_string(), "query_parse".to_string()));
    url_funcs.insert("query_stringify".to_string(), Value::NativeFunction("url".to_string(), "query_stringify".to_string()));
    url_funcs.insert("join".to_string(), Value::NativeFunction("url".to_string(), "join".to_string()));
    url_funcs.insert("is_valid".to_string(), Value::NativeFunction("url".to_string(), "is_valid".to_string()));

    Value::Module(url_funcs)
}

/// Call a native function
pub fn call_native(module: &str, func: &str, args: Vec<Value>, verbose: bool) -> Value {
    if verbose {
        println!("  Calling native {}.{} with {:?}", module, func, args);
    }

    match module {
        "math" => call_math(func, args),
        "string" => call_string(func, args),
        "array" => call_array(func, args),
        "fs" => call_fs(func, args),
        "json" => call_json(func, args),
        "type" => call_type(func, args),
        "os" => call_os(func, args),
        "table" => call_table(func, args),
        "base64" => call_base64(func, args),
        "datetime" => call_datetime(func, args),
        "regex" => call_regex(func, args),
        "crypto" => call_crypto(func, args),
        "http" => call_http(func, args),
        "csv" => call_csv(func, args),
        "path" => call_path(func, args),
        "assert" => call_assert(func, args),
        "set" => call_set(func, args),
        "io" => call_io(func, args),
        "log" => call_log(func, args),
        "url" => call_url(func, args),
        _ => Value::Nil,
    }
}

fn call_math(func: &str, args: Vec<Value>) -> Value {
    match func {
        "floor" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.floor())
            } else {
                Value::Nil
            }
        }
        "ceil" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.ceil())
            } else {
                Value::Nil
            }
        }
        "round" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.round())
            } else {
                Value::Nil
            }
        }
        "abs" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.abs())
            } else {
                Value::Nil
            }
        }
        "sqrt" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.sqrt())
            } else {
                Value::Nil
            }
        }
        "pow" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(base)), Some(Value::Number(exp))) = (args.get(0), args.get(1)) {
                    Value::Number(base.powf(*exp))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "sin" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.sin())
            } else {
                Value::Nil
            }
        }
        "cos" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.cos())
            } else {
                Value::Nil
            }
        }
        "tan" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.tan())
            } else {
                Value::Nil
            }
        }
        "asin" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.asin())
            } else {
                Value::Nil
            }
        }
        "acos" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.acos())
            } else {
                Value::Nil
            }
        }
        "atan" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.atan())
            } else {
                Value::Nil
            }
        }
        "atan2" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(y)), Some(Value::Number(x))) = (args.get(0), args.get(1)) {
                    Value::Number(y.atan2(*x))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "log" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.ln())
            } else {
                Value::Nil
            }
        }
        "log10" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.log10())
            } else {
                Value::Nil
            }
        }
        "exp" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.exp())
            } else {
                Value::Nil
            }
        }
        "min" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a.min(*b))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "max" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a.max(*b))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "random" => {
            use std::time::{SystemTime, UNIX_EPOCH};
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            let random = ((seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31)) as f64
                / (1u64 << 31) as f64;
            Value::Number(random)
        }
        "randomint" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(min)), Some(Value::Number(max))) = (args.get(0), args.get(1)) {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let seed = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64;
                    let random = ((seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31)) as f64
                        / (1u64 << 31) as f64;
                    let range = max - min;
                    Value::Number((min + random * range).floor())
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "sign" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(if *n > 0.0 { 1.0 } else if *n < 0.0 { -1.0 } else { 0.0 })
            } else {
                Value::Nil
            }
        }
        "clamp" => {
            if args.len() >= 3 {
                if let (Some(Value::Number(val)), Some(Value::Number(min)), Some(Value::Number(max))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    Value::Number(val.max(*min).min(*max))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "log2" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.log2())
            } else {
                Value::Nil
            }
        }
        "hypot" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(x)), Some(Value::Number(y))) = (args.get(0), args.get(1)) {
                    Value::Number(x.hypot(*y))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "sinh" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.sinh())
            } else {
                Value::Nil
            }
        }
        "cosh" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.cosh())
            } else {
                Value::Nil
            }
        }
        "tanh" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.tanh())
            } else {
                Value::Nil
            }
        }
        "deg" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.to_degrees())
            } else {
                Value::Nil
            }
        }
        "rad" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.to_radians())
            } else {
                Value::Nil
            }
        }
        "lerp" => {
            if args.len() >= 3 {
                if let (Some(Value::Number(a)), Some(Value::Number(b)), Some(Value::Number(t))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    Value::Number(a + (b - a) * t)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "isnan" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Boolean(n.is_nan())
            } else {
                Value::Nil
            }
        }
        "isinf" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Boolean(n.is_infinite())
            } else {
                Value::Nil
            }
        }
        "fmod" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(x)), Some(Value::Number(y))) = (args.get(0), args.get(1)) {
                    Value::Number(x % y)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "trunc" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.trunc())
            } else {
                Value::Nil
            }
        }
        "fract" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.fract())
            } else {
                Value::Nil
            }
        }
        "cbrt" => {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n.cbrt())
            } else {
                Value::Nil
            }
        }
        "gcd" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    let mut a = a.abs() as u64;
                    let mut b = b.abs() as u64;
                    while b != 0 {
                        let t = b;
                        b = a % b;
                        a = t;
                    }
                    Value::Number(a as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "lcm" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    let a_abs = a.abs() as u64;
                    let b_abs = b.abs() as u64;
                    if a_abs == 0 || b_abs == 0 {
                        Value::Number(0.0)
                    } else {
                        // Calculate GCD first
                        let mut x = a_abs;
                        let mut y = b_abs;
                        while y != 0 {
                            let t = y;
                            y = x % y;
                            x = t;
                        }
                        let gcd = x;
                        Value::Number((a_abs / gcd * b_abs) as f64)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "factorial" => {
            if let Some(Value::Number(n)) = args.first() {
                let n = *n as u64;
                if n > 20 {
                    Value::Number(f64::INFINITY) // Overflow protection
                } else {
                    let mut result: u64 = 1;
                    for i in 2..=n {
                        result *= i;
                    }
                    Value::Number(result as f64)
                }
            } else {
                Value::Nil
            }
        }
        "sum" => {
            if let Some(Value::Table(arr)) = args.first() {
                let sum: f64 = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .sum();
                Value::Number(sum)
            } else {
                Value::Nil
            }
        }
        "product" => {
            if let Some(Value::Table(arr)) = args.first() {
                let product: f64 = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .product();
                Value::Number(product)
            } else {
                Value::Nil
            }
        }
        "mean" => {
            if let Some(Value::Table(arr)) = args.first() {
                let numbers: Vec<f64> = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .collect();
                if numbers.is_empty() {
                    Value::Nil
                } else {
                    let sum: f64 = numbers.iter().sum();
                    Value::Number(sum / numbers.len() as f64)
                }
            } else {
                Value::Nil
            }
        }
        "median" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut numbers: Vec<f64> = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .collect();
                if numbers.is_empty() {
                    Value::Nil
                } else {
                    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let mid = numbers.len() / 2;
                    if numbers.len() % 2 == 0 {
                        Value::Number((numbers[mid - 1] + numbers[mid]) / 2.0)
                    } else {
                        Value::Number(numbers[mid])
                    }
                }
            } else {
                Value::Nil
            }
        }
        "variance" => {
            if let Some(Value::Table(arr)) = args.first() {
                let numbers: Vec<f64> = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .collect();
                if numbers.is_empty() {
                    Value::Nil
                } else {
                    let mean: f64 = numbers.iter().sum::<f64>() / numbers.len() as f64;
                    let variance = numbers.iter()
                        .map(|x| (x - mean).powi(2))
                        .sum::<f64>() / numbers.len() as f64;
                    Value::Number(variance)
                }
            } else {
                Value::Nil
            }
        }
        "stddev" => {
            if let Some(Value::Table(arr)) = args.first() {
                let numbers: Vec<f64> = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .collect();
                if numbers.is_empty() {
                    Value::Nil
                } else {
                    let mean: f64 = numbers.iter().sum::<f64>() / numbers.len() as f64;
                    let variance = numbers.iter()
                        .map(|x| (x - mean).powi(2))
                        .sum::<f64>() / numbers.len() as f64;
                    Value::Number(variance.sqrt())
                }
            } else {
                Value::Nil
            }
        }
        "prime" => {
            if let Some(Value::Number(n)) = args.first() {
                let n = *n as u64;
                if n < 2 {
                    Value::Boolean(false)
                } else if n == 2 {
                    Value::Boolean(true)
                } else if n % 2 == 0 {
                    Value::Boolean(false)
                } else {
                    let sqrt_n = (n as f64).sqrt() as u64;
                    let mut is_prime = true;
                    for i in (3..=sqrt_n).step_by(2) {
                        if n % i == 0 {
                            is_prime = false;
                            break;
                        }
                    }
                    Value::Boolean(is_prime)
                }
            } else {
                Value::Nil
            }
        }
        "fibonacci" => {
            if let Some(Value::Number(n)) = args.first() {
                let n = *n as usize;
                if n == 0 {
                    Value::Number(0.0)
                } else if n == 1 {
                    Value::Number(1.0)
                } else {
                    let mut a: u64 = 0;
                    let mut b: u64 = 1;
                    for _ in 2..=n {
                        let temp = a + b;
                        a = b;
                        b = temp;
                    }
                    Value::Number(b as f64)
                }
            } else {
                Value::Nil
            }
        }
        "map" => {
            // Map a value from one range to another: map(value, in_min, in_max, out_min, out_max)
            if args.len() >= 5 {
                if let (Some(Value::Number(v)), Some(Value::Number(in_min)), Some(Value::Number(in_max)),
                        Some(Value::Number(out_min)), Some(Value::Number(out_max))) =
                    (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4))
                {
                    let result = (v - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
                    Value::Number(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_string(func: &str, args: Vec<Value>) -> Value {
    match func {
        "length" => {
            if let Some(Value::String(s)) = args.first() {
                Value::Number(s.len() as f64)
            } else {
                Value::Nil
            }
        }
        "upper" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(s.to_uppercase())
            } else {
                Value::Nil
            }
        }
        "lower" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(s.to_lowercase())
            } else {
                Value::Nil
            }
        }
        "trim" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(s.trim().to_string())
            } else {
                Value::Nil
            }
        }
        "substring" => {
            if args.len() >= 3 {
                if let (Some(Value::String(s)), Some(Value::Number(start)), Some(Value::Number(end))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let start = *start as usize;
                    let end = *end as usize;
                    if start <= end && end <= s.len() {
                        Value::String(s[start..end].to_string())
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(start))) = (args.get(0), args.get(1)) {
                    let start = *start as usize;
                    if start <= s.len() {
                        Value::String(s[start..].to_string())
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "split" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(delim))) = (args.get(0), args.get(1)) {
                    let parts: Vec<Value> = s.split(delim.as_str())
                        .map(|p| Value::String(p.to_string()))
                        .collect();
                    Value::Table(parts)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "replace" => {
            if args.len() >= 3 {
                if let (Some(Value::String(s)), Some(Value::String(from)), Some(Value::String(to))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    Value::String(s.replace(from.as_str(), to.as_str()))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "contains" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(sub))) = (args.get(0), args.get(1)) {
                    Value::Boolean(s.contains(sub.as_str()))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "starts_with" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(prefix))) = (args.get(0), args.get(1)) {
                    Value::Boolean(s.starts_with(prefix.as_str()))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "ends_with" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(suffix))) = (args.get(0), args.get(1)) {
                    Value::Boolean(s.ends_with(suffix.as_str()))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "find" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(sub))) = (args.get(0), args.get(1)) {
                    match s.find(sub.as_str()) {
                        Some(pos) => Value::Number(pos as f64),
                        None => Value::Number(-1.0),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "reverse" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(s.chars().rev().collect())
            } else {
                Value::Nil
            }
        }
        "char" => {
            if let Some(Value::Number(n)) = args.first() {
                if let Some(c) = char::from_u32(*n as u32) {
                    Value::String(c.to_string())
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "byte" => {
            if let Some(Value::String(s)) = args.first() {
                if let Some(c) = s.chars().next() {
                    Value::Number(c as u32 as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "repeat" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(count))) = (args.get(0), args.get(1)) {
                    Value::String(s.repeat(*count as usize))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "ltrim" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(s.trim_start().to_string())
            } else {
                Value::Nil
            }
        }
        "rtrim" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(s.trim_end().to_string())
            } else {
                Value::Nil
            }
        }
        "pad_left" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(width))) = (args.get(0), args.get(1)) {
                    let width = *width as usize;
                    let pad_char = if let Some(Value::String(c)) = args.get(2) {
                        c.chars().next().unwrap_or(' ')
                    } else {
                        ' '
                    };
                    if s.len() >= width {
                        Value::String(s.clone())
                    } else {
                        let padding: String = std::iter::repeat(pad_char).take(width - s.len()).collect();
                        Value::String(format!("{}{}", padding, s))
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "pad_right" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(width))) = (args.get(0), args.get(1)) {
                    let width = *width as usize;
                    let pad_char = if let Some(Value::String(c)) = args.get(2) {
                        c.chars().next().unwrap_or(' ')
                    } else {
                        ' '
                    };
                    if s.len() >= width {
                        Value::String(s.clone())
                    } else {
                        let padding: String = std::iter::repeat(pad_char).take(width - s.len()).collect();
                        Value::String(format!("{}{}", s, padding))
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "count" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(sub))) = (args.get(0), args.get(1)) {
                    Value::Number(s.matches(sub.as_str()).count() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "is_empty" => {
            if let Some(Value::String(s)) = args.first() {
                Value::Boolean(s.is_empty())
            } else {
                Value::Nil
            }
        }
        "is_numeric" => {
            if let Some(Value::String(s)) = args.first() {
                Value::Boolean(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-'))
            } else {
                Value::Nil
            }
        }
        "is_alpha" => {
            if let Some(Value::String(s)) = args.first() {
                Value::Boolean(!s.is_empty() && s.chars().all(|c| c.is_alphabetic()))
            } else {
                Value::Nil
            }
        }
        "is_alphanumeric" => {
            if let Some(Value::String(s)) = args.first() {
                Value::Boolean(!s.is_empty() && s.chars().all(|c| c.is_alphanumeric()))
            } else {
                Value::Nil
            }
        }
        "capitalize" => {
            if let Some(Value::String(s)) = args.first() {
                let mut chars = s.chars();
                match chars.next() {
                    None => Value::String(String::new()),
                    Some(first) => {
                        Value::String(first.to_uppercase().collect::<String>() + chars.as_str())
                    }
                }
            } else {
                Value::Nil
            }
        }
        "title" => {
            if let Some(Value::String(s)) = args.first() {
                let result = s.split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                Value::String(result)
            } else {
                Value::Nil
            }
        }
        "lines" => {
            if let Some(Value::String(s)) = args.first() {
                let lines: Vec<Value> = s.lines()
                    .map(|line| Value::String(line.to_string()))
                    .collect();
                Value::Table(lines)
            } else {
                Value::Nil
            }
        }
        "chars" => {
            if let Some(Value::String(s)) = args.first() {
                let chars: Vec<Value> = s.chars()
                    .map(|c| Value::String(c.to_string()))
                    .collect();
                Value::Table(chars)
            } else {
                Value::Nil
            }
        }
        "match" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(pattern))) = (args.get(0), args.get(1)) {
                    // Simple glob-style matching (* matches any chars)
                    let regex_pattern = pattern
                        .replace(".", "\\.")
                        .replace("*", ".*")
                        .replace("?", ".");
                    match regex::Regex::new(&format!("^{}$", regex_pattern)) {
                        Ok(re) => Value::Boolean(re.is_match(s)),
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "replace_first" => {
            if args.len() >= 3 {
                if let (Some(Value::String(s)), Some(Value::String(from)), Some(Value::String(to))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    Value::String(s.replacen(from.as_str(), to.as_str(), 1))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "insert" => {
            if args.len() >= 3 {
                if let (Some(Value::String(s)), Some(Value::Number(pos)), Some(Value::String(ins))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let pos = *pos as usize;
                    if pos <= s.len() {
                        let mut result = s.clone();
                        result.insert_str(pos, ins);
                        Value::String(result)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "remove" => {
            if args.len() >= 3 {
                if let (Some(Value::String(s)), Some(Value::Number(start)), Some(Value::Number(end))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let start = *start as usize;
                    let end = *end as usize;
                    if start <= end && end <= s.len() {
                        let mut result = s.clone();
                        result.replace_range(start..end, "");
                        Value::String(result)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "slug" => {
            if let Some(Value::String(s)) = args.first() {
                let slug: String = s.to_lowercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' { c } else { ' ' })
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join("-");
                Value::String(slug)
            } else {
                Value::Nil
            }
        }
        "truncate" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(max_len))) = (args.get(0), args.get(1)) {
                    let max_len = *max_len as usize;
                    let suffix = args.get(2)
                        .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("...");
                    if s.len() <= max_len {
                        Value::String(s.clone())
                    } else if max_len <= suffix.len() {
                        Value::String(suffix[..max_len].to_string())
                    } else {
                        let end = max_len - suffix.len();
                        Value::String(format!("{}{}", &s[..end], suffix))
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "word_wrap" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(width))) = (args.get(0), args.get(1)) {
                    let width = *width as usize;
                    let mut result = String::new();
                    let mut line_len = 0;
                    for word in s.split_whitespace() {
                        let word_len = word.len();
                        if line_len + word_len > width && line_len > 0 {
                            result.push('\n');
                            line_len = 0;
                        }
                        if line_len > 0 {
                            result.push(' ');
                            line_len += 1;
                        }
                        result.push_str(word);
                        line_len += word_len;
                    }
                    Value::String(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "center" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::Number(width))) = (args.get(0), args.get(1)) {
                    let width = *width as usize;
                    let fill = args.get(2)
                        .and_then(|v| if let Value::String(s) = v { s.chars().next() } else { None })
                        .unwrap_or(' ');
                    if s.len() >= width {
                        Value::String(s.clone())
                    } else {
                        let padding = width - s.len();
                        let left = padding / 2;
                        let right = padding - left;
                        Value::String(format!(
                            "{}{}{}",
                            fill.to_string().repeat(left),
                            s,
                            fill.to_string().repeat(right)
                        ))
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "escape_html" => {
            if let Some(Value::String(s)) = args.first() {
                let escaped = s
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
                    .replace('\'', "&#39;");
                Value::String(escaped)
            } else {
                Value::Nil
            }
        }
        "unescape_html" => {
            if let Some(Value::String(s)) = args.first() {
                let unescaped = s
                    .replace("&amp;", "&")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&quot;", "\"")
                    .replace("&#39;", "'");
                Value::String(unescaped)
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_array(func: &str, args: Vec<Value>) -> Value {
    match func {
        "length" => {
            if let Some(Value::Table(arr)) = args.first() {
                Value::Number(arr.len() as f64)
            } else {
                Value::Nil
            }
        }
        "push" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(val)) = (args.get(0), args.get(1)) {
                    let mut new_arr = arr.clone();
                    new_arr.push(val.clone());
                    Value::Table(new_arr)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "pop" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut new_arr = arr.clone();
                new_arr.pop();
                Value::Table(new_arr)
            } else {
                Value::Nil
            }
        }
        "shift" => {
            if let Some(Value::Table(arr)) = args.first() {
                if arr.is_empty() {
                    Value::Table(Vec::new())
                } else {
                    Value::Table(arr[1..].to_vec())
                }
            } else {
                Value::Nil
            }
        }
        "unshift" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(val)) = (args.get(0), args.get(1)) {
                    let mut new_arr = vec![val.clone()];
                    new_arr.extend(arr.clone());
                    Value::Table(new_arr)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "slice" => {
            if args.len() >= 3 {
                if let (Some(Value::Table(arr)), Some(Value::Number(start)), Some(Value::Number(end))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let start = *start as usize;
                    let end = *end as usize;
                    if start <= end && end <= arr.len() {
                        Value::Table(arr[start..end].to_vec())
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "concat" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let mut new_arr = arr1.clone();
                    new_arr.extend(arr2.clone());
                    Value::Table(new_arr)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "join" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::String(delim))) = (args.get(0), args.get(1)) {
                    let strings: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                    Value::String(strings.join(delim.as_str()))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "reverse" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut new_arr = arr.clone();
                new_arr.reverse();
                Value::Table(new_arr)
            } else {
                Value::Nil
            }
        }
        "sort" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut new_arr = arr.clone();
                new_arr.sort_by(|a, b| {
                    match (a, b) {
                        (Value::Number(x), Value::Number(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
                        (Value::String(x), Value::String(y)) => x.cmp(y),
                        _ => std::cmp::Ordering::Equal,
                    }
                });
                Value::Table(new_arr)
            } else {
                Value::Nil
            }
        }
        "contains" => {
            if args.len() >= 2 {
                if let Some(Value::Table(arr)) = args.get(0) {
                    let search = args.get(1).unwrap();
                    Value::Boolean(arr.iter().any(|v| v == search))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "find" => {
            if args.len() >= 2 {
                if let Some(Value::Table(arr)) = args.get(0) {
                    let search = args.get(1).unwrap();
                    match arr.iter().position(|v| v == search) {
                        Some(pos) => Value::Number(pos as f64),
                        None => Value::Number(-1.0),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "range" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(start)), Some(Value::Number(end))) = (args.get(0), args.get(1)) {
                    let step = if args.len() >= 3 {
                        if let Some(Value::Number(s)) = args.get(2) { *s } else { 1.0 }
                    } else {
                        1.0
                    };
                    let mut arr = Vec::new();
                    let mut i = *start;
                    while i < *end {
                        arr.push(Value::Number(i));
                        i += step;
                    }
                    Value::Table(arr)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "first" => {
            if let Some(Value::Table(arr)) = args.first() {
                arr.first().cloned().unwrap_or(Value::Nil)
            } else {
                Value::Nil
            }
        }
        "last" => {
            if let Some(Value::Table(arr)) = args.first() {
                arr.last().cloned().unwrap_or(Value::Nil)
            } else {
                Value::Nil
            }
        }
        "flatten" => {
            fn flatten_recursive(val: &Value, result: &mut Vec<Value>) {
                match val {
                    Value::Table(arr) => {
                        for item in arr {
                            flatten_recursive(item, result);
                        }
                    }
                    _ => result.push(val.clone()),
                }
            }
            if let Some(Value::Table(arr)) = args.first() {
                let mut result = Vec::new();
                for item in arr {
                    flatten_recursive(item, &mut result);
                }
                Value::Table(result)
            } else {
                Value::Nil
            }
        }
        "unique" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut seen = Vec::new();
                let mut result = Vec::new();
                for item in arr {
                    let key = format!("{:?}", item);
                    if !seen.contains(&key) {
                        seen.push(key);
                        result.push(item.clone());
                    }
                }
                Value::Table(result)
            } else {
                Value::Nil
            }
        }
        "filter_nil" => {
            if let Some(Value::Table(arr)) = args.first() {
                let result: Vec<Value> = arr.iter()
                    .filter(|v| !matches!(v, Value::Nil))
                    .cloned()
                    .collect();
                Value::Table(result)
            } else {
                Value::Nil
            }
        }
        "fill" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(size)), Some(val)) = (args.get(0), args.get(1)) {
                    let size = *size as usize;
                    let result: Vec<Value> = std::iter::repeat(val.clone()).take(size).collect();
                    Value::Table(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "insert" => {
            if args.len() >= 3 {
                if let (Some(Value::Table(arr)), Some(Value::Number(index)), Some(val)) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let index = *index as usize;
                    let mut result = arr.clone();
                    if index <= result.len() {
                        result.insert(index, val.clone());
                        Value::Table(result)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "remove_at" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(index))) = (args.get(0), args.get(1)) {
                    let index = *index as usize;
                    if index < arr.len() {
                        let mut result = arr.clone();
                        result.remove(index);
                        Value::Table(result)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "min" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut min_val: Option<f64> = None;
                for item in arr {
                    if let Value::Number(n) = item {
                        min_val = Some(min_val.map_or(*n, |m| m.min(*n)));
                    }
                }
                min_val.map_or(Value::Nil, Value::Number)
            } else {
                Value::Nil
            }
        }
        "max" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut max_val: Option<f64> = None;
                for item in arr {
                    if let Value::Number(n) = item {
                        max_val = Some(max_val.map_or(*n, |m| m.max(*n)));
                    }
                }
                max_val.map_or(Value::Nil, Value::Number)
            } else {
                Value::Nil
            }
        }
        "sum" => {
            if let Some(Value::Table(arr)) = args.first() {
                let sum: f64 = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .sum();
                Value::Number(sum)
            } else {
                Value::Nil
            }
        }
        "avg" => {
            if let Some(Value::Table(arr)) = args.first() {
                let numbers: Vec<f64> = arr.iter()
                    .filter_map(|v| if let Value::Number(n) = v { Some(*n) } else { None })
                    .collect();
                if numbers.is_empty() {
                    Value::Nil
                } else {
                    let sum: f64 = numbers.iter().sum();
                    Value::Number(sum / numbers.len() as f64)
                }
            } else {
                Value::Nil
            }
        }
        "zip" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let result: Vec<Value> = arr1.iter()
                        .zip(arr2.iter())
                        .map(|(a, b)| Value::Table(vec![a.clone(), b.clone()]))
                        .collect();
                    Value::Table(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "count" => {
            if args.len() >= 2 {
                if let Some(Value::Table(arr)) = args.get(0) {
                    let search = args.get(1).unwrap();
                    let count = arr.iter().filter(|v| *v == search).count();
                    Value::Number(count as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "copy" => {
            if let Some(Value::Table(arr)) = args.first() {
                Value::Table(arr.clone())
            } else {
                Value::Nil
            }
        }
        "clear" => {
            if args.first().map(|v| matches!(v, Value::Table(_))).unwrap_or(false) {
                Value::Table(Vec::new())
            } else {
                Value::Nil
            }
        }
        "swap" => {
            if args.len() >= 3 {
                if let (Some(Value::Table(arr)), Some(Value::Number(i)), Some(Value::Number(j))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let i = *i as usize;
                    let j = *j as usize;
                    if i < arr.len() && j < arr.len() {
                        let mut result = arr.clone();
                        result.swap(i, j);
                        Value::Table(result)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "shuffle" => {
            use rand::seq::SliceRandom;
            if let Some(Value::Table(arr)) = args.first() {
                let mut result = arr.clone();
                let mut rng = rand::thread_rng();
                result.shuffle(&mut rng);
                Value::Table(result)
            } else {
                Value::Nil
            }
        }
        "sample" => {
            use rand::seq::SliceRandom;
            if let Some(Value::Table(arr)) = args.first() {
                let count = args.get(1)
                    .and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None })
                    .unwrap_or(1);
                let mut rng = rand::thread_rng();
                let sampled: Vec<Value> = arr.choose_multiple(&mut rng, count.min(arr.len()))
                    .cloned()
                    .collect();
                if count == 1 && sampled.len() == 1 {
                    sampled.into_iter().next().unwrap_or(Value::Nil)
                } else {
                    Value::Table(sampled)
                }
            } else {
                Value::Nil
            }
        }
        "chunk" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(size))) = (args.get(0), args.get(1)) {
                    let size = *size as usize;
                    if size == 0 {
                        Value::Nil
                    } else {
                        let chunks: Vec<Value> = arr.chunks(size)
                            .map(|chunk| Value::Table(chunk.to_vec()))
                            .collect();
                        Value::Table(chunks)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "partition" => {
            // Partition array at given index: partition(arr, index) -> [left, right]
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(idx))) = (args.get(0), args.get(1)) {
                    let idx = (*idx as usize).min(arr.len());
                    let (left, right) = arr.split_at(idx);
                    Value::Table(vec![
                        Value::Table(left.to_vec()),
                        Value::Table(right.to_vec())
                    ])
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "rotate" => {
            // Rotate array by n positions (positive = left, negative = right)
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(n))) = (args.get(0), args.get(1)) {
                    if arr.is_empty() {
                        Value::Table(arr.clone())
                    } else {
                        let len = arr.len();
                        let n = (*n as i64).rem_euclid(len as i64) as usize;
                        let mut result = arr.clone();
                        result.rotate_left(n);
                        Value::Table(result)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "take" => {
            // Take first n elements
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(n))) = (args.get(0), args.get(1)) {
                    let n = (*n as usize).min(arr.len());
                    Value::Table(arr[..n].to_vec())
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "drop" => {
            // Drop first n elements
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(n))) = (args.get(0), args.get(1)) {
                    let n = (*n as usize).min(arr.len());
                    Value::Table(arr[n..].to_vec())
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "repeat" => {
            // Repeat array n times
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(Value::Number(n))) = (args.get(0), args.get(1)) {
                    let n = *n as usize;
                    let mut result = Vec::with_capacity(arr.len() * n);
                    for _ in 0..n {
                        result.extend(arr.iter().cloned());
                    }
                    Value::Table(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_fs(func: &str, args: Vec<Value>) -> Value {
    match func {
        "read" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::read_to_string(path) {
                    Ok(content) => Value::String(content),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "write" => {
            if args.len() >= 2 {
                if let (Some(Value::String(path)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
                    match fs::write(path, content) {
                        Ok(_) => Value::Boolean(true),
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "append" => {
            if args.len() >= 2 {
                if let (Some(Value::String(path)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
                    match fs::OpenOptions::new().append(true).create(true).open(path) {
                        Ok(mut file) => {
                            match file.write_all(content.as_bytes()) {
                                Ok(_) => Value::Boolean(true),
                                Err(_) => Value::Boolean(false),
                            }
                        }
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "exists" => {
            if let Some(Value::String(path)) = args.first() {
                Value::Boolean(std::path::Path::new(path).exists())
            } else {
                Value::Nil
            }
        }
        "isdir" => {
            if let Some(Value::String(path)) = args.first() {
                Value::Boolean(std::path::Path::new(path).is_dir())
            } else {
                Value::Nil
            }
        }
        "isfile" => {
            if let Some(Value::String(path)) = args.first() {
                Value::Boolean(std::path::Path::new(path).is_file())
            } else {
                Value::Nil
            }
        }
        "readdir" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::read_dir(path) {
                    Ok(entries) => {
                        let files: Vec<Value> = entries
                            .filter_map(|e| e.ok())
                            .map(|e| Value::String(e.file_name().to_string_lossy().to_string()))
                            .collect();
                        Value::Table(files)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "mkdir" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::create_dir_all(path) {
                    Ok(_) => Value::Boolean(true),
                    Err(_) => Value::Boolean(false),
                }
            } else {
                Value::Nil
            }
        }
        "remove" => {
            if let Some(Value::String(path)) = args.first() {
                let path = std::path::Path::new(path);
                let result = if path.is_dir() {
                    fs::remove_dir_all(path)
                } else {
                    fs::remove_file(path)
                };
                match result {
                    Ok(_) => Value::Boolean(true),
                    Err(_) => Value::Boolean(false),
                }
            } else {
                Value::Nil
            }
        }
        "copy" => {
            if args.len() >= 2 {
                if let (Some(Value::String(src)), Some(Value::String(dst))) = (args.get(0), args.get(1)) {
                    match fs::copy(src, dst) {
                        Ok(_) => Value::Boolean(true),
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "rename" => {
            if args.len() >= 2 {
                if let (Some(Value::String(src)), Some(Value::String(dst))) = (args.get(0), args.get(1)) {
                    match fs::rename(src, dst) {
                        Ok(_) => Value::Boolean(true),
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "basename" => {
            if let Some(Value::String(path)) = args.first() {
                let path = std::path::Path::new(path);
                Value::String(path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "dirname" => {
            if let Some(Value::String(path)) = args.first() {
                let path = std::path::Path::new(path);
                Value::String(path.parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "extension" => {
            if let Some(Value::String(path)) = args.first() {
                let path = std::path::Path::new(path);
                Value::String(path.extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "join_path" => {
            if args.len() >= 2 {
                let mut path = std::path::PathBuf::new();
                for arg in &args {
                    if let Value::String(part) = arg {
                        path.push(part);
                    }
                }
                Value::String(path.to_string_lossy().to_string())
            } else {
                Value::Nil
            }
        }
        "absolute" => {
            if let Some(Value::String(path)) = args.first() {
                match std::fs::canonicalize(path) {
                    Ok(abs_path) => Value::String(abs_path.to_string_lossy().to_string()),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "filesize" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::metadata(path) {
                    Ok(meta) => Value::Number(meta.len() as f64),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "modified" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::metadata(path) {
                    Ok(meta) => {
                        match meta.modified() {
                            Ok(time) => {
                                use std::time::UNIX_EPOCH;
                                let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
                                Value::Number(duration.as_secs() as f64)
                            }
                            Err(_) => Value::Nil,
                        }
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "created" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::metadata(path) {
                    Ok(meta) => {
                        match meta.created() {
                            Ok(time) => {
                                use std::time::UNIX_EPOCH;
                                let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
                                Value::Number(duration.as_secs() as f64)
                            }
                            Err(_) => Value::Nil,
                        }
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "is_symlink" => {
            if let Some(Value::String(path)) = args.first() {
                let path = std::path::Path::new(path);
                Value::Boolean(path.is_symlink())
            } else {
                Value::Nil
            }
        }
        "read_bytes" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::read(path) {
                    Ok(bytes) => {
                        let arr: Vec<Value> = bytes.iter().map(|b| Value::Number(*b as f64)).collect();
                        Value::Table(arr)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "write_bytes" => {
            if args.len() >= 2 {
                if let (Some(Value::String(path)), Some(Value::Table(arr))) = (args.get(0), args.get(1)) {
                    let bytes: Vec<u8> = arr.iter()
                        .filter_map(|v| if let Value::Number(n) = v { Some(*n as u8) } else { None })
                        .collect();
                    match fs::write(path, bytes) {
                        Ok(_) => Value::Boolean(true),
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "glob" => {
            if let Some(Value::String(pattern)) = args.first() {
                match glob::glob(pattern) {
                    Ok(paths) => {
                        let result: Vec<Value> = paths
                            .filter_map(|p| p.ok())
                            .map(|p| Value::String(p.to_string_lossy().to_string()))
                            .collect();
                        Value::Table(result)
                    }
                    Err(_) => Value::Table(vec![]),
                }
            } else {
                Value::Nil
            }
        }
        "walk" => {
            if let Some(Value::String(dir)) = args.first() {
                let mut result = Vec::new();
                for entry in walkdir::WalkDir::new(dir)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    result.push(Value::String(entry.path().to_string_lossy().to_string()));
                }
                Value::Table(result)
            } else {
                Value::Nil
            }
        }
        "stat" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::metadata(path) {
                    Ok(meta) => {
                        let mut stat_dict = HashMap::new();
                        stat_dict.insert("size".to_string(), Value::Number(meta.len() as f64));
                        stat_dict.insert("is_dir".to_string(), Value::Boolean(meta.is_dir()));
                        stat_dict.insert("is_file".to_string(), Value::Boolean(meta.is_file()));
                        stat_dict.insert("is_symlink".to_string(), Value::Boolean(meta.is_symlink()));
                        stat_dict.insert("readonly".to_string(), Value::Boolean(meta.permissions().readonly()));
                        if let Ok(modified) = meta.modified() {
                            if let Ok(dur) = modified.duration_since(std::time::UNIX_EPOCH) {
                                stat_dict.insert("modified".to_string(), Value::Number(dur.as_secs() as f64));
                            }
                        }
                        if let Ok(created) = meta.created() {
                            if let Ok(dur) = created.duration_since(std::time::UNIX_EPOCH) {
                                stat_dict.insert("created".to_string(), Value::Number(dur.as_secs() as f64));
                            }
                        }
                        Value::Dictionary(stat_dict)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "temp_file" => {
            let prefix = args.first()
                .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                .unwrap_or("pickup_");
            let temp_dir = std::env::temp_dir();
            let filename = format!("{}{}", prefix, uuid::Uuid::new_v4());
            let path = temp_dir.join(filename);
            Value::String(path.to_string_lossy().to_string())
        }
        "temp_dir" => {
            Value::String(std::env::temp_dir().to_string_lossy().to_string())
        }
        "read_lines" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        let lines: Vec<Value> = content.lines()
                            .map(|l| Value::String(l.to_string()))
                            .collect();
                        Value::Table(lines)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "touch" => {
            if let Some(Value::String(path)) = args.first() {
                let path = std::path::Path::new(path);
                if path.exists() {
                    // Update modification time
                    match fs::OpenOptions::new().write(true).open(path) {
                        Ok(file) => {
                            match file.set_len(file.metadata().map(|m| m.len()).unwrap_or(0)) {
                                Ok(_) => Value::Boolean(true),
                                Err(_) => Value::Boolean(false),
                            }
                        }
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    // Create the file
                    match fs::File::create(path) {
                        Ok(_) => Value::Boolean(true),
                        Err(_) => Value::Boolean(false),
                    }
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_json(func: &str, args: Vec<Value>) -> Value {
    match func {
        "parse" => {
            if let Some(Value::String(s)) = args.first() {
                match serde_json::from_str::<serde_json::Value>(s) {
                    Ok(json) => json_to_value(json),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "stringify" => {
            if let Some(val) = args.first() {
                let json = value_to_json(val);
                Value::String(json.to_string())
            } else {
                Value::Nil
            }
        }
        "pretty" => {
            if let Some(val) = args.first() {
                let json = value_to_json(val);
                match serde_json::to_string_pretty(&json) {
                    Ok(s) => Value::String(s),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "valid" => {
            if let Some(Value::String(s)) = args.first() {
                let is_valid = serde_json::from_str::<serde_json::Value>(s).is_ok();
                Value::Boolean(is_valid)
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn json_to_value(json: serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Table(arr.into_iter().map(json_to_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let map: HashMap<String, Value> = obj.into_iter()
                .map(|(k, v)| (k, json_to_value(v)))
                .collect();
            Value::Dictionary(map)
        }
    }
}

fn value_to_json(val: &Value) -> serde_json::Value {
    match val {
        Value::Nil => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => serde_json::json!(*n),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Table(arr) => {
            serde_json::Value::Array(arr.iter().map(value_to_json).collect())
        }
        Value::Dictionary(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
        Value::Function(_, _) => serde_json::Value::Null,
        Value::NativeFunction(_, _) => serde_json::Value::Null,
        Value::Module(_) => serde_json::Value::Null,
    }
}

fn call_type(func: &str, args: Vec<Value>) -> Value {
    match func {
        "typeof" => {
            if let Some(val) = args.first() {
                let type_name = match val {
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Boolean(_) => "boolean",
                    Value::Table(_) => "table",
                    Value::Dictionary(_) => "dictionary",
                    Value::Function(_, _) => "function",
                    Value::NativeFunction(_, _) => "function",
                    Value::Module(_) => "module",
                    Value::Nil => "nil",
                };
                Value::String(type_name.to_string())
            } else {
                Value::Nil
            }
        }
        "tonumber" => {
            if let Some(val) = args.first() {
                match val {
                    Value::Number(n) => Value::Number(*n),
                    Value::String(s) => {
                        match s.parse::<f64>() {
                            Ok(n) => Value::Number(n),
                            Err(_) => Value::Nil,
                        }
                    }
                    Value::Boolean(b) => Value::Number(if *b { 1.0 } else { 0.0 }),
                    _ => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "tostring" => {
            if let Some(val) = args.first() {
                Value::String(val.to_string())
            } else {
                Value::Nil
            }
        }
        "tobool" => {
            if let Some(val) = args.first() {
                Value::Boolean(val.is_truthy())
            } else {
                Value::Nil
            }
        }
        "isnil" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Nil))
            } else {
                Value::Boolean(true)
            }
        }
        "isnumber" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Number(_)))
            } else {
                Value::Nil
            }
        }
        "isstring" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::String(_)))
            } else {
                Value::Nil
            }
        }
        "isbool" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Boolean(_)))
            } else {
                Value::Nil
            }
        }
        "istable" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Table(_)))
            } else {
                Value::Nil
            }
        }
        "isfunction" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Function(_, _) | Value::NativeFunction(_, _)))
            } else {
                Value::Nil
            }
        }
        "isdict" | "isdictionary" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Dictionary(_)))
            } else {
                Value::Nil
            }
        }
        "isarray" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Table(_)))
            } else {
                Value::Nil
            }
        }
        "ismodule" => {
            if let Some(val) = args.first() {
                Value::Boolean(matches!(val, Value::Module(_)))
            } else {
                Value::Nil
            }
        }
        "isempty" => {
            if let Some(val) = args.first() {
                let empty = match val {
                    Value::String(s) => s.is_empty(),
                    Value::Table(arr) => arr.is_empty(),
                    Value::Dictionary(dict) => dict.is_empty(),
                    Value::Nil => true,
                    _ => false,
                };
                Value::Boolean(empty)
            } else {
                Value::Nil
            }
        }
        "default" => {
            if args.len() >= 2 {
                if let Some(val) = args.get(0) {
                    let default_val = args.get(1).cloned().unwrap_or(Value::Nil);
                    if matches!(val, Value::Nil) {
                        default_val
                    } else {
                        val.clone()
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_os(func: &str, args: Vec<Value>) -> Value {
    match func {
        "time" => {
            use std::time::{SystemTime, UNIX_EPOCH};
            let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            Value::Number(duration.as_secs() as f64)
        }
        "clock" => {
            use std::time::{SystemTime, UNIX_EPOCH};
            let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            Value::Number(duration.as_secs_f64())
        }
        "exit" => {
            let code = if let Some(Value::Number(n)) = args.first() {
                *n as i32
            } else {
                0
            };
            std::process::exit(code);
        }
        "getenv" => {
            if let Some(Value::String(name)) = args.first() {
                match std::env::var(name) {
                    Ok(val) => Value::String(val),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "execute" => {
            if let Some(Value::String(cmd)) = args.first() {
                match std::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        Value::String(stdout)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "sleep" => {
            if let Some(Value::Number(ms)) = args.first() {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                Value::Nil
            } else {
                Value::Nil
            }
        }
        "platform" => {
            Value::String(std::env::consts::OS.to_string())
        }
        "arch" => {
            Value::String(std::env::consts::ARCH.to_string())
        }
        "hostname" => {
            match std::process::Command::new("hostname").output() {
                Ok(output) => {
                    let hostname = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string();
                    Value::String(hostname)
                }
                Err(_) => Value::Nil,
            }
        }
        "cwd" => {
            match std::env::current_dir() {
                Ok(path) => Value::String(path.to_string_lossy().to_string()),
                Err(_) => Value::Nil,
            }
        }
        "chdir" => {
            if let Some(Value::String(path)) = args.first() {
                match std::env::set_current_dir(path) {
                    Ok(_) => Value::Boolean(true),
                    Err(_) => Value::Boolean(false),
                }
            } else {
                Value::Nil
            }
        }
        "home" => {
            match std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                Ok(home) => Value::String(home),
                Err(_) => Value::Nil,
            }
        }
        "setenv" => {
            if args.len() >= 2 {
                if let (Some(Value::String(name)), Some(Value::String(value))) = (args.get(0), args.get(1)) {
                    std::env::set_var(name, value);
                    Value::Boolean(true)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "unsetenv" => {
            if let Some(Value::String(name)) = args.first() {
                std::env::remove_var(name);
                Value::Boolean(true)
            } else {
                Value::Nil
            }
        }
        "envvars" => {
            let env_map: HashMap<String, Value> = std::env::vars()
                .map(|(k, v)| (k, Value::String(v)))
                .collect();
            Value::Dictionary(env_map)
        }
        "tmpdir" => {
            Value::String(std::env::temp_dir().to_string_lossy().to_string())
        }
        "pid" => {
            Value::Number(std::process::id() as f64)
        }
        "args" => {
            let args: Vec<Value> = std::env::args()
                .map(Value::String)
                .collect();
            Value::Table(args)
        }
        "user" => {
            match std::env::var("USER").or_else(|_| std::env::var("USERNAME")) {
                Ok(user) => Value::String(user),
                Err(_) => Value::Nil,
            }
        }
        "cpus" => {
            Value::Number(std::thread::available_parallelism()
                .map(|p| p.get() as f64)
                .unwrap_or(1.0))
        }
        "version" => {
            Value::String(std::env::consts::OS.to_string())
        }
        "family" => {
            Value::String(std::env::consts::FAMILY.to_string())
        }
        "shell" => {
            let shell = if cfg!(target_os = "windows") {
                std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
            } else {
                std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
            };
            Value::String(shell)
        }
        "spawn" => {
            // Spawn a process in background
            if let Some(Value::String(cmd)) = args.first() {
                use std::process::Command;
                let args_vec: Vec<String> = args.iter().skip(1)
                    .filter_map(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                    .collect();

                let result = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", cmd])
                        .args(&args_vec)
                        .spawn()
                } else {
                    Command::new("sh")
                        .args(["-c", cmd])
                        .args(&args_vec)
                        .spawn()
                };

                match result {
                    Ok(child) => Value::Number(child.id() as f64),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "which" => {
            if let Some(Value::String(cmd)) = args.first() {
                let path_var = std::env::var("PATH").unwrap_or_default();
                let separator = if cfg!(target_os = "windows") { ";" } else { ":" };
                for path in path_var.split(separator) {
                    let full_path = std::path::Path::new(path).join(cmd);
                    if full_path.exists() {
                        return Value::String(full_path.to_string_lossy().to_string());
                    }
                    // On Windows, try with .exe extension
                    if cfg!(target_os = "windows") {
                        let exe_path = std::path::Path::new(path).join(format!("{}.exe", cmd));
                        if exe_path.exists() {
                            return Value::String(exe_path.to_string_lossy().to_string());
                        }
                    }
                }
                Value::Nil
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_table(func: &str, args: Vec<Value>) -> Value {
    match func {
        "keys" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                let keys: Vec<Value> = dict.keys()
                    .map(|k| Value::String(k.clone()))
                    .collect();
                Value::Table(keys)
            } else {
                Value::Nil
            }
        }
        "values" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                let values: Vec<Value> = dict.values().cloned().collect();
                Value::Table(values)
            } else {
                Value::Nil
            }
        }
        "entries" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                let entries: Vec<Value> = dict.iter()
                    .map(|(k, v)| Value::Table(vec![Value::String(k.clone()), v.clone()]))
                    .collect();
                Value::Table(entries)
            } else {
                Value::Nil
            }
        }
        "has" => {
            if args.len() >= 2 {
                if let (Some(Value::Dictionary(dict)), Some(Value::String(key))) = (args.get(0), args.get(1)) {
                    Value::Boolean(dict.contains_key(key))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "get" => {
            if args.len() >= 2 {
                if let (Some(Value::Dictionary(dict)), Some(Value::String(key))) = (args.get(0), args.get(1)) {
                    let default = args.get(2).cloned().unwrap_or(Value::Nil);
                    dict.get(key).cloned().unwrap_or(default)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "set" => {
            if args.len() >= 3 {
                if let (Some(Value::Dictionary(dict)), Some(Value::String(key)), Some(val)) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let mut new_dict = dict.clone();
                    new_dict.insert(key.clone(), val.clone());
                    Value::Dictionary(new_dict)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "delete" => {
            if args.len() >= 2 {
                if let (Some(Value::Dictionary(dict)), Some(Value::String(key))) = (args.get(0), args.get(1)) {
                    let mut new_dict = dict.clone();
                    new_dict.remove(key);
                    Value::Dictionary(new_dict)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "merge" => {
            if args.len() >= 2 {
                if let (Some(Value::Dictionary(dict1)), Some(Value::Dictionary(dict2))) = (args.get(0), args.get(1)) {
                    let mut new_dict = dict1.clone();
                    for (k, v) in dict2 {
                        new_dict.insert(k.clone(), v.clone());
                    }
                    Value::Dictionary(new_dict)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "size" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                Value::Number(dict.len() as f64)
            } else {
                Value::Nil
            }
        }
        "copy" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                Value::Dictionary(dict.clone())
            } else {
                Value::Nil
            }
        }
        "clear" => {
            if args.first().map(|v| matches!(v, Value::Dictionary(_))).unwrap_or(false) {
                Value::Dictionary(HashMap::new())
            } else {
                Value::Nil
            }
        }
        "from_entries" => {
            if let Some(Value::Table(arr)) = args.first() {
                let mut dict = HashMap::new();
                for entry in arr {
                    if let Value::Table(pair) = entry {
                        if pair.len() >= 2 {
                            if let Value::String(key) = &pair[0] {
                                dict.insert(key.clone(), pair[1].clone());
                            }
                        }
                    }
                }
                Value::Dictionary(dict)
            } else {
                Value::Nil
            }
        }
        "invert" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                let mut new_dict = HashMap::new();
                for (k, v) in dict {
                    if let Value::String(val_str) = v {
                        new_dict.insert(val_str.clone(), Value::String(k.clone()));
                    }
                }
                Value::Dictionary(new_dict)
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_base64(func: &str, args: Vec<Value>) -> Value {
    use base64::{Engine as _, engine::general_purpose};

    match func {
        "encode" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(general_purpose::STANDARD.encode(s.as_bytes()))
            } else {
                Value::Nil
            }
        }
        "decode" => {
            if let Some(Value::String(s)) = args.first() {
                match general_purpose::STANDARD.decode(s) {
                    Ok(bytes) => {
                        match String::from_utf8(bytes) {
                            Ok(decoded) => Value::String(decoded),
                            Err(_) => Value::Nil,
                        }
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_datetime(func: &str, args: Vec<Value>) -> Value {
    use chrono::{DateTime, Local, NaiveDateTime, Datelike, Timelike, Duration};

    match func {
        "now" => {
            let now = Local::now();
            Value::Number(now.timestamp() as f64)
        }
        "timestamp" => {
            // Same as now, returns current Unix timestamp
            let now = Local::now();
            Value::Number(now.timestamp() as f64)
        }
        "format" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(ts)), Some(Value::String(fmt))) = (args.get(0), args.get(1)) {
                    let datetime = DateTime::from_timestamp(*ts as i64, 0)
                        .map(|d| d.with_timezone(&Local));
                    if let Some(dt) = datetime {
                        Value::String(dt.format(fmt).to_string())
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else if let Some(Value::Number(ts)) = args.first() {
                // Default format
                let datetime = DateTime::from_timestamp(*ts as i64, 0)
                    .map(|d| d.with_timezone(&Local));
                if let Some(dt) = datetime {
                    Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "parse" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(fmt))) = (args.get(0), args.get(1)) {
                    match NaiveDateTime::parse_from_str(s, fmt) {
                        Ok(dt) => Value::Number(dt.and_utc().timestamp() as f64),
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "year" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.year() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "month" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.month() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "day" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.day() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "hour" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.hour() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "minute" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.minute() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "second" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.second() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "weekday" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    // 0 = Sunday, 1 = Monday, etc. (matching common conventions)
                    let weekday = dt.weekday().num_days_from_sunday();
                    Value::Number(weekday as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "day_of_year" => {
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    Value::Number(dt.ordinal() as f64)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "is_leap_year" => {
            if let Some(Value::Number(year)) = args.first() {
                let year = *year as i32;
                let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
                Value::Boolean(is_leap)
            } else {
                Value::Nil
            }
        }
        "add_days" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(ts)), Some(Value::Number(days))) = (args.get(0), args.get(1)) {
                    let datetime = DateTime::from_timestamp(*ts as i64, 0);
                    if let Some(dt) = datetime {
                        let new_dt = dt + Duration::days(*days as i64);
                        Value::Number(new_dt.timestamp() as f64)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "add_hours" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(ts)), Some(Value::Number(hours))) = (args.get(0), args.get(1)) {
                    let datetime = DateTime::from_timestamp(*ts as i64, 0);
                    if let Some(dt) = datetime {
                        let new_dt = dt + Duration::hours(*hours as i64);
                        Value::Number(new_dt.timestamp() as f64)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "add_minutes" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(ts)), Some(Value::Number(minutes))) = (args.get(0), args.get(1)) {
                    let datetime = DateTime::from_timestamp(*ts as i64, 0);
                    if let Some(dt) = datetime {
                        let new_dt = dt + Duration::minutes(*minutes as i64);
                        Value::Number(new_dt.timestamp() as f64)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "add_seconds" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(ts)), Some(Value::Number(seconds))) = (args.get(0), args.get(1)) {
                    let datetime = DateTime::from_timestamp(*ts as i64, 0);
                    if let Some(dt) = datetime {
                        let new_dt = dt + Duration::seconds(*seconds as i64);
                        Value::Number(new_dt.timestamp() as f64)
                    } else {
                        Value::Nil
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "diff" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(ts1)), Some(Value::Number(ts2))) = (args.get(0), args.get(1)) {
                    // Returns difference in seconds
                    Value::Number(ts1 - ts2)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "from_timestamp" => {
            // Convert timestamp to a dictionary with date components
            if let Some(Value::Number(ts)) = args.first() {
                let datetime = DateTime::from_timestamp(*ts as i64, 0);
                if let Some(dt) = datetime {
                    let mut dict = HashMap::new();
                    dict.insert("year".to_string(), Value::Number(dt.year() as f64));
                    dict.insert("month".to_string(), Value::Number(dt.month() as f64));
                    dict.insert("day".to_string(), Value::Number(dt.day() as f64));
                    dict.insert("hour".to_string(), Value::Number(dt.hour() as f64));
                    dict.insert("minute".to_string(), Value::Number(dt.minute() as f64));
                    dict.insert("second".to_string(), Value::Number(dt.second() as f64));
                    dict.insert("weekday".to_string(), Value::Number(dt.weekday().num_days_from_sunday() as f64));
                    dict.insert("day_of_year".to_string(), Value::Number(dt.ordinal() as f64));
                    Value::Dictionary(dict)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_regex(func: &str, args: Vec<Value>) -> Value {
    use regex::Regex;

    match func {
        "match" => {
            if args.len() >= 2 {
                if let (Some(Value::String(pattern)), Some(Value::String(text))) = (args.get(0), args.get(1)) {
                    match Regex::new(pattern) {
                        Ok(re) => Value::Boolean(re.is_match(text)),
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "find" => {
            if args.len() >= 2 {
                if let (Some(Value::String(pattern)), Some(Value::String(text))) = (args.get(0), args.get(1)) {
                    match Regex::new(pattern) {
                        Ok(re) => {
                            if let Some(m) = re.find(text) {
                                let mut dict = HashMap::new();
                                dict.insert("text".to_string(), Value::String(m.as_str().to_string()));
                                dict.insert("start".to_string(), Value::Number(m.start() as f64));
                                dict.insert("end".to_string(), Value::Number(m.end() as f64));
                                Value::Dictionary(dict)
                            } else {
                                Value::Nil
                            }
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "find_all" => {
            if args.len() >= 2 {
                if let (Some(Value::String(pattern)), Some(Value::String(text))) = (args.get(0), args.get(1)) {
                    match Regex::new(pattern) {
                        Ok(re) => {
                            let matches: Vec<Value> = re.find_iter(text)
                                .map(|m| {
                                    let mut dict = HashMap::new();
                                    dict.insert("text".to_string(), Value::String(m.as_str().to_string()));
                                    dict.insert("start".to_string(), Value::Number(m.start() as f64));
                                    dict.insert("end".to_string(), Value::Number(m.end() as f64));
                                    Value::Dictionary(dict)
                                })
                                .collect();
                            Value::Table(matches)
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "replace" => {
            if args.len() >= 3 {
                if let (Some(Value::String(pattern)), Some(Value::String(text)), Some(Value::String(replacement))) =
                    (args.get(0), args.get(1), args.get(2)) {
                    match Regex::new(pattern) {
                        Ok(re) => Value::String(re.replace(text, replacement.as_str()).to_string()),
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "replace_all" => {
            if args.len() >= 3 {
                if let (Some(Value::String(pattern)), Some(Value::String(text)), Some(Value::String(replacement))) =
                    (args.get(0), args.get(1), args.get(2)) {
                    match Regex::new(pattern) {
                        Ok(re) => Value::String(re.replace_all(text, replacement.as_str()).to_string()),
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "split" => {
            if args.len() >= 2 {
                if let (Some(Value::String(pattern)), Some(Value::String(text))) = (args.get(0), args.get(1)) {
                    match Regex::new(pattern) {
                        Ok(re) => {
                            let parts: Vec<Value> = re.split(text)
                                .map(|s| Value::String(s.to_string()))
                                .collect();
                            Value::Table(parts)
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "escape" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(regex::escape(s))
            } else {
                Value::Nil
            }
        }
        "is_valid" => {
            if let Some(Value::String(pattern)) = args.first() {
                Value::Boolean(Regex::new(pattern).is_ok())
            } else {
                Value::Nil
            }
        }
        "captures" => {
            if args.len() >= 2 {
                if let (Some(Value::String(pattern)), Some(Value::String(text))) = (args.get(0), args.get(1)) {
                    match Regex::new(pattern) {
                        Ok(re) => {
                            if let Some(caps) = re.captures(text) {
                                let groups: Vec<Value> = caps.iter()
                                    .map(|m| m.map(|m| Value::String(m.as_str().to_string())).unwrap_or(Value::Nil))
                                    .collect();
                                Value::Table(groups)
                            } else {
                                Value::Nil
                            }
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_crypto(func: &str, args: Vec<Value>) -> Value {
    use sha2::{Sha256, Sha512, Digest};
    use md5::Md5;
    use sha1::Sha1;
    use hmac::{Hmac, Mac};
    use rand::Rng;

    match func {
        "md5" => {
            if let Some(Value::String(s)) = args.first() {
                let result = Md5::digest(s.as_bytes());
                Value::String(format!("{:x}", result))
            } else {
                Value::Nil
            }
        }
        "sha1" => {
            if let Some(Value::String(s)) = args.first() {
                let result = Sha1::digest(s.as_bytes());
                Value::String(format!("{:x}", result))
            } else {
                Value::Nil
            }
        }
        "sha256" => {
            if let Some(Value::String(s)) = args.first() {
                let result = Sha256::digest(s.as_bytes());
                Value::String(format!("{:x}", result))
            } else {
                Value::Nil
            }
        }
        "sha512" => {
            if let Some(Value::String(s)) = args.first() {
                let result = Sha512::digest(s.as_bytes());
                Value::String(format!("{:x}", result))
            } else {
                Value::Nil
            }
        }
        "hmac_sha256" => {
            if args.len() >= 2 {
                if let (Some(Value::String(key)), Some(Value::String(msg))) = (args.get(0), args.get(1)) {
                    type HmacSha256 = Hmac<Sha256>;
                    match HmacSha256::new_from_slice(key.as_bytes()) {
                        Ok(mut mac) => {
                            mac.update(msg.as_bytes());
                            let result = mac.finalize();
                            Value::String(format!("{:x}", result.into_bytes()))
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "uuid" | "uuid_v4" => {
            Value::String(uuid::Uuid::new_v4().to_string())
        }
        "random_bytes" => {
            if let Some(Value::Number(n)) = args.first() {
                let n = *n as usize;
                let mut rng = rand::thread_rng();
                let bytes: Vec<Value> = (0..n).map(|_| Value::Number(rng.gen::<u8>() as f64)).collect();
                Value::Table(bytes)
            } else {
                Value::Nil
            }
        }
        "random_hex" => {
            if let Some(Value::Number(n)) = args.first() {
                let n = *n as usize;
                let mut rng = rand::thread_rng();
                let bytes: Vec<u8> = (0..n).map(|_| rng.gen::<u8>()).collect();
                let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
                Value::String(hex)
            } else {
                Value::Nil
            }
        }
        "hash" => {
            if args.len() >= 2 {
                if let (Some(Value::String(algo)), Some(Value::String(data))) = (args.get(0), args.get(1)) {
                    match algo.to_lowercase().as_str() {
                        "md5" => {
                            let result = Md5::digest(data.as_bytes());
                            Value::String(format!("{:x}", result))
                        }
                        "sha1" => {
                            let result = Sha1::digest(data.as_bytes());
                            Value::String(format!("{:x}", result))
                        }
                        "sha256" => {
                            let result = Sha256::digest(data.as_bytes());
                            Value::String(format!("{:x}", result))
                        }
                        "sha512" => {
                            let result = Sha512::digest(data.as_bytes());
                            Value::String(format!("{:x}", result))
                        }
                        _ => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_http(func: &str, args: Vec<Value>) -> Value {
    match func {
        "get" => {
            if let Some(Value::String(url)) = args.first() {
                match ureq::get(url).call() {
                    Ok(response) => {
                        let status = response.status();
                        match response.into_string() {
                            Ok(body) => {
                                let mut dict = HashMap::new();
                                dict.insert("status".to_string(), Value::Number(status as f64));
                                dict.insert("body".to_string(), Value::String(body));
                                dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                                Value::Dictionary(dict)
                            }
                            Err(_) => Value::Nil,
                        }
                    }
                    Err(ureq::Error::Status(code, response)) => {
                        let body = response.into_string().unwrap_or_default();
                        let mut dict = HashMap::new();
                        dict.insert("status".to_string(), Value::Number(code as f64));
                        dict.insert("body".to_string(), Value::String(body));
                        dict.insert("ok".to_string(), Value::Boolean(false));
                        Value::Dictionary(dict)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "post" => {
            if args.len() >= 2 {
                if let (Some(Value::String(url)), Some(body)) = (args.get(0), args.get(1)) {
                    let body_str = match body {
                        Value::String(s) => s.clone(),
                        Value::Dictionary(_) | Value::Table(_) => {
                            let json = value_to_json(body);
                            json.to_string()
                        }
                        _ => body.to_string(),
                    };

                    let request = ureq::post(url).set("Content-Type", "application/json");
                    match request.send_string(&body_str) {
                        Ok(response) => {
                            let status = response.status();
                            match response.into_string() {
                                Ok(resp_body) => {
                                    let mut dict = HashMap::new();
                                    dict.insert("status".to_string(), Value::Number(status as f64));
                                    dict.insert("body".to_string(), Value::String(resp_body));
                                    dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                                    Value::Dictionary(dict)
                                }
                                Err(_) => Value::Nil,
                            }
                        }
                        Err(ureq::Error::Status(code, response)) => {
                            let resp_body = response.into_string().unwrap_or_default();
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(code as f64));
                            dict.insert("body".to_string(), Value::String(resp_body));
                            dict.insert("ok".to_string(), Value::Boolean(false));
                            Value::Dictionary(dict)
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "put" => {
            if args.len() >= 2 {
                if let (Some(Value::String(url)), Some(body)) = (args.get(0), args.get(1)) {
                    let body_str = match body {
                        Value::String(s) => s.clone(),
                        Value::Dictionary(_) | Value::Table(_) => {
                            let json = value_to_json(body);
                            json.to_string()
                        }
                        _ => body.to_string(),
                    };

                    match ureq::put(url).set("Content-Type", "application/json").send_string(&body_str) {
                        Ok(response) => {
                            let status = response.status();
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(status as f64));
                            dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                            dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                            Value::Dictionary(dict)
                        }
                        Err(ureq::Error::Status(code, response)) => {
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(code as f64));
                            dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                            dict.insert("ok".to_string(), Value::Boolean(false));
                            Value::Dictionary(dict)
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "delete" => {
            if let Some(Value::String(url)) = args.first() {
                match ureq::delete(url).call() {
                    Ok(response) => {
                        let status = response.status();
                        let mut dict = HashMap::new();
                        dict.insert("status".to_string(), Value::Number(status as f64));
                        dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                        dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                        Value::Dictionary(dict)
                    }
                    Err(ureq::Error::Status(code, response)) => {
                        let mut dict = HashMap::new();
                        dict.insert("status".to_string(), Value::Number(code as f64));
                        dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                        dict.insert("ok".to_string(), Value::Boolean(false));
                        Value::Dictionary(dict)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "patch" => {
            if args.len() >= 2 {
                if let (Some(Value::String(url)), Some(body)) = (args.get(0), args.get(1)) {
                    let body_str = match body {
                        Value::String(s) => s.clone(),
                        Value::Dictionary(_) | Value::Table(_) => {
                            let json = value_to_json(body);
                            json.to_string()
                        }
                        _ => body.to_string(),
                    };

                    match ureq::request("PATCH", url).set("Content-Type", "application/json").send_string(&body_str) {
                        Ok(response) => {
                            let status = response.status();
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(status as f64));
                            dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                            dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                            Value::Dictionary(dict)
                        }
                        Err(ureq::Error::Status(code, response)) => {
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(code as f64));
                            dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                            dict.insert("ok".to_string(), Value::Boolean(false));
                            Value::Dictionary(dict)
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "head" => {
            if let Some(Value::String(url)) = args.first() {
                match ureq::head(url).call() {
                    Ok(response) => {
                        let status = response.status();
                        let mut dict = HashMap::new();
                        dict.insert("status".to_string(), Value::Number(status as f64));
                        dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                        Value::Dictionary(dict)
                    }
                    Err(ureq::Error::Status(code, _)) => {
                        let mut dict = HashMap::new();
                        dict.insert("status".to_string(), Value::Number(code as f64));
                        dict.insert("ok".to_string(), Value::Boolean(false));
                        Value::Dictionary(dict)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "request" => {
            // request(method, url, options)
            if args.len() >= 2 {
                if let (Some(Value::String(method)), Some(Value::String(url))) = (args.get(0), args.get(1)) {
                    let mut request = ureq::request(method.to_uppercase().as_str(), url);

                    // Handle options dictionary if provided
                    if let Some(Value::Dictionary(opts)) = args.get(2) {
                        if let Some(Value::Dictionary(headers)) = opts.get("headers") {
                            for (key, value) in headers {
                                if let Value::String(v) = value {
                                    request = request.set(key, v);
                                }
                            }
                        }
                    }

                    let body = args.get(2)
                        .and_then(|opts| {
                            if let Value::Dictionary(d) = opts {
                                d.get("body").cloned()
                            } else {
                                None
                            }
                        });

                    let result = if let Some(b) = body {
                        let body_str = match b {
                            Value::String(s) => s,
                            _ => value_to_json(&b).to_string(),
                        };
                        request.send_string(&body_str)
                    } else {
                        request.call()
                    };

                    match result {
                        Ok(response) => {
                            let status = response.status();
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(status as f64));
                            dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                            dict.insert("ok".to_string(), Value::Boolean(status >= 200 && status < 300));
                            Value::Dictionary(dict)
                        }
                        Err(ureq::Error::Status(code, response)) => {
                            let mut dict = HashMap::new();
                            dict.insert("status".to_string(), Value::Number(code as f64));
                            dict.insert("body".to_string(), Value::String(response.into_string().unwrap_or_default()));
                            dict.insert("ok".to_string(), Value::Boolean(false));
                            Value::Dictionary(dict)
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "download" => {
            if args.len() >= 2 {
                if let (Some(Value::String(url)), Some(Value::String(path))) = (args.get(0), args.get(1)) {
                    match ureq::get(url).call() {
                        Ok(response) => {
                            let mut bytes = Vec::new();
                            if response.into_reader().read_to_end(&mut bytes).is_ok() {
                                match fs::write(path, &bytes) {
                                    Ok(_) => Value::Boolean(true),
                                    Err(_) => Value::Boolean(false),
                                }
                            } else {
                                Value::Boolean(false)
                            }
                        }
                        Err(_) => Value::Boolean(false),
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_csv(func: &str, args: Vec<Value>) -> Value {
    match func {
        "parse" => {
            if let Some(Value::String(s)) = args.first() {
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .from_reader(s.as_bytes());

                let headers: Vec<String> = match reader.headers() {
                    Ok(h) => h.iter().map(|s| s.to_string()).collect(),
                    Err(_) => return Value::Nil,
                };

                let mut rows: Vec<Value> = Vec::new();
                for result in reader.records() {
                    if let Ok(record) = result {
                        let mut row = HashMap::new();
                        for (i, field) in record.iter().enumerate() {
                            if let Some(header) = headers.get(i) {
                                row.insert(header.clone(), Value::String(field.to_string()));
                            }
                        }
                        rows.push(Value::Dictionary(row));
                    }
                }
                Value::Table(rows)
            } else {
                Value::Nil
            }
        }
        "stringify" => {
            if let Some(Value::Table(rows)) = args.first() {
                let mut wtr = csv::Writer::from_writer(vec![]);

                // Collect all headers from all rows
                let mut all_headers: Vec<String> = Vec::new();
                for row in rows {
                    if let Value::Dictionary(dict) = row {
                        for key in dict.keys() {
                            if !all_headers.contains(key) {
                                all_headers.push(key.clone());
                            }
                        }
                    }
                }
                all_headers.sort();

                // Write headers
                let _ = wtr.write_record(&all_headers);

                // Write rows
                for row in rows {
                    if let Value::Dictionary(dict) = row {
                        let record: Vec<String> = all_headers.iter()
                            .map(|h| {
                                dict.get(h)
                                    .map(|v| match v {
                                        Value::String(s) => s.clone(),
                                        Value::Number(n) => n.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        Value::Nil => String::new(),
                                        _ => v.to_string(),
                                    })
                                    .unwrap_or_default()
                            })
                            .collect();
                        let _ = wtr.write_record(&record);
                    }
                }

                match wtr.into_inner() {
                    Ok(bytes) => Value::String(String::from_utf8_lossy(&bytes).to_string()),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "read" => {
            if let Some(Value::String(path)) = args.first() {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        // Reuse parse logic
                        call_csv("parse", vec![Value::String(content)])
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "write" => {
            if args.len() >= 2 {
                if let (Some(Value::String(path)), Some(data)) = (args.get(0), args.get(1)) {
                    let csv_str = call_csv("stringify", vec![data.clone()]);
                    if let Value::String(content) = csv_str {
                        match fs::write(path, content) {
                            Ok(_) => Value::Boolean(true),
                            Err(_) => Value::Boolean(false),
                        }
                    } else {
                        Value::Boolean(false)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "parse_row" => {
            if let Some(Value::String(s)) = args.first() {
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(s.as_bytes());

                if let Some(Ok(record)) = reader.records().next() {
                    let fields: Vec<Value> = record.iter()
                        .map(|s| Value::String(s.to_string()))
                        .collect();
                    Value::Table(fields)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "stringify_row" => {
            if let Some(Value::Table(fields)) = args.first() {
                let mut wtr = csv::Writer::from_writer(vec![]);
                let record: Vec<String> = fields.iter()
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        Value::Nil => String::new(),
                        _ => v.to_string(),
                    })
                    .collect();
                let _ = wtr.write_record(&record);
                match wtr.into_inner() {
                    Ok(bytes) => Value::String(String::from_utf8_lossy(&bytes).trim().to_string()),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_path(func: &str, args: Vec<Value>) -> Value {
    use std::path::{Path, PathBuf, MAIN_SEPARATOR};

    match func {
        "join" => {
            let mut path = PathBuf::new();
            for arg in &args {
                if let Value::String(part) = arg {
                    path.push(part);
                }
            }
            Value::String(path.to_string_lossy().to_string())
        }
        "basename" => {
            if let Some(Value::String(p)) = args.first() {
                let path = Path::new(p);
                Value::String(path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "dirname" => {
            if let Some(Value::String(p)) = args.first() {
                let path = Path::new(p);
                Value::String(path.parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "extname" => {
            if let Some(Value::String(p)) = args.first() {
                let path = Path::new(p);
                Value::String(path.extension()
                    .map(|e| format!(".{}", e.to_string_lossy()))
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "stem" => {
            if let Some(Value::String(p)) = args.first() {
                let path = Path::new(p);
                Value::String(path.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default())
            } else {
                Value::Nil
            }
        }
        "normalize" => {
            if let Some(Value::String(p)) = args.first() {
                let path = PathBuf::from(p);
                let mut normalized = PathBuf::new();
                for component in path.components() {
                    use std::path::Component;
                    match component {
                        Component::ParentDir => { normalized.pop(); }
                        Component::CurDir => {}
                        c => normalized.push(c),
                    }
                }
                Value::String(normalized.to_string_lossy().to_string())
            } else {
                Value::Nil
            }
        }
        "is_absolute" => {
            if let Some(Value::String(p)) = args.first() {
                Value::Boolean(Path::new(p).is_absolute())
            } else {
                Value::Nil
            }
        }
        "is_relative" => {
            if let Some(Value::String(p)) = args.first() {
                Value::Boolean(Path::new(p).is_relative())
            } else {
                Value::Nil
            }
        }
        "resolve" => {
            if let Some(Value::String(p)) = args.first() {
                match std::fs::canonicalize(p) {
                    Ok(abs_path) => Value::String(abs_path.to_string_lossy().to_string()),
                    Err(_) => {
                        // If file doesn't exist, try to resolve relative to cwd
                        if let Ok(cwd) = std::env::current_dir() {
                            Value::String(cwd.join(p).to_string_lossy().to_string())
                        } else {
                            Value::Nil
                        }
                    }
                }
            } else {
                Value::Nil
            }
        }
        "relative" => {
            if args.len() >= 2 {
                if let (Some(Value::String(from)), Some(Value::String(to))) = (args.get(0), args.get(1)) {
                    let from_path = PathBuf::from(from);
                    let to_path = PathBuf::from(to);

                    // Simple relative path calculation
                    if let Ok(to_abs) = std::fs::canonicalize(&to_path) {
                        if let Ok(from_abs) = std::fs::canonicalize(&from_path) {
                            if let Ok(rel) = to_abs.strip_prefix(&from_abs) {
                                return Value::String(rel.to_string_lossy().to_string());
                            }
                        }
                    }
                    Value::Nil
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "split" => {
            if let Some(Value::String(p)) = args.first() {
                let path = Path::new(p);
                let components: Vec<Value> = path.components()
                    .map(|c| Value::String(c.as_os_str().to_string_lossy().to_string()))
                    .collect();
                Value::Table(components)
            } else {
                Value::Nil
            }
        }
        "with_extension" => {
            if args.len() >= 2 {
                if let (Some(Value::String(p)), Some(Value::String(ext))) = (args.get(0), args.get(1)) {
                    let path = PathBuf::from(p);
                    let new_path = path.with_extension(ext);
                    Value::String(new_path.to_string_lossy().to_string())
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "separator" => {
            Value::String(MAIN_SEPARATOR.to_string())
        }
        _ => Value::Nil,
    }
}

fn call_assert(func: &str, args: Vec<Value>) -> Value {
    match func {
        "equal" => {
            if args.len() >= 2 {
                let equal = format!("{:?}", args.get(0)) == format!("{:?}", args.get(1));
                if equal {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: values are not equal");
                    eprintln!("ASSERT FAILED: {}", msg);
                    eprintln!("  Expected: {:?}", args.get(1));
                    eprintln!("  Got: {:?}", args.get(0));
                    Value::Boolean(false)
                }
            } else {
                Value::Nil
            }
        }
        "not_equal" => {
            if args.len() >= 2 {
                let equal = format!("{:?}", args.get(0)) == format!("{:?}", args.get(1));
                if !equal {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: values are equal");
                    eprintln!("ASSERT FAILED: {}", msg);
                    Value::Boolean(false)
                }
            } else {
                Value::Nil
            }
        }
        "true" => {
            if let Some(val) = args.first() {
                if val.is_truthy() {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(1).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: expected true");
                    eprintln!("ASSERT FAILED: {}", msg);
                    Value::Boolean(false)
                }
            } else {
                Value::Nil
            }
        }
        "false" => {
            if let Some(val) = args.first() {
                if !val.is_truthy() {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(1).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: expected false");
                    eprintln!("ASSERT FAILED: {}", msg);
                    Value::Boolean(false)
                }
            } else {
                Value::Nil
            }
        }
        "nil" => {
            if let Some(val) = args.first() {
                if matches!(val, Value::Nil) {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(1).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: expected nil");
                    eprintln!("ASSERT FAILED: {}", msg);
                    Value::Boolean(false)
                }
            } else {
                Value::Boolean(true)
            }
        }
        "not_nil" => {
            if let Some(val) = args.first() {
                if !matches!(val, Value::Nil) {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(1).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: expected non-nil value");
                    eprintln!("ASSERT FAILED: {}", msg);
                    Value::Boolean(false)
                }
            } else {
                Value::Nil
            }
        }
        "type" => {
            if args.len() >= 2 {
                if let Some(Value::String(expected_type)) = args.get(1) {
                    let actual_type = match args.get(0) {
                        Some(Value::Number(_)) => "number",
                        Some(Value::String(_)) => "string",
                        Some(Value::Boolean(_)) => "boolean",
                        Some(Value::Table(_)) => "table",
                        Some(Value::Dictionary(_)) => "dictionary",
                        Some(Value::Function(_, _)) => "function",
                        Some(Value::NativeFunction(_, _)) => "function",
                        Some(Value::Module(_)) => "module",
                        Some(Value::Nil) | None => "nil",
                    };
                    if actual_type == expected_type.as_str() {
                        Value::Boolean(true)
                    } else {
                        let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                            .unwrap_or("Assertion failed: type mismatch");
                        eprintln!("ASSERT FAILED: {} (expected {}, got {})", msg, expected_type, actual_type);
                        Value::Boolean(false)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "fail" => {
            let msg = args.first()
                .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                .unwrap_or("Assertion failed");
            eprintln!("ASSERT FAILED: {}", msg);
            Value::Boolean(false)
        }
        "greater" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    if a > b {
                        Value::Boolean(true)
                    } else {
                        let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                            .unwrap_or("Assertion failed: first value is not greater");
                        eprintln!("ASSERT FAILED: {} ({} is not > {})", msg, a, b);
                        Value::Boolean(false)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "less" => {
            if args.len() >= 2 {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    if a < b {
                        Value::Boolean(true)
                    } else {
                        let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                            .unwrap_or("Assertion failed: first value is not less");
                        eprintln!("ASSERT FAILED: {} ({} is not < {})", msg, a, b);
                        Value::Boolean(false)
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "contains" => {
            if args.len() >= 2 {
                let contains = match (args.get(0), args.get(1)) {
                    (Some(Value::String(s)), Some(Value::String(sub))) => s.contains(sub.as_str()),
                    (Some(Value::Table(arr)), Some(val)) => arr.contains(val),
                    _ => false,
                };
                if contains {
                    Value::Boolean(true)
                } else {
                    let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                        .unwrap_or("Assertion failed: does not contain value");
                    eprintln!("ASSERT FAILED: {}", msg);
                    Value::Boolean(false)
                }
            } else {
                Value::Nil
            }
        }
        "matches" => {
            if args.len() >= 2 {
                if let (Some(Value::String(s)), Some(Value::String(pattern))) = (args.get(0), args.get(1)) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            if re.is_match(s) {
                                Value::Boolean(true)
                            } else {
                                let msg = args.get(2).and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                                    .unwrap_or("Assertion failed: does not match pattern");
                                eprintln!("ASSERT FAILED: {}", msg);
                                Value::Boolean(false)
                            }
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}

fn call_set(func: &str, args: Vec<Value>) -> Value {
    // Sets are represented as Tables with unique elements
    fn value_key(v: &Value) -> String {
        format!("{:?}", v)
    }

    fn to_set(arr: &[Value]) -> Vec<Value> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for v in arr {
            let key = value_key(v);
            if !seen.contains(&key) {
                seen.insert(key);
                result.push(v.clone());
            }
        }
        result
    }

    match func {
        "new" => {
            Value::Table(Vec::new())
        }
        "from_array" => {
            if let Some(Value::Table(arr)) = args.first() {
                Value::Table(to_set(arr))
            } else {
                Value::Nil
            }
        }
        "add" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(val)) = (args.get(0), args.get(1)) {
                    let mut new_arr = arr.clone();
                    let key = value_key(val);
                    if !arr.iter().any(|v| value_key(v) == key) {
                        new_arr.push(val.clone());
                    }
                    Value::Table(new_arr)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "remove" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(val)) = (args.get(0), args.get(1)) {
                    let key = value_key(val);
                    let new_arr: Vec<Value> = arr.iter()
                        .filter(|v| value_key(v) != key)
                        .cloned()
                        .collect();
                    Value::Table(new_arr)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "has" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr)), Some(val)) = (args.get(0), args.get(1)) {
                    let key = value_key(val);
                    Value::Boolean(arr.iter().any(|v| value_key(v) == key))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "size" => {
            if let Some(Value::Table(arr)) = args.first() {
                Value::Number(arr.len() as f64)
            } else {
                Value::Nil
            }
        }
        "union" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let mut combined = arr1.clone();
                    combined.extend(arr2.iter().cloned());
                    Value::Table(to_set(&combined))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "intersection" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let set2: std::collections::HashSet<String> = arr2.iter().map(value_key).collect();
                    let result: Vec<Value> = arr1.iter()
                        .filter(|v| set2.contains(&value_key(v)))
                        .cloned()
                        .collect();
                    Value::Table(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "difference" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let set2: std::collections::HashSet<String> = arr2.iter().map(value_key).collect();
                    let result: Vec<Value> = arr1.iter()
                        .filter(|v| !set2.contains(&value_key(v)))
                        .cloned()
                        .collect();
                    Value::Table(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "symmetric_difference" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let set1: std::collections::HashSet<String> = arr1.iter().map(value_key).collect();
                    let set2: std::collections::HashSet<String> = arr2.iter().map(value_key).collect();
                    let mut result: Vec<Value> = arr1.iter()
                        .filter(|v| !set2.contains(&value_key(v)))
                        .cloned()
                        .collect();
                    result.extend(arr2.iter()
                        .filter(|v| !set1.contains(&value_key(v)))
                        .cloned());
                    Value::Table(result)
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "is_subset" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let set2: std::collections::HashSet<String> = arr2.iter().map(value_key).collect();
                    Value::Boolean(arr1.iter().all(|v| set2.contains(&value_key(v))))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "is_superset" => {
            if args.len() >= 2 {
                if let (Some(Value::Table(arr1)), Some(Value::Table(arr2))) = (args.get(0), args.get(1)) {
                    let set1: std::collections::HashSet<String> = arr1.iter().map(value_key).collect();
                    Value::Boolean(arr2.iter().all(|v| set1.contains(&value_key(v))))
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "to_array" => {
            if let Some(Value::Table(arr)) = args.first() {
                Value::Table(arr.clone())
            } else {
                Value::Nil
            }
        }
        "clear" => {
            Value::Table(Vec::new())
        }
        _ => Value::Nil,
    }
}

fn call_io(func: &str, args: Vec<Value>) -> Value {
    use std::io::{self, BufRead, Write};

    match func {
        "read_line" => {
            let stdin = io::stdin();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(_) => Value::String(line.trim_end_matches('\n').trim_end_matches('\r').to_string()),
                Err(_) => Value::Nil,
            }
        }
        "read_all" => {
            let stdin = io::stdin();
            let mut buffer = String::new();
            for line in stdin.lock().lines() {
                match line {
                    Ok(l) => {
                        buffer.push_str(&l);
                        buffer.push('\n');
                    }
                    Err(_) => break,
                }
            }
            Value::String(buffer)
        }
        "print" => {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                match arg {
                    Value::String(s) => print!("{}", s),
                    _ => print!("{}", arg),
                }
            }
            let _ = io::stdout().flush();
            Value::Nil
        }
        "println" => {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                match arg {
                    Value::String(s) => print!("{}", s),
                    _ => print!("{}", arg),
                }
            }
            println!();
            Value::Nil
        }
        "eprint" => {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    eprint!(" ");
                }
                match arg {
                    Value::String(s) => eprint!("{}", s),
                    _ => eprint!("{}", arg),
                }
            }
            let _ = io::stderr().flush();
            Value::Nil
        }
        "eprintln" => {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    eprint!(" ");
                }
                match arg {
                    Value::String(s) => eprint!("{}", s),
                    _ => eprint!("{}", arg),
                }
            }
            eprintln!();
            Value::Nil
        }
        "flush" => {
            let _ = io::stdout().flush();
            Value::Nil
        }
        "input" => {
            // Print prompt if provided
            if let Some(Value::String(prompt)) = args.first() {
                print!("{}", prompt);
                let _ = io::stdout().flush();
            }
            let stdin = io::stdin();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(_) => Value::String(line.trim_end_matches('\n').trim_end_matches('\r').to_string()),
                Err(_) => Value::Nil,
            }
        }
        _ => Value::Nil,
    }
}

fn call_log(func: &str, args: Vec<Value>) -> Value {
    use chrono::Local;

    fn format_log(level: &str, args: &[Value]) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let message: String = args.iter()
            .map(|v| match v {
                Value::String(s) => s.clone(),
                _ => v.to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        format!("[{}] [{}] {}", timestamp, level, message)
    }

    match func {
        "debug" => {
            eprintln!("{}", format_log("DEBUG", &args));
            Value::Nil
        }
        "info" => {
            eprintln!("{}", format_log("INFO", &args));
            Value::Nil
        }
        "warn" => {
            eprintln!("{}", format_log("WARN", &args));
            Value::Nil
        }
        "error" => {
            eprintln!("{}", format_log("ERROR", &args));
            Value::Nil
        }
        "fatal" => {
            eprintln!("{}", format_log("FATAL", &args));
            Value::Nil
        }
        "trace" => {
            eprintln!("{}", format_log("TRACE", &args));
            Value::Nil
        }
        "level" => {
            // This would need global state to be useful, for now just return the level
            if let Some(Value::String(level)) = args.first() {
                Value::String(level.to_uppercase())
            } else {
                Value::String("INFO".to_string())
            }
        }
        _ => Value::Nil,
    }
}

fn call_url(func: &str, args: Vec<Value>) -> Value {
    use url::Url;
    use percent_encoding::{utf8_percent_encode, percent_decode_str, NON_ALPHANUMERIC, AsciiSet};

    // URL-safe characters
    const QUERY_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');

    match func {
        "parse" => {
            if let Some(Value::String(s)) = args.first() {
                match Url::parse(s) {
                    Ok(url) => {
                        let mut dict = HashMap::new();
                        dict.insert("scheme".to_string(), Value::String(url.scheme().to_string()));
                        dict.insert("host".to_string(), url.host_str().map(|h| Value::String(h.to_string())).unwrap_or(Value::Nil));
                        dict.insert("port".to_string(), url.port().map(|p| Value::Number(p as f64)).unwrap_or(Value::Nil));
                        dict.insert("path".to_string(), Value::String(url.path().to_string()));
                        dict.insert("query".to_string(), url.query().map(|q| Value::String(q.to_string())).unwrap_or(Value::Nil));
                        dict.insert("fragment".to_string(), url.fragment().map(|f| Value::String(f.to_string())).unwrap_or(Value::Nil));
                        dict.insert("username".to_string(), if url.username().is_empty() { Value::Nil } else { Value::String(url.username().to_string()) });
                        dict.insert("password".to_string(), url.password().map(|p| Value::String(p.to_string())).unwrap_or(Value::Nil));
                        dict.insert("origin".to_string(), Value::String(url.origin().unicode_serialization()));
                        Value::Dictionary(dict)
                    }
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "format" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                let scheme = dict.get("scheme").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None }).unwrap_or("https");
                let host = dict.get("host").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None }).unwrap_or("");
                let port = dict.get("port").and_then(|v| if let Value::Number(n) = v { Some(*n as u16) } else { None });
                let path = dict.get("path").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None }).unwrap_or("/");
                let query = dict.get("query").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None });
                let fragment = dict.get("fragment").and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None });

                let mut url = format!("{}://{}", scheme, host);
                if let Some(p) = port {
                    url.push_str(&format!(":{}", p));
                }
                url.push_str(path);
                if let Some(q) = query {
                    url.push_str(&format!("?{}", q));
                }
                if let Some(f) = fragment {
                    url.push_str(&format!("#{}", f));
                }
                Value::String(url)
            } else {
                Value::Nil
            }
        }
        "encode" | "encode_component" => {
            if let Some(Value::String(s)) = args.first() {
                Value::String(utf8_percent_encode(s, QUERY_ENCODE_SET).to_string())
            } else {
                Value::Nil
            }
        }
        "decode" | "decode_component" => {
            if let Some(Value::String(s)) = args.first() {
                match percent_decode_str(s).decode_utf8() {
                    Ok(decoded) => Value::String(decoded.to_string()),
                    Err(_) => Value::Nil,
                }
            } else {
                Value::Nil
            }
        }
        "query_parse" => {
            if let Some(Value::String(s)) = args.first() {
                let pairs: HashMap<String, Value> = s.split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        let key = parts.next()?;
                        let value = parts.next().unwrap_or("");
                        let decoded_key = percent_decode_str(key).decode_utf8().ok()?.to_string();
                        let decoded_value = percent_decode_str(value).decode_utf8().ok()?.to_string();
                        Some((decoded_key, Value::String(decoded_value)))
                    })
                    .collect();
                Value::Dictionary(pairs)
            } else {
                Value::Nil
            }
        }
        "query_stringify" => {
            if let Some(Value::Dictionary(dict)) = args.first() {
                let query: String = dict.iter()
                    .map(|(k, v)| {
                        let value = match v {
                            Value::String(s) => s.clone(),
                            _ => v.to_string(),
                        };
                        format!("{}={}",
                            utf8_percent_encode(k, QUERY_ENCODE_SET),
                            utf8_percent_encode(&value, QUERY_ENCODE_SET))
                    })
                    .collect::<Vec<_>>()
                    .join("&");
                Value::String(query)
            } else {
                Value::Nil
            }
        }
        "join" => {
            if args.len() >= 2 {
                if let (Some(Value::String(base)), Some(Value::String(path))) = (args.get(0), args.get(1)) {
                    match Url::parse(base) {
                        Ok(base_url) => {
                            match base_url.join(path) {
                                Ok(joined) => Value::String(joined.to_string()),
                                Err(_) => Value::Nil,
                            }
                        }
                        Err(_) => Value::Nil,
                    }
                } else {
                    Value::Nil
                }
            } else {
                Value::Nil
            }
        }
        "is_valid" => {
            if let Some(Value::String(s)) = args.first() {
                Value::Boolean(Url::parse(s).is_ok())
            } else {
                Value::Nil
            }
        }
        _ => Value::Nil,
    }
}
