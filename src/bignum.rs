use std::fmt;
use std::ops::{Add, Mul, Rem, Sub};

#[derive(Debug, Copy, Clone)]
pub struct BigU288([u8; 36]); // 288 bit unsigned integer (8x36)

impl fmt::Display for BigU288 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Add for BigU288 {
    type Output = BigU288;
    fn add(self, other: Self) -> Self::Output {
        let mut output = self;
        let mut carry = 0;
        for (i, byte) in output.0.iter_mut().enumerate() {
            // LSB first
            let sum: u64 = *byte as u64 + other.0[i] as u64 + carry as u64;
            *byte = (sum % 256) as u8;
            carry = sum / 256;
        }
        if carry > 0 {
            panic!("overflow");
        }
        output
    }
}

impl Sub for BigU288 {
    type Output = BigU288;
    fn sub(self, other: Self) -> Self::Output {
        let mut output = self;
        let mut carry = 0;
        for (i, byte) in output.0.iter_mut().enumerate() {
            let difference: i64 = *byte as i64 - other.0[i] as i64 - carry as i64;
            *byte = ((difference + 256) % 256) as u8;
            carry = difference.is_negative() as u8;
        }
        if carry > 0 {
            panic!("overflow");
        }
        output
    }
}

impl Mul for BigU288 {
    type Output = BigU288;
    fn mul(self, other: Self) -> Self::Output {
        let mut total_sum = BigU288::new();
        for (i, byte_self) in self.0.iter().enumerate() {
            // Multiply entire second number by each byte in self
            let mut working_sum = other;
            let mut carry = 0;
            for byte_other in working_sum.0.iter_mut() {
                let product = *byte_other as u64 * *byte_self as u64 + carry as u64;
                *byte_other = (product % 256) as u8;
                carry = product / 256;
            }
            if carry > 0 {
                panic!("overflow");
            }
            working_sum.0.rotate_right(i);
            total_sum = total_sum + working_sum;
        }
        total_sum
    }
}

// This is slow. TODO: Look into implementing a more performant algorithm!
impl Rem for BigU288 {
    type Output = BigU288;
    fn rem(self, other: Self) -> Self::Output {
        // let mut quotient = BigU288::new();
        // let mut numerator = self;
        // while numerator<0 {
        //     numerator = numerator - other;
        // }

        todo!("rem not implemented");
    }
}

// I don't actually know if a simple == is constant time, but to be on the safe side I implemented
// a constant time loop.
impl PartialEq<BigU288> for BigU288 {
    fn eq(&self, other: &BigU288) -> bool {
        let mut equal = 1;
        for (i, byte_self) in self.0.iter().enumerate() {
            equal &= (*byte_self == other.0[i]) as u8;
        }
        equal == 1
    }
}

// impl PartialEq<u8> for BigU288 {
//     fn eq(&self, other: &u8) -> bool {
//         self.0[0] == *other
//     }
// }

impl PartialOrd<BigU288> for BigU288 {
    fn lt(&self, other: &Self) -> bool {
        let mut lt = 0;
        for (i, byte_self) in self.0.iter().enumerate() {
            lt = (*byte_self < other.0[i]) as u8 | (lt & (*byte_self==other.0[i]) as u8) as u8;
        }
        lt == 1
    }
    fn gt(&self, other: &Self) -> bool {
        let mut gt = 0;
        todo!("implement gt");
    }
    fn le(&self, other: &Self) -> bool {
        let mut le = 0;
        todo!("implement le");
    }
    fn ge(&self, other: &Self) ->  bool {
        let mut ge = 0;
        todo!("implement ge");
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!("implement partialcmp");
    }
}

impl Eq for BigU288 {}

impl BigU288 {
    pub fn from_slice(bytes: &[u8]) -> BigU288 {
        let mut big_u288 = BigU288::new();
        big_u288.0 = pad_array_bigu288(bytes).as_slice().try_into().unwrap();
        big_u288
    }
    pub fn from_hex(input: &str) -> BigU288 {
        let mut big_u288 = BigU288::new();
        // Iterate over the string backwards (we want little endian)
        let input_padded_le: [u8; 72] = pad_array_hex(&input.bytes().rev().collect::<Vec<_>>()[..]);
        for (i, char) in input_padded_le.iter().enumerate() {
            let hex_digit = u8::from_str_radix(
                &String::from_utf8(vec![*char]).unwrap_or("0".to_string()),
                16,
            )
            .unwrap_or(0);
            big_u288.0[i / 2] += hex_digit << 4 * (i % 2);
        }
        big_u288
    }
    pub fn to_hex(&self) -> String {
        let mut out = String::new();
        for byte in self.get_bytes().iter().rev() {
            out += &format!("{:x}{:x}", byte >> 4, byte & 15);
        }
        out
    }
    pub fn get_bytes(&self) -> [u8; 36] {
        self.0
    }
    pub fn new() -> BigU288 {
        BigU288([0; 36])
    }
}

fn pad_array_hex(input: &[u8]) -> [u8; 72] {
    let mut padded = [0u8; 72]; // TODO: Make this configurable
    padded[..input.len()].copy_from_slice(input);
    padded
}

