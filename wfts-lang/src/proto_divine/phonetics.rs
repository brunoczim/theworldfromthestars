#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Triggers {
    pub palatalizes: bool,
    pub palatalizable: bool,
    pub dissocs_palatal: bool,
    pub dissocs_labial: bool,
    pub voices: bool,
}

impl Triggers {
    pub fn with_ctx(self, ctx: Context) -> Triggers {
        let palatalized = self.palatalizable && ctx.palatalized;
        Triggers {
            palatalizes: self.palatalizes || palatalized,
            palatalizable: self.palatalizable,
            dissocs_palatal: self.dissocs_palatal,
            dissocs_labial: self.dissocs_labial,
            voices: self.voices || ctx.voiced,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Context {
    pub palatalized: bool,
    pub voiced: bool,
    pub palatal_dissoc: bool,
    pub labial_dissoc: bool,
}

impl Context {
    pub fn from_triggers(prev: Triggers, next: Triggers) -> Self {
        Context {
            palatalized: prev.palatalizes || next.palatalizes,
            voiced: prev.voices && next.voices,
            palatal_dissoc: prev.dissocs_palatal || next.dissocs_palatal,
            labial_dissoc: prev.dissocs_labial || next.dissocs_labial,
        }
    }
}
