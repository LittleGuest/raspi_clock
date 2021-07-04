//! 字符映射

/// 数字 0
#[rustfmt::skip]
pub const NUM_0: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00101000,
    0b00101000,
    0b00101000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 数字 1
#[rustfmt::skip]
pub const NUM_1: [u8; 8] = [
    0b00000000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00000000,
    0b00000000,
];

/// 数字 2
#[rustfmt::skip]
pub const NUM_2: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00001000,
    0b00111000,
    0b00100000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 数字 3
#[rustfmt::skip]
pub const NUM_3: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00001000,
    0b00111000,
    0b00001000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 数字 4
#[rustfmt::skip]
pub const NUM_4: [u8; 8] = [
    0b00000000,
    0b00101000,
    0b00101000,
    0b00111000,
    0b00001000,
    0b00001000,
    0b00000000,
    0b00000000,
];

/// 数字 5
#[rustfmt::skip]
pub const NUM_5: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00100000,
    0b00111000,
    0b00001000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 数字 6
#[rustfmt::skip]
pub const NUM_6: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00100000,
    0b00111000,
    0b00101000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 数字 7
#[rustfmt::skip]
pub const NUM_7: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00001000,
    0b00001000,
    0b00001000,
    0b00001000,
    0b00000000,
    0b00000000,
];

/// 数字 8
#[rustfmt::skip]
pub const NUM_8: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00101000,
    0b00111000,
    0b00101000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 数字 9
#[rustfmt::skip]
pub const NUM_9: [u8; 8] = [
    0b00000000,
    0b00111000,
    0b00101000,
    0b00111000,
    0b00001000,
    0b00111000,
    0b00000000,
    0b00000000,
];

/// 冒号
#[rustfmt::skip]
pub const COLON: [u8; 8] = [
    0b00000000,
    0b00000000,
    0b00010000,
    0b00000000,
    0b00010000,
    0b00000000,
    0b00000000,
    0b00000000,
];

/// 字符 !
#[rustfmt::skip]
pub const EXCLAMATION_MARK: [u8; 8] = [
    0b00000000,
    0b00100000,
    0b00100000,
    0b00100000,
    0b00000000,
    0b00100000,
    0b00000000,
    0b00000000,
];

/// 无效字符
#[rustfmt::skip]
pub const INVALID: [u8; 8] = [
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
];

/// 大写英文字母 C
#[rustfmt::skip]
pub const UPPER_C: [u8; 8] = [
    0b00000000,
    0b00110000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b00110000,
    0b00000000,
    0b00000000,
];
/// 大写英文字母 L
#[rustfmt::skip]
pub const UPPER_L: [u8; 8] = [
    0b00000000,
    0b00100000,
    0b00100000,
    0b00100000,
    0b00100000,
    0b00111000,
    0b00000000,
    0b00000000,
];
/// 大写英文字母 K
#[rustfmt::skip]
pub const UPPER_K: [u8; 8] = [
    0b00000000,
    0b00101000,
    0b00110000,
    0b00100000,
    0b00110000,
    0b00101000,
    0b00000000,
    0b00000000,
];

/// 编码
pub fn encode_char(c: char) -> [u8; 8] {
    match c {
        '0' => NUM_0,
        '1' => NUM_1,
        '2' => NUM_2,
        '3' => NUM_3,
        '4' => NUM_4,
        '5' => NUM_5,
        '6' => NUM_6,
        '7' => NUM_7,
        '8' => NUM_8,
        '9' => NUM_9,
        ':' => COLON,
        '!' => EXCLAMATION_MARK,
        'C' | 'c' => UPPER_C,
        'L' | 'l' => UPPER_L,
        'K' | 'k' => UPPER_K,
        _ => INVALID,
    }
}

/// 编码
pub fn encode_string(s: &str) -> Vec<[u8; 8]> {
    s.chars().into_iter().map(encode_char).collect()
}

