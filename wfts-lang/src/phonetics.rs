use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phone {
    A,
    Ae,
    Ao,
    AoRaised,
    E,
    EMid,
    I,
    IMidCent,
    O,
    OMid,
    U,
    UMidCent,
    NonSylI,
    P,
    B,
    T,
    D,
    C,
    Gj,
    K,
    G,
    F,
    Ph,
    V,
    Bh,
    S,
    Z,
    Ch,
    Jh,
    X,
    Gh,
    H,
    M,
    N,
    Nj,
    Ng,
    W,
    Bw,
    Vw,
    L,
    R,
    Rd,
    J,
    SylBreak,
    Stress,
    SecStress,
    Hyphen,
}

impl fmt::Display for Phone {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            Phone::A => "a",
            Phone::Ae => "æ",
            Phone::Ao => "ɒ",
            Phone::AoRaised => "ɒ̝",
            Phone::E => "e",
            Phone::EMid => "e̞",
            Phone::I => "i",
            Phone::IMidCent => "ɪ",
            Phone::O => "o",
            Phone::OMid => "o̞",
            Phone::U => "u",
            Phone::UMidCent => "ʊ",
            Phone::NonSylI => "i̯",
            Phone::P => "p",
            Phone::B => "b",
            Phone::T => "t",
            Phone::D => "d",
            Phone::C => "c",
            Phone::Gj => "ɟ",
            Phone::K => "k",
            Phone::G => "g",
            Phone::F => "f",
            Phone::Ph => "ɸ",
            Phone::V => "v",
            Phone::Bh => "β",
            Phone::S => "s",
            Phone::Z => "z",
            Phone::Ch => "ç",
            Phone::Jh => "ʝ",
            Phone::X => "x",
            Phone::Gh => "ɣ",
            Phone::H => "h",
            Phone::M => "m",
            Phone::N => "n",
            Phone::Nj => "ɲ",
            Phone::Ng => "ŋ",
            Phone::W => "w",
            Phone::Bw => "β̞",
            Phone::Vw => "ʋ",
            Phone::L => "l",
            Phone::R => "ɹ",
            Phone::Rd => "ɾ",
            Phone::J => "j",
            Phone::SylBreak => ".",
            Phone::Stress => "ˈ",
            Phone::SecStress => "ˌ",
            Phone::Hyphen => "-",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Pronunc {
    root: Vec<Phone>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Variation {
    pronuncs: Vec<Pronunc>,
}

impl Variation {
    pub fn add_phones(&mut self, phones: &[Phone]) {
        let capacity = phones.len() * self.pronuncs.len();
        let mut new_pronuncs = Vec::with_capacity(capacity);
        for &phone in phones {
            for pronunc in &self.pronuncs {
                let mut pronunc = pronunc.clone();
                pronunc.root.push(phone);
                new_pronuncs.push(pronunc);
            }
        }
        self.pronuncs = new_pronuncs;
    }
}
