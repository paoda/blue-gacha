# Blue Archive Gacha Simulator 

Here's an implementation of Blue Archive's Gacha System. I believe it to be correct, though I'm always willing to accept corrections if you have any!


## Example

Here's what's needed in order to call `.roll()` and `.roll10()`.

```rust
let mut students_str = String::new();
let mut students = File::open("./examples/students.json").unwrap();
students.read_to_string(&mut students_str).unwrap();

let students: Vec<Student> = serde_json::from_str(&students_str).unwrap();
```

This Repo contains `students.json` (in the examples directory) which is an Array where each object within the array contains a Student's Japanese name, English TL name and rarity.

```rust
// Here we construct a hypothetical banner featuring a gacha poll of
// everybody but Nozomi
let banner_students: Vec<Student> = students
    .iter()
    .filter(|student| student.name != "ノゾミ")
    .map(|student| student.clone())
    .collect();

let hoshino = find_student(&students, "ホシノ").unwrap()
    .into_priority_student(7.0 / 2.0);
let shiroko = find_student(&students, "シロコ").unwrap()
    .into_priority_student(7.0 / 2.0);

let priority_students = vec![shiroko, hoshino];

let gacha = GachaBuilder::new(79.0, 18.5, 2.5)
    .with_pool(banner_students)
    .with_priority(&priority_students)
    .finish()
    .unwrap();

let pickup_banner = BannerBuilder::new("ピックアップ募集")
    .with_name_translation(Language::English, "Rate-Up Registration")
    .with_sparkable_students(&priority_students)
    .with_gacha(&gacha)
    .finish()
    .unwrap();
```

After selecting all the students for the banner, we want to declare which units are on rate-up if there are any. In this example, Hoshino and Shiroko have unique rates.

The `BannerBuilder` and `GachaBuilder` structs are builders that set up the actual gacha system. After a `Banner` has successfully been built using the `Banner Builder`, we can call: 

```rust
let student: Student = pickup_banner.roll();
// or 
let students: [Student; 10] = pickup_banner.roll10();
```
to perform gacha rolls using the configurations encoded above.