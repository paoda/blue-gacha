# Blue Archive Gacha Simulator 

Here's an implementation of Blue Archive's Gacha System. I believe it to be correct, though I'm always willing to accept corrections if you have any!


## Example

Here's what's needed in order to call `.roll()` and `.roll10()`.

```rust
let mut file = File::open("students.json").unwrap();
let mut json = String::new();
file.read_to_string(&mut json).unwrap();

let students: Vec<Student> = serde_json::from_str(&json).unwrap();
```

This Repo contains `students.json` which is an Array where each object within the arry contains a Student's Japanese name, English TL name and rarity.

```rust
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

This example creates the ピックアップ募集 Banner which ran from 2021-02-04 to 2021-02-11. The only student in the game who was **not** in This banner was ノゾミ, so you can see her being filtered out of the list of *all** students to create a list of students in the ピックアップ募集.

After this, we want to determine which units are on rate-up if there are any. In this example, ホシノ and シロコ have increased pull rates. 

The Rest of the code consists of instantiating the Gacha and Banner Structs using their respective Builders. 

After this: 

```rust
let student: Student = pickup_banner.roll();
// or 
let students: [Student; 10] = pickup_banner.roll10();
```
can be called (when the `Recruitment` trait is in scope) to allow for acurate simulation of Blue Archive's Gacha.