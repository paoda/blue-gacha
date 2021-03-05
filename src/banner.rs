use crate::gacha::{Gacha, Rarity, Recruitment};
use crate::i18n::{I18nString, Language};
use crate::student::{PriorityStudent, Student};
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
    /// # use bluearch_recruitment::banner::BannerBuilder;
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
    /// # use bluearch_recruitment::banner::BannerBuilder;
    /// # use bluearch_recruitment::i18n::Language;
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
    /// # use bluearch_recruitment::gacha::GachaBuilder;
    /// # use bluearch_recruitment::banner::BannerBuilder;
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
    /// # use bluearch_recruitment::student::Student;
    /// # use bluearch_recruitment::banner::BannerBuilder;
    /// let students = Vec::new();
    /// let banner_builder = BannerBuilder::new("ピックアップ募集")
    ///     .with_sparkable_students(&students);
    /// ```
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

#[derive(Debug, Clone)]
enum StudentType<'a> {
    One,   // One Star
    Two,   // Two Stars
    Three, // Three Stars
    Priority(&'a PriorityStudent),
}

impl<'a> From<Rarity> for StudentType<'a> {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::One => Self::One,
            Rarity::Two => Self::Two,
            Rarity::Three => Self::Three,
        }
    }
}

impl<'a> TryFrom<StudentType<'a>> for Rarity {
    type Error = &'static str;

    fn try_from(value: StudentType) -> Result<Self, Self::Error> {
        Rarity::try_from(&value)
    }
}

impl<'a> TryFrom<&StudentType<'a>> for Rarity {
    type Error = &'static str;

    fn try_from(value: &StudentType<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            StudentType::One => Self::One,
            StudentType::Two => Self::Two,
            StudentType::Three => Self::Three,
            StudentType::Priority(_) => return Err("Can not convert from Priority to Rarity"),
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

        let empty_vec = Vec::new();
        let priority_students = self.gacha.priority.as_ref().unwrap_or(&empty_vec);
        let mut rates = (
            self.gacha.get_rate(Rarity::One),
            self.gacha.get_rate(Rarity::Two),
            self.gacha.get_rate(Rarity::Three),
        );

        for priority_student in priority_students {
            match priority_student.student().rarity {
                Rarity::One => rates.0 -= priority_student.rate,
                Rarity::Two => rates.1 -= priority_student.rate,
                Rarity::Three => rates.2 -= priority_student.rate,
            };
        }

        let mut items: Vec<(StudentType, usize)> = Vec::with_capacity(3 + priority_students.len());
        items.push((StudentType::One, rates.0));
        items.push((StudentType::Two, rates.1));
        items.push((StudentType::Three, rates.2));

        items.extend(
            priority_students
                .iter()
                .map(|student| (StudentType::Priority(student), student.rate)),
        );

        let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        let student_pool = &self.gacha.pool;

        match &items[dist.sample(&mut rng)] {
            (StudentType::Priority(priority_student), _) => priority_student.student().clone(),
            (student_type, _) => {
                let students: Vec<&Student> = student_pool
                    .iter()
                    .filter(|student| student.rarity == student_type.try_into().unwrap())
                    .filter(|student| {
                        // Remove any Rate-Up Units
                        // TODO: Determine whether this is the right way of implementing priority gacha
                        !priority_students
                            .iter()
                            .any(|priority_student| student.name == priority_student.student().name)
                    })
                    .collect();
                let index = rng.gen_range(0..students.len());
                students[index].clone()
            }
        }
    }

    fn get_random_student_of_rarity(&self, rarity: Rarity) -> Student {
        // NOTE: This does not actually follow the rules of any given banner. Only get_random_student() does.
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

        if !two_star_present && students[students.len() - 1].rarity != Rarity::Three {
            students[students.len() - 1] = self.get_random_student_of_rarity(Rarity::Two);
        }

        students
    }
}
