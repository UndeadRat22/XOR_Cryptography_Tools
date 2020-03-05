use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1].to_string();
    let key = &args[2];

    let mut file = File::open(file_name)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let cyphered = xor_cypher(file_content, key.to_string());
    println!("{:?}", cyphered);

    let mut file = File::create("C:\\Users\\Valdas\\Desktop\\result.bin")?;
    file.write_all(cyphered.as_bytes())?;

    file.sync_data()?;

    Ok(())
}

fn xor_cypher(_text: String, _key: String) -> String {
    let key_bytes = _key.as_bytes();
    let mut result_array = vec![0; _text.len()];
    for (i, char_byte) in _text.as_bytes().iter().enumerate() {
        result_array[i] = char_byte ^ &key_bytes[i % _key.len()];
    }
    return String::from_utf8(result_array).unwrap();
}