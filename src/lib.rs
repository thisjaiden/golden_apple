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

#[derive(Clone, Copy, Eq, PartialEq)]
/// Represents a position in the Minecraft world. Not the floating point values used for player
/// movement, but the whole number values used for things like block positions.
pub struct Position {
    // 26 bits for x and z, rounds up to 32
    // 12 for y rounds up to 16
    x: i32,
    y: i16,
    z: i32
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position {{ {}, {}, {} }}", self.x, self.y, self.z)
    }
}

impl Position {
    /// Returns the x coordinate of this Position.
    pub fn get_x(self) -> i32 {
        return self.x;
    }
    /// Returns the y coordinate of this Position.
    pub fn get_y(self) -> i16 {
        return self.y;
    }
    /// Returns the z coordinate of this Position.
    pub fn get_z(self) -> i32 {
        return self.z
    }
    /// Creates a Position from a series of bytes. Requires 8 bytes or more in the buffer. Also
    /// returns how many bytes were used in this function, which should always be 8.
    pub fn from_bytes(data: &[u8]) -> Result<(Position, usize), Error> {
        if data.len() < 8 {
            return Err(Error::MissingData);
        }

        let mut toconvert = [0; 8];
        let indexable_data = data.split_at(8).0;
        for i in 0..8 {
            toconvert[i] = indexable_data[i]
        }
        // convert to one big u64
        let u64val = u64::from_be_bytes(toconvert);

        // strip out values with bitmasks
        let mut x = (u64val >> 38) as i32;
        let mut y = (u64val & 0xfff) as i16;
        let mut z = (u64val << 26 >> 38) as i32;

        // convert to negative if appropriate
        if x >= 2^25 {
            x -= 2^26;
        }
        if y >= 2^11 {
            y -= 2^12;
        }
        if z >= 2^25 {
            z -= 2^26
        }
        return Ok((Position { x, y, z }, 8));
    }
    /// Creates a Position from a Read type.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Position, Error> {
        let mut toconvert = [0; 8];
        for i in 0..8 {
            toconvert[i] = read_byte(reader)?;
        }
        let u64val = u64::from_be_bytes(toconvert);

        // strip out values with bitmasks
        let mut x = (u64val >> 38) as i32;
        let mut y = (u64val & 0xfff) as i16;
        let mut z = (u64val << 26 >> 38) as i32;

        // convert to negative if appropriate
        if x >= 2^25 {
            x -= 2^26;
        }
        if y >= 2^11 {
            y -= 2^12;
        }
        if z >= 2^25 {
            z -= 2^26
        }
        return Ok(Position { x, y, z });
    }
    /// Creates a Position from coordinate values.
    pub fn from_values(x: i32, y: i16, z: i32) -> Position {
        Position {
            x, y, z
        }
    }
    /// Converts a Position into a series of bytes.
    /// WARNING: THIS FUNCTION HAS NOT BEEN VERIFIED TO WORK WITH NEGATIVE POSITIONS PROPERLY
    pub fn to_bytes(&mut self) -> Result<Vec<u8>, Error> {
        // This is some magic voodo from wiki.vg. No gaurentee it works, especially with the typecasts on negatives.
        let u64val: u64 = ((self.x as u64 & 0x3FFFFFF) << 38) | ((self.z as u64 & 0x3FFFFFF) << 12) | (self.y as u64 & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        return Ok(u64bytes.to_vec());
    }
    /// Writes a Position to a Write type.
    /// WARNING: THIS FUNCTION HAS NOT BEEN VERIFIED TO WORK WITH NEGATIVE POSITIONS PROPERLY
    pub fn to_writer<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), Error> {
        // This is some magic voodo from wiki.vg. No gaurentee it works, especially with the typecasts on negatives.
        let u64val: u64 = ((self.x as u64 & 0x3FFFFFF) << 38) | ((self.z as u64 & 0x3FFFFFF) << 12) | (self.y as u64 & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        match writer.write_all(&u64bytes) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
}


pub mod generalized {
    use super::Error;
    use super::read_byte;
    use super::VarInt;

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
    /// Writes a `String` to a Write interface.
    pub fn string_to_writer<W: std::io::Write>(writer: &mut W, data: String) -> Result<(), Error> {
        let as_bytes = data.as_bytes();
        let length_prefix = VarInt::from_value(as_bytes.len() as i32)?;
        match writer.write_all(&length_prefix.to_bytes()?) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
        match writer.write_all(as_bytes) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
        return Ok(());
    }
    /// Converts a `String` to a VarInt length prefixed series of bytes.
    pub fn string_to_bytes(data: String) -> Result<Vec<u8>, Error> {
        let as_bytes = data.as_bytes();
        let length_prefix = VarInt::from_value(as_bytes.len() as i32)?;
        let mut vec_vals = as_bytes.to_vec();
        for byte in length_prefix.to_bytes()? {
            vec_vals.push(byte);
        }
        return Ok(vec_vals);
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
    /// Either writes 0x00 or 0x01 to the writer. Come on, you don't need this.
    pub fn bool_to_writer<W: std::io::Write>(writer: &mut W, data: bool) -> Result<(), Error> {
        if data {
            match writer.write_all(&[0x01]) {
                Ok(_) => {},
                Err(e) => {
                    return Err(Error::WriterError(e));
                }
            }
        }
        else {
            match writer.write_all(&[0x00]) {
                Ok(_) => {},
                Err(e) => {
                    return Err(Error::WriterError(e));
                }
            }
        }
        return Ok(());
    }
    /// What's wrong with you? This isn't something you should need or use. It's one byte. It's not
    /// even possible to get an error here.
    pub fn bool_to_bytes(data: bool) -> Result<Vec<u8>, Error> {
        if data {
            return Ok(vec![0x01]);
        }
        else {
            return Ok(vec![0x00]);
        }
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
