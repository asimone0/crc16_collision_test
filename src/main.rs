use crc16::*;

fn main() {
    let crc16_target = State::<XMODEM>::calculate(b"UCF");

    let mut checks = 0;
    let mut matches = 0;

    for a in 0..=255 {
        let a8 = a as u8;
        for b in 0..=255 {
            let b8 = b as u8;
            for c in 0..=255 {
                let c8 = c as u8;
                let test_case: [u8; 3] = [a8, b8, c8];

                let crc16 = State::<XMODEM>::calculate(&test_case);
                checks += 1;

                if crc16_target == crc16 {
                    matches += 1;

                    let text = test_case.iter().map(|i| *i as char).collect::<String>();
                    println!("{}.) {} {:02x?}", matches, text, test_case)
                }
            }
        }
    }
    println!("total checked: {}", checks);
    println!("total matches: {}", matches);
}
