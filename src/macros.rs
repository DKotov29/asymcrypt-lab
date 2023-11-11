#[macro_export]
macro_rules! test_n_print {
    (  $bitvec:expr, $interals_r:expr, $a:expr, $test_name:expr) => {
        let bitvec: WrapperBitVec<_> = $bitvec.to_bitvec().into();
        let byte_vec: Vec<u8> = bitvec.into();
        let r: usize = $interals_r;
        let a: f64 = $a;

        let sign_independency = crate::criterion::sign_independency::test(byte_vec.as_slice(), a);
        let sign_uniformity = crate::criterion::sign_uniformity::test(byte_vec.as_slice(), a);
        let sequence_homogeneity =
            crate::criterion::sequence_homogeneity::test(byte_vec.as_slice(), r, a);

        let mut table = Table::new();
        let test_name: &str = $test_name;
        table.add_row(row![
            test_name,
            "Sign independency",
            "Sign uniformity",
            "Sequence homogeneity"
        ]);
        table.add_row(row![
            "statistic_value <= critic_value",
            sign_independency.0,
            sign_uniformity.0,
            sequence_homogeneity.0
        ]);
        table.add_row(row![
            "statistic_value",
            sign_independency.1,
            sign_uniformity.1,
            sequence_homogeneity.1
        ]);
        table.add_row(row![
            "critic_value",
            sign_independency.2,
            sign_uniformity.2,
            sequence_homogeneity.2
        ]);
        table.add_row(row![
            "time elapsed",
            format!("{:?}", sign_independency.3),
            format!("{:?}", sign_uniformity.3),
            format!("{:?}", sequence_homogeneity.3)
        ]);
        table.printstd();
    };
}
