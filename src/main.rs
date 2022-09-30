fn main() {
    // let mut number_of_terms = String::new();
    // let mut first_value = String::new();
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let n = 5;
    let mut i = 1;

    print!("{a} {b}");

    while i <= n-2 {
        let c = a + b;
        print!(" {c}");
        i += 1;

        a = b;
        b = c;
    }


}
