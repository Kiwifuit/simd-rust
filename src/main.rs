#![feature(portable_simd)]

use rand::random;
use std::simd::*;

fn main() {
    let a = u16x2::from([1, 2]);
    let b = u16x2::from([3, 4]);

    // This counts as [(1 + 3 ), (2 + 4)],
    // which will output [4, 6]
    let c = a + b;

    // reduce_sum sums all the items in
    // the array
    dbg!(c.reduce_sum());

    let a = u8x32::from(random::<[u8; 32]>());
    let b = u8x32::from(random::<[u8; 32]>());

    println!(
        "{0}\nSIMD vector A:\n{1}\n{0}",
        "=".chars().cycle().take(32).collect::<String>(),
        render_vec(&a)
    );
    println!(
        "{0}\nSIMD vector B:\n{1}\n{0}",
        "=".chars().cycle().take(32).collect::<String>(),
        render_vec(&b)
    );

    println!("Hexdump of a + b: ");
    // Code to render a hexdump
    (a + b)
        .as_array()
        .chunks(8)
        .map(|row| {
            (
                row.iter()
                    .map(render_hex)
                    .collect::<Vec<String>>()
                    .join(" "),
                row.iter()
                    .map(render_dec)
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        })
        .enumerate()
        .for_each(|(rowno, (row_hex, row_dec))| {
            println!("0x{:<03x}: {}\t{}", rowno, row_hex, row_dec);
        });
}

#[inline]
fn render_dec(i: &u8) -> String {
    format!("{:>3}", i)
}

#[inline]
fn render_hex(i: &u8) -> String {
    format!("{:<02x}", i)
}

#[inline]
fn render_vec(i: &Simd<u8, 32>) -> String {
    i.as_array()
        .chunks(8)
        .map(|row| {
            row.iter()
                .map(render_dec)
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
