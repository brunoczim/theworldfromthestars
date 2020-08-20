pub mod class1;

use crate::{
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology,
};
use wfts_pedia_ssg::{
    component::{table, Component, DynComponent},
    fmt::StrExt,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub case: BasicCase,
    pub gender: Gender,
    pub number: Number,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Word {
    Class1 { word: class1::Word, gender: Gender, number: Number },
}

pub fn full_inflection_table<F>(
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(BasicCase, Gender, Number) -> DynComponent,
{
    let mut table = vec![];
    let mut row = vec![table::Entry {
        header: true,
        rowspan: 1,
        colspan: 2,
        data: "".blocking().to_dyn(),
    }];
    for &number in Number::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: 1,
            data: number.to_string().capitalize().blocking().to_dyn(),
        });
    }
    table.push(row);

    row = Vec::new();
    for &gender in Gender::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: BasicCase::ALL.len() as u32,
            colspan: 1,
            data: gender.to_string().capitalize().blocking().to_dyn(),
        });
        for &case in BasicCase::ALL {
            row.push(table::Entry {
                header: true,
                rowspan: 1,
                colspan: 1,
                data: case.to_string().capitalize().blocking().to_dyn(),
            });
            for &number in Number::ALL {
                row.push(table::Entry::new(make_data(case, gender, number)));
            }
            table.push(row);
            row = Vec::new();
        }
    }

    table
}
