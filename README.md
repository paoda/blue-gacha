# Blue Archive Gacha Simulator 

Here's an implementation of Blue Archive's Gacha System
I believe it to be correct, though I'm always willing to accept corrections if you have any!


## Example

Here's what's needed in order to call `.roll()` and `.roll10()`.

```rust
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
```