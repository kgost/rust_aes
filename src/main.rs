const SBOX: [u8; 256] = [
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
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d 
];

const RMATRIX: [u8; 16] = [
    2, 3, 1, 1,
    1, 2, 3, 1,
    1, 1, 2, 3,
    3, 1, 1, 2
];

const INV_RMATRIX: [u8; 16] = [
    14, 11, 13, 9,
    9, 14, 11, 13,
    13, 9, 14, 11,
    11, 13, 9, 14
];

mod aes {
    fn rotate( word: &mut [u8; 4] ) {
        let a = word[0];

        for i in 0..3 {
            word[i] = word[i + 1];
        }

        word[3] = a;
    }

    fn rcon( mut input: u8 ) -> u8 {
        let mut c = 1;

        while input != 1 {
            let b = c & 0x80;
            c <<= 1;

            if b == 0x80 {
                c ^= 0x1b;
            }

            input -= 1;
        }

        c
    }

    fn schedule_core( word: &mut[u8; 4], index: u8 ) {
        rotate( word );

        for i in 0..4 {
            word[i] = super::SBOX[word[i] as usize];
        }

        let mut tmp = word[0];

        tmp ^= rcon( index );

        word[0] = tmp;
    }

    fn key_expansion( key: &[u8; 16] ) -> [[u8; 16]; 11] {
        let mut round_keys = [[0 as u8; 16]; 11];
        let mut expansion = [0 as u8; 176];
        let mut count = 16;
        let mut t = [0 as u8; 4];
        let mut index = 1;

        for i in 0..16 {
            expansion[i] = key[i];
        }

        while count < 176 {
            for i in 0..4 {
                t[i] = expansion[i + count - 4];
            }

            if count % 16 == 0 {
                schedule_core( &mut t, index );
                index += 1;
            }

            for i in 0..4 {
                expansion[count] = expansion[count - 16] ^ t[i];
                count += 1;
            }
        }

        for i in 0..176 {
            round_keys[i / 16][i % 16] = expansion[i];
        }

        round_keys
    }

    fn add_round_key( state: &mut [u8; 16], round_key: [u8; 16] ) {
        for i in 0..16 {
            state[i] ^= round_key[i];
        }
    }

    fn g_mul( a: u8, b: u8 ) -> u8 {
        let mut a_cp = a;
        let mut b_cp = b;
        let mut p: u8 = 0;

        for _ in 0..8 {
            let a_high = a_cp & 0x80;

            if b_cp & 1 == 1 {
                p ^= a_cp;
            }

            a_cp <<= 1;

            if a_high == 0x80 {
                a_cp ^= 0x1b
            }

            b_cp >>= 1;
        }

        p
    }

    pub fn encrypt( message: &[u8; 16], key: &[u8; 16] ) -> [u8; 16] {
        let mut state = message.clone();

        let round_keys: [[u8; 16]; 11] = key_expansion( key );

        add_round_key( &mut state, round_keys[0] );

        for i in 1..10 {
            forward::sub_bytes( &mut state );
            forward::shift_rows( &mut state );
            forward::mix_columns( &mut state );
            add_round_key( &mut state, round_keys[i] );
        }

        forward::sub_bytes( &mut state );
        forward::shift_rows( &mut state );
        add_round_key( &mut state, round_keys[10] );

        state
    }

    pub fn decrypt( cipher_text: &[u8; 16], key: &[u8; 16] ) -> [u8; 16] {
        let mut state = cipher_text.clone();

        let round_keys: [[u8; 16]; 11] = key_expansion( key );

        add_round_key( &mut state, round_keys[10] );
        backward::shift_rows( &mut state );
        backward::sub_bytes( &mut state );

        for i in ( 1..10 ).rev() {
            add_round_key( &mut state, round_keys[i] );
            backward::mix_columns( &mut state );
            backward::shift_rows( &mut state );
            backward::sub_bytes( &mut state );
        }

        add_round_key( &mut state, round_keys[0] );

        state
    }

    mod forward {
        pub fn sub_bytes( state: &mut [u8; 16] ) {
            for i in 0..16 {
                state[i] = super::super::SBOX[state[i] as usize];
            }
        }

