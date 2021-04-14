use crate::student::{PriorityStudent, Student};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::cmp::Ordering;

const THREE_STAR_RATE: usize = 25;
const TWO_STAR_RATE: usize = 185;
const ONE_STAR_RATE: usize = 790;

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
/// The Available Rarities in Blue Archive's Gacha System
pub enum Rarity {
    One = 1,
    Two,
    Three,
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Rarity::One => f.write_str("1★"),
            Rarity::Two => f.write_str("2★"),
            Rarity::Three => f.write_str("3★"),
        }
    }
}

impl Default for Rarity {
    fn default() -> Self {
        Self::One
    }
}

impl PartialOrd for Rarity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (*self, other) {
            (Rarity::One, Rarity::One) => Ordering::Equal,
            (Rarity::One, Rarity::Two) => Ordering::Less,
            (Rarity::One, Rarity::Three) => Ordering::Less,
            (Rarity::Two, Rarity::One) => Ordering::Greater,
            (Rarity::Two, Rarity::Two) => Ordering::Equal,
            (Rarity::Two, Rarity::Three) => Ordering::Less,
            (Rarity::Three, Rarity::One) => Ordering::Greater,
            (Rarity::Three, Rarity::Two) => Ordering::Greater,
            (Rarity::Three, Rarity::Three) => Ordering::Equal,
        })
    }
}

impl Ord for Rarity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Recruitment is a trait that consists of two methods
/// Representing single and 10-rolls
///
/// Every Banner is expected to implement the Recruitment trait
pub trait Recruitment {
    fn roll(&self) -> Student;
    fn roll10(&self) -> [Student; 10];
}

/// Gacha Structs are built using this GachaBuilder Class
///
/// By Default, GachaBuilder assumes the rates:
/// * 1★: 79.0%
/// * 2★: 18.5%
/// * 3★: 2.5%
pub struct GachaBuilder {
    rates: Option<(usize, usize, usize)>,
    pool: Option<Vec<Student>>,
    priority: Option<Vec<PriorityStudent>>,
}

impl Default for GachaBuilder {
    fn default() -> Self {
        Self {
            rates: Some((ONE_STAR_RATE, TWO_STAR_RATE, THREE_STAR_RATE)),
            pool: Default::default(),
            priority: Default::default(),
        }
    }
}

impl GachaBuilder {
    /// Creates a new instance of a GachaBuilder.
    ///
    ///
    /// # Arguments
    /// * `one` - The percent chance of pulling a 1★ Student
    /// * `two` - The percent chance of pulling a 2★ Student
    /// * `three` - The percent chance of pulling a 3★ Student
    ///
    /// # Examples
    /// ```
    /// # use blue_gacha::gacha::GachaBuilder;
    /// let gacha_builder: GachaBuilder = GachaBuilder::new(79.0, 18.5, 2.5)
    ///     .with_pool(Vec::new());
    /// ```
    pub fn new(one: f32, two: f32, three: f32) -> Self {
        let one = (one * 10.0) as usize;
        let two = (two * 10.0) as usize;
        let three = (three * 10.0) as usize;

        assert_eq!(one + two + three, 1000);

        Self {
            rates: Some((one, two, three)),
            ..Default::default()
        }
    }

    /// Attaches a Student Gacha Pool to the GachaBuilder
    ///
    /// # Arguments
    /// * `students` - A Vector containing every single pullable student in the Gacha
    ///
    /// # Examples
    /// ```
    /// # use blue_gacha::gacha::{GachaBuilder, Rarity};
    /// # use blue_gacha::student::Student;
    ///
    /// let aru = Student::new("アル", Rarity::Three);
    /// let hina = Student::new("ヒナ", Rarity::Three);
    /// let gacha_builder = GachaBuilder::default().with_pool(vec![aru, hina]);
    /// ```
    pub fn with_pool(self, students: Vec<Student>) -> Self {
        Self {
            pool: Some(students),
            ..self
        }
    }

    /// Attaches a pool of Students who have increased rates
    ///
    /// # Arguments
    /// * `students` - A Vector of Students who have increased rates
    /// * `rate` - The rate of the students in the previous argument
    ///
    /// # Examples
    /// ```
    /// # use blue_gacha::gacha::{GachaBuilder, Rarity};
    /// # use blue_gacha::student::{Student, PriorityStudent};
    /// let aru = Student::new("アル", Rarity::Three).into_priority_student(3.5 / 2.0);
    /// let hina = Student::new("ヒナ", Rarity::Three).into_priority_student(3.5 / 2.0);
    /// let pool = vec![aru.student().clone(), hina.student().clone()];
    /// let priority = vec![aru, hina];
    /// let gacha_builder = GachaBuilder::new(79.0, 18.5, 2.5)
    ///     .with_pool(pool)
    ///     .with_priority(&priority);
    /// ```
    pub fn with_priority(self, students: Vec<PriorityStudent>) -> Self {
        Self {
            priority: Some(students),
            ..self
        }
    }

    /// Consumes a GachaBuilder and returns a Gacha Struct.
    ///
    /// Will return `None` if the `rates` or `pool` property of
    /// GachaBuilder have not been set.
    ///
    /// # Examples
    /// ```
    /// # use blue_gacha::gacha::{GachaBuilder, Gacha, Rarity};
    /// # use blue_gacha::student::Student;
    /// let aru = Student::new("アル", Rarity::Three);
    /// let hina = Student::new("ヒナ", Rarity::Three);
    /// let gacha = GachaBuilder::default()
    ///     .with_pool(vec![aru, hina])
    ///     .finish().unwrap();
    /// ```
    pub fn finish(self) -> Option<Gacha> {
        Some(Gacha {
            rates: self.rates?,
            pool: self.pool?,
            priority: self.priority,
        })
    }
}

/// Provides the necessary information to facilitate a "pull", which is
/// to randomly select a Student from the gacha pool
#[derive(Debug, Default, Clone)]
pub struct Gacha {
    /// (1★, 2★, 3★)
    pub rates: (usize, usize, usize),
    pub pool: Vec<Student>,
    pub priority: Option<Vec<PriorityStudent>>,
}

impl Gacha {
    /// Returns a usize representing the percent chance of pulling a specific rarity
    /// (in terms of 1000)
    ///
    /// # Arguments
    /// * `rarity` - The Rarity who's gacha pull rate will be returned
    ///
    /// # Examples
    /// ```
    /// # use blue_gacha::gacha::{GachaBuilder, Rarity};
    /// let gacha = GachaBuilder::new(79.0, 18.5, 2.5)
    ///     .with_pool(Vec::new())
    ///     .finish()
    ///     .unwrap();
    ///
    /// assert_eq!(gacha.get_rate(Rarity::One), 790);
    /// ```
    pub fn get_rate(&self, rarity: Rarity) -> usize {
        match rarity {
            Rarity::One => self.rates.0,
            Rarity::Two => self.rates.1,
            Rarity::Three => self.rates.2,
        }
    }
}
