use crate::aes::STATE;

mod aes;

fn main() {
    let test_key = aes::Key {
        bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    let data: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut test_state: STATE = aes::STATE::init(test_key);

    test_state.update(&data, 16);

    let mut test_buffer: [u8; 16] = [0; 16];

    test_state.encrypt(&test_buffer);
}
