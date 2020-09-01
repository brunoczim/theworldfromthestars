use crate::{
    grammar::grammemes::{BasicCase, Case, Gender, Number, Person},
    morphology::Morpheme,
};
use std::fmt;
use wfts_pedia_ssg::{
    component::{
        audio::Audio,
        list::UnorderedList,
        table,
        text::{Bold, Italic},
        BlockComponent,
        Component,
        Context,
        DynComponent,
        InlineComponent,
    },
    fmt::StrExt,
};

#[derive(Debug, Clone, Copy)]
pub struct WithStarAlphabet<T, K>(pub T)
where
    T: Component<Kind = K>;

impl<T> Component for WithStarAlphabet<T, InlineComponent>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<span class=\"star-alphabet\">{}</span>",
            ctx.renderer(&self.0)
        )
    }
}

impl<T> Component for WithStarAlphabet<T, BlockComponent>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"star-alphabet\">{}</div>",
            ctx.renderer(&self.0)
        )
    }
}

#[derive(Debug, Clone)]
pub struct DefinitionHead {
    pub name: String,
    pub inflected_for: Vec<String>,
}

impl Component for DefinitionHead {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"definition-head\">{} (inflected for ",
            ctx.renderer(Bold(WithStarAlphabet(&self.name))),
        )?;

        let mut first = true;
        for key in &self.inflected_for {
            if first {
                first = false;
            } else {
                write!(fmt, ", ")?;
            }
            write!(fmt, "{}", ctx.renderer(Bold(key)))?;
        }

        write!(fmt, ")</div>")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct PronunciationKey {
    name: String,
    pronunciation: String,
    audio: Option<Audio>,
}

impl Component for PronunciationKey {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"pronunciation-name\">{}</div>:<div \
             class=\"pronunciation-val\">{}</div>",
            ctx.renderer(Italic(&self.name)),
            ctx.renderer(&self.pronunciation)
        )?;

        if let Some(audio) = &self.audio {
            write!(
                fmt,
                "<div class=\"pronunciation-audio\">{}</div>",
                ctx.renderer(audio)
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Pronunciation {
    pub morpheme: Morpheme,
    pub audio_early: Option<Audio>,
    pub audio_late: Option<Audio>,
}

impl Component for Pronunciation {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        let mut list = vec![PronunciationKey {
            name: "Phonemic".to_owned(),
            pronunciation: format!("/{}/", self.morpheme.to_broad_ipa()),
            audio: None,
        }];
        if let Morpheme::Word(word) = &self.morpheme {
            list.push(PronunciationKey {
                name: "Early CSL Accents".to_owned(),
                pronunciation: format!("[{}]", word.to_early_narrow_ipa()),
                audio: self.audio_early.clone(),
            });
            list.push(PronunciationKey {
                name: "Some Late CSL Accents".to_owned(),
                pronunciation: format!("[{}]", word.to_late_narrow_ipa()),
                audio: self.audio_late.clone(),
            });
        }
        write!(fmt, "{}", ctx.renderer(UnorderedList(list)))
    }
}

/// BasicCase x Gender x Number table.
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

/// BasicCase x Gender x Number table.
pub fn bcase_gender_number_table<F>(
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(BasicCase, Gender, Number) -> DynComponent,
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

/// BasicCase x Fixed Gender x Number table.
pub fn bcase_fgender_number_table<F>(
    gender: Gender,
    mut make_data: F,
) -> table::Entries<DynComponent>
where
    F: FnMut(BasicCase, Number) -> DynComponent,
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
