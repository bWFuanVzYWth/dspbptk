const K: [u32; 64] = [
    0xd76a_a478,
    0xe8c7_b756,
    0x2420_70db,
    0xc1bd_ceee,
    0xf57c_0faf,
    0x4787_c62a,
    0xa830_4613,
    0xfd46_9501,
    0x6980_98d8,
    0x8b44_f7af,
    0xffff_5bb1,
    0x895c_d7be,
    0x6b90_1122,
    0xfd98_7193,
    0xa679_438e,
    0x49b4_0821,
    0xf61e_2562,
    0xc040_b340,
    0x265e_5a51,
    0xe9b6_c7aa,
    0xd62f_105d,
    0x0244_1453,
    0xd8a1_e681,
    0xe7d3_fbc8,
    0x21e1_cde6,
    0xc337_07d6,
    0xf4d5_0d87,
    0x455a_14ed,
    0xa9e3_e905,
    0xfcef_a3f8,
    0x676f_02d9,
    0x8d2a_4c8a,
    0xfffa_3942,
    0x8771_f681,
    0x6d9d_6122,
    0xfde5_380c,
    0xa4be_ea44,
    0x4bde_cfa9,
    0xf6bb_4b60,
    0xbebf_bc70,
    0x289b_7ec6,
    0xeaa1_27fa,
    0xd4ef_3085,
    0x0488_1d05,
    0xd9d4_d039,
    0xe6db_99e5,
    0x1fa2_7cf8,
    0xc4ac_5665,
    0xf429_2244,
    0x432a_ff97,
    0xab94_23a7,
    0xfc93_a039,
    0x655b_59c3,
    0x8f0c_cc92,
    0xffef_f47d,
    0x8584_5dd1,
    0x6fa8_7e4f,
    0xfe2c_e6e0,
    0xa301_4314,
    0x4e08_11a1,
    0xf753_7e82,
    0xbd3a_f235,
    0x2ad7_d2bb,
    0xeb86_d391,
];

const K_MD5F: [u32; 64] = {
    let mut arr = K;
    arr[1] = 0xe8d7_b756;
    arr[6] = 0xa830_4623;
    arr[12] = 0x6b9f_1122;
    arr[15] = 0x39b4_0821;
    arr[19] = 0xc9b6_c7aa;
    arr[21] = 0x0244_3453;
    arr[24] = 0x21f1_cde6;
    arr[27] = 0x475a_14ed;
    arr
};

const K_MD5FC: [u32; 64] = {
    let mut arr = K;
    arr[1] = 0xe8d7_b756;
    arr[3] = 0xc1bd_ceef;
    arr[6] = 0xa830_4623;
    arr[12] = 0x6b9f_1122;
    arr[15] = 0x39b4_0821;
    arr[19] = 0xc9b6_c7aa;
    arr[21] = 0x0244_3453;
    arr[24] = 0x23f1_cde6;
    arr[27] = 0x475a_14ed;
    arr[34] = 0x6d9d_6121;
    arr
};

const S: &[u32; 64] = &[
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const INIT_MD5: [u32; 4] = [
    u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]),
    u32::from_le_bytes([0x89, 0xab, 0xcd, 0xef]),
    u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]),
    u32::from_le_bytes([0x76, 0x54, 0x32, 0x10]),
];

const INIT_MD5F: [u32; 4] = [
    u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]),
    u32::from_le_bytes([0x89, 0xab, 0xdc, 0xef]),
    u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]),
    u32::from_le_bytes([0x46, 0x57, 0x32, 0x10]),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Algo {
    MD5,
    MD5F,
    MD5FC,
}

pub struct MD5 {
    s: [u32; 4],
    k_table: &'static [u32; 64],
}

pub type MD5Hash = [u8; 16];

impl MD5 {
    const fn new(algo: Algo) -> Self {
        let (s, k_table) = match algo {
            Algo::MD5 => (INIT_MD5, &K),
            Algo::MD5F => (INIT_MD5F, &K_MD5F),
            Algo::MD5FC => (INIT_MD5F, &K_MD5FC), // 注意此处用的仍然是INIT_MD5F
        };

        Self { s, k_table }
    }

    #[expect(clippy::many_single_char_names)]
    fn update_block(&mut self, buf: &[u8; 64]) {
        let mut buf_iter = buf.array_chunks::<4>();
        let words: [u32; 16] = std::array::from_fn(|_| {
            u32::from_le_bytes(*buf_iter.next().expect("unreachable: buf.len() < 64"))
        });

        let mut a = self.s[0];
        let mut b = self.s[1];
        let mut c = self.s[2];
        let mut d = self.s[3];

        for (idx, &s) in S[0..16].iter().enumerate() {
            let i = idx;
            let function_result = (b & c) | (!b & d);
            let word_index = i;
            let f = function_result
                .wrapping_add(a)
                .wrapping_add(self.k_table[i])
                .wrapping_add(words[word_index]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s));
        }

