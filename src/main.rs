mod xorshift;
mod lcg;

use crate::lcg::LCG;
use crate::xorshift::Xorshift64;
use std::io::Write;

const SEED: u64 = 1972487521648;
const NUM_BYTES: u64 = 20_000_000;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Некорректный запуск! \nПример запуска -> 'cargo run -- lcg' || 'cargo run -- xorshift'");
        return
    }

    match args[1].as_str() {
        "lcg" => {
            let mut lcg = LCG::new(SEED);
            let mut file = std::fs::File::create("lcg.bin")
                .expect("Ошибка создания файла!");

            let mut buffer = Vec::with_capacity(NUM_BYTES as usize);
            for _ in 0..NUM_BYTES {
                buffer.push(lcg.next_u8());
            }

            file.write_all(&buffer)
                .expect("Ошибка записи в файл!");
        }
        "xorshift" => {
            let mut xorshift64 = Xorshift64::new(SEED);
            let mut file = std::fs::File::create("xorshift.bin")
                .expect("Ошибка создания файла!");

            let mut buffer = Vec::with_capacity(NUM_BYTES as usize);
            for _ in 0..NUM_BYTES / 8 {
                let random_value = xorshift64.next();
                buffer.extend_from_slice(&random_value.to_le_bytes());
            }

            file.write_all(&buffer)
                .expect("Ошибка записи в файл!");
        }
        _ => {eprintln!("Некорректный аргумент! \nИспользуйте 'lcg' или 'xorshift'")}
    }
}
