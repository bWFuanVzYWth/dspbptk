const T_MD5: [u32; 64] = [
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

const T_MD5F: [u32; 64] = {
    let mut arr = T_MD5;
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

const T_MD5FC: [u32; 64] = {
    let mut arr = T_MD5;
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

const S11: u32 = 7;
const S12: u32 = 12;
const S13: u32 = 17;
const S14: u32 = 22;
const S21: u32 = 5;
const S22: u32 = 9;
const S23: u32 = 14;
const S24: u32 = 20;
const S31: u32 = 4;
const S32: u32 = 11;
const S33: u32 = 16;
const S34: u32 = 23;
const S41: u32 = 6;
const S42: u32 = 10;
const S43: u32 = 15;
const S44: u32 = 21;

const INIT_BUFFER_MD5: [u32; 4] = [
    u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]),
    u32::from_le_bytes([0x89, 0xab, 0xcd, 0xef]),
    u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]),
    u32::from_le_bytes([0x76, 0x54, 0x32, 0x10]),
];

const INIT_BUFFER_MD5F: [u32; 4] = [
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
    state: [u32; 4],
    table: &'static [u32; 64],
}

pub type MD5Hash = [u32; 4];

impl MD5 {
    const fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    const fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    const fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    const fn i(x: u32, y: u32, z: u32) -> u32 {
        y ^ (x | !z)
    }

    #[expect(clippy::many_single_char_names)]
    const fn ff(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(Self::f(b, c, d))
            .wrapping_add(x)
            .wrapping_add(ac)
            .rotate_left(s)
            .wrapping_add(b)
    }

    #[expect(clippy::many_single_char_names)]
    const fn gg(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(Self::g(b, c, d))
            .wrapping_add(x)
            .wrapping_add(ac)
            .rotate_left(s)
            .wrapping_add(b)
    }

    #[expect(clippy::many_single_char_names)]
    const fn hh(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(Self::h(b, c, d))
            .wrapping_add(x)
            .wrapping_add(ac)
            .rotate_left(s)
            .wrapping_add(b)
    }

    #[expect(clippy::many_single_char_names)]
    const fn ii(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32, ac: u32) -> u32 {
        a.wrapping_add(Self::i(b, c, d))
            .wrapping_add(x)
            .wrapping_add(ac)
            .rotate_left(s)
            .wrapping_add(b)
    }

    const fn new(algo: Algo) -> Self {
        let (state, table) = match algo {
            Algo::MD5 => (INIT_BUFFER_MD5, &T_MD5),
            Algo::MD5F => (INIT_BUFFER_MD5F, &T_MD5F),
            Algo::MD5FC => (INIT_BUFFER_MD5F, &T_MD5FC), // 注意此处用的仍然是INIT_MD5F
        };

        Self { state, table }
    }

