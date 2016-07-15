#![feature(test)]

extern crate ftoa;
extern crate test;

macro_rules! benches {
    ($($name:ident($value:expr),)*) => {
        mod bench_ftoa {
            use test::{Bencher, black_box};
            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    use ftoa;

                    let mut buf = Vec::with_capacity(20);

                    b.iter(|| {
                        buf.clear();
                        ftoa::write(&mut buf, black_box($value)).unwrap()
                    });
                }
            )*
        }

        mod bench_fmt {
            use test::{Bencher, black_box};
            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    use std::io::Write;

                    let mut buf = Vec::with_capacity(20);

                    b.iter(|| {
                        buf.clear();
                        write!(&mut buf, "{}", black_box($value)).unwrap()
                    });
                }
            )*
        }
    }
}

benches!(
    bench_0(0f64),
    bench_short(0.1234),
    bench_e(2.718281828459045),
    bench_max(::std::f64::MAX),
);
