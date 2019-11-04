mod machine_vector;
mod machine_string;
mod machine_hash_map;

fn main() {
    machine_vector::test_vector();
    println!("======================");
    machine_string::test_string();
    println!("======================");
    machine_hash_map::test_hash_map();
}
