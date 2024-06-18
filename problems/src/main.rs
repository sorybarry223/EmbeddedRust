fn ret_string() -> String {
    String::from("  A String object  ")
}

fn main() {
    let s = ret_string();
    let q = s.trim();
    assert_eq!(q, "A String object");
}