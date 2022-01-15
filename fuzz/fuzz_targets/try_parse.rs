#![no_main]
use libfuzzer_sys::fuzz_target;

use onoff::parse::OffParser;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = OffParser::new(&s).try_parse();
    }
});
