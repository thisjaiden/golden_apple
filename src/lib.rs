//! # Overview
//! `golden_apple` is a library for decoding, encoding, and using common types found in Minecraft:
//! Java Edition.
//!
//! # Goals
//! - Provide a generalized format for sharing and using Minecraft's data types
//! - Simplify the decoding and encoding of network data
//!
//! # Usage
//! Proprietary Minecraft types like `VarInt`, `VarLong`, and `Position` are a part of the top
//! level crate. Types that can be fully represented in Rust have encoders/decoders under
//! `golden_apple::generalized`, in case it isn't striaghtforward to do so.

#[derive(Debug)]
/// Represents an error that can occur while using one of the libraries functions.
pub enum Error {
    /// The datastream representing a VarInt exceded the maximum acceptable size.
    VarIntTooLong,
    /// An error occured while using a `Read` type to parse.
    ReaderError(std::io::Error),
    /// An error occured while using a `Write` type to parse.
    WriterError(std::io::Error),
    /// There was not enough data present to parse.
    MissingData,
    /// A boolean had a value other than true or false.
    InvalidBool
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<Error> for () {
    fn from(_: Error) -> Self {
        ()
    }
}

/// Represents a Java Int (i32) using between 1-5 bytes.
#[derive(Eq, Clone, Copy, Debug)]
pub struct VarInt {
    value: i32,
    length: u8
}

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VarInt {{ {:?} }}", self.value)
    }
}

impl PartialEq for VarInt {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            return true;
        }
        else {
            return false;
        }
    }
}

impl VarInt {
    /// Returns the value of a given VarInt
    pub fn value(self) -> i32 {
        return self.value;
    }
    /// Creates a VarInt from a series of bytes. Returns the value and the amount of bytes used if
    /// creation is successful.
    pub fn from_bytes(data: &[u8]) -> Result<(VarInt, usize), Error> {
        let mut iterator = data.iter();
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;

        for i in 0..5 {
            let read;
            match iterator.next() {
                Some(val) => {
                    read = val;
                }
                None => {
                    return Err(Error::MissingData);
                }
            }

            result |= ((read & mask) as i32) << (7 * i);

            // The 5th byte is only allowed to have the 4 smallest bits set
            if i == 4 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }

            if (read & msb) == 0 {
                return Ok((VarInt {value: result, length: i}, i as usize));
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::from_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarInt from a reader containing bytes.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<VarInt, Error> {
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;
    
        for i in 0..5 {
            let read = read_byte(reader)?;
    
            result |= ((read & mask) as i32) << (7 * i);
    
            // The 5th byte is only allowed to have the 4 smallest bits set
            if i == 4 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }
    
            if (read & msb) == 0 {
                return Ok(VarInt {value: result, length: i});
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::from_reader reached end of function, which should not be possible");
    }
    /// Writes a VarInt to a writer as a series of bytes.
    pub fn to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        let msb: u8 = 0b10000000;
        let mask: i32 = 0b01111111;
        let mut val = self.value;

        for _ in 0..5 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                match writer.write_all(&[tmp | msb]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
            } else {
                match writer.write_all(&[tmp]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
                return Ok(());
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_writer reached end of function, which should not be possible");
    } 
    /// Converts a VarInt to a series of bytes.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        let msb: u8 = 0b10000000;
        let mask: i32 = 0b01111111;
        let mut val = self.value;

        for _ in 0..5 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                bytes.push(tmp | msb);
            } else {
                bytes.push(tmp);
                return Ok(bytes);
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarInt from a given value.
    pub fn from_value(value: i32) -> Result<VarInt, Error> {
        Ok(VarInt {
            value,
            length: VarInt::get_len_from_value(value)?
        })
    }
    fn get_len_from_value(value: i32) -> Result<u8, Error> {
        Ok(VarInt { value, length: 0 }.to_bytes()?.len() as u8)
    }
}


/// Represents a Java Long (i64) using between 1-10 bytes.
#[derive(Eq, Clone, Copy, Debug)]
pub struct VarLong {
    value: i64,
    length: u8
}

impl std::fmt::Display for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VarLong {{ {:?} }}", self.value)
    }
}

impl PartialEq for VarLong {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            return true;
        }
        else {
            return false;
        }
    }
}

