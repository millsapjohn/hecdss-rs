fn main() -> () {
    let bytes =
        std::fs::read("/mnt/c/Users/john/Documents/Coding/Standalone/hecdss-rs/src/test.dss")
            .unwrap();
    let slice: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    for byte in slice {
        let new_array = [byte, 0, 0, 0];
        let num = u32::from_le_bytes(new_array);
        let letter = char::from_u32(num).unwrap();
        println!("{}", letter);
    }
    // let num = u32::from_le_bytes(slice);
    // let letter = char::from_u32(num).unwrap();
    // println!("{}", letter);
}
