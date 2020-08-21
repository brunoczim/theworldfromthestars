pub mod variable1;
pub mod divine1;

use crate::{
    dictionary::Entry,
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology,
};
use wfts_pedia_ssg::{
    component::{table, Component, DynComponent},
    fmt::StrExt,
};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in variable1::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in divine1::definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub case: BasicCase,
    pub gender: Gender,
    pub number: Number,
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

pub fn fixed_gender_inflection_table<F>(
    gender: Gender,
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(BasicCase, Number) -> DynComponent,
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
            row.push(table::Entry::new(make_data(case, number)));
        }
        table.push(row);
        row = Vec::new();
    }

    table
}
