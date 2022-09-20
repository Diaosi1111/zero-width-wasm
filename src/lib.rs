mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::char;

const ZWSP: &str = "\u{200B}"; // 1
const ZWNJ: &str = "\u{200C}"; // 0
const ZWJ: &str = "\u{200D}"; // bytes intent
const ZWNB: &str = "\u{FEFF}"; // word

fn to_bits(s: &str) -> String {
    let mut binary = vec![];
    for character in s.as_bytes() {
        binary.push(format!("{:b}", *character));
    }
    binary.join(" ")
}

#[wasm_bindgen]
pub fn zero_width_encode(s: &str) -> String {
    let bin_str = to_bits(s);
    bin_str
        .replace("0", ZWNJ)
        .replace("1", ZWSP)
        .replace(" ", ZWJ)
}

#[wasm_bindgen]
pub fn zero_width_decode(s: &str) -> String {
    let mut bin = vec![];
    let bin_word = s.replace(ZWJ, " ").replace(ZWSP, "1").replace(ZWNJ, "0");
    for byte in bin_word.split(" ") {
        // println!("{}", byte);
        // println!("{:?}", u8::from_str_radix(byte, 2));
        bin.push(u8::from_str_radix(byte, 2).unwrap());
    }
    String::from_utf8(bin).unwrap()
}

#[wasm_bindgen]
pub fn wrap(data: &str, msg: &str) -> String {
    let mut tmp = String::new();
    if data.len() == 1 {
        tmp.push_str(data);
        tmp.push_str(ZWNB);
        tmp.push_str(&*zero_width_encode(msg));
        tmp.push_str(ZWNB);
    } else {
        let encoded = zero_width_encode(msg);
        let mut encoded_chars = data.chars();
        tmp.push(encoded_chars.next().unwrap());
        tmp.push_str(ZWNB);
        tmp.push_str(&*encoded);
        tmp.push_str(ZWNB);
        tmp.extend(encoded_chars.collect::<Vec<char>>());
    }
    tmp
}

#[wasm_bindgen]
pub fn unwrap(msg: &str) -> String {
    let mut tmp = msg.split(ZWNB);
    tmp.next();
    let key = tmp.next().unwrap();
    zero_width_decode(key)
}
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn messagebox(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_unwrap() {
        assert_eq!(unwrap(&wrap("你好", "111")), "111");
    }

    // #[test]
    // fn test_bad_add() {
    //     // 这个断言会导致测试失败。注意私有的函数也可以被测试！
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
