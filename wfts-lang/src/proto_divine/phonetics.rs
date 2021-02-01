//! This module defines items useful for dealing with phonetic details of the
//! Proto-Divine language.

/// Triggers of phonetic attributes of a phoneme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Triggers {
    /// Does this phoneme palatalizes the surrounding consonants?
    pub palatalizes: bool,
    /// Is this phoneme palatalizable?
    pub palatalizable: bool,
    /// Does this dissociates vowels from palatal articulation?
    pub dissocs_palatal: bool,
    /// Does this dissociates vowels from labial articulation?
    pub dissocs_labial: bool,
    /// Does this voices surrounding consonants? (if the other neighbour of
    /// such consonant also voices).
    pub voices: bool,
}

impl Triggers {
    /// Recreates this trigger data given a phonetic context with more accurate
    /// triggers.
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

/// Phonetic context of a phoneme, containing the phonetic attributes which it
/// is realized with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Context {
    /// Would this phoneme be palatalized?
    pub palatalized: bool,
    /// Would this phoneme be voiced?
    pub voiced: bool,
    /// Would this phoneme dissociate from palatal articulation?
    pub palatal_dissoc: bool,
    /// Would this phoneme dissociate from labial articulation?
    pub labial_dissoc: bool,
}

impl Context {
    /// Constructs phonetic context from neighbours' triggers.
    pub fn from_triggers(prev: Triggers, next: Triggers) -> Self {
        Context {
            palatalized: prev.palatalizes || next.palatalizes,
            voiced: prev.voices && next.voices,
            palatal_dissoc: prev.dissocs_palatal || next.dissocs_palatal,
            labial_dissoc: prev.dissocs_labial || next.dissocs_labial,
        }
    }
}
