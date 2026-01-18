mod aes;
mod big_int;

fn main() {
    let test_key = aes::Key {
        bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    aes::Key::expand(test_key);
}
