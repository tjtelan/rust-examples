use attribute_macro::attribute_macro_example;

#[attribute_macro_example(slow, number = 1, language = "rust")]
fn main() {
    // We only care about the compile-time output from our macro
    println!("Hello world!")
}