        for (idx, &s) in S[16..32].iter().enumerate() {
            let i = 16 + idx;
            let function_result = (d & b) | (!d & c);
            let word_index = (5 * i + 1) % 16;
            let f = function_result
                .wrapping_add(a)
                .wrapping_add(self.k_table[i])
                .wrapping_add(words[word_index]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s));
        }

        for (idx, &s) in S[32..48].iter().enumerate() {
            let i = 32 + idx;
            let function_result = b ^ c ^ d;
            let word_index = (3 * i + 5) % 16;
            let f = function_result
                .wrapping_add(a)
                .wrapping_add(self.k_table[i])
                .wrapping_add(words[word_index]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s));
        }

        for (idx, &s) in S[48..64].iter().enumerate() {
            let i = 48 + idx;
            let function_result = c ^ (b | !d);
            let word_index = (7 * i) % 16;
            let f = function_result
                .wrapping_add(a)
                .wrapping_add(self.k_table[i])
                .wrapping_add(words[word_index]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s));
        }

        self.s[0] = self.s[0].wrapping_add(a);
        self.s[1] = self.s[1].wrapping_add(b);
        self.s[2] = self.s[2].wrapping_add(c);
        self.s[3] = self.s[3].wrapping_add(d);
    }

    fn process(&mut self, data: &[u8]) -> MD5Hash {
        let chunks = data.array_chunks::<64>();
        let remainder = chunks.remainder();

        let remainder_len = remainder.len();
        let pad_len = (56 - ((remainder_len + 1) % 64)) % 64;
        let bit_len = data.len() * 8;
        let bit_start = remainder_len + 1 + pad_len;

        let last: [u8; 128] = std::array::from_fn(|idx| match idx {
            i if i < remainder_len => remainder[i],
            i if i == remainder_len => 0x80,
            i if i >= bit_start && i < bit_start + 8 => bit_len.to_le_bytes()[i - bit_start],
            _ => 0,
        });

        for chunk in chunks.chain(last[..bit_start + 8].array_chunks::<64>()) {
            self.update_block(chunk);
        }

        std::array::from_fn(|i| {
            let word_index = i / 4;
            let byte_index = i % 4;
            self.s[word_index].to_le_bytes()[byte_index]
        })
    }
}

#[must_use]
pub fn compute_md5f_string(header_content: &str) -> String {
    let md5f = MD5::new(Algo::MD5F).process(header_content.as_bytes());
    format!(
        "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        md5f[0],
        md5f[1],
        md5f[2],
        md5f[3],
        md5f[4],
        md5f[5],
        md5f[6],
        md5f[7],
        md5f[8],
        md5f[9],
        md5f[10],
        md5f[11],
        md5f[12],
        md5f[13],
        md5f[14],
        md5f[15]
    )
}

#[cfg(test)]
mod test {
    use super::{Algo, MD5, MD5Hash};

    #[test]
    fn test_md5_empty() {
        let hash = MD5::new(Algo::MD5).process(&[]);
        let expected: MD5Hash = [
            0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8,
            0x42, 0x7e,
        ];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5_reference() {
        let text =
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890";
        let hash = MD5::new(Algo::MD5).process(text.as_bytes());
        let expected: MD5Hash = [
            0x57, 0xed, 0xf4, 0xa2, 0x2b, 0xe3, 0xc9, 0x55, 0xac, 0x49, 0xda, 0x2e, 0x21, 0x07,
            0xb6, 0x7a,
        ];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5f_empty() {
        let hash = MD5::new(Algo::MD5F).process(&[]);
        let expected: MD5Hash = [
            0x84, 0xd1, 0xce, 0x3b, 0xd6, 0x8f, 0x49, 0xab, 0x26, 0xeb, 0x0f, 0x96, 0x41, 0x66,
            0x17, 0xcf,
        ];
        assert!(hash == expected);
    }

    #[test]
    fn test_md5f_reference() {
        let text =
            "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA";
        let hash = MD5::new(Algo::MD5F).process(text.as_bytes());
        let expected: MD5Hash = [
            0xe4, 0xe5, 0xa1, 0xcf, 0x28, 0xf1, 0xec, 0x61, 0x1e, 0x33, 0x49, 0x8c, 0xbd, 0x0d,
            0xf0, 0x2b,
        ];
        assert!(hash == expected);
    }
}
