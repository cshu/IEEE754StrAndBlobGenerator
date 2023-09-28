use std::fs::*;
use std::io;
use std::io::*;
use std::str;

fn ieee754verify(f64str: &str, blob: [u8; 8]) {
    let f64_from_str = f64str.parse::<f64>().unwrap();
    println!("{}", f64str);
    assert_eq!(f64::from_le_bytes(blob), f64_from_str);
    assert_eq!(u64::from_le_bytes(blob), f64_from_str.to_bits());
}

fn ieee754_verify_pair_str_and_blob(in_file: &str) -> io::Result<()> {
    println!("{}", in_file);
    //let xx = fs::read(in_file);
    let mut p_file = File::open(in_file)?;
    let mut bytesbuf = [0u8; 8];
    let mut strbuf = vec![0u8; u16::MAX.into()];
    loop {
        //let read_len = p_file.read_exact(&mut bytesbuf[0..2]);
        match p_file.read_exact(&mut bytesbuf[0..2]) {
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
            }
            _ => {}
        }
        let str_len = u16::from_le_bytes(bytesbuf[0..2].try_into().unwrap());
        p_file.read_exact(&mut strbuf[0..str_len.into()])?;
        p_file.read_exact(&mut bytesbuf)?;
        ieee754verify(
            str::from_utf8(&strbuf[0..str_len.into()]).unwrap(),
            bytesbuf,
        );
    }
    Ok(())
}

fn main() {
    ieee754_verify_pair_str_and_blob("/tmp/tmp_ieee754_str_n_blob").unwrap();
    println!("Verified");
}
