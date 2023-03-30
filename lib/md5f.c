#include "md5f.h"

uint32_t F(uint32_t x, uint32_t y, uint32_t z) {
    return (x & y) | (~x & z);
}

uint32_t G(uint32_t x, uint32_t y, uint32_t z) {
    return (x & z) | (y & ~z);
}

uint32_t H(uint32_t x, uint32_t y, uint32_t z) {
    return x ^ y ^ z;
}

uint32_t I(uint32_t x, uint32_t y, uint32_t z) {
    return y ^ (x | ~z);
}

void FF(uint32_t* a, uint32_t b, uint32_t c, uint32_t d, uint32_t mj, int32_t s, uint32_t ti) {
    *a = *a + F(b, c, d) + mj + ti;
    *a = (*a << s) | (*a >> (32 - s));
    *a += b;
}

void GG(uint32_t* a, uint32_t b, uint32_t c, uint32_t d, uint32_t mj, int32_t s, uint32_t ti) {
    *a = *a + G(b, c, d) + mj + ti;
    *a = (*a << s) | (*a >> (32 - s));
    *a += b;
}

void HH(uint32_t* a, uint32_t b, uint32_t c, uint32_t d, uint32_t mj, int32_t s, uint32_t ti) {
    *a = *a + H(b, c, d) + mj + ti;
    *a = (*a << s) | (*a >> (32 - s));
    *a += b;
}

void II(uint32_t* a, uint32_t b, uint32_t c, uint32_t d, uint32_t mj, int32_t s, uint32_t ti) {
    *a = *a + I(b, c, d) + mj + ti;
    *a = (*a << s) | (*a >> (32 - s));
    *a += b;
}

void MD5_Append(uint32_t* buffer, size_t* buffer_len, const char* input, size_t input_len) {
    int32_t num = 0;
    int32_t num2 = 1;
    int32_t num3 = 0;
    int32_t num4 = input_len;
    int32_t num5 = num4 % 64;

    if(num5 < 56) {
        num = 55 - num5;
        num3 = num4 - num5 + 64;
    }
    else if(num5 == 56) {
        num = 63;
        num2 = 1;
        num3 = num4 + 8 + 64;
    }
    else {
        num = 63 - num5 + 56;
        num3 = num4 + 64 - num5 + 64;
    }

    uint8_t* array = (uint8_t*)buffer;
    *buffer_len = num3 / 4;

    for(int32_t i = 0; i < input_len; i++) {
        array[i] = input[i];
    }
    size_t index = input_len;

    if(num2 == 1) {
        array[index++] = (uint8_t)128;
    }
    for(int32_t i = 0; i < num; i++) {
        array[index++] = (uint8_t)0;
    }
    int64_t num6 = (int64_t)num4 * 8L;
    uint8_t b = (uint8_t)(num6 & 0xFF);
    uint8_t b2 = (uint8_t)(((uint64_t)num6 >> 8) & 0xFF);
    uint8_t b3 = (uint8_t)(((uint64_t)num6 >> 16) & 0xFF);
    uint8_t b4 = (uint8_t)(((uint64_t)num6 >> 24) & 0xFF);
    uint8_t b5 = (uint8_t)(((uint64_t)num6 >> 32) & 0xFF);
    uint8_t b6 = (uint8_t)(((uint64_t)num6 >> 40) & 0xFF);
    uint8_t b7 = (uint8_t)(((uint64_t)num6 >> 48) & 0xFF);
    uint8_t b8 = (uint8_t)((uint64_t)num6 >> 56);
    array[index++] = (b);
    array[index++] = (b2);
    array[index++] = (b3);
    array[index++] = (b4);
    array[index++] = (b5);
    array[index++] = (b6);
    array[index++] = (b7);
    array[index++] = (b8);

}

