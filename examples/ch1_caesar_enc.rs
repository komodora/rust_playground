fn encrypt(text: &str, shift: i16) -> String {
    // 文字コードに変換
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    let mut result = String::new();
    for ch in text.chars() {
        let mut code = ch as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        // 文字コードから文字に戻す(一度u8を経由)
        result.push((code as u8) as char)
    }
    return result;
}

fn encrypt2(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c - a + shift + 26) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    return text.chars().map(|c| enc1(c)).collect();
}
fn main() {
    let enc = encrypt("I LOVE YOU", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);

    let enc = encrypt2("I LOVE YOU", 3);
    let dec = encrypt2(&enc, -3);
    println!("{} => {}", enc, dec);
}
