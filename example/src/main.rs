extern crate webplatform;

fn main() {
    print!("HELLO FROM RUST");

    webplatform::init().element_query("#container").unwrap().html_set("<h1>HELLO FROM RUST</h1>");
}

// Functions that you wish to access from Javascript must be marked as no_mangle
#[no_mangle]
pub fn doub(a: i32) -> i32 {
    return a + a
}
