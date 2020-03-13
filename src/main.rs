mod syntax;

use combine::parser::byte::*;
use combine::parser::*;
use combine::stream::position;
use combine::*;
use combine::{EasyParser, Parser};

// use std::convert::
fn main() {
    let input = b"c0:)\x04\x00\x00\x001234";
    let res = p_document().easy_parse(input);
    match res {
        Ok((parse, rest)) => println!("Yay! {:?}", parse),
        Err(e) => println!("{:?}", e),
    }
}

fn p_u32(res: &[u8]) -> u32 {
    (res[0] as u32) + ((res[1] as u32) << 8) + ((res[2] as u32) << 16) + ((res[3] as u32) << 24)
}

fn p_document<'a>() -> impl combine::EasyParser<&'a [u8], Output = Vec<u8>> {
    p_header().then(|len| range::take(len as usize).map(|res: &[u8]| res.to_vec()))
}

fn p_header<'a>() -> impl combine::EasyParser<&'a [u8], Output = u32> {
    bytes(b"c0:)").then(|_| range::take(4).map(|res: &[u8]| p_u32(res)))
}