void MD5_Trasform(uint32_t array[4], uint32_t* buffer, size_t buffer_len) {
    uint32_t A = 1732584193u;
    uint32_t B = 4024216457u;
    uint32_t C = 2562383102u;
    uint32_t D = 271734598u;

    for(int32_t i = 0; i < buffer_len; i += 16) {
        uint32_t a = A;
        uint32_t a2 = B;
        uint32_t a3 = C;
        uint32_t a4 = D;

        FF(&a, a2, a3, a4, buffer[i], 7, 3614090360u);
        FF(&a4, a, a2, a3, buffer[i + 1], 12, 3906451286u);
        FF(&a3, a4, a, a2, buffer[i + 2], 17, 606105819u);
        FF(&a2, a3, a4, a, buffer[i + 3], 22, 3250441966u);
        FF(&a, a2, a3, a4, buffer[i + 4], 7, 4118548399u);
        FF(&a4, a, a2, a3, buffer[i + 5], 12, 1200080426u);
        FF(&a3, a4, a, a2, buffer[i + 6], 17, 2821735971u);
        FF(&a2, a3, a4, a, buffer[i + 7], 22, 4249261313u);
        FF(&a, a2, a3, a4, buffer[i + 8], 7, 1770035416u);
        FF(&a4, a, a2, a3, buffer[i + 9], 12, 2336552879u);
        FF(&a3, a4, a, a2, buffer[i + 10], 17, 4294925233u);
        FF(&a2, a3, a4, a, buffer[i + 11], 22, 2304563134u);
        FF(&a, a2, a3, a4, buffer[i + 12], 7, 1805586722u);
        FF(&a4, a, a2, a3, buffer[i + 13], 12, 4254626195u);
        FF(&a3, a4, a, a2, buffer[i + 14], 17, 2792965006u);
        FF(&a2, a3, a4, a, buffer[i + 15], 22, 968099873u);
        GG(&a, a2, a3, a4, buffer[i + 1], 5, 4129170786u);
        GG(&a4, a, a2, a3, buffer[i + 6], 9, 3225465664u);
        GG(&a3, a4, a, a2, buffer[i + 11], 14, 643717713u);
        GG(&a2, a3, a4, a, buffer[i], 20, 3384199082u);
        GG(&a, a2, a3, a4, buffer[i + 5], 5, 3593408605u);
        GG(&a4, a, a2, a3, buffer[i + 10], 9, 38024275u);
        GG(&a3, a4, a, a2, buffer[i + 15], 14, 3634488961u);
        GG(&a2, a3, a4, a, buffer[i + 4], 20, 3889429448u);
        GG(&a, a2, a3, a4, buffer[i + 9], 5, 569495014u);
        GG(&a4, a, a2, a3, buffer[i + 14], 9, 3275163606u);
        GG(&a3, a4, a, a2, buffer[i + 3], 14, 4107603335u);
        GG(&a2, a3, a4, a, buffer[i + 8], 20, 1197085933u);
        GG(&a, a2, a3, a4, buffer[i + 13], 5, 2850285829u);
        GG(&a4, a, a2, a3, buffer[i + 2], 9, 4243563512u);
        GG(&a3, a4, a, a2, buffer[i + 7], 14, 1735328473u);
        GG(&a2, a3, a4, a, buffer[i + 12], 20, 2368359562u);
        HH(&a, a2, a3, a4, buffer[i + 5], 4, 4294588738u);
        HH(&a4, a, a2, a3, buffer[i + 8], 11, 2272392833u);
        HH(&a3, a4, a, a2, buffer[i + 11], 16, 1839030562u);
        HH(&a2, a3, a4, a, buffer[i + 14], 23, 4259657740u);
        HH(&a, a2, a3, a4, buffer[i + 1], 4, 2763975236u);
        HH(&a4, a, a2, a3, buffer[i + 4], 11, 1272893353u);
        HH(&a3, a4, a, a2, buffer[i + 7], 16, 4139469664u);
        HH(&a2, a3, a4, a, buffer[i + 10], 23, 3200236656u);
        HH(&a, a2, a3, a4, buffer[i + 13], 4, 681279174u);
        HH(&a4, a, a2, a3, buffer[i], 11, 3936430074u);
        HH(&a3, a4, a, a2, buffer[i + 3], 16, 3572445317u);
        HH(&a2, a3, a4, a, buffer[i + 6], 23, 76029189u);
        HH(&a, a2, a3, a4, buffer[i + 9], 4, 3654602809u);
        HH(&a4, a, a2, a3, buffer[i + 12], 11, 3873151461u);
        HH(&a3, a4, a, a2, buffer[i + 15], 16, 530742520u);
        HH(&a2, a3, a4, a, buffer[i + 2], 23, 3299628645u);
        II(&a, a2, a3, a4, buffer[i], 6, 4096336452u);
        II(&a4, a, a2, a3, buffer[i + 7], 10, 1126891415u);
        II(&a3, a4, a, a2, buffer[i + 14], 15, 2878612391u);
        II(&a2, a3, a4, a, buffer[i + 5], 21, 4237533241u);
        II(&a, a2, a3, a4, buffer[i + 12], 6, 1700485571u);
        II(&a4, a, a2, a3, buffer[i + 3], 10, 2399980690u);
        II(&a3, a4, a, a2, buffer[i + 10], 15, 4293915773u);
        II(&a2, a3, a4, a, buffer[i + 1], 21, 2240044497u);
        II(&a, a2, a3, a4, buffer[i + 8], 6, 1873313359u);
        II(&a4, a, a2, a3, buffer[i + 15], 10, 4264355552u);
        II(&a3, a4, a, a2, buffer[i + 6], 15, 2734768916u);
        II(&a2, a3, a4, a, buffer[i + 13], 21, 1309151649u);
        II(&a, a2, a3, a4, buffer[i + 4], 6, 4149444226u);
        II(&a4, a, a2, a3, buffer[i + 11], 10, 3174756917u);
        II(&a3, a4, a, a2, buffer[i + 2], 15, 718787259u);
        II(&a2, a3, a4, a, buffer[i + 9], 21, 3951481745u);

        A += a;
        B += a2;
        C += a3;
        D += a4;
    }

    array[0] = A;
    array[1] = B;
    array[2] = C;
    array[3] = D;

}

void md5f(uint32_t md5f_u32[4], void* buffer, const char* stream, size_t stream_len) {
    size_t buffer_len;

    MD5_Append(buffer, &buffer_len, stream, stream_len);
    MD5_Trasform(md5f_u32, buffer, buffer_len);

}

void to_str(char* md5f_hex, uint32_t md5f_u32[4]) {
    for(int i = 0; i < 4; i++) {
        sprintf(md5f_hex + 8 * i, "%02X%02X%02X%02X",
            (uint8_t)((md5f_u32[i]) & 0xFFu),
            (uint8_t)((md5f_u32[i] >> 8) & 0xFFu),
            (uint8_t)((md5f_u32[i] >> 16) & 0xFFu),
            (uint8_t)((md5f_u32[i] >> 24) & 0xFFu));
    }
}

void md5f_str(char* md5f_hex, void* buffer, const char* stream, size_t stream_len) {
    uint32_t md5f_u32[4];
    md5f(md5f_u32, buffer, stream, stream_len);
    to_str(md5f_hex, md5f_u32);

}