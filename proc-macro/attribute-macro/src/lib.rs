use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Lit, Meta, NestedMeta, Path};

#[proc_macro_attribute]
pub fn attribute_macro_example(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("args: \"{}\"", args.to_string());
    println!("input: \"{}\"", input.to_string());

    let parsed_args = parse_macro_input!(args as AttributeArgs);

    walk_args(parsed_args);

    input
}

fn walk_args(arg: AttributeArgs) {
    for a in arg {
        println!("{}", walk_nested_meta(&a));
    }
}

fn walk_nested_meta(nested: &NestedMeta) -> String {
    match nested {
        NestedMeta::Meta(m) => match m {
            Meta::Path(p) => {
                format!("Path: {:?}", print_path(&p))
            }
            Meta::List(ml) => {
                let nested_ident: Vec<String> = ml
                    .nested
                    .iter()
                    .map(|nested_meta| format!("Nested: {:?}", walk_nested_meta(nested_meta)))
                    .collect();

                format!("{:?}List Path: {:?}", nested_ident, print_path(&ml.path))
            }
            Meta::NameValue(mnv) => {
                format!(
                    "NameValue Path: {:?} -- Value: {}",
                    print_path(&mnv.path),
                    print_lit(&mnv.lit)
                )
            }
        },
        NestedMeta::Lit(l) => print_lit(l),
    }
}

fn print_path(path: &Path) -> Vec<String> {
    let identifiers: Vec<String> = path
        .segments
        .iter()
        .map(|args| args.ident.to_string())
        .collect();
    identifiers
}

fn print_lit(lit: &Lit) -> String {
    match lit {
        Lit::Str(lit_str) => {
            format!("Str: {}", lit_str.value())
        }
        Lit::ByteStr(lit_byte_str) => {
            format!("LitByteStr: {:?}", lit_byte_str.value())
        }
        Lit::Byte(lit_byte) => {
            format!("LitByte: {}", lit_byte.value())
        }
        Lit::Char(lit_char) => {
            format!("LitChar: {}", lit_char.value())
        }
        Lit::Int(lit_int) => {
            format!("LitInt: {}", lit_int.base10_digits())
        }
        Lit::Float(lit_float) => {
            format!("LitFloat: {}", lit_float.base10_digits())
        }
        Lit::Bool(lit_bool) => {
            format!("LitBool: {}", lit_bool.value)
        }
        Lit::Verbatim(literal) => {
            format!("Literal: {}", literal.to_string())
        }
    }
}
