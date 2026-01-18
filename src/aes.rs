/**
* This is an implementation of the AES encryption algorithim
* It has a block size of 16 bytes and currently assumes a key size of 16 bytes as well
*/
const KEY_SIZE: u8 = 16;
const KEY_CONST_1: u16 = (0x1 << 8) + 0x1B;
const KEY_CONST_2: u16 = (0x01 << 8) + 0x00;

pub union Key {
    pub bytes: [u8; 16],
    words: [AESWord; 4],
}

#[derive(Debug, Copy, Clone)]
pub struct AESWord([u8; 4]);

impl Key {
    fn generate_rconi() -> [AESWord; 10] {
        let mut rconi: [AESWord; 10] = [AESWord([0, 0, 0, 0]); 10];
        rconi[0] = AESWord([1, 0, 0, 0]);

        for i in 1..10 {
            if rconi[i - 1].0[0] < 0x80 {
                rconi[i] = AESWord([2 * rconi[i - 1].get_0th(), 0, 0, 0]);
            } else {
                rconi[i] = AESWord([
                    ((((2 * (rconi[i - 1].get_0th() as u16)) ^ KEY_CONST_1) % KEY_CONST_2) as u8),
                    0,
                    0,
                    0,
                ]);
            }
        }

        return rconi;
    }

    pub fn expand(key: Key) -> [AESWord; 44] {
        let rconi: [AESWord; 10] = Self::generate_rconi();

        let mut expanded_words: [AESWord; 44] = [AESWord([0, 0, 0, 0]); 44];

        for i in 0..44 {
            if i < 4 {
                expanded_words[i] = unsafe { key.words[i] };
            }
        }

        return expanded_words;
    }
}

impl AESWord {
    pub fn get_0th(self: Self) -> u8 {
        return self.0[0];
    }

    pub fn rot_word(self: Self) -> AESWord {
        return AESWord([self.0[1], self.0[2], self.0[3], self.0[4]]);
    }
}
