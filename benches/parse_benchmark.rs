use criterion::{black_box, criterion_group, criterion_main, Criterion};
use off_rs::parser::DocumentParser;
use onoff::colorformat;
use onoff::parse::OffParser;

const WIKI_OFF: &'static str = include_str!("../resources/wiki.off");
const PRINSTON_OFF: &'static str = include_str!("../resources/prinston.off");
const SOCKET_OFF: &'static str = include_str!("../resources/socket.off");

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

    c.bench_function("parse socket - onoff", |b| {
        let opts = onoff::parse::ParserOptions {
            color_format: colorformat::RgbU8,
            limits: Default::default(),
        };

        b.iter(|| (black_box(OffParser::new_with_options(&SOCKET_OFF, opts).try_parse())))
    });
    c.bench_function("parse socket - off-rs", |b| {
        let opts = off_rs::document::ParserOptions {
            color_format: off_rs::geometry::ColorFormat::RGBAFloat,
        };

        b.iter(|| black_box(DocumentParser::new(&SOCKET_OFF, opts).parse()))
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
    c.bench_function("parse socket - onoff @ ANY", |b| {
        let opts = onoff::parse::ParserOptions {
            color_format: colorformat::None,
            limits: Default::default(),
        };

        b.iter(|| (black_box(OffParser::new_with_options(&SOCKET_OFF, opts).try_parse())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
