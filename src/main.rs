mod big_int;

fn main() {
    let mut test: big_int::BigInt = big_int::BigInt::init(3);
    let input_data = vec![255, 255, 1];
    test.set(input_data);
    test.print_all();

    let test2: big_int::BigInt = big_int::BigInt::init(6);
    test2.print_all();

    let mut test3 = test + test2;
    test3.print_all();

    test3.trim();

    test3.print_all();
}
