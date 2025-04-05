use std::env;
use std::str::FromStr;
use num_bigint::BigUint;
use num_traits::Num; //


fn to_field_bytes(value: &BigUint) -> String {
    let mut bytes = value.to_bytes_be();
    while bytes.len() < 32 {
        bytes.insert(0, 0);
    }
    format!("0x{}", hex::encode(bytes))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return;
    }

    let input = &args[1];
    let value = if input.starts_with("0x") || input.starts_with("0X") {
        BigUint::from_str_radix(&input[2..], 16)
    } else {
        BigUint::from_str(input)
    };

    match value {
        Ok(val) => {
            let clean_hex = format!("0x{:x}", val);
            let padded_hex = to_field_bytes(&val);

            let decoded = hex::decode(&padded_hex.trim_start_matches("0x"))
                .expect("Ошибка при декодировании hex");
            let back_to_decimal = BigUint::from_bytes_be(&decoded);

            println!("🧮 Десятичное:    {}", val);
            println!("🔹 Hex без паддинга: {}", clean_hex);
            println!("🔸 Hex с паддингом:  {}", padded_hex);
            println!("↩ Обратно в dec:  {}", back_to_decimal);
        }
        Err(_) => {
            eprintln!("❌ Неверный формат ввода.");
        }
    }
}
