pub mod class1;

use crate::{
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology,
};
use wfts_pedia_ssg::{
    component::{
        table::{self, Table},
        Component,
        DynComponent,
        InlineComponent,
    },
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

pub fn make_inflection_table<T, F>(
    title: T,
    mut make_data: F,
) -> Table<T, DynComponent>
where
    F: FnMut(BasicCase, Gender, Number) -> DynComponent,
    T: Component<Kind = InlineComponent>,
{
    let mut table = Table { title, entries: vec![] };
    let mut row = vec![table::Entry {
        header: true,
        rowspan: 2,
        colspan: 2,
        data: "".blocking().to_dyn(),
    }];
    for &number in Number::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: 3,
            data: number.to_string().capitalize().blocking().to_dyn(),
        });
    }
    table.entries.push(row);

    row = Vec::new();
    for _ in Number::ALL {
        for &gender in Gender::ALL {
            row.push(table::Entry {
                header: true,
                rowspan: 1,
                colspan: 1,
                data: gender.to_string().capitalize().blocking().to_dyn(),
            });
        }
    }
    table.entries.push(row);

    for &case in BasicCase::ALL {
        row = Vec::new();
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: 2,
            data: case.to_string().capitalize().blocking().to_dyn(),
        });
        for &number in Number::ALL {
            for &gender in Gender::ALL {
                row.push(table::Entry::new(make_data(case, gender, number)));
            }
        }
        table.entries.push(row);
    }

    table
}
