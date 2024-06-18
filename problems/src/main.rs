fn ret_string() -> String {
    String::from("  A String object  ")
}

fn main() {
    let s = ret_string();
    let l = s.trim();
    assert_eq!(l, "A String object");
}