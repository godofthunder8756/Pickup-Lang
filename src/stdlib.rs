use crate::compiler::Value;
use std::collections::HashMap;

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
    
    stdlib
}

fn create_json_module() -> Value {
    let mut json_funcs = HashMap::new();
    
    // JSON functions with implementations
    json_funcs.insert("parse".to_string(), Value::String("<native:json.parse>".to_string()));
    json_funcs.insert("stringify".to_string(), Value::String("<native:json.stringify>".to_string()));
    
    Value::Module(json_funcs)
}

fn create_fs_module() -> Value {
    let mut fs_funcs = HashMap::new();
    
    // FS functions
    fs_funcs.insert("read".to_string(), Value::String("<native:fs.read>".to_string()));
    fs_funcs.insert("write".to_string(), Value::String("<native:fs.write>".to_string()));
    fs_funcs.insert("exists".to_string(), Value::String("<native:fs.exists>".to_string()));
    fs_funcs.insert("readdir".to_string(), Value::String("<native:fs.readdir>".to_string()));
    fs_funcs.insert("mkdir".to_string(), Value::String("<native:fs.mkdir>".to_string()));
    
    Value::Module(fs_funcs)
}

fn create_string_module() -> Value {
    let mut string_funcs = HashMap::new();
    
    // String utility functions
    string_funcs.insert("length".to_string(), Value::String("<native:string.length>".to_string()));
    string_funcs.insert("upper".to_string(), Value::String("<native:string.upper>".to_string()));
    string_funcs.insert("lower".to_string(), Value::String("<native:string.lower>".to_string()));
    string_funcs.insert("substring".to_string(), Value::String("<native:string.substring>".to_string()));
    string_funcs.insert("split".to_string(), Value::String("<native:string.split>".to_string()));
    string_funcs.insert("trim".to_string(), Value::String("<native:string.trim>".to_string()));
    string_funcs.insert("replace".to_string(), Value::String("<native:string.replace>".to_string()));
    
    Value::Module(string_funcs)
}

fn create_math_module() -> Value {
    let mut math_funcs = HashMap::new();
    
    // Math constants
    math_funcs.insert("pi".to_string(), Value::Number(std::f64::consts::PI));
    math_funcs.insert("e".to_string(), Value::Number(std::f64::consts::E));
    
    // Math functions
    math_funcs.insert("floor".to_string(), Value::String("<native:math.floor>".to_string()));
    math_funcs.insert("ceil".to_string(), Value::String("<native:math.ceil>".to_string()));
    math_funcs.insert("round".to_string(), Value::String("<native:math.round>".to_string()));
    math_funcs.insert("abs".to_string(), Value::String("<native:math.abs>".to_string()));
    math_funcs.insert("min".to_string(), Value::String("<native:math.min>".to_string()));
    math_funcs.insert("max".to_string(), Value::String("<native:math.max>".to_string()));
    math_funcs.insert("sqrt".to_string(), Value::String("<native:math.sqrt>".to_string()));
    math_funcs.insert("pow".to_string(), Value::String("<native:math.pow>".to_string()));
    math_funcs.insert("sin".to_string(), Value::String("<native:math.sin>".to_string()));
    math_funcs.insert("cos".to_string(), Value::String("<native:math.cos>".to_string()));
    math_funcs.insert("tan".to_string(), Value::String("<native:math.tan>".to_string()));
    math_funcs.insert("random".to_string(), Value::String("<native:math.random>".to_string()));
    
    Value::Module(math_funcs)
}
