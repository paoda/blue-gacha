use std::io::Write;

use ba_gacha::{
    banner::BannerBuilder,
    gacha::{GachaBuilder, Rarity, Recruitment},
    i18n::Language,
    student::Student,
};

fn main() {
    // Create a Struct representing Shiroko
    let mut shiroko = Student::new("シロコ", Rarity::Three);
    shiroko.add_translation(Language::English, "Shiroko"); // Yes, this is a translation.

    // Create a Struct representing Hoshino
    let mut hoshino = Student::new("ホシノ", Rarity::Three);
    hoshino.add_translation(Language::English, "Hoshino");

    // Create a Vector Containing all the Sparkable Students
    let sparkable_students = vec![shiroko, hoshino];

    let rates = GachaBuilder::new(79.0, 18.5, 2.5)
        .with_pool(pickup_banner_2021_02_11())
        .finish()
        .unwrap();

    // Create the Banner Itself
    let pickup_banner = BannerBuilder::new("ピックアップ募集")
        .with_name_translation(Language::English, "Priority")
        .with_sparkable_students(sparkable_students)
        .with_rates(&rates)
        .finish()
        .unwrap();

    // Roll Banner 1 Time
    let student = pickup_banner.roll();

    // Roll Banner 10 Times
    let students = pickup_banner.roll10();

    // Display Results
    println!("{} Single: {}", pickup_banner.name, student);
    println!();

    println!("{} 10-roll:", pickup_banner.name);
    for student in students.iter() {
        println!("{}", student);
    }

    // let mut file = std::fs::File::create("students.json").unwrap();
    // let students = all_students();
    // let json = serde_json::to_string(&students).unwrap();
    // file.write_all(json.as_bytes()).unwrap();
}

fn create_student(jpn_name: &str, eng_name: &str, rarity: Rarity) -> Student {
    let mut student = Student::new(jpn_name, rarity);
    student.add_translation(Language::English, eng_name);

    student
}

fn pickup_banner_2021_02_11() -> Vec<Student> {
    let mut test: Vec<Student> = vec![];

    // 3 Star
    test.push(create_student("ヒナ", "Hina", Rarity::Three));
    test.push(create_student("イオリ", "Iori", Rarity::Three));
    test.push(create_student("ハルナ", "Haruna", Rarity::Three));
    test.push(create_student("イズミ", "Izumi", Rarity::Three));
    test.push(create_student("アル", "Aru", Rarity::Three));
    test.push(create_student("スミレ", "Sumire", Rarity::Three));
    test.push(create_student("エイミ", "Iemi", Rarity::Three));
    test.push(create_student("カリン", "Karin", Rarity::Three));
    test.push(create_student("ネル", "Neru", Rarity::Three));
    test.push(create_student("マキ", "Maki", Rarity::Three));
    test.push(create_student("ヒビキ", "Hibiki", Rarity::Three));
    test.push(create_student("サヤ", "Saya", Rarity::Three));
    test.push(create_student("シュン", "Shun", Rarity::Three));
    test.push(create_student("シロコ", "Shiroko", Rarity::Three));
    test.push(create_student("ホシノ", "Hoshino", Rarity::Three));
    test.push(create_student("ヒフミ", "Hifumi", Rarity::Three));
    test.push(create_student("ツルギ", "Tsurugi", Rarity::Three));

    // Two Star
    test.push(create_student("アカリ", "Akari", Rarity::Two));
    test.push(create_student("ジュンコ", "Junko", Rarity::Two));
    test.push(create_student("ムツキ", "Mutsuki", Rarity::Two));
    test.push(create_student("カヨコ", "Kayoko", Rarity::Two));
    test.push(create_student("フウカ", "Fuuka", Rarity::Two));
    test.push(create_student("ユウカ", "Yuuka", Rarity::Two));
    test.push(create_student("アカネ", "Akane", Rarity::Two));
    test.push(create_student("ハル", "Haru", Rarity::Two));
    test.push(create_student("ウタハ", "Utaha", Rarity::Two));
    test.push(create_student("チセ", "Chise", Rarity::Two));
    test.push(create_student("ツバキ", "Tsubaki", Rarity::Two));
    test.push(create_student("セリカ", "Serika", Rarity::Two));
    test.push(create_student("アヤネ", "Ayane", Rarity::Two));
    test.push(create_student("ハスミ", "Hasumi", Rarity::Two));
    test.push(create_student("ハナエ", "Hanae", Rarity::Two));
    test.push(create_student("アイリ", "Airi", Rarity::Two));

    // 1 Star:
    test.push(create_student("チナツ", "Chinatsu", Rarity::One));
    test.push(create_student("ハルカ", "Haruka", Rarity::One));
    test.push(create_student("ジュリ", "Juri", Rarity::One));
    test.push(create_student("コタマ", "Kotama", Rarity::One));
    test.push(create_student("アスナ", "Asuna", Rarity::One));
    test.push(create_student("コトリ", "Kotori", Rarity::One));
    test.push(create_student("フィーナ", "Pina", Rarity::One));
    test.push(create_student("スズミ", "Suzumi", Rarity::One));
    test.push(create_student("シミコ", "Shimiko", Rarity::One));
    test.push(create_student("セリナ", "Serina", Rarity::One));
    test.push(create_student("ヨシミ", "Yoshimi", Rarity::One));

    test
}