    #[expect(clippy::many_single_char_names)]
    fn update_block(&mut self, block: &[u8; 64]) {
        let x: [u32; 16] = std::array::from_fn(|i| {
            let i4 = i * 4;
            u32::from_le_bytes([block[i4], block[i4 + 1], block[i4 + 2], block[i4 + 3]])
        });

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        // Round 1
        a = Self::ff(a, b, c, d, x[0], S11, self.table[0]);
        d = Self::ff(d, a, b, c, x[1], S12, self.table[1]);
        c = Self::ff(c, d, a, b, x[2], S13, self.table[2]);
        b = Self::ff(b, c, d, a, x[3], S14, self.table[3]);
        a = Self::ff(a, b, c, d, x[4], S11, self.table[4]);
        d = Self::ff(d, a, b, c, x[5], S12, self.table[5]);
        c = Self::ff(c, d, a, b, x[6], S13, self.table[6]);
        b = Self::ff(b, c, d, a, x[7], S14, self.table[7]);
        a = Self::ff(a, b, c, d, x[8], S11, self.table[8]);
        d = Self::ff(d, a, b, c, x[9], S12, self.table[9]);
        c = Self::ff(c, d, a, b, x[10], S13, self.table[10]);
        b = Self::ff(b, c, d, a, x[11], S14, self.table[11]);
        a = Self::ff(a, b, c, d, x[12], S11, self.table[12]);
        d = Self::ff(d, a, b, c, x[13], S12, self.table[13]);
        c = Self::ff(c, d, a, b, x[14], S13, self.table[14]);
        b = Self::ff(b, c, d, a, x[15], S14, self.table[15]);

        // Round 2
        a = Self::gg(a, b, c, d, x[1], S21, self.table[16]);
        d = Self::gg(d, a, b, c, x[6], S22, self.table[17]);
        c = Self::gg(c, d, a, b, x[11], S23, self.table[18]);
        b = Self::gg(b, c, d, a, x[0], S24, self.table[19]);
        a = Self::gg(a, b, c, d, x[5], S21, self.table[20]);
        d = Self::gg(d, a, b, c, x[10], S22, self.table[21]);
        c = Self::gg(c, d, a, b, x[15], S23, self.table[22]);
        b = Self::gg(b, c, d, a, x[4], S24, self.table[23]);
        a = Self::gg(a, b, c, d, x[9], S21, self.table[24]);
        d = Self::gg(d, a, b, c, x[14], S22, self.table[25]);
        c = Self::gg(c, d, a, b, x[3], S23, self.table[26]);
        b = Self::gg(b, c, d, a, x[8], S24, self.table[27]);
        a = Self::gg(a, b, c, d, x[13], S21, self.table[28]);
        d = Self::gg(d, a, b, c, x[2], S22, self.table[29]);
        c = Self::gg(c, d, a, b, x[7], S23, self.table[30]);
        b = Self::gg(b, c, d, a, x[12], S24, self.table[31]);

        // Round 3
        a = Self::hh(a, b, c, d, x[5], S31, self.table[32]);
        d = Self::hh(d, a, b, c, x[8], S32, self.table[33]);
        c = Self::hh(c, d, a, b, x[11], S33, self.table[34]);
        b = Self::hh(b, c, d, a, x[14], S34, self.table[35]);
        a = Self::hh(a, b, c, d, x[1], S31, self.table[36]);
        d = Self::hh(d, a, b, c, x[4], S32, self.table[37]);
        c = Self::hh(c, d, a, b, x[7], S33, self.table[38]);
        b = Self::hh(b, c, d, a, x[10], S34, self.table[39]);
        a = Self::hh(a, b, c, d, x[13], S31, self.table[40]);
        d = Self::hh(d, a, b, c, x[0], S32, self.table[41]);
        c = Self::hh(c, d, a, b, x[3], S33, self.table[42]);
        b = Self::hh(b, c, d, a, x[6], S34, self.table[43]);
        a = Self::hh(a, b, c, d, x[9], S31, self.table[44]);
        d = Self::hh(d, a, b, c, x[12], S32, self.table[45]);
        c = Self::hh(c, d, a, b, x[15], S33, self.table[46]);
        b = Self::hh(b, c, d, a, x[2], S34, self.table[47]);

        // Round 4
        a = Self::ii(a, b, c, d, x[0], S41, self.table[48]);
        d = Self::ii(d, a, b, c, x[7], S42, self.table[49]);
        c = Self::ii(c, d, a, b, x[14], S43, self.table[50]);
        b = Self::ii(b, c, d, a, x[5], S44, self.table[51]);
        a = Self::ii(a, b, c, d, x[12], S41, self.table[52]);
        d = Self::ii(d, a, b, c, x[3], S42, self.table[53]);
        c = Self::ii(c, d, a, b, x[10], S43, self.table[54]);
        b = Self::ii(b, c, d, a, x[1], S44, self.table[55]);
        a = Self::ii(a, b, c, d, x[8], S41, self.table[56]);
        d = Self::ii(d, a, b, c, x[15], S42, self.table[57]);
        c = Self::ii(c, d, a, b, x[6], S43, self.table[58]);
        b = Self::ii(b, c, d, a, x[13], S44, self.table[59]);
        a = Self::ii(a, b, c, d, x[4], S41, self.table[60]);
        d = Self::ii(d, a, b, c, x[11], S42, self.table[61]);
        c = Self::ii(c, d, a, b, x[2], S43, self.table[62]);
        b = Self::ii(b, c, d, a, x[9], S44, self.table[63]);

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    fn process(&mut self, input: &[u8]) -> MD5Hash {
        let chunks = input.array_chunks::<64>();
        let remainder = chunks.remainder();
        let remainder_len = remainder.len();
        let input_bit = (input.len() * 8).to_le_bytes();
        let input_bit_offset = remainder_len + 1 + 55_usize.wrapping_sub(remainder_len) % 64;

        let last: [u8; 128] = std::array::from_fn(|idx| match idx {
            i if i < remainder_len => remainder[i],
            i if i == remainder_len => 0x80,
            i if i >= input_bit_offset && i < input_bit_offset + 8 => input_bit[i - input_bit_offset],
            _ /* i if i > remainder_len && i < bit_len_offset */ => 0x00,
        });

        for chunk in chunks.chain(last[..input_bit_offset + 8].array_chunks::<64>()) {
            self.update_block(chunk);
        }

        self.state
    }
}

#[must_use]
pub fn compute_md5f_string(header_content: &str) -> String {
    let md5f = MD5::new(Algo::MD5F).process(header_content.as_bytes());
    format!(
        "{:08X}{:08X}{:08X}{:08X}",
        md5f[0].to_be(),
        md5f[1].to_be(),
        md5f[2].to_be(),
        md5f[3].to_be(),
    )
}

#[cfg(test)]
mod test {
    use super::{Algo, MD5, MD5Hash};

    #[test]
    fn test_md5_empty() {
        let hash = MD5::new(Algo::MD5).process(&[]);
        let expected: MD5Hash = [0xD98C_1DD4, 0x4B2_008F, 0x9809_80E9, 0x7E42_F8EC];
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_md5_reference() {
        let text =
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890";
        let hash = MD5::new(Algo::MD5).process(text.as_bytes());
        let expected: MD5Hash = [0xA2F4_ED57, 0x55C9_E32B, 0x2EDA_49AC, 0x7AB6_0721];
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_md5f_empty() {
        let hash = MD5::new(Algo::MD5F).process(&[]);
        let expected: MD5Hash = [0x3BCE_D184, 0xAB49_8FD6, 0x960F_EB26, 0xCF17_6641];
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_md5f_reference() {
        let text =
            "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA";
        let hash = MD5::new(Algo::MD5F).process(text.as_bytes());
        let expected: MD5Hash = [0xCFA1_E5E4, 0x61EC_F128, 0x8C49_331E, 0x2BF0_0DBD];
        assert_eq!(hash, expected);
    }
}
