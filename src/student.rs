use crate::gacha::Rarity;
use crate::i18n::{I18nString, Language};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// A Student which has just the information necessary for gacha associated with it
pub struct Student {
    /// There is guaranteed to be a Japanese Name available at all times
    pub name: I18nString,
    pub rarity: Rarity,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.name, self.rarity)
    }
}

impl Student {
    /// Creates a new Instance of a Student
    ///
    /// # Arguments
    /// * `jpn_name` - The name of the Student from the Japanese Version of Blue Archive
    /// * `rarity` - The Rarity of the Student
    ///
    /// # Examples
    /// ```
    /// # use bluearch_recruitment::gacha::Rarity;
    /// # use bluearch_recruitment::student::Student;
    ///
    /// let mutsuki = Student::new("ムツキ", Rarity::Two);
    /// ```
    pub fn new(jpn_name: &str, rarity: Rarity) -> Self {
        Self {
            name: I18nString::new(jpn_name),
            rarity,
        }
    }

    /// Adds a new Translation to the internal [`I18nString`].
    ///
    /// # Arguments
    /// * `language` - The Language of the translation to be added
    /// * `name` - The Translated name, in the language of the previous argument.
    ///
    /// # Examples
    /// ```
    /// # use bluearch_recruitment::gacha::Rarity;
    /// # use bluearch_recruitment::student::Student;
    /// # use bluearch_recruitment::i18n::Language;
    ///
    /// let mut mutsuki = Student::new("ムツキ", Rarity::Two);
    /// mutsuki.add_translation(Language::English, "Mutsuki");
    /// ```
    pub fn add_translation(&mut self, language: Language, name: &str) {
        self.name.update(language, name);
    }

    pub fn into_priority_student(self, rate: f32) -> PriorityStudent {
        PriorityStudent {
            inner: self,
            rate: (rate * 10.0) as usize,
        }
    }
}

/// A Priority Student is a student who has a pull-rate that is unique from
/// the rest of the rest of their peers in their star rating
#[derive(Debug, Clone)]
pub struct PriorityStudent {
    inner: Student,
    pub rate: usize,
}

impl PriorityStudent {
    pub fn new(student: Student, rate: f32) -> Self {
        Self {
            inner: student,
            rate: (rate * 10.0) as usize,
        }
    }

    pub fn student(&self) -> &Student {
        &self.inner
    }
}