        pub fn shift_rows( state: &mut [u8; 16] ) {
            // 0 1 2 3
            // 4 5 6 7
            // 8 9 0 1
            // 2 3 4 5
            //
            // 0 1 2 3
            // 5 6 7 4
            // 0 1 8 9
            // 5 2 3 4
            let tmp = state.clone();

            state[4] = tmp[5];
            state[5] = tmp[6];
            state[6] = tmp[7];
            state[7] = tmp[4];

            state[8] = tmp[10];
            state[9] = tmp[11];
            state[10] = tmp[8];
            state[11] = tmp[9];

            state[12] = tmp[15];
            state[13] = tmp[12];
            state[14] = tmp[13];
            state[15] = tmp[14];
        }

        pub fn mix_columns( state: &mut [u8; 16] ) {
            // 0 1 2 3
            // 4 5 6 7
            // 8 9 0 1
            // 2 3 4 5
            
            // 2 3 1 1
            // 1 2 3 1
            // 1 1 2 3
            // 3 1 1 2
            let state_cp = state.clone();

            for i in 0..16 {
                let mut r_start = i;

                while r_start % 4 != 0 {
                    r_start -= 1;
                }

                let mut state_start = i;

                while state_start > 3 {
                    state_start -= 4;
                }

                state[i] =  super::g_mul( super::super::RMATRIX[r_start], state_cp[state_start] ) ^
                            super::g_mul( super::super::RMATRIX[r_start + 1], state_cp[state_start + 4] ) ^
                            super::g_mul( super::super::RMATRIX[r_start + 2], state_cp[state_start + 8] ) ^
                            super::g_mul( super::super::RMATRIX[r_start + 3], state_cp[state_start + 12] );
            }
        }
    }

    mod backward {
        pub fn sub_bytes( state: &mut [u8; 16] ) {
            for i in 0..16 {
                state[i] = super::super::INV_SBOX[state[i] as usize];
            }
        }

        pub fn shift_rows( state: &mut [u8; 16] ) {
            // 0 1 2 3
            // 4 5 6 7
            // 8 9 0 1
            // 2 3 4 5
            //
            // 0 1 2 3
            // 7 4 5 6
            // 0 1 8 9
            // 3 4 5 2
            let tmp = state.clone();

            state[4] = tmp[7];
            state[5] = tmp[4];
            state[6] = tmp[5];
            state[7] = tmp[6];

            state[8] = tmp[10];
            state[9] = tmp[11];
            state[10] = tmp[8];
            state[11] = tmp[9];

            state[12] = tmp[13];
            state[13] = tmp[14];
            state[14] = tmp[15];
            state[15] = tmp[12];
        }

        pub fn mix_columns( state: &mut [u8; 16] ) {
            // 0 1 2 3
            // 4 5 6 7
            // 8 9 0 1
            // 2 3 4 5
            
            // 2 3 1 1
            // 1 2 3 1
            // 1 1 2 3
            // 3 1 1 2
            let state_cp = state.clone();

            for i in 0..16 {
                let mut r_start = i;

                while r_start % 4 != 0 {
                    r_start -= 1;
                }

                let mut state_start = i;

                while state_start > 3 {
                    state_start -= 4;
                }

                state[i] =  super::g_mul( super::super::INV_RMATRIX[r_start], state_cp[state_start] ) ^
                            super::g_mul( super::super::INV_RMATRIX[r_start + 1], state_cp[state_start + 4] ) ^
                            super::g_mul( super::super::INV_RMATRIX[r_start + 2], state_cp[state_start + 8] ) ^
                            super::g_mul( super::super::INV_RMATRIX[r_start + 3], state_cp[state_start + 12] );
            }
        }
    }
}

fn main() {
    let message = "thisismyexamplem";
    let key =     "feffjefffeffjeff";

    let mut message_chars = message.chars();
    let mut key_chars = key.chars();

    let mut message_u8: [u8; 16] = [0; 16];
    let mut key_u8: [u8; 16] = [0; 16];

    for i in 0..16 {
        message_u8[i] = message_chars.next().unwrap() as u8;
        key_u8[i] = key_chars.next().unwrap() as u8;
    }

    let cipher_text = aes::encrypt( &message_u8, &key_u8 );
    let mut cipher_text_char: [char; 16] = ['0'; 16];

    for i in 0..16 {
        cipher_text_char[i] = cipher_text[i] as char;
    }

    println!( "{:?}", cipher_text_char );

    let plain_text_u8 = aes::decrypt( &cipher_text, &key_u8 );
    let mut plain_text_char: [char; 16] = ['0'; 16];

    for i in 0..16 {
        plain_text_char[i] = plain_text_u8[i] as char;
    }

    println!( "{:?}", plain_text_char );
}
