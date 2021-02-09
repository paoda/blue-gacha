use ba_gacha::banner::BannerBuilder;
use ba_gacha::gacha::{GachaBuilder, Recruitment};
use ba_gacha::i18n::Language;
use ba_gacha::student::Student;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("students.json").unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let students: Vec<Student> = serde_json::from_str(&json).unwrap();

    let banner_students: Vec<Student> = students
        .iter()
        .filter(|student| student.name != "ノゾミ")
        .map(|student| student.clone())
        .collect();

    let hoshino = find_student(&students, "ホシノ").unwrap();
    let shiroko = find_student(&students, "シロコ").unwrap();
    let rate_up_students = vec![shiroko, hoshino];

    let gacha = GachaBuilder::new(79.0, 18.5, 2.5)
        .with_pool(banner_students)
        .with_priority(&rate_up_students, 0.7)
        .finish()
        .unwrap();

    let pickup_banner = BannerBuilder::new("ピックアップ募集")
        .with_name_translation(Language::English, "Rate-Up Registration")
        .with_sparkable_students(&rate_up_students)
        .with_gacha(&gacha)
        .finish()
        .unwrap();

    // let student = pickup_banner.roll();
    // let students = pickup_banner.roll10();
}

pub fn find_student(students: &[Student], jpn_name: &str) -> Option<Student> {
    students
        .iter()
        .find(|student| student.name == jpn_name)
        .map(|student| student.clone())
}
