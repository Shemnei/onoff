use criterion::{black_box, criterion_group, criterion_main, Criterion};
use off_rs::parser::DocumentParser;
use onoff::colorformat;
use onoff::parse::OffParser;

const WIKI_OFF: &'static str = r#"OFF
# cube.off
# A cube
 
8 6 12
 1.0  0.0 1.4142
 0.0  1.0 1.4142
-1.0  0.0 1.4142
 0.0 -1.0 1.4142
 1.0  0.0 0.0
 0.0  1.0 0.0
-1.0  0.0 0.0
 0.0 -1.0 0.0
4  0 1 2 3  255 0 0 #red
4  7 4 0 3  0 255 0 #green
4  4 5 1 0  0 0 255 #blue
4  5 6 2 1  0 255 0 
4  3 2 6 7  0 0 255
4  6 5 4 7  255 0 0
"#;

const PRINSTON_OFF: &'static str = r#"OFF
#
#  cube.off
#  A cube.
#  There is extra RGBA color information specified for the faces.
#
8 6 12
  1.632993   0.000000   1.154701
  0.000000   1.632993   1.154701
 -1.632993   0.000000   1.154701
  0.000000  -1.632993   1.154701
  1.632993   0.000000  -1.154701
  0.000000   1.632993  -1.154701
 -1.632993   0.000000  -1.154701
  0.000000  -1.632993  -1.154701
  4  0 1 2 3  1.000 0.000 0.000 0.75
  4  7 4 0 3  0.300 0.400 0.000 0.75
  4  4 5 1 0  0.200 0.500 0.100 0.75
  4  5 6 2 1  0.100 0.600 0.200 0.75
  4  3 2 6 7  0.000 0.700 0.300 0.75
  4  6 5 4 7  0.000 1.000 0.000 0.75
"#;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse wiki - onoff", |b| {
        let opts = onoff::parse::ParserOptions {
            color_format: colorformat::RgbU8,
            limits: Default::default(),
        };

        b.iter(|| (black_box(OffParser::new_with_options(&WIKI_OFF, opts).try_parse())))
    });
    c.bench_function("parse wiki - off-rs", |b| {
        let opts = off_rs::document::ParserOptions {
            color_format: off_rs::geometry::ColorFormat::RGBAFloat,
        };

        b.iter(|| black_box(DocumentParser::new(&WIKI_OFF, opts).parse()))
    });

    c.bench_function("parse prinston - onoff", |b| {
        let opts = onoff::parse::ParserOptions {
            color_format: colorformat::RgbU8,
            limits: Default::default(),
        };

        b.iter(|| (black_box(OffParser::new_with_options(&PRINSTON_OFF, opts).try_parse())))
    });
    c.bench_function("parse prinston - off-rs", |b| {
        let opts = off_rs::document::ParserOptions {
            color_format: off_rs::geometry::ColorFormat::RGBAFloat,
        };

        b.iter(|| black_box(DocumentParser::new(&PRINSTON_OFF, opts).parse()))
    });

    c.bench_function("parse wiki - onoff @ ANY", |b| {
        let opts = onoff::parse::ParserOptions {
            color_format: colorformat::Any,
            limits: Default::default(),
        };

        b.iter(|| (black_box(OffParser::new_with_options(&WIKI_OFF, opts).try_parse())))
    });
    c.bench_function("parse prinston - onoff @ ANY", |b| {
        let opts = onoff::parse::ParserOptions {
            color_format: colorformat::Any,
            limits: Default::default(),
        };

        b.iter(|| (black_box(OffParser::new_with_options(&PRINSTON_OFF, opts).try_parse())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
