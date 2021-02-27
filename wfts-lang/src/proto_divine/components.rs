use crate::{
    fmt::{NarrowPronunc, WriteBroadPronunc, WriteOrthography},
    phonetics::Variation,
    proto_divine::{
        grammar::grammemes::{Case, Gender, Number},
        phonology::Word,
    },
};
use std::fmt;
use wfts_pedia_ssg::{
    component::{
        audio::Audio,
        table,
        text::Bold,
        BlockComponent,
        Component,
        Context,
        DynComponent,
    },
    fmt::StrExt,
};

#[derive(Debug, Clone)]
pub struct DefinitionHead {
    pub word: Word,
    pub inflected_for: Vec<String>,
}

impl Component for DefinitionHead {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"definition-head\">*{}",
            ctx.renderer(Bold(self.word.orthography_ref())),
        )?;

        if let Some((first, rest)) = self.inflected_for.split_first() {
            write!(fmt, "(inflected for {}", ctx.renderer(Bold(first)))?;

            for key in rest {
                write!(fmt, ", {}", ctx.renderer(Bold(key)))?;
            }
            write!(fmt, ")")?;
        }

        write!(fmt, "</div>")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PronuncSection {
    broad: String,
    narrow: Variation,
    audio: Option<Audio>,
}

impl PronuncSection {
    pub fn from_word(word: &Word) -> Self {
        PronuncSection {
            broad: word.broad_pronunc().to_string(),
            narrow: word.narrow_pronunc(),
            audio: word.audio(),
        }
    }
}

impl Component for PronuncSection {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"pronunc-phonemic\">Phonemic: <div \
             class=\"pronunc-phonemic-ipa\">{}</div></div>",
            self.broad,
        )?;
        write!(
            fmt,
            "<div class=\"pronunc-phonetic\">Phonetic: <div \
             class=\"pronunc-phonetic-ipa\">{}</div></div>",
            self.narrow,
        )?;

        if let Some(audio) = &self.audio {
            write!(
                fmt,
                "<div class=\"pronunc-audio\">Reconstructed Audio: <div \
                 class=\"pronunc-audio-file\">{}</div></div>",
                ctx.renderer(audio)
            )?;
        }

        Ok(())
    }
}

/*
/// Case table.
pub fn case_table<F>(mut make_data: F) -> table::Entries<DynComponent>
where
    F: FnMut(Case) -> DynComponent,
{
    let mut table = Vec::new();
    let mut row = Vec::new();
    for &case in Case::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: 1,
            data: case.to_string().capitalize().blocking().to_dyn(),
        });
        row.push(table::Entry::new(make_data(case)));
        table.push(row);
        row = Vec::new();
    }

    table
}

/// ClauseCase table.
pub fn ccase_table<F>(mut make_data: F) -> table::Entries<DynComponent>
where
    F: FnMut(ClauseCase) -> DynComponent,
{
    let mut table = Vec::new();
    let mut row = Vec::new();
    for &case in ClauseCase::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: 1,
            data: case.to_string().capitalize().blocking().to_dyn(),
        });
        row.push(table::Entry::new(make_data(case)));
        table.push(row);
        row = Vec::new();
    }

    table
}

*/

/// Case x Gender x Number table.
pub fn case_gender_number_table<F>(
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(Case, Gender, Number) -> DynComponent,
{
    let mut table = Vec::new();
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
            rowspan: Case::ALL.len() as u32,
            colspan: 1,
            data: gender.to_string().capitalize().blocking().to_dyn(),
        });
        for &case in Case::ALL {
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

/// Case x Fixed Gender x Number table.
pub fn case_fgender_number_table<F>(
    gender: Gender,
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(Case, Number) -> DynComponent,
{
    let mut table = Vec::new();
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

/*

/// Person x Case x Gender x Number table.
pub fn person_case_gender_number_table<F>(
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(Person, Case, Gender, Number) -> DynComponent,
{
    let mut table = Vec::new();
    let mut row = vec![table::Entry {
        header: true,
        rowspan: 2,
        colspan: 2,
        data: "".blocking().to_dyn(),
    }];
    for &person in Person::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: Number::ALL.len() as u32,
            data: person.to_string().capitalize().blocking().to_dyn(),
        });
    }
    table.push(row);

    row = Vec::new();
    for _ in 0 .. Person::ALL.len() {
        for &number in Number::ALL {
            row.push(table::Entry {
                header: true,
                rowspan: 1,
                colspan: 1,
                data: number.to_string().capitalize().blocking().to_dyn(),
            });
        }
    }
    table.push(row);

    row = Vec::new();
    for &gender in Gender::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: Case::ALL.len() as u32,
            colspan: 1,
            data: gender.to_string().capitalize().blocking().to_dyn(),
        });
        for &case in Case::ALL {
            row.push(table::Entry {
                header: true,
                rowspan: 1,
                colspan: 1,
                data: case.to_string().capitalize().blocking().to_dyn(),
            });
            for &person in Person::ALL {
                for &number in Number::ALL {
                    row.push(table::Entry::new(make_data(
                        person, case, gender, number,
                    )));
                }
            }
            table.push(row);
            row = Vec::new();
        }
    }

    table
}

/// Person x Tense table.
pub fn person_tense_table<F>(mut make_data: F) -> table::Entries<DynComponent>
where
    F: FnMut(Person, Tense) -> DynComponent,
{
    let mut table = Vec::new();
    let mut row = vec![table::Entry {
        header: true,
        rowspan: 1,
        colspan: 2,
        data: "".blocking().to_dyn(),
    }];
    for &person in Person::ALL {
        row.push(table::Entry {
            header: true,
            rowspan: 1,
            colspan: 1,
            data: person.to_string().capitalize().blocking().to_dyn(),
        });
    }
    table.push(row);
    row = Vec::new();

    for &mood in BasicMood::ALL {
        match mood {
            BasicMood::Indicative => {
                row.push(table::Entry {
                    header: true,
                    rowspan: IndicativeTense::ALL.len() as u32,
                    colspan: 1,
                    data: mood.to_string().capitalize().blocking().to_dyn(),
                });

                for &tense in IndicativeTense::ALL {
                    row.push(table::Entry {
                        header: true,
                        rowspan: 1,
                        colspan: 1,
                        data: tense
                            .to_string()
                            .capitalize()
                            .blocking()
                            .to_dyn(),
                    });
                    for &person in Person::ALL {
                        row.push(table::Entry::new(make_data(
                            person,
                            tense.into(),
                        )));
                    }
                    table.push(row);
                    row = Vec::new();
                }
            },

            BasicMood::Imperative => {
                row.push(table::Entry {
                    header: true,
                    rowspan: ImperativeTense::ALL.len() as u32,
                    colspan: 1,
                    data: mood.to_string().capitalize().blocking().to_dyn(),
                });

                for &tense in ImperativeTense::ALL {
                    row.push(table::Entry {
                        header: true,
                        rowspan: 1,
                        colspan: 1,
                        data: tense
                            .to_string()
                            .capitalize()
                            .blocking()
                            .to_dyn(),
                    });
                    for &person in Person::ALL {
                        row.push(table::Entry::new(make_data(
                            person,
                            tense.into(),
                        )));
                    }
                    table.push(row);
                    row = Vec::new();
                }
            },
        }
    }

    table
}
*/
