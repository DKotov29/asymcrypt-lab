#[macro_export]
macro_rules! test_n_print {
    ($bitvec:expr, $interals_r:expr, $a:expr, $test_name:expr) => {{
        let bitvec: WrapperBitVec<_> = $bitvec.to_bitvec().into();
        let byte_vec: Vec<u8> = bitvec.into();
        let r: usize = $interals_r;
        let a: f64 = $a;
        let sign_independency = crate::criterion::sign_independency::test(byte_vec.as_slice(), a);
        let sign_uniformity = crate::criterion::sign_uniformity::test(byte_vec.as_slice(), a);
        let sequence_homogeneity =
            crate::criterion::sequence_homogeneity::test(byte_vec.as_slice(), r, a);
        let sequence_homogeneity0;
        let sequence_homogeneity1;
        let sequence_homogeneity2;
        let sequence_homogeneity3;
        if sequence_homogeneity.is_none() {
            println!("{}", "there are no sequence homogeneity test: bytevec length is not enough to divide it into intervals");
            sequence_homogeneity0 = String::from("-");
            sequence_homogeneity1 = String::from("-");
            sequence_homogeneity2 = String::from("-");
            sequence_homogeneity3 = String::from("-");
        } else {
            sequence_homogeneity0 = sequence_homogeneity.unwrap().0.to_string();
            sequence_homogeneity1 = sequence_homogeneity.unwrap().1.to_string();
            sequence_homogeneity2 = sequence_homogeneity.unwrap().2.to_string();
            sequence_homogeneity3 = format!("{:?}", sequence_homogeneity.unwrap().3);
        }
        let mut table = Table::new();
        let test_name: &str = $test_name;
        table.add_row(row![
            test_name,
            "Sign independency",
            "Sign uniformity",
            "Sequence homogeneity"
        ]);
        table.add_row(row![
            "statistic <= critic",
            sign_independency.0,
            sign_uniformity.0,
            sequence_homogeneity0,
        ]);
        table.add_row(row![
            "statistic_value",
            sign_independency.1,
            sign_uniformity.1,
            sequence_homogeneity1
        ]);
        table.add_row(row![
            "critic_value",
            sign_independency.2,
            sign_uniformity.2,
            sequence_homogeneity2
        ]);
        table.add_row(row![
            "time elapsed",
            format!("{:?}", sign_independency.3),
            format!("{:?}", sign_uniformity.3),
            sequence_homogeneity3
        ]);
        table.printstd();
        println!();
    }};
}

#[macro_export]
macro_rules! bitvec_write_to_file {
    ($bitvec:expr, $name: expr, $file: expr) => {{
        use std::io::Write;
        let bitvec = $bitvec.to_bitvec();
        let name: &str = $name;
        let file: &mut File = $file;
        let mut buf = String::new();
        buf.push_str(format!("{name}: ").as_str());

        let d = bitvec.as_bitptr().pointer() as *const u8;
        let mut leen = bitvec.len();
        let mut start = 0isize;
        while leen >= 8 {
            let on_pos = unsafe { *(d.offset(start)) };
            buf.push_str(format!("{:08b}", on_pos).as_str());
            leen -= 8;
            start += 1;
        }
        if leen > 0 {
            println!("{leen}");
            unreachable!("something left to read, so something failed");
        }
        buf.push('\n');
        write!(file, "{}", buf).unwrap();
    }};
}
