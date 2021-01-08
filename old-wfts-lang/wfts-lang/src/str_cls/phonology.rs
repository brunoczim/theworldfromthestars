use crate::phonology::Phoneme as PhonemeTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phoneme {
    B,
    Gw,
    D,
    J,
    G,
    P,
    Kw,
    T,
    C,
    K,
    M,
    Mg,
    N,
    Nj,
    Ng,
    F,
    Xw,
    W,
    S,
    R,
    Y,
    Ii,
    X,
    I,
    Ee,
    H,
    E,
    Rr,
    A,
    Aa,
}

impl PhonemeTrait for Phoneme {
    fn broad_ipa(&self) -> &str {
        use Phoneme::*;

        match self {
            B => "pʼ",
            Gw => "kʷʼ",
            D => "tʼ",
            J => "cʼ",
            G => "kʼ",
            P => "pʰ",
            Kw => "kʷʰ",
            T => "tʰ",
            C => "cʰ",
            K => "kʰ",
            M => "m",
            Mg => "ŋʷ",
            N => "n",
            Nj => "ɲ",
            Ng => "ŋ",
            F => "ɸ",
            Xw => "xʷ",
            W => "w",
            S => "s",
            R => "ɹ",
            Y => "j",
            Ii => "iː",
            X => "x",
            I => "i",
            Ee => "eː",
            H => "ħ",
            E => "e",
            Rr => "ʕ",
            A => "a",
            Aa => "aː",
        }
    }
}
