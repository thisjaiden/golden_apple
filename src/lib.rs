//! # Overview
//! `golden_apple` is a library for decoding, encoding, and using common types found in Minecraft:
//! Java Edition's code.
//!
//! # Goals
//! - Provide a generalized format for sharing and using Minecraft's data types
//! - Simplify the decoding and encoding of network data
//!
//! # Examples
//! Reading a VarInt from bytes:
//! ```rust
//! fn main() -> Result<()> {
//!     // This should be a valid varint (TODO)
//!     let sample_data = [0x00, 0x24];
//!     let decoded_value = golden_apple::VarInt::from_bytes(&sample_data)?;
//!     assert_eq!(decoded_value.0, golden_apple::VarInt::from_value(24));
//!     Ok(())
//! }
//! ```
//!
#[derive(Debug)]
pub enum Error {
    VarIntTooLong,
    VarIntMissingData,
    VarIntReaderError,
    VarIntWriterError
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

/// Represents a Java Int using between 1-5 bytes.
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
    pub fn from_bytes(data: &[u8]) -> Result<(VarInt, u8), Error> {
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
                    return Err(Error::VarIntMissingData);
                }
            }

            result |= ((read & mask) as i32) << (7 * i);

            // The 5th byte is only allowed to have the 4 smallest bits set
            if i == 4 && (read & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }

            if (read & msb) == 0 {
                return Ok((VarInt {value: result, length: i}, i));
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
            let mut read: [u8; 1] = [0x00];
            match reader.read_exact(&mut read) {
                Ok(_) => {},
                Err(_) => {
                    return Err(Error::VarIntReaderError);
                }
            }
    
            result |= ((read[0] & mask) as i32) << (7 * i);
    
            // The 5th byte is only allowed to have the 4 smallest bits set
            if i == 4 && (read[0] & 0xf0 != 0) {
                return Err(Error::VarIntTooLong);
            }
    
            if (read[0] & msb) == 0 {
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
                    Err(_) => {
                        return Err(Error::VarIntWriterError);
                    }
                }
            } else {
                match writer.write_all(&[tmp]) {
                    Ok(_) => {},
                    Err(_) => {
                        return Err(Error::VarIntWriterError);
                    }
                }
                return Ok(());
            }
        }
        // This will never occur.
        panic!("golden_apple::VarInt::to_writer reached end of function, which should not be possible");
    } 
    /// Converts a VarInt to a series of bytes.
    pub fn to_bytes(&mut self) -> Result<Vec<u8>, Error> {
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

pub struct String {

}

#[cfg(test)]
mod tests {
    #[test]
    fn varint_values() {
        // construct known values:
        // shortest possible
        // longest possible
        // mid value
        // negative
    }
}
