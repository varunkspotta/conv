use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arg {
    number: String,
}

fn as_hex(number: u32) -> String {
    format!("{:0>8X}", number) // formats number to hex string, padded with 0's
        .as_bytes()
        .chunks(2)
        .skip_while(|chunk| chunk == &[48, 48]) // 48 is '0' in ascii, we skip chunks that start with 00
        .map(|chunk| std::str::from_utf8(chunk).unwrap()) // convert ascii chunks to string refs
        .collect::<Vec<&str>>() // collect into a vector of string refs
        .join("") // collect into final string
}

fn as_binary(number: u32) -> String {
    format!("{:0>32b}", number) // formats number to binary string of 32 bits, padded with 0's
        .as_bytes()
        .chunks(8)
        .skip_while(|chunk| chunk == &[48, 48, 48, 48, 48, 48, 48, 48]) // 48 is '0' in ascii, we skip chunks that start with 00000000
        .map(|chunk| std::str::from_utf8(chunk).unwrap()) // convert ascii chunks to string refs
        .collect::<Vec<&str>>() // collect into a vector of string refs
        .join(" ") // collect into final string, seperating each chunks of 8bits with a space
}

fn main() {
    let arg = Arg::parse();

    let num = if arg.number.starts_with("0b") {
        u32::from_str_radix(&arg.number[2..], 2).expect("Invalid binary number")
    } else if arg.number.starts_with("0x") {
        u32::from_str_radix(&arg.number[2..], 16).expect("Invalid hex number")
    } else {
        arg.number.parse::<u32>().expect("Invalid decimal number")
    };

    println!("Dec : {}", num);
    println!("Hex : 0x{}", as_hex(num));
    println!("Bin : {}", as_binary(num));
}
