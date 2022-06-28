mod platform;
use platform::h5::h5_write;

const LANG: [&'static str;4]= ["cn", "en", "jp", "ar"];
fn main() {
    h5_write::write_h5_with_wiki();
}
