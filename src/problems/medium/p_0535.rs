use std::collections::HashMap;

struct Codec {
    dictionary: HashMap<String, String>,
}

impl Codec {
    fn new() -> Self {
        Self {
            dictionary: HashMap::new(),
        }
    }

    fn encode(&mut self, longURL: String) -> String {
        let path = self.get_url_path(longURL);
        todo!()
    }

    fn decode(&self, shortURL: String) -> String {
        let url = self.get_url_path(shortURL);
        todo!()
    }

    fn get_url_path(&self, url: String) -> String {
        let domain = url.split("://").collect::<Vec<_>>()[1];
        let b = domain.split('/').nth(1).unwrap_or("");
        b.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        // let url = "https://leetcode.com/problems/design-tinyurl";
        // let expected = "https://leetcode.com/...";
        //
        // let c = Codec::new();
        // let tiny = c.encode(url.to_string());
        // assert_eq!(expected, tiny);
        // let origin = c.decode(tiny.to_string());
        // assert_eq!(url, origin);
    }
}
