fn dereferencing_a_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}


fn main() {


        let v: Vec<u32> = vec![1, 2, 3];


    dereferencing_a_raw_pointer();
}
