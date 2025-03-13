use afl::fuzz;
use oxylc::parser;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = parser::parse(s);
        }
    });
}
