pub trait StrExt {
    fn capitalize_first(&self) -> String;
    fn capitalize(&self) -> String;
}

impl StrExt for str {
    fn capitalize_first(&self) -> String {
        let mut string = String::with_capacity(self.len());
        let mut chars = self.chars();
        if let Some(ch) = chars.next() {
            string.extend(ch.to_uppercase());
            string.push_str(chars.as_str());
        };
        string
    }

    fn capitalize(&self) -> String {
        let mut string = String::with_capacity(self.len());
        let mut slice = self;

        while slice.len() > 0 {
            let index = match slice
                .char_indices()
                .find(|&(i, ch)| i > 0 && !ch.is_alphabetic())
            {
                Some((index, _)) => index + 1,
                None => slice.len(),
            };
            let mut chars = slice[.. index].chars();
            if let Some(ch) = chars.next() {
                string.extend(ch.to_uppercase());
                string.push_str(chars.as_str());
            };
            slice = &slice[index ..];
        }
        string
    }
}
