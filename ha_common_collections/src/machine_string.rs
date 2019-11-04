pub fn test_string() {
    let mut s_01 = String::from("foo");
    let s_02 = "bar";
    s_01.push_str(s_02);
    println!("s_01:{}   s_02:{}", s_01, s_02);


    let s_03 = String::from("Hello, ");
    let s_04 = String::from("world!");
    let s_05 = s_03 + &s_04; // note s1 has been moved here and can no longer be used

    println!("s_04:{}  s_05:{} ", s_04, s_05);

    let s_06 = String::from("tic");
    let s_07 = String::from("tac");
    let s_08 = String::from("toe");

    let s_10 = format!("{}-{}-{}", s_06, s_07, s_08);
    println!("s_10:{}", s_10);


    let len_01 = String::from("Hola").len();
    println!("len_01:{}", len_01);

    let len_02 = String::from("Здравствуйте").len();
    println!("len_02:{}", len_02);

    let hello = "Здравствуйте";
    let s_11 = &hello[0..4];
    println!("s_11:{}", s_11);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }


    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}