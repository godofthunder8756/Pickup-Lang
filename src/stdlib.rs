use crate::compiler::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

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

    stdlib
}

fn create_json_module() -> Value {
    let mut json_funcs = HashMap::new();

    json_funcs.insert("parse".to_string(), Value::NativeFunction("json".to_string(), "parse".to_string()));
    json_funcs.insert("stringify".to_string(), Value::NativeFunction("json".to_string(), "stringify".to_string()));

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

    Value::Module(os_funcs)
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
        _ => Value::Nil,
    }
}
