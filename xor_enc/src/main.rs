use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = &args[1];
    let key = &args[2];

    let encrypted = xor_cypher(text.to_string(), key.to_string());
    println!("{:?}", encrypted);
    
    let decrypted = xor_cypher(encrypted, key.to_string());
    println!("{:?}", decrypted);
}

fn xor_cypher(_text: String, _key: String) -> String {
    let key_bytes = _key.as_bytes();
    let mut result_array = vec![0; _text.len()];
    for (i, char_byte) in _text.as_bytes().iter().enumerate() {
        result_array[i] = char_byte ^ &key_bytes[i % _key.len()];
    }
    return String::from_utf8(result_array).unwrap();
}
