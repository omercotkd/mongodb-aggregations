pub(crate) trait ToCamalCase {
    fn to_camal_case(&self) -> String;
}

impl ToCamalCase for String {
    #[doc = "Converts a snake_case string to a camelCase string."]
    fn to_camal_case(&self) -> String {
        let mut result = String::from("");
        let mut make_upper = false;
        let mut first = true;

        for c in self.chars() {
            match c {
                '_' => make_upper = true,
                _ if first => {
                    result.push(c.to_ascii_lowercase());
                    first = false;
                }
                _ if make_upper => {
                    result.push(c.to_ascii_uppercase());
                    make_upper = false;
                }
                _ => result.push(c),
            }
        }

        result
    }
}