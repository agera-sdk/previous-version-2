use lazy_regex::{regex_replace_all};

/// Escapes certain URI characters. Escapes all characters except:
/// ```text
/// A–Z a–z 0–9 - _ . ! ~ * ' ( )
/// 
/// ; / ? : @ & = + $ , #
/// ```
pub fn encode_uri(s: impl AsRef<str>) -> String {
    regex_replace_all!(r"[^A-Za-z0-9\-_\.!\~*'();/?:@&=+&,\#]", s.as_ref(), |seq: &str| {
        let mut r = String::new();
        for ch in seq.to_owned().bytes() {
            r.push('%');
            r.push_str(octet_to_hex(ch).as_ref());
        }
        r.clone()
    }).into_owned()
}

/// Decodes URIs previously creatd by `encode_uri`.
/// Any invalid character sequences are ignored.
pub fn decode_uri(s: impl AsRef<str>) -> String {
    regex_replace_all!(r"(%[A-Fa-f0-9]{2})+", s.as_ref(), |seq: &str, _| {
        let mut r = Vec::<u8>::new();
        let inp: Vec<u8> = seq.to_owned().bytes().collect();
        let mut i: usize = 0;
        while i != inp.len() {
            r.push(u8::from_str_radix(String::from_utf8_lossy(&[inp[i + 1], inp[i + 2]]).as_ref(), 16).unwrap_or(0));
            i += 3;
        }
        String::from_utf8_lossy(r.as_ref()).into_owned().to_owned()
    }).into_owned()
}

/// Escapes certain characters from URI component. Escapes all characters except:
/// ```text
/// A–Z a–z 0–9 - _ . ! ~ * ' ( )
/// ```
pub fn encode_uri_component(s: impl AsRef<str>) -> String {
    regex_replace_all!(r"[^A-Za-z0-9\-_\.!~*'()]", s.as_ref(), |seq: &str| {
        let mut r = String::new();
        for ch in seq.to_owned().bytes() {
            r.push('%');
            r.push_str(octet_to_hex(ch).as_ref());
        }
        r.clone()
    }).into_owned()
}

/// Decodes URI components previously creatd by `encode_uri_component`.
/// Any invalid character sequences are ignored.
pub fn decode_uri_component(s: impl AsRef<str>) -> String {
    decode_uri(s)
}

fn octet_to_hex(arg: u8) -> String {
    format!("{:02X}", arg)
}