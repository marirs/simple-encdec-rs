use std::collections::VecDeque;

/// Decrypt a given encrypted string
///
/// ## Example
/// ```rust
/// use simple_encdec::decrypt;
///
/// let x = decrypt("a=GQVGsbbyG982gd");
/// assert!(x.is_some());
/// assert_eq!(x.unwrap(), "hello world")l
/// ```
pub fn decrypt(s: &str) -> Option<String> {
    let mut data = VecDeque::<char>::new();

    for c in s.chars().collect::<Vec<_>>().chunks(2) {
        if c.len() < 2 {
            return None;
        }
        data.push_back(c[0]);
        data.push_front(c[1]);
    }

    data.rotate_left(data.len() / 2);

    data.iter()
        .enumerate()
        .filter_map(|(j, val)| {
            val.to_string()
                .trim()
                .parse::<i32>()
                .map(|val_parsed| (j, val_parsed))
                .ok()
        })
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .for_each(|items| {
            if let [x, y] = items {
                let xor = x.1 ^ y.1;
                if xor < 10 {
                    data[x.0] = xor.to_string().parse().unwrap();
                }
            }
        });

    let base64_enc = data.drain(..).collect::<String>();

    match base64::decode(base64_enc) {
        Ok(s) => String::from_utf8(s).ok(),
        _ => None,
    }
}
/// Encrypt a given plain text string
///
/// ## Example
/// ```rust
/// use simple_encdec::encrypt;
///
/// let x = encrypt("hello world");
/// assert!(x.is_some());
/// assert_eq!(x.unwrap(), "a=GQVGsbbyG982gd");
/// ```
pub fn encrypt(s: &str) -> Option<String> {
    let base64_enc = base64::encode(s.as_bytes());
    let mut data = base64_enc.as_bytes().to_vec();
    let mut first: Option<(usize, u8)> = None;
    for (i, c) in base64_enc.as_bytes().iter().enumerate() {
        if (0x30..0x3a).any(|x| x == *c) {
            if let Some(s) = first {
                let f = s.1 ^ (c - 0x30);
                if f < 10 {
                    data[s.0] = f + 0x30;
                }
                first = None;
            } else {
                first = Some((i, c - 0x30));
            }
        }
    }

    let mut ss = vec![];
    for i in 0..data.len() / 2 {
        ss.push(data[i]);
        ss.push(data[data.len() - 1 - i]);
    }
    if data.len() % 2 > 0 {
        ss.push(data[data.len() / 2 + 1]);
    }
    String::from_utf8(ss).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original = "hello world";
        let encoded = encrypt(original);
        assert!(encoded.is_some());
        let decoded = decrypt(&encoded.unwrap());
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), original);
    }
}