fn pad_array_bigu288(input: &[u8]) -> [u8; 36] {
    let mut padded = [0u8; 36]; // TODO: Make this configurable
    padded[..input.len()].copy_from_slice(input);
    padded
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn from_hex_1() {
    //     let big_u288 = BigU288::from_hex("3fffffffffffffffffffffffffffffffb");
    //     assert_eq!(
    //         BigU288::from_hex()
    //     )
    // }

    #[test]
    fn less_than_1() {
        assert_eq!(
            BigU288::from_hex("f0") < BigU288::from_hex("ff"),
            true
        );
    }

    #[test]
    fn less_than_2() {
        assert_eq!(
            BigU288::from_slice(&[0, 255, 0]) < BigU288::from_slice(&[255, 0, 255]),
            true
        );
    }

    #[test]
    fn less_than_3() {
        assert_eq!(
            BigU288::from_hex("ffffff") < BigU288::from_hex("ffffff"),
            false
        );
    }
 
    #[test]
    fn to_hex_1() {
        assert_eq!(
            BigU288::from_hex("BABAFAFA").to_hex(),
            "0000000000000000000000000000000000000000000000000000000000000000babafafa"
        );
    }

    #[test]
    fn to_hex_2() {
        assert_eq!(
            BigU288::from_hex("3fffffffffffffffffffffffffffffffb").to_hex(),
            "0000000000000000000000000000000000000003fffffffffffffffffffffffffffffffb"
        );
    }

    #[test]
    fn from_slice_1() {
        assert_eq!(BigU288::from_slice(&[1, 1]), BigU288::from_hex("101"));
    }

    #[test]
    fn from_slice_2() {
        assert_eq!(BigU288::from_slice(&[255, 16]), BigU288::from_hex("10FF"));
    }

    #[test]
    fn from_hex_3() {
        assert_eq!(
            BigU288::from_hex("ff").get_bytes(),
            [
                255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn from_hex_4() {
        assert_eq!(
            BigU288::from_hex("1fff").get_bytes(),
            [
                255u8, 31u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    // #[test]
    // fn add_msb_1() {
    //     let mut big_u288 = BigU288::from_hex("9");
    //     big_u288.add_msb();
    // }

    #[test]
    fn add_1() {
        assert_eq!(
            BigU288::from_hex("1") + BigU288::from_hex("ff"),
            BigU288::from_hex("100")
        );
    }

    #[test]
    fn add_2() {
        assert_eq!(
            BigU288::from_hex("C1583054D5A6350B37E23A")
                + BigU288::from_hex("2A677ACE04C0037CA98B6BC"),
            BigU288::from_hex("367cfdd3521a66cd5d098f6")
        );
    }

    #[test]
    fn add_3() {
        assert_eq!(
            BigU288::from_slice(&[0, 255, 255]) + BigU288::from_slice(&[255, 255, 0]),
            BigU288::from_slice(&[255, 254, 0, 1])
        );
    }

    #[test]
    fn sub_1() {
        assert_eq!(
            BigU288::from_hex("ff") - BigU288::from_hex("0f"),
            BigU288::from_hex("f0")
        );
    }

    #[test]
    fn sub_2() {
        assert_eq!(
            BigU288::from_slice(&[0, 255]) - BigU288::from_slice(&[255, 0]),
            BigU288::from_slice(&[1, 254])
        );
    }

    #[test]
    fn sub_3() {
        assert_eq!(
            BigU288::from_hex(
                "f40d1ebbd170aa4d28a333d8b12a27a70535f29f3e841e5655201f4ef7f31afc36ec06be"
            ) - BigU288::from_hex(
                "546030bdb669182f46cecd7a76c9ebb8249caa348f243cdce2a692ad90e9b15fe4f29116"
            ),
            BigU288::from_hex(
                "9facedfe1b07921de1d4665e3a603beee099486aaf5fe17972798ca16709699c51f975a8"
            )
        );
    }

    #[test]
    fn pad_array_hex_1() {
        assert_eq!(
            pad_array_hex(&[255, 255]),
            [
                255u8, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn multiply_1() {
        assert_eq!(
            BigU288::from_slice(&[255, 100]) * BigU288::from_slice(&[005, 000]),
            BigU288::from_slice(&[251, 248, 001])
        );
    }

    #[test]
    fn multiply_2() {
        assert_eq!(
            BigU288::from_slice(&[255, 255, 255, 000]) * BigU288::from_slice(&[255, 000, 000, 000]),
            BigU288::from_slice(&[001, 255, 255, 254])
        );
    }

    #[test]
    fn multiply_3() {
        assert_eq!(
            BigU288::from_slice(&[255, 255, 255]) * BigU288::from_slice(&[255, 255, 255]),
            BigU288::from_hex("fffffe000001")
        );
    }

    #[test]
    fn multiply_4() {
        assert_eq!(
            BigU288::from_slice(&[
                67, 114, 121, 112, 116, 111, 103, 114, 97, 112, 104, 105, 99, 32, 70
            ]) * BigU288::from_slice(&[
                133, 214, 190, 8, 84, 85, 109, 3, 124, 68, 82, 14, 64, 213, 6, 8
            ]),
            BigU288::from_hex("232e2481e77d27fa798895e14ee9e0f2779453994ac90ed284034da565ecf")
        );
    }

    #[test]
    fn multiply_5() {
        assert_eq!(
            BigU288::from_slice(&[255, 255]) * BigU288::from_slice(&[0, 1]),
            BigU288::from_slice(&[0, 255, 255])
        );
    }
}
