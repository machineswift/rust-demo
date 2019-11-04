pub fn test_vector() {
    let v_01: Vec<i32> = Vec::new();
    println!("v_01:{:#?}",v_01);

    let v_02 = vec![1, 2, 3];
    println!("v_02.len:{}",v_02.len());

    let mut v_03 = Vec::new();

    v_03.push(0);
    v_03.push(1);
    v_03.push(2);
    v_03.push(3);

    match v_03.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v_04 = vec![100, 32, 57];
    for i in &v_04 {
        println!("{}", i);
    }

    let mut v_05 = vec![100, 32, 57];
    for i in &mut v_05 {
        *i += 50;
    }
    for i in &v_05 {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row:{:#?}",row);
}