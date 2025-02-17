extern crate proc_macro;

use proc_macro::TokenStream;
use std::fmt::Write;

use serde_json::Value;

/// 将相连的若干的 list 分割
fn split_to_vec(input: &str) -> Vec<&str> {
    let mut start = 0;
    let mut cnt = 0;
    let mut ret = vec![];
    for (idx, c) in input.chars().enumerate() {
        match c {
            '[' => {
                if cnt == 0 {
                    start = idx;
                }
                cnt += 1;
            }
            ']' => {
                cnt -= 1;
                if cnt == 0 {
                    ret.push(&input[start..=idx]);
                    start = idx + 1;
                }
            }
            _ => (),
        }
    }
    ret
}

/// 将名字转换为 Rust 风格的命名
/// 除第一个单词外, 其余大写字母替换为 下划线 + 小写字母 的组合
fn turn_to_legal_name(name: &str) -> String {
    let mut ret = String::new();
    let mut skiped_first = false;
    for c in name.chars() {
        if c.is_ascii_uppercase() && skiped_first {
            ret.push('_');
            ret.push(c.to_ascii_lowercase());
        } else {
            ret.push(c);
        }
        if c == ',' {
            skiped_first = true;
        }
    }
    ret
}

/// 转换为 Rust 语法的参数
trait ToArgs {
    fn to_args(&self) -> String;
}

impl ToArgs for Value {
    fn to_args(&self) -> String {
        let array = self.as_array().unwrap();
        if array.is_empty() {
            "".to_owned()
        } else {
            array
                .iter()
                .map(|value| {
                    if value.is_string() {
                        format!("{}.to_owned()", value)
                    } else {
                        value.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

fn json_to_code(input: TokenStream) -> String {
    let input = input.to_string();
    let v = split_to_vec(&input);
    let (funcs, args, rets) = (v[0], v[1], v[2]);

    // 反序列化 json
    // 这个地方有个坑点, "-123" 在先前 to_string 的时候会变成 "- 123", 所以单独替换一下 "- " => "-"
    let funcs = serde_json::from_str::<Value>(&turn_to_legal_name(funcs)).unwrap();
    let args = serde_json::from_str::<Value>(&args.replace("- ", "-")).unwrap();
    let rets = serde_json::from_str::<Value>(&rets.replace("- ", "-")).unwrap();

    let mut code = String::new();
    let _ = writeln!(
        code,
        "let mut obj = {}::new({});\n",
        funcs[0].as_str().unwrap(),
        args[0].to_args(),
    );

    for i in 1..funcs.as_array().unwrap().len() {
        let mut stmt = format!("obj.{}({})", funcs[i].as_str().unwrap(), args[i].to_args());
        if !rets[i].is_null() {
            stmt = format!(r##"assert_eq!({}, {}, r#"{}"#)"##, stmt, rets[i], stmt);
        }
        stmt.push_str(";\n");
        code.push_str(&stmt);
    }
    code
}

/// Generate code from leetcode json
///
/// # Example
///
/// ```ignore
/// leetcode_test!(
///     ["Trie", "insert", "search"]
///     [[], ["apple"], ["apple"]]
///     [null, null, true]
/// )
/// ```
///
/// this will be expaned to
///
/// ``` ignore
/// let mut obj = Trie::new();
/// obj.insert("apple".to_owned());
/// assert_eq!(obj.search("apple".to_owned()), true);
/// ```
#[proc_macro]
pub fn leetcode_test(input: TokenStream) -> TokenStream {
    json_to_code(input).parse().unwrap()
}

#[proc_macro]
pub fn leetcode_test_debug(input: TokenStream) -> TokenStream {
    format!(r###"r##"{}"##"###, json_to_code(input))
        .parse()
        .unwrap()
}
