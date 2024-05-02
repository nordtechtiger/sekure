use std::ops::Add;


#[derive(Debug)]
pub struct BigU288([u8; 36]); // 288 bit unsigned integer (8x36)


impl Add for BigU288 {
    type Output = BigU288;
    fn add(self, other: Self) -> Self::Output {
        let mut output = BigU288::new();
        for (index, byte) in self.0.iter().enumerate() { // LSB first
            output.0[index] += byte.wrapping_add(other.0[index]);
            let carry = (*byte as u16 + other.0[index] as u16) - output.0[index] as u16;
            output.0[std::cmp::min(index+1, output.0.len()-1)] = carry as u8;
        };
        output
    }
}

impl PartialEq for BigU288 {
    fn eq(&self, other: &BigU288) -> bool {
        self.0 == other.0
    }
}
impl Eq for BigU288 {}

impl BigU288 {
    fn add_msb(&mut self) {
        // TODO: Important! Attempt to solve this in constant time
        let mut i: bool = false; // Flag to see if we've hit the msb yet
        for (index, byte) in self.0.iter().rev().enumerate() { // Enumerate backwards (msb first)
            // let bit_1 = byte & 0b1000_0000; // Shift 
            // let bit_1 = byte & 0b0000_0001; // Shift 
        }
        todo!("msb isn't implemented yet");
    }
    fn from_slice(bytes: &[u8]) -> BigU288 {
        todo!();
    }
    fn from_hex(input: &str) -> BigU288 {
        let mut big_u288 = BigU288::new();
        // Iterate over the string backwards (we want little endian)
        for (index, char) in input.chars().rev().enumerate() {
            let nibble = u8::from_str_radix(&char.to_string(), 16).expect("invalid character found");
            big_u288.0[index/2] += nibble << 4*(index%2);
        };
        big_u288
    }
    fn get_bytes(&self) -> [u8;36] {
        self.0
    }
    fn new() -> BigU288 {
        BigU288([0; 36])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn add_msb_1() {
    // }
    
    // #[test]
    // fn from_hex_1() {
    //     let big_u288 = BigU288::from_hex("3fffffffffffffffffffffffffffffffb");
    //     assert_eq!(
    //         BigU288::from_hex()
    //     )
    // }

    #[test]
    fn from_hex_2() {
        assert_eq!(
            BigU288::from_hex("f").get_bytes(),
            [15u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        );
    }

    #[test]
    fn from_hex_3() {
        assert_eq!(
            BigU288::from_hex("ff").get_bytes(),
            [255u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        );
    }

    #[test]
    fn from_hex_4() {
        assert_eq!(
            BigU288::from_hex("1fff").get_bytes(),
            [255u8,31u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        );
    }

    // #[test]
    // fn add_msb_1() {
    //     let mut big_u288 = BigU288::from_hex("9");
    //     big_u288.add_msb();
    // }

    #[test]
    fn add_1() {
        let big_u288 = BigU288::from_hex("1");
        let add = BigU288::from_hex("ff");
        assert_eq!(
            BigU288::from_hex("1") + BigU288::from_hex("ff"),
            BigU288::from_hex("101")
        );
    }
}
