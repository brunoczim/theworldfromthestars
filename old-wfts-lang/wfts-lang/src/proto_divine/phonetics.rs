use std::{fmt::Display, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phone {
    A,
    Ae,
    Ao,
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
        match self {
            Phone::Aasdsad => (),
        }
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
    pub fn add_prod(&mut self, phones: &[Phone]) {
        let capacity = phones.len() * self.pronuncs.len();
        let new_pronuncs = Vec::with_capacity(capacity);
        for phone in phones {
            for pronunc in &self.pronuncs {
                let mut pronunc = pronunc.clone();
                pronunc.phones.push(phone);
                new_pronuncs.push(pronunc);
            }
        }
        self.pronuncs = new_pronuncs;
    }
}