/// 合并
/// 为了显示美观，将时间（时分秒）合并显示
/// 13:46:50
pub fn merge_time(a: &mut [[u8; 8]]) -> Vec<[u8; 8]> {
    if a.len() != 8 {
        return Vec::with_capacity(0);
    }

    a.iter_mut().enumerate().for_each(|(i, b)| match i {
        0 => {
            mv_left(b, 2);
        }
        1 => {
            mv_right(b, 3);
        }
        2 => {
            mv_left(b, 2);
        }
        3 => {
            mv_right(b, 2);
        }
        4 => {
            mv_left(b, 1);
        }
        5 => {
            mv_right(b, 3);
        }
        6 => {
            mv_left(b, 2);
        }
        7 => {
            mv_right(b, 3);
        }
        _ => {}
    });

    vec![
        merge_bit(&a[0], &a[1]),
        merge_bit(&a[2], &a[3]),
        merge_bit(&a[4], &a[5]),
        merge_bit(&a[6], &a[7]),
    ]
}

/// 向左移动
fn mv_left(a: &mut [u8; 8], offset: u32) {
    a.iter_mut().for_each(|v| {
        *v <<= offset;
    });
}

/// 向右移动
fn mv_right(a: &mut [u8; 8], offset: u32) {
    a.iter_mut().for_each(|v| {
        *v >>= offset;
    });
}

/// 合并两个数组：按位与
fn merge_bit(a: &[u8; 8], b: &[u8; 8]) -> [u8; 8] {
    let mut bits: [u8; 8] = [0; 8];
    (0..8).enumerate().for_each(|(i, _)| {
        bits[i] = a[i] | b[i];
    });
    bits
}

#[test]
fn test_rotate() {
    let a = 0b00111000_u8;
    assert_eq!(0b00001110_u8, a.rotate_right(2));
    let a = 0b00001000_u8;
    assert_eq!(0b00100000_u8, a.rotate_left(2));
}

#[test]
#[rustfmt::skip]
fn test_merge_time() {
    let mut a = encode_string("13:46:50");
    let mut except: Vec<[u8; 8]> = Vec::with_capacity(4);
    except.push([
        0b00000000,
        0b01000111,
        0b01000001,
        0b01000111,
        0b01000001,
        0b01000111,
        0b00000000,
        0b00000000,
    ]);
    except.push([
        0b00000000,
        0b00001010,
        0b01001010,
        0b00001110,
        0b01000010,
        0b00000010,
        0b00000000,
        0b00000000,
    ]);
    except.push([
        0b00000000,
        0b01110000,
        0b01000010,
        0b01110000,
        0b01010010,
        0b01110000,
        0b00000000,
        0b00000000,
    ]);
    except.push([
        0b00000000,
        0b11100111,
        0b10000101,
        0b11100101,
        0b00100101,
        0b11100111,
        0b00000000,
        0b00000000,
    ]);
    assert_eq!(except, merge_time(&mut a));
}

#[test]
#[rustfmt::skip]
fn test_merge_bit() {
    let num_2: [u8; 8] = [
        0b00000000,
        0b00111000,
        0b00001000,
        0b00111000,
        0b00100000,
        0b00111000,
        0b00000000,
        0b00000000,
    ];
    let num_1: [u8; 8] = [
        0b00000000,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000000,
        0b00000000,
    ];

    let except: Vec<u8> = vec![
        0b00000000,
        0b00111010,
        0b00001010,
        0b00111010,
        0b00100010,
        0b00111010,
        0b00000000,
        0b00000000,
    ];
    assert_eq!(except, merge_bit(&num_2, &num_1));

    let num_1: [u8; 8] = [
        0b00000000,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000000,
        0b00000000,
    ];
    let upper_k: [u8; 8] = [
        0b00000000,
        0b00101000,
        0b00110000,
        0b00100000,
        0b00110000,
        0b00101000,
        0b00000000,
        0b00000000,
    ];

    let except: Vec<u8> = vec![
        0b00000000,
        0b00101010,
        0b00110010,
        0b00100010,
        0b00110010,
        0b00101010,
        0b00000000,
        0b00000000,
    ];
    assert_eq!(except, merge_bit(&num_1, &upper_k));
}
