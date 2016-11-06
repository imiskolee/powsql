use std::collections::HashMap;
use super::data_type::Int1;

lazy_static! {

    static ref collections: HashMap<&'static str, Int1> = {

    let mut h = HashMap::<&'static str, Int1>::new();

    h.insert("big5_chinese_ci", 1);
    h.insert("latin2_czech_cs", 2);
    h.insert("dec8_swedish_ci", 3);
    h.insert("cp850_general_ci", 4);
    h.insert("latin1_german1_ci", 5);
    h.insert("hp8_english_ci", 6);
    h.insert("koi8r_general_ci", 7);
    h.insert("latin1_swedish_ci", 8);
    h.insert("latin2_general_ci", 9);
    h.insert("swe7_swedish_ci", 10);
    h.insert("ascii_general_ci", 11);
    h.insert("ujis_japanese_ci", 12);
    h.insert("sjis_japanese_ci", 13);
    h.insert("cp1251_bulgarian_ci", 14);
    h.insert("latin1_danish_ci", 15);
    h.insert("hebrew_general_ci", 16);
    h.insert("tis620_thai_ci", 18);
    h.insert("euckr_korean_ci", 19);
    h.insert("latin7_estonian_cs", 20);
    h.insert("latin2_hungarian_ci", 21);
    h.insert("koi8u_general_ci", 22);
    h.insert("cp1251_ukrainian_ci", 23);
    h.insert("gb2312_chinese_ci", 24);
    h.insert("greek_general_ci", 25);
    h.insert("cp1250_general_ci", 26);
    h.insert("latin2_croatian_ci", 27);
    h.insert("gbk_chinese_ci", 28);
    h.insert("cp1257_lithuanian_ci", 29);
    h.insert("latin5_turkish_ci", 30);
    h.insert("latin1_german2_ci", 31);
    h.insert("armscii8_general_ci", 32);
    h.insert("utf8_general_ci", 33);
    h.insert("cp1250_czech_cs", 34);
    h.insert("ucs2_general_ci", 35);
    h.insert("cp866_general_ci", 36);
    h.insert("keybcs2_general_ci", 37);
    h.insert("macce_general_ci", 38);
    h.insert("macroman_general_ci", 39);
    h.insert("cp852_general_ci", 40);
    h.insert("latin7_general_ci", 41);
    h.insert("latin7_general_cs", 42);
    h.insert("macce_bin", 43);
    h.insert("cp1250_croatian_ci", 44);
    h.insert("utf8mb4_general_ci", 45);
    h.insert("utf8mb4_bin", 46);
    h.insert("latin1_bin", 47);
    h.insert("latin1_general_ci", 48);
    h.insert("latin1_general_cs", 49);
    h.insert("cp1251_bin", 50);
    h.insert("cp1251_general_ci", 51);
    h.insert("cp1251_general_cs", 52);
    h.insert("macroman_bin", 53);
    h.insert("utf16_general_ci", 54);
    h.insert("utf16_bin", 55);
    h.insert("utf16le_general_ci", 56);
    h.insert("cp1256_general_ci", 57);
    h.insert("cp1257_bin", 58);
    h.insert("cp1257_general_ci", 59);
    h.insert("utf32_general_ci", 60);
    h.insert("utf32_bin", 61);
    h.insert("utf16le_bin", 62);
    h.insert("binary", 63);
    h.insert("armscii8_bin", 64);
    h.insert("ascii_bin", 65);
    h.insert("cp1250_bin", 66);
    h.insert("cp1256_bin", 67);
    h.insert("cp866_bin", 68);
    h.insert("dec8_bin", 69);
    h.insert("greek_bin", 70);
    h.insert("hebrew_bin", 71);
    h.insert("hp8_bin", 72);
    h.insert("keybcs2_bin", 73);
    h.insert("koi8r_bin", 74);
    h.insert("koi8u_bin", 75);
    h.insert("latin2_bin", 77);
    h.insert("latin5_bin", 78);
    h.insert("latin7_bin", 79);
    h.insert("cp850_bin", 80);
    h.insert("cp852_bin", 81);
    h.insert("swe7_bin", 82);
    h.insert("utf8_bin", 83);
    h.insert("big5_bin", 84);
    h.insert("euckr_bin", 85);
    h.insert("gb2312_bin", 86);
    h.insert("gbk_bin", 87);
    h.insert("sjis_bin", 88);
    h.insert("tis620_bin", 89);
    h.insert("ucs2_bin", 90);
    h.insert("ujis_bin", 91);
    h.insert("geostd8_general_ci", 92);
    h.insert("geostd8_bin", 93);
    h.insert("latin1_spanish_ci", 94);
    h.insert("cp932_japanese_ci", 95);
    h.insert("cp932_bin", 96);
    h.insert("eucjpms_japanese_ci", 97);
    h.insert("eucjpms_bin", 98);
    h.insert("cp1250_polish_ci", 99);
    h.insert("utf16_unicode_ci", 101);
    h.insert("utf16_icelandic_ci", 102);
    h.insert("utf16_latvian_ci", 103);

    h
    };
}





fn valid(charset: &'static str) -> Option<Int1> {
    unsafe {
        match collections.get(charset) {
            None => None,
            Some(v) => Some(*v),
        }
    }
}
