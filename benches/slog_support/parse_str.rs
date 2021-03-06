use test::Bencher;

#[bench]
pub fn bench_log_discard_kv(b: &mut Bencher) {
    #[cfg(feature = "slog")]
    {
        let u1 =
            Uuid::parse_str("F9168C5E-CEB2-4FAB-B6BF-329BF39FA1E4").unwrap();
        let root = slog::Logger::root(slog::Discard.fuse(), o!());
    }
    b.iter(|| {
        #[cfg(feature = "slog")]
        crit!(root, "test"; "u1" => u1);
    });
}
