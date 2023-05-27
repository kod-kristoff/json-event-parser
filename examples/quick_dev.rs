use json_event_parser::parse_tree;

fn main() {
    println!("{:?}", parse_tree(r#"{"a": "b"}"#));
}
