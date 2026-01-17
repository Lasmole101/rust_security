//Library of functions to deal with unsigned integers n bytes long
//Has the lsb at position 0

#[derive(Debug)]
pub struct BigInt {
    size: usize,
    content: Vec<u8>,
    error: bool,
}

impl BigInt {
    pub fn init(size_: usize) -> Self {
        Self {
            size: size_,
            content: vec![0; size_],
            error: false,
        }
    }

    pub fn copy(in_: &BigInt) -> BigInt {
        BigInt {
            size: in_.size.clone(),
            content: in_.content.clone(),
            error: in_.error.clone(),
        }
    }

    pub fn new(content_: Vec<u8>) -> BigInt {
        BigInt {
            size: content_.len(),
            content: content_,
            error: false,
        }
    }

    pub fn set(&mut self, content_: Vec<u8>) {
        for i in 0..std::cmp::min(content_.len(), self.size) {
            self.content[i] = content_[i];
        }
    }

    pub fn print_all(&self) {
        println!("size: {0}", self.size);
        print!("contents:");
        for i in self.content.iter() {
            print!("{0:x}", i);
        }
        print!("\n");
    }

    pub fn trim(&mut self) {
        let temp = self.size - 1;
        for i in temp..0 {
            if self.content[i] == 0 {
                self.content.pop();
            } else {
                break;
            }
        }
    }
}
use std::ops;

impl ops::Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
        let mut _size: usize = 0;
        if ((self.content[self.size - 1] as u16) + (rhs.content[rhs.size - 1] + 1) as u16)
            > (u8::MAX as u16)
        {
            _size = std::cmp::max(self.size, rhs.size) + 1;
        } else {
            _size = std::cmp::max(self.size, rhs.size);
        }

        let mut _content: Vec<u8> = vec![0; _size];
        let mut intermediate_sum: u16;
        let mut remainder: u8 = 0;

        for i in 0.._size {
            if (i < self.size) && (i < rhs.size) {
                intermediate_sum =
                    (self.content[i] as u16) + (rhs.content[i] as u16) + (remainder as u16) as u16;
                _content[i] = (intermediate_sum % ((u8::MAX as u16) + 1)) as u8;
                remainder = (intermediate_sum / ((u8::MAX as u16) + 1)) as u8;
            } else if i < rhs.size {
                _content[i] = rhs.content[i];
                remainder = 0;
            } else if i < self.size {
                _content[i] = self.content[i];
                remainder = 0;
            } else {
                _content[i] = remainder;
            }
        }

        BigInt {
            size: _size,
            content: _content,
            error: false,
        }
    }
}

impl ops::Sub for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: BigInt) -> BigInt {
        let mut result: BigInt = BigInt::copy(&self);

        if rhs > self {
            result.error = true;
            return result;
        }

        let mut result: BigInt = BigInt::copy(&self);
        let mut temp: i16;
        let mut carry: bool = false;

        for i in 0..self.size {
            if carry {
                temp = (self.content[i] as i16) - (rhs.content[i] as i16) - 1;
            } else {
                temp = (self.content[i] as i16) - (rhs.content[i] as i16);
            }

            if temp >= 0 {
                result.content[i] = temp as u8;
                carry = false;
            } else {
                temp += 256;
                result.content[i] = temp as u8;
                carry = true;
            }
        }
        if carry {
            result.error = true;
        }
        return result;
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        if self.size != other.size {
            return false;
        }
        if self.content != other.content {
            return false;
        } else {
            return true;
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.size > other.size {
            for i in other.size..self.size {
                if self.content[i] != 0 {
                    return Some(std::cmp::Ordering::Greater);
                }
            }
        } else if other.size > self.size {
            for i in self.size..other.size {
                if other.content[i] != 0 {
                    return Some(std::cmp::Ordering::Less);
                }
            }
        } else {
            for i in 0..self.size {
                if self.content[self.size - 1 - i] > other.content[self.size - 1 - i] {
                    return Some(std::cmp::Ordering::Greater);
                }
                if self.content[self.size - 1 - i] < other.content[self.size - 1 - i] {
                    return Some(std::cmp::Ordering::Less);
                } else if i == self.size - 1 {
                    return Some(std::cmp::Ordering::Equal);
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::big_int;

    #[test]
    fn test_copy_constructor() {
        let test_data = vec![255, 255, 1];
        let test_rhs = big_int::BigInt::new(test_data);
        let test_result = big_int::BigInt::copy(&test_rhs);
        assert_eq!(test_result, test_rhs);
    }

    #[test]
    fn test_zero_lhs() {
        let test_lhs = big_int::BigInt::init(4);
        let test_data = vec![255, 255, 1];
        let test_rhs = big_int::BigInt::new(test_data);
        let test_result = big_int::BigInt::new(vec![255, 255, 1, 0]);
        assert_eq!(test_lhs + test_rhs, test_result);
    }

    #[test]
    fn test_zero_rhs() {
        let test_rhs = big_int::BigInt::init(4);
        let test_data = vec![255, 255, 1];
        let test_lhs = big_int::BigInt::new(test_data);
        let test_result = big_int::BigInt::new(vec![255, 255, 1, 0]);
        assert_eq!(test_lhs + test_rhs, test_result);
    }

    #[test]
    fn easy_sum() {
        let test_data1 = vec![255, 255, 1];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = big_int::BigInt::new(vec![254, 255, 4]);
        assert_eq!(test_lhs + test_rhs, test_result);
    }

    #[test]
    fn easy_sub() {
        let test_data1 = vec![255, 5];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = big_int::BigInt::new(vec![0, 3]);
        assert_eq!(test_lhs - test_rhs, test_result);
    }

    #[test]
    fn sub_carry() {
        let test_data1 = vec![2, 5];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = big_int::BigInt::new(vec![3, 2]);
        assert_eq!(test_lhs - test_rhs, test_result);
    }

    #[test]
    fn greater_than() {
        let test_data1 = vec![255, 255, 3];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = true;
        assert_eq!(test_lhs > test_rhs, test_result);
    }

    #[test]
    fn greater_than_size_diff() {
        let test_data1 = vec![255, 255, 3];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = true;
        assert_eq!(test_lhs > test_rhs, test_result);
    }

    #[test]
    fn less_than() {
        let test_data1 = vec![255, 255, 1];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = true;
        assert_eq!(test_lhs < test_rhs, test_result);
    }

    #[test]
    fn less_than_size_diff() {
        let test_data1 = vec![255, 255];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = true;
        assert_eq!(test_lhs < test_rhs, test_result);
    }

    #[test]
    fn equal_to() {
        let test_data1 = vec![255, 255, 2];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = true;
        assert_eq!(test_lhs == test_rhs, test_result);
    }

    #[test]
    fn sum_with_overflow() {
        let test_data1 = vec![255, 255, 255];
        let test_lhs = big_int::BigInt::new(test_data1);
        let test_data2 = vec![255, 255, 2];
        let test_rhs = big_int::BigInt::new(test_data2);
        let test_result = big_int::BigInt::new(vec![254, 255, 2, 1]);
        assert_eq!(test_lhs + test_rhs, test_result);
    }
}
