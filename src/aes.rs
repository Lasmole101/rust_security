use core::error;
use std::ops::BitXor;

/**
* This is an implementation of the AES encryption algorithim
* It has a block size of 16 bytes and currently assumes a key size of 16 bytes as well.
*/

///Fixed key size for AES-128 for now
const KEY_SIZE: u8 = 16;

///Constants used for key arithmetic in AES
const KEY_CONST_1: u16 = (0x1 << 8) + 0x1B;
const KEY_CONST_2: u16 = (0x01 << 8) + 0x00;

///A struct to hold the key
pub union Key {
    pub bytes: [u8; 16],
    words: [AESWord; 4],
}

impl Key {
    /// This function generates the first 10 round constants
    fn generate_rconi() -> [AESWord; 10] {
        let mut rconi: [AESWord; 10] = [AESWord {
            bytes: [0, 0, 0, 0],
        }; 10];
        rconi[0] = AESWord {
            bytes: [1, 0, 0, 0],
        };

        for i in 1..10 {
            if unsafe { rconi[i - 1].bytes[0] < 0x80 } {
                rconi[i] = AESWord {
                    bytes: [2 * rconi[i - 1].get_0th(), 0, 0, 0],
                };
            } else {
                rconi[i] = AESWord {
                    bytes: [
                        unsafe {
                            ((((2 * (rconi[i - 1].bytes[0] as u16)) ^ KEY_CONST_1) % KEY_CONST_2)
                                as u8)
                        },
                        0,
                        0,
                        0,
                    ],
                };
            }
        }

        return rconi;
    }

    pub fn expand(&self) -> [AESWord; 44] {
        let rconi: [AESWord; 10] = Self::generate_rconi();

        let mut expanded_words: [AESWord; 44] = [AESWord {
            bytes: [0, 0, 0, 0],
        }; 44];

        for i in 0..44 {
            if i < 4 {
                expanded_words[i] = unsafe { self.words[i].clone() };
            } else if i % 4 == 0 {
                expanded_words[i] = expanded_words[i - 4]
                    ^ AESWord {
                        num: unsafe { expanded_words[i - 1].num.rotate_left(8) },
                    }
                    .sub_word()
                    ^ rconi[(i / 4) - 1];
            } else {
                expanded_words[i] = expanded_words[i - 4] ^ expanded_words[i - 1];
            }
        }

        return expanded_words;
    }
}

#[derive(Copy, Clone)]
union AESWord {
    bytes: [u8; 4],
    num: u32,
}

impl AESWord {
    pub fn get_0th(self: Self) -> u8 {
        return unsafe { self.bytes[0] };
    }

    pub fn sub_word(self: Self) -> AESWord {
        return AESWord {
            bytes: [
                unsafe { s_box_lookup(self.bytes[0]) },
                unsafe { s_box_lookup(self.bytes[1]) },
                unsafe { s_box_lookup(self.bytes[2]) },
                unsafe { s_box_lookup(self.bytes[3]) },
            ],
        };
    }
    pub fn zeroes() -> AESWord {
        return AESWord {
            bytes: [0, 0, 0, 0],
        };
    }
}

impl BitXor for AESWord {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            num: unsafe { self.num ^ rhs.num },
        }
    }
}

// AES S-box lookup table
pub const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// Apply the AES S-box to a single byte
#[inline(always)]
pub fn s_box_lookup(byte: u8) -> u8 {
    AES_SBOX[byte as usize]
}
/*
@TODO: implement full s-box algo

pub fn s_box(byte_in: u8) -> u8 {
    let s_box_const: u8 = 0x63;
    let rijndael_const:u16 = 0x11B;
    let mut output: u8 = 0;
    if byte_in != 0 {}
    else{


    output = byte_in
        ^ (byte_in.rotate_left(1))
        ^ (byte_in.rotate_left(2))
        ^ (byte_in.rotate_left(3))
        ^ (byte_in.rotate_left(4))
        ^ s_box_const;

    return output;
}
*/
union state_data {
    words: [AESWord; 4],
    bytes: [u8; 16],
}

pub struct STATE {
    data: state_data,
    key: Key,
}

impl STATE {
    pub fn init(key: Key) -> STATE {
        STATE {
            data: state_data {
                words: [
                    AESWord::zeroes(),
                    AESWord::zeroes(),
                    AESWord::zeroes(),
                    AESWord::zeroes(),
                ],
            },
            key,
        }
    }

    pub fn update(&mut self, new_data: &[u8], len: usize) {
        if len > 16 {
            println!("too big");
        } else {
            if len < 16 {
                unsafe { self.data.bytes[len - 1..15].fill(0) };
            }
            for i in 0..len {
                unsafe { self.data.bytes[i] = new_data[i] };
            }
        }
    }

    fn add_round_key(&mut self, key_start_index: usize) {
        for i in 0..3 {
            unsafe {
                self.data.words[i] = self.data.words[i] ^ self.key.words[key_start_index + i]
            };
        }
    }

    fn sub_bytes(&mut self) {
        unsafe { self.data.words[0].sub_word() };
        unsafe { self.data.words[1].sub_word() };
        unsafe { self.data.words[2].sub_word() };
        unsafe { self.data.words[3].sub_word() };
    }

    fn shift_rows(&mut self) {
        unsafe { self.data.words[1].num = self.data.words[1].num.rotate_left(8) };
        unsafe { self.data.words[2].num = self.data.words[2].num.rotate_left(16) };
        unsafe { self.data.words[3].num = self.data.words[3].num.rotate_left(24) };
    }

    fn mix_columns(&mut self) {
        for i in 0..3 {
            let new_0th: u8 = unsafe {
                self.data.words[0].bytes[i] << 2
                    ^ (self.data.words[1].bytes[i] * 3)
                    ^ (self.data.words[2].bytes[i])
                    ^ (self.data.words[3].bytes[i])
            };
            let new_1st: u8 = unsafe {
                self.data.words[0].bytes[i]
                    ^ (self.data.words[1].bytes[i] << 2)
                    ^ (self.data.words[2].bytes[i] * 3)
                    ^ (self.data.words[3].bytes[i])
            };
            let new_2nd: u8 = unsafe {
                self.data.words[0].bytes[i]
                    ^ (self.data.words[1].bytes[i])
                    ^ (self.data.words[2].bytes[i] << 2)
                    ^ (self.data.words[3].bytes[i] * 3)
            };
            let new_3rd: u8 = unsafe {
                self.data.words[0].bytes[i] * 3
                    ^ (self.data.words[1].bytes[i])
                    ^ (self.data.words[2].bytes[i])
                    ^ (self.data.words[3].bytes[i] << 2)
            };
            unsafe { self.data.words[0].bytes[i] = new_0th };
            unsafe { self.data.words[1].bytes[i] = new_1st };
            unsafe { self.data.words[2].bytes[i] = new_2nd };
            unsafe { self.data.words[3].bytes[i] = new_3rd };
        }
    }

    pub fn encrypt(mut self: Self, output_buffer: &[u8]) {
        // expand the key
        let expanded_key: [AESWord; 44] = self.key.expand().clone();

        // round 0
        self.add_round_key(0);

        for i in 1..10 {
            self.sub_bytes();
            self.shift_rows();
            self.mix_columns();
            self.add_round_key(4 * i + i * 3);
        }
    }
}
