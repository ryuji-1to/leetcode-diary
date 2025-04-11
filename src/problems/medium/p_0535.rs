struct Codec {
    dictionary: (String, String),
    original: String,
    scheme: String,
}

impl Codec {
    fn new() -> Self {
        Self {
            dictionary: ("".to_string(), "".to_string()),
            original: String::new(),
            scheme: String::new(),
        }
    }

    fn encode(&mut self, longURL: String) -> String {
        let path = self.get_url_path(longURL);
        let split = path.split_once('/').unwrap();
        let encoded = split
            .1
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|x| {
                let a = x.get(0);
                let b = x.get(1);
                let a = if let Some(&c) = a { c as u8 } else { 1 };
                let b = if let Some(&c) = b { c as u8 } else { 1 };
                ((a + b) % 10 + b'a') as char
            })
            .collect::<String>();
        let mut result = format!("{}://{}/{}", self.scheme, split.0, encoded);
        self.original = split.0.to_string();
        self.dictionary = (split.1.to_string(), encoded);
        result
    }

    fn decode(&self, shortURL: String) -> String {
        format!("{}://{}/{}", self.scheme, self.original, self.dictionary.0)
    }

    fn get_url_path(&mut self, url: String) -> String {
        let domain = url.split("://").collect::<Vec<_>>();
        self.scheme = domain[0].to_string();
        domain[1].into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let url = "https://leetcode.com/problems/design-tinyurl";

        let mut c = Codec::new();
        let encoded = c.encode(url.to_string());
        let decoded = c.decode(encoded);
        assert_eq!(url, decoded);
    }
}
