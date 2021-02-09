use crate::gacha::{Gacha, Rarity, Recruitment};
use crate::i18n::{I18nString, Language};
use crate::student::Student;
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use std::convert::{TryFrom, TryInto};

/// Used to Construct a Banner
#[derive(Debug, Default)]
pub struct BannerBuilder {
    name: I18nString,
    gacha: Option<Gacha>,
    sparkable: Option<Vec<Student>>,
}

impl BannerBuilder {
    /// Creates a new instance of a BannerBuilder
    ///
    /// # Arguments
    /// * `jpn_name` - The name of the Banner as seen in Blue Archive
    ///
    /// # Examples
    /// ```
    /// # use ba_gacha::banner::BannerBuilder;
    /// let banner_builder = BannerBuilder::new("ピックアップ募集");
    /// ```
    pub fn new(jpn_name: &str) -> Self {
        Self {
            name: I18nString::new(jpn_name),
            ..Default::default()
        }
    }

    /// Adds a Translation to the internal I18nString
    ///
    /// # Arguments
    /// * `language` - The target language.
    /// * `name` - The name of the Banner in the target language.
    ///
    /// # Examples
    /// ```
    /// # use ba_gacha::banner::BannerBuilder;
    /// # use ba_gacha::i18n::Language;
    /// let banner_builder = BannerBuilder::new("ピックアップ募集")
    ///     .with_name_translation(Language::English, "Focus Recruitment");
    /// ```
    pub fn with_name_translation(mut self, language: Language, name: &str) -> Self {
        self.name.update(language, name);
        self
    }

    /// Adds an instance of a Gacha struct
    ///
    /// # Arguments
    /// * `gacha` - An instance of a Gacha struct
    ///
    /// # Examples
    /// ```
    /// # use ba_gacha::gacha::GachaBuilder;
    /// # use ba_gacha::banner::BannerBuilder;
    /// let gacha = GachaBuilder::default()
    ///     .with_pool(Vec::new())
    ///     .finish().unwrap();
    ///
    /// let banner_builder = BannerBuilder::new("ピックアップ募集")
    ///     .with_gacha(&gacha);
    /// ```
    pub fn with_gacha(self, gacha: &Gacha) -> Self {
        Self {
            gacha: Some(gacha.to_owned()),
            ..self
        }
    }

    /// Adds a vector containing all sparkable students in the current Banner
    ///
    /// # Arguments
    /// * `students` - A Vector of Students which are sparkable
    ///
    /// # Examples
    /// ```
    /// # use ba_gacha::banner::BannerBuilder;
    pub fn with_sparkable_students(self, students: &[Student]) -> Self {
        Self {
            sparkable: Some(students.to_vec()),
            ..self
        }
    }

    pub fn finish(self) -> Option<Banner> {
        Some(Banner {
            name: self.name,
            gacha: self.gacha?,
            sparkable: self.sparkable,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum StudentType {
    One = 1,  // One Star
    Two,      // Two Stars
    Three,    // Three Stars
    Priority, // Rate-up Student (Presumably 3*)
}

impl From<Rarity> for StudentType {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::One => Self::One,
            Rarity::Two => Self::Two,
            Rarity::Three => Self::Three,
        }
    }
}

impl TryFrom<StudentType> for Rarity {
    type Error = &'static str;

    fn try_from(value: StudentType) -> Result<Self, Self::Error> {
        Ok(match value {
            StudentType::One => Self::One,
            StudentType::Two => Self::Two,
            StudentType::Three => Self::Three,
            StudentType::Priority => return Err("Can not convert from Priority to Rarity"),
        })
    }
}

pub struct Banner {
    pub name: I18nString,
    gacha: Gacha,
    sparkable: Option<Vec<Student>>,
}

impl Banner {
    fn get_random_student(&self) -> Student {
        let mut rng = rand::thread_rng();
        let priority_rate = self.gacha.priority.as_ref().map_or(0, |tuple| tuple.1);
        let three_star_rate = self.gacha.get_rate(Rarity::Three) - priority_rate;

        let items: [(StudentType, usize); 4] = [
            (StudentType::One, self.gacha.get_rate(Rarity::One)),
            (StudentType::Two, self.gacha.get_rate(Rarity::Two)),
            (StudentType::Three, three_star_rate),
            (StudentType::Priority, priority_rate),
        ];

        let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        let students = &self.gacha.pool;

        match items[dist.sample(&mut rng)] {
            (StudentType::Priority, _) => {
                let priority_students = &self.gacha.priority.as_ref().unwrap().0;

                let index: usize = rng.gen_range(0..priority_students.len());
                priority_students[index].clone()
            }
            (rarity, _) => {
                let students: Vec<&Student> = students
                    .iter()
                    .filter(|student| student.rarity == rarity.try_into().unwrap())
                    .collect();

                let index: usize = rng.gen_range(0..students.len());
                students[index].clone()
            }
        }
    }

    fn get_random_student_of_rarity(&self, rarity: Rarity) -> Student {
        let students = &self.gacha.pool;
        let two_star_students: Vec<&Student> = students
            .iter()
            .filter(|student| student.rarity == rarity)
            .collect();

        let index = rand::thread_rng().gen_range(0..two_star_students.len());
        two_star_students[index].clone()
    }
}

impl Recruitment for Banner {
    fn roll(&self) -> Student {
        self.get_random_student()
    }

    fn roll10(&self) -> [Student; 10] {
        let mut students: [Student; 10] = vec![Default::default(); 10].try_into().unwrap();

        // Fill students with 10 random students
        for student in students.iter_mut() {
            *student = self.get_random_student()
        }

        let two_star_present = students.iter().any(|student| student.rarity == Rarity::Two);

        if !two_star_present {
            if students[students.len() - 1].rarity != Rarity::Three {
                students[students.len() - 1] = self.get_random_student_of_rarity(Rarity::Two);
            }
        }

        students
    }
}
