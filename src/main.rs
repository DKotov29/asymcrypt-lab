mod convert;
mod criterion;
mod generator;
mod macros;

use crate::convert::{u8vec_to_bits, WrapperBitVec};
use bitvec::prelude::*;
use num_bigint::BigUint;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

#[derive(Debug, Deserialize)]
struct InputSeqGen {
    general: InputSeqGenGeneral,
    buildin: InputSeqGenBuildin,
    lehmer: InputSeqGenLehmer,
    l20: InputSeqGenL20,
    l89: InputSeqGenL89,
    geffe: InputSeqGenGeffe,
    librarian: InputSeqGenLibrarian,
    wolfram: InputSeqGenWolfram,
    bm: InputSeqGenBm,
    bm_bytes: InputSeqGenBmBytes,
    bbs: InputSeqGenBbs,
    bbs_bytes: InputSeqGenBbsBytes,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenGeneral {
    a: f64,
    r: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenBuildin {
    bytes_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenLehmer {
    start_num: u32,
    bytes_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenL20 {
    start_num: u32,
    bits_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenL89 {
    start_num: Vec<u8>,
    bits_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenGeffe {
    x: Vec<u8>,
    y: Vec<u8>,
    s: Vec<u8>,
    bits_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenLibrarian {
    text: String,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenWolfram {
    start_num: u32,
    bits_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenBm {
    p: String,
    a: String,
    q: String,
    t: String,
    bits_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenBmBytes {
    p: String,
    a: String,
    q: String,
    t: String,
    bytes_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenBbs {
    p: String,
    q: String,
    r: String,
    bits_add: usize,
}

#[derive(Debug, Deserialize)]
struct InputSeqGenBbsBytes {
    p: String,
    q: String,
    r: String,
    bytes_add: usize,
}

fn main() {
    let main_start_time = Instant::now();
    let input: InputSeqGen;
    {
        let mut input_info_file =
            File::open("input_info.toml").expect("Failed to read input_info.toml file");
        let mut input_info_file_content = String::new();
        input_info_file
            .read_to_string(&mut input_info_file_content)
            .expect("Failed to read file");
        input = toml::from_str(input_info_file_content.as_str())
            .expect("Failed to deserialize input_info.toml file");
    }
    let mut output = File::create("seq.txt").expect("File seq.txt failes (while creating)");

    let r = input.general.r;
    println!("r: {r}");
    let a = input.general.a;
    println!("a: {a}");
    {
        let buildin = generator::buildin::generate(input.buildin.bytes_add);
        bitvec_write_to_file!(buildin, "buidin sequence", &mut output);
        test_n_print!(buildin, r, a, "buildin sequence");
    }
    {
        let num = input.lehmer.start_num;
        let bytes_num = input.lehmer.bytes_add;

        let lehmer_low = generator::lehmer::low(num, bytes_num);
        bitvec_write_to_file!(lehmer_low, "lehmer low sequence", &mut output);
        test_n_print!(lehmer_low, r, a, "lehmer low sequence");

        let lehmer_high = generator::lehmer::high(num, bytes_num);
        bitvec_write_to_file!(lehmer_high, "lehmer high sequence", &mut output);
        test_n_print!(lehmer_high, r, a, "lehmer high sequence");
    }
    {
        match generator::l20::generate(BitArray::from(input.l20.start_num), input.l20.bits_add) {
            Some(l20) => {
                bitvec_write_to_file!(l20, "l20 sequence", &mut output);
                test_n_print!(l20, r, a, "l20 sequence");
            }
            None => {
                println!("l20 sequence wasnt created, starting bits are len!=20");
            }
        }
    }
    {
        let mut vec = BitVec::with_capacity(89);
        input.l89.start_num.iter().for_each(|x| vec.push(*x != 0));
        match generator::l89::generate(vec, input.l89.bits_add) {
            Some(l89) => {
                bitvec_write_to_file!(l89, "l89 sequence", &mut output);
                test_n_print!(l89, r, a, "l89 sequence");
            }
            None => {
                println!("l89 sequence wasnt created, starting bits are len<89");
            }
        }
    }
    {
        match generator::geffe::generate(
            u8vec_to_bits(input.geffe.x.as_slice()),
            u8vec_to_bits(input.geffe.y.as_slice()),
            u8vec_to_bits(input.geffe.s.as_slice()),
            input.geffe.bits_add,
        ) {
            Some(geffe) => {
                bitvec_write_to_file!(geffe, "geffe sequence", &mut output);
                test_n_print!(geffe, r, a, "geffe sequence");
            }
            None => {
                println!("geffe sequence wasnt created, starting bits are not that length (x 11, y 9, s 10)");
            }
        }
    }
    {
        let librarian = generator::librarian::convert(input.librarian.text.as_str());
        bitvec_write_to_file!(librarian, "librarian sequence", &mut output);
        test_n_print!(librarian, r, a, "librarian sequence");
    }
    {
        let wolfram =
            generator::wolfram::generate(input.wolfram.start_num, input.wolfram.bits_add).unwrap();
        bitvec_write_to_file!(wolfram, "wolfram sequence", &mut output);
        test_n_print!(wolfram, r, a, "wolfram sequence");
    }
    {
        let bm = generator::bm::generate(BigUint::parse_bytes(input.bm.p.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bm.a.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bm.q.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bm.t.as_bytes(), 16).unwrap(),
                                         input.bm.bits_add ).unwrap();
        bitvec_write_to_file!(bm, "bm sequence", &mut output);
        test_n_print!(bm, r, a, "bm sequence");
    }
    {
        let bm_bytes = generator::bm_bytes::generate(BigUint::parse_bytes(input.bm_bytes.p.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bm_bytes.a.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bm_bytes.q.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bm_bytes.t.as_bytes(), 16).unwrap(),
                                         input.bm_bytes.bytes_add ).unwrap();
        bitvec_write_to_file!(bm_bytes, "bm_bytes sequence", &mut output);
        test_n_print!(bm_bytes, r, a, "bm_bytes sequence");
    }
    {
        let bbs = generator::bbs::generate(BigUint::parse_bytes(input.bbs.p.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bbs.q.as_bytes(), 16).unwrap(),
                                         BigUint::parse_bytes(input.bbs.r.as_bytes(), 16).unwrap(),
                                         input.bbs.bits_add ).unwrap();
        bitvec_write_to_file!(bbs, "bbs sequence", &mut output);
        test_n_print!(bbs, r, a, "bbs sequence");
    }
    {
        let bbs_bytes = generator::bbs_bytes::generate(
            BigUint::parse_bytes(input.bbs_bytes.p.as_bytes(), 16).unwrap(),
            BigUint::parse_bytes(input.bbs_bytes.q.as_bytes(), 16).unwrap(),
            BigUint::parse_bytes(input.bbs_bytes.r.as_bytes(), 16).unwrap(),
            input.bbs_bytes.bytes_add,
        )
        .unwrap();
        bitvec_write_to_file!(bbs_bytes, "bbs_bytes sequence", &mut output);
        test_n_print!(bbs_bytes, r, a, "bbs_bytes sequence");
    }

    let elapsed = main_start_time.elapsed();
    println!("exec duration: {elapsed:?}");
}