impl VarLong {
    /// Returns the value of a given VarInt
    pub fn value(self) -> i64 {
        return self.value;
    }
    /// Creates a VarLong from a series of bytes. Returns the value and the amount of bytes used if
    /// creation is successful.
    pub fn from_bytes(data: &[u8]) -> Result<(VarInt, usize), Error> {
        let mut iterator = data.iter();
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;

        for i in 0..10 {
            let read;
            match iterator.next() {
                Some(val) => {
                    read = val;
                }
                None => {
                    return Err(Error::MissingData);
                }
            }

            result |= ((read & mask) as i32) << (7 * i);

            // The 10th byte is only allowed to have the 4 smallest bits set
            if i == 9 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }

            if (read & msb) == 0 {
                return Ok((VarInt {value: result, length: i}, i as usize));
            }
        }
        // This will never occur.
        panic!("golden_apple::VarLong::from_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarLong from a reader containing bytes.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<VarInt, Error> {
        let mut result = 0;

        let msb: u8 = 0b10000000;
        let mask: u8 = !msb;
    
        for i in 0..10 {
            let read = read_byte(reader)?;
    
            result |= ((read & mask) as i32) << (7 * i);
    
            // The 10th byte is only allowed to have the 4 smallest bits set
            if i == 9 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }
    
            if (read & msb) == 0 {
                return Ok(VarInt {value: result, length: i});
            }
        }
        // This will never occur.
        panic!("golden_apple::VarLong::from_reader reached end of function, which should not be possible");
    }
    /// Writes a VarLong to a writer as a series of bytes.
    pub fn to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        let msb: u8 = 0b10000000;
        let mask: i64 = 0b01111111;
        let mut val = self.value;

        for _ in 0..5 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                match writer.write_all(&[tmp | msb]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
            } else {
                match writer.write_all(&[tmp]) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(Error::WriterError(e));
                    }
                }
                return Ok(());
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_writer reached end of function, which should not be possible");
    } 
    /// Converts a VarLong to a series of bytes.
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let mut bytes = vec![];
        let msb: u8 = 0b10000000;
        let mask: i64 = 0b01111111;
        let mut val = self.value;

        for _ in 0..10 {
            let tmp = (val & mask) as u8;
            val &= !mask;
            val = val.rotate_right(7);

            if val != 0 {
                bytes.push(tmp | msb);
            } else {
                bytes.push(tmp);
                return Ok(bytes);
            }
        }
        // This will never occur.
        panic!("golden_apple::VarLong::to_bytes reached end of function, which should not be possible");
    }
    /// Creates a VarLong from a given value.
    pub fn from_value(value: i64) -> Result<VarLong, Error> {
        Ok(VarLong {
            value,
            length: VarLong::get_len_from_value(value)?
        })
    }
    fn get_len_from_value(value: i64) -> Result<u8, Error> {
        Ok(VarLong { value, length: 0 }.to_bytes()?.len() as u8)
    }
}



pub mod generalized {
    use super::Error;
    use super::read_byte;

    /// Reads a `String` from a type implimenting `Read`. This function returns the string without the
    /// VarInt length prefix, and does not verify that the text is utf8.
    pub fn string_from_reader<R: std::io::Read>(reader: &mut R) -> Result<String, Error> {
        let string_len = super::VarInt::from_reader(reader)?.value();
        let mut text: Vec<u8> = vec![0; string_len as usize];
        match reader.read_exact(&mut text) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::ReaderError(e));
            }
        }
        unsafe {
            // Minecraft is known to put weird stuff in their strings, so we're not going to double check.
            return Ok(String::from_utf8_unchecked(text));
        }
    }
    /// Reads a `String` from a series of bytes. This function returns the string without the VarInt
    /// length prefix, but does include the size of that VarInt in the final size calculation. The text
    /// is not verified to be utf8.
    pub fn string_from_bytes(bytes: &[u8]) -> Result<(String, usize), Error> {
        let string_len = super::VarInt::from_bytes(bytes)?;
        let mut text: Vec<u8> = vec![0; string_len.0.value() as usize];
        let finbytes = bytes.split_at(string_len.1).1;
        for i in 0..text.len() {
            text[i] = finbytes[i];
        }
        unsafe {
            // Minecraft is known to put weird stuff in their strings, so we're not going to double check.
            return Ok((String::from_utf8_unchecked(text), string_len.0.value() as usize + string_len.1));
        }
    }
    /// Woefully unnessicary. Seriously, bools are just 0x00 or 0x01.
    pub fn bool_from_reader<R: std::io::Read>(reader: &mut R) -> Result<bool, Error> {
        let byte = read_byte(reader)?;
        if byte == 0x00 {
            return Ok(false);
        }
        if byte == 0x01 {
            return Ok(true);
        }
        return Err(Error::InvalidBool);
    }
    /// Woefully unnessicary. Seriously, bools are just 0x00 or 0x01.
    /// Side note: this function will always read just a single byte, making half of the
    /// return type pointless.
    pub fn bool_from_bytes(bytes: &[u8]) -> Result<(bool, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        if bytes[0] == 0x00 {
            return Ok((false, 1));
        }
        if bytes[0] == 0x01 {
            return Ok((true, 1));
        }
        return Err(Error::InvalidBool);
    }
}

