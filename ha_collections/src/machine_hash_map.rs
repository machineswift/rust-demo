use std::collections::HashMap;

pub fn test_hash_map() {
    let mut scores_01 = HashMap::new();
    scores_01.insert(String::from("Blue"), 10);
    scores_01.insert(String::from("Yellow"), 50);
    println!("scores_01:{:?}", scores_01);

    let teams_01 = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores_02 = vec![10, 50];
    let scores_03: HashMap<_, _> = teams_01.iter().zip(initial_scores_02.iter()).collect();
    println!("scores_03:{:?}", scores_03);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map_03 = HashMap::new();
    map_03.insert(field_name, field_value);

    let mut scores_04 = HashMap::new();
    scores_04.insert(String::from("Blue"), 10);
    scores_04.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score_05 = scores_04.get(&team_name);
    println!("score_05:{:?}", score_05);

    for (key, value) in &scores_04 {
        println!("{}: {}", key, value);
    }

    let mut scores_06 = HashMap::new();
    scores_06.insert(String::from("Blue"), 10);

    scores_06.entry(String::from("Yellow")).or_insert(50);
    scores_06.entry(String::from("Blue")).or_insert(50);
    println!("scores_06:{:?}", scores_06);


    let text = "hello world wonderful world";
    let mut map_07 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map_07.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map_07:{:?}", map_07);
}