use blue_gacha::banner::BannerBuilder;
use blue_gacha::gacha::{GachaBuilder, Recruitment};
use blue_gacha::i18n::Language;
use blue_gacha::student::Student;
use std::{fs::File, io::Read};

const THREE_STAR_RATE: f32 = 2.5;
const TWO_STAR_RATE: f32 = 18.5;
const ONE_STAR_RATE: f32 = 79.0;

const KARIN_RATE: f32 = 0.7;
const MUTSUKI_RATE: f32 = 0.3;

fn main() {
    // The Banner we're rolling from is a hypothetical banner which includes every unit in the game
    // (e.g. including Nozomi)
    //
    // Karin (3*) and Mutsuki (2*) will have increased rates because I like them the most.
    // Karin will have a pull-rate of 0.7%, and Mutsuki will have a pull-rate of 3.0%

    let mut students_str = String::new();
    let mut students = File::open("./examples/students.json").unwrap();
    students.read_to_string(&mut students_str).unwrap();

    let students: Vec<Student> = serde_json::from_str(&students_str).unwrap();

    let karin = find_student(&students, "カリン")
        .expect("カリン is not present in ./examples/students.json")
        .into_priority_student(KARIN_RATE);

    let mutsuki = find_student(&students, "ムツキ")
        .expect("ムツキ is not present in ./examples/students.json")
        .into_priority_student(MUTSUKI_RATE);

    let sparkable = vec![karin.student().clone()];
    let priority = vec![karin, mutsuki];

    let gacha = GachaBuilder::new(ONE_STAR_RATE, TWO_STAR_RATE, THREE_STAR_RATE)
        .with_pool(students)
        .with_priority(&priority)
        .finish()
        .unwrap();

    // I'm some N5 loser don't judge too hard pls...
    let banner = BannerBuilder::new("不運ですね。")
        .with_name_translation(Language::English, "Unlucky, right?")
        .with_sparkable_students(&sparkable)
        .with_gacha(&gacha)
        .finish()
        .unwrap();

    let students = banner.roll10();

    println!("{} 10-pull: \n", banner.name);
    for student in students.iter() {
        println!(
            "{} {}",
            student.name.get(Language::English).unwrap(),
            student.rarity
        );
    }
}

pub fn find_student(students: &[Student], jpn_name: &str) -> Option<Student> {
    students
        .iter()
        .find(|student| student.name == jpn_name)
        .map(|student| student.clone())
}