fn read_byte<R: std::io::Read>(reader: &mut R) -> Result<u8, Error> {
    let mut read: [u8; 1] = [0x00];
    match reader.read_exact(&mut read) {
        Ok(_) => {
            return Ok(read[0]);
        },
        Err(e) => {
            return Err(Error::ReaderError(e));
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn varint_standard_values() -> Result<(), Error> {
        // Create the list of standard values
        let val_0 = VarInt::from_value(0)?;
        let val_1 = VarInt::from_value(1)?;
        let val_largest_num = VarInt::from_value(2147483647)?;
        let val_minus_one = VarInt::from_value(-1)?;
        let val_smallest_num = VarInt::from_value(-2147483648)?;

        // Check that the values are still the same
        assert_eq!(val_0.value(), 0);
        assert_eq!(val_1.value(), 1);
        assert_eq!(val_largest_num.value(), 2147483647);
        assert_eq!(val_minus_one.value(), -1);
        assert_eq!(val_smallest_num.value(), -2147483648);

        // Check that encoding works properly
        assert_eq!(val_0.to_bytes()?, [0x00]);
        assert_eq!(val_1.to_bytes()?, [0x01]);
        assert_eq!(val_largest_num.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0x07]);
        assert_eq!(val_minus_one.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0x0f]);
        assert_eq!(val_smallest_num.to_bytes()?, [0x80, 0x80, 0x80, 0x80, 0x08]);

        // Check that decoding works properly
        assert_eq!(val_0.value(), VarInt::from_bytes(&[0x00])?.0.value());
        assert_eq!(val_1.value(), VarInt::from_bytes(&[0x01])?.0.value());
        assert_eq!(val_largest_num.value(), VarInt::from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x07])?.0.value());
        assert_eq!(val_minus_one.value(), VarInt::from_bytes(&[0xff, 0xff, 0xff, 0xff, 0x0f])?.0.value());
        assert_eq!(val_smallest_num.value(), VarInt::from_bytes(&[0x80, 0x80, 0x80, 0x80, 0x08])?.0.value());
        return Ok(());
    }
    #[test]
    fn varlong_standard_values() -> Result<(), Error> {
        // Create the list of standard values
        let val_0 = VarLong::from_value(0)?;
        let val_1 = VarLong::from_value(1)?;
        let val_largest_num = VarLong::from_value(9223372036854775807)?;
        let val_minus_one = VarLong::from_value(-1)?;
        let val_smallest_num = VarLong::from_value(-9223372036854775808)?;

        // Check that the values are still the same
        assert_eq!(val_0.value(), 0);
        assert_eq!(val_1.value(), 1);
        assert_eq!(val_largest_num.value(), 9223372036854775807);
        assert_eq!(val_minus_one.value(), -1);
        assert_eq!(val_smallest_num.value(), -9223372036854775808);

        // Check that encoding works properly
        assert_eq!(val_0.to_bytes()?, [0x00]);
        assert_eq!(val_1.to_bytes()?, [0x01]);
        assert_eq!(val_largest_num.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
        assert_eq!(val_minus_one.to_bytes()?, [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);
        assert_eq!(val_smallest_num.to_bytes()?, [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
        return Ok(());
    }
}