fn all_students() -> Vec<Student> {
    let mut students: Vec<Student> = Vec::with_capacity(45);

    // 3 Star
    students.push(create_student("ヒナ", "Hina", Rarity::Three));
    students.push(create_student("イオリ", "Iori", Rarity::Three));
    students.push(create_student("ハルナ", "Haruna", Rarity::Three));
    students.push(create_student("イズミ", "Izumi", Rarity::Three));
    students.push(create_student("アル", "Aru", Rarity::Three));
    students.push(create_student("スミレ", "Sumire", Rarity::Three));
    students.push(create_student("エイミ", "Iemi", Rarity::Three));
    students.push(create_student("カリン", "Karin", Rarity::Three));
    students.push(create_student("ネル", "Neru", Rarity::Three));
    students.push(create_student("マキ", "Maki", Rarity::Three));
    students.push(create_student("ヒビキ", "Hibiki", Rarity::Three));
    students.push(create_student("サヤ", "Saya", Rarity::Three));
    students.push(create_student("シュン", "Shun", Rarity::Three));
    students.push(create_student("シロコ", "Shiroko", Rarity::Three));
    students.push(create_student("ホシノ", "Hoshino", Rarity::Three));
    students.push(create_student("ヒフミ", "Hifumi", Rarity::Three));
    students.push(create_student("ツルギ", "Tsurugi", Rarity::Three));

    // Two Star
    students.push(create_student("アカリ", "Akari", Rarity::Two));
    students.push(create_student("ジュンコ", "Junko", Rarity::Two));
    students.push(create_student("ムツキ", "Mutsuki", Rarity::Two));
    students.push(create_student("カヨコ", "Kayoko", Rarity::Two));
    students.push(create_student("フウカ", "Fuuka", Rarity::Two));
    students.push(create_student("ユウカ", "Yuuka", Rarity::Two));
    students.push(create_student("アカネ", "Akane", Rarity::Two));
    students.push(create_student("ハル", "Haru", Rarity::Two));
    students.push(create_student("ウタハ", "Utaha", Rarity::Two));
    students.push(create_student("チセ", "Chise", Rarity::Two));
    students.push(create_student("ツバキ", "Tsubaki", Rarity::Two));
    students.push(create_student("セリカ", "Serika", Rarity::Two));
    students.push(create_student("アヤネ", "Ayane", Rarity::Two));
    students.push(create_student("ハスミ", "Hasumi", Rarity::Two));
    students.push(create_student("ハナエ", "Hanae", Rarity::Two));
    students.push(create_student("アイリ", "Airi", Rarity::Two));
    students.push(create_student("ノゾミ", "Nozomi", Rarity::Two)); // Not a part of Gacha

    // 1 Star:
    students.push(create_student("チナツ", "Chinatsu", Rarity::One));
    students.push(create_student("ハルカ", "Haruka", Rarity::One));
    students.push(create_student("ジュリ", "Juri", Rarity::One));
    students.push(create_student("コタマ", "Kotama", Rarity::One));
    students.push(create_student("アスナ", "Asuna", Rarity::One));
    students.push(create_student("コトリ", "Kotori", Rarity::One));
    students.push(create_student("フィーナ", "Pina", Rarity::One));
    students.push(create_student("スズミ", "Suzumi", Rarity::One));
    students.push(create_student("シミコ", "Shimiko", Rarity::One));
    students.push(create_student("セリナ", "Serina", Rarity::One));
    students.push(create_student("ヨシミ", "Yoshimi", Rarity::One));

    students
}
