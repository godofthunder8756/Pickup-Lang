use crate::compiler::Value;
use std::collections::HashMap;

/// Initialize standard library modules
pub fn create_stdlib() -> HashMap<String, Value> {
    let mut stdlib = HashMap::new();
    
    // JSON module
    stdlib.insert("json".to_string(), create_json_module());
    
    // FS module
    stdlib.insert("fs".to_string(), create_fs_module());
    
    stdlib
}

fn create_json_module() -> Value {
    let mut json_funcs = HashMap::new();
    
    // For now, we'll use placeholders - these would be implemented as native functions
    json_funcs.insert("parse".to_string(), Value::String("<json.parse>".to_string()));
    json_funcs.insert("stringify".to_string(), Value::String("<json.stringify>".to_string()));
    
    Value::Module(json_funcs)
}

fn create_fs_module() -> Value {
    let mut fs_funcs = HashMap::new();
    
    // FS functions placeholders
    fs_funcs.insert("read".to_string(), Value::String("<fs.read>".to_string()));
    fs_funcs.insert("write".to_string(), Value::String("<fs.write>".to_string()));
    fs_funcs.insert("exists".to_string(), Value::String("<fs.exists>".to_string()));
    
    Value::Module(fs_funcs)
}
