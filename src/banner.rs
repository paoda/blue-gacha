use std::convert::TryInto;

use rand::{prelude::SliceRandom, Rng};

use crate::{
    gacha::Rarity,
    student::{self, Student},
};
use crate::{
    gacha::{Gacha, Recruitment},
    i18n::{I18nString, Language},
};

#[derive(Debug, Default)]
pub struct BannerBuilder {
    name: I18nString,
    rates: Option<Gacha>,
    sparkable: Option<Vec<Student>>,
}

impl BannerBuilder {
    pub fn new(jpn_name: &str) -> Self {
        Self {
            name: I18nString::new(jpn_name),
            ..Default::default()
        }
    }

    pub fn with_name_translation(mut self, language: Language, name: &str) -> Self {
        self.name.update(language, name);
        self
    }

    pub fn with_rates(self, rates: &Gacha) -> Self {
        Self {
            rates: Some(rates.to_owned()),
            ..self
        }
    }

    pub fn with_sparkable_students(self, students: Vec<Student>) -> Self {
        Self {
            sparkable: Some(students),
            ..self
        }
    }

    pub fn finish(self) -> Option<Banner> {
        Some(Banner {
            name: self.name,
            rates: self.rates?,
            sparkable: self.sparkable?,
        })
    }
}

pub struct Banner {
    pub name: I18nString,
    rates: Gacha,
    sparkable: Vec<Student>, // FIXME: Can we safely assume this will only contain 2 studnets?
}

impl Banner {
    fn get_random_student(&self, rarity: Rarity) -> Student {
        let mut rng = rand::thread_rng();

        let mut filtered_students: Vec<&Student> = self
            .rates
            .pool
            .iter()
            .filter(|student| student.rarity == rarity)
            .collect();

        filtered_students.shuffle(&mut rng);
        let index: usize = rng.gen_range(0..filtered_students.len());

        filtered_students[index].clone()
    }

    fn get_random_rarity(&self) -> Rarity {
        let mut rng = rand::thread_rng();
        let random_num: usize = rng.gen_range(0..1000);

        let one_rate = self.rates.get_rate(Rarity::One);
        let two_rate = self.rates.get_rate(Rarity::Two);
        let three_rate = self.rates.get_rate(Rarity::Three);

        if random_num < three_rate {
            Rarity::Three
        } else if random_num < three_rate + two_rate {
            Rarity::Two
        } else {
            Rarity::One
        }
    }
}

impl Recruitment for Banner {
    fn roll(&self) -> Student {
        self.get_random_student(self.get_random_rarity())
    }

    fn roll10(&self) -> [Student; 10] {
        let mut students: [Student; 10] = vec![Default::default(); 10].try_into().unwrap();

        // Fill students with 10 random students
        for student in students.iter_mut() {
            *student = self.get_random_student(self.get_random_rarity());
        }

        let two_star_present = students.iter().any(|student| student.rarity == Rarity::Two);

        if !two_star_present {
            if students[students.len() - 1].rarity != Rarity::Three {
                students[students.len() - 1] = self.get_random_student(Rarity::Two);
            }
        }

        students
    }
}
