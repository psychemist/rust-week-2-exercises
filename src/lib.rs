use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // Decode hex string into Vec<u8>, return error string on failure
    match decode(hex_str) {
        Ok(raw_bytes) => Ok(raw_bytes),
        Err(_) => Err(String::from("Could not decode hex string"))
    }
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // Reverse the byte order of input slice and return as Vec<u8>
    let mut bytes_vec: Vec<u8> = bytes.try_into().unwrap();
    bytes_vec.reverse();
    bytes_vec
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // Implement conversion of bytes slice to hex string
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // Implement conversion of hex string to bytes vector
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    dbg!("0: {}", num);
    let swapped_bytes = num.swap_bytes();
    dbg!("0: {}", swapped_bytes);

    // Check if system is little-endian
    if cfg!(target_endian = "little") {
        // If input is in little-endian, convert to big-endian
        dbg!("1: {}\n", num.to_le_bytes());
        dbg!("1: {}\n", num.to_be_bytes());
        dbg!("1: {}\n", swapped_bytes.to_le_bytes());
        dbg!("1: {}\n", swapped_bytes.to_be_bytes());
        swapped_bytes.to_be_bytes()
    } else {
        // If input is in big-endian, convert to little-endian
        dbg!("2: {}\n", num.to_le_bytes());
        dbg!("2: {}\n", num.to_be_bytes());
        dbg!("2: {}\n", swapped_bytes.to_le_bytes());
        dbg!("2: {}\n", swapped_bytes.to_be_bytes());
        swapped_bytes.to_le_bytes()
    }
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // Parse input string to u64, return error string if invalid
    match input.parse::<u64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(String::from("Invalid satoshi amount"))
    }
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // Match script pattern and return corresponding ScriptType
    if script.len() == 3 {
        match (script[0], script[1], script[2]) {
            // P2PKH: OP_DUP OP_HASH160 0P_PUSHBYTES_20
            (0x76, 0xa9, 0x14) => ScriptType::P2PKH,
            // P2WPKH: 0P_0 0P_PUSHBYTES_20
            (0x00, 0x14, _) => ScriptType::P2WPKH,
            _ => ScriptType::Unknown
        }
    } else {
        ScriptType::Unknown
    }
}

// // TODO: complete Outpoint tuple struct
// pub struct Outpoint();

// pub fn read_pushdata(script: &[u8]) -> &[u8] {
//     // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
// }

// pub trait Wallet {
//     fn balance(&self) -> u64;
// }

// pub struct TestWallet {
//     pub confirmed: u64,
// }

// impl Wallet for TestWallet {
//     fn balance(&self) -> u64 {
//         // TODO: Return the wallet's confirmed balance
//     }
// }

// pub fn apply_fee(balance: &mut u64, fee: u64) {
//     // TODO: Subtract fee from mutable balance reference
// }

// pub fn move_txid(txid: String) -> String {
//     // TODO: Return formatted string including the txid for display or logging
// }

// // TODO: Add necessary derive traits
// pub enum Opcode {
//     OpChecksig,
//     OpDup,
//     OpInvalid,
// }

// impl Opcode {
//     pub fn from_byte(byte: u8) -> Result<Self, String> {
//         // TODO: Implement mapping from byte to Opcode variant
//     }
// }

// // TODO: Add necessary derive traits
// pub struct UTXO {
//     pub txid: Vec<u8>,
//     pub vout: u32,
//     pub value: u64,
// }

// pub fn consume_utxo(utxo: UTXO) -> UTXO {
//     // TODO: Implement UTXO consumption logic (if any)
// }
