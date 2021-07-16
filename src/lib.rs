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
    InvalidBool,
    /// While reading NBT, the stream started with a value other than 0x0a.
    InvalidNBTHeader,
    /// While reading NBT, the stream had an invalid data type ID.
    InvalidNBTType
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

/// Provides tools for reading and managing NBT types.
pub mod nbt {
    use super::{Error, read_byte};
    /// Reads an entire NBT compound from a Read type.
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<NamedTag, Error> {
        if read_byte(reader)? != 0x0a {
            return Err(Error::InvalidNBTHeader);
        }
        let root_name = named_tag_name_reader(reader)?;
        let mut elements = vec![];
        loop {
            let next_tag = read_named_tag(reader)?;
            match next_tag.tag {
                Tag::End => {
                    break;
                }
                _ => {
                    elements.push(next_tag);
                }
            }
        }
        return Ok(NamedTag { name: root_name, tag: Tag::Compound(elements) });
    }
    fn named_tag_name_reader<R: std::io::Read>(reader: &mut R) -> Result<String, Error> {
        let string_len = u16::from_be_bytes([read_byte(reader)?; 2]);
        let mut bytes = vec![];
        for _ in 0..string_len {
            bytes.push(read_byte(reader)?);
        }
        // This is required because Mojang uses Java's modified utf-8 which isn't supported here
        unsafe {
            let string = String::from_utf8_unchecked(bytes);
            return Ok(string);
        }
    }
    fn read_named_tag<R: std::io::Read>(reader: &mut R) -> Result<NamedTag, Error> {
        let tag_type = read_byte(reader)?;
        let tag_name;
        if !(tag_type == 0x00) {
            tag_name = named_tag_name_reader(reader)?;
        }
        else {
            tag_name = String::from("N/A");
        }
        let tag_val = read_from_type(reader, tag_type)?;
        return Ok(NamedTag { name: tag_name, tag: tag_val });
    }
    fn read_tag<R: std::io::Read>(reader: &mut R) -> Result<Tag, Error> {
        let tag_type = read_byte(reader)?;
        let tag_val = read_from_type(reader, tag_type)?;
        return Ok(tag_val);
    }
    fn read_from_type<R: std::io::Read>(reader: &mut R, type_id: u8) -> Result<Tag, Error> {
        match type_id {
            0x00 => {
                return Ok(Tag::End);
            }
            0x01 => {
                return Ok(Tag::Byte(i8::from_be_bytes([read_byte(reader)?])));
            }
            0x02 => {
                return Ok(Tag::Short(i16::from_be_bytes([read_byte(reader)?; 2])));
            }
            0x03 => {
                return Ok(Tag::Int(i32::from_be_bytes([read_byte(reader)?; 4])));
            }
            0x04 => {
                return Ok(Tag::Long(i64::from_be_bytes([read_byte(reader)?; 8])));
            }
            0x05 => {
                return Ok(Tag::Float(f32::from_be_bytes([read_byte(reader)?; 4])));
            }
            0x06 => {
                return Ok(Tag::Double(f64::from_be_bytes([read_byte(reader)?; 8])));
            }
            0x07 => {
                let array_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                let mut array = vec![];
                for _ in 0..array_len {
                    array.push(i8::from_be_bytes([read_byte(reader)?]));
                }
                return Ok(Tag::ByteArray(array));
            }
            0x08 => {
                return Ok(Tag::String(named_tag_name_reader(reader)?));
            }
            0x09 => {
                let _list_type = read_byte(reader)?;
                let list_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                if list_len < 1 {
                    return Ok(Tag::List(vec![Tag::End]));
                }
                let mut list_elements = vec![];
                for _ in 0..list_len {
                    list_elements.push(read_tag(reader)?);
                }
                return Ok(Tag::List(list_elements))
            }
            0x0A => {
                let mut compound_elements = vec![];
                loop {
                    let tag = read_named_tag(reader)?;
                    if tag.tag == Tag::End {
                        break;
                    }
                    else {
                        compound_elements.push(tag);
                    }
                }
                return Ok(Tag::Compound(compound_elements));
            }
            0x0B => {
                let array_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                let mut array = vec![];
                for _ in 0..array_len {
                    array.push(i32::from_be_bytes([read_byte(reader)?; 4]));
                }
                return Ok(Tag::IntArray(array));
            }
            0x0C => {
                let array_len = i32::from_be_bytes([read_byte(reader)?; 4]);
                let mut array = vec![];
                for _ in 0..array_len {
                    array.push(i64::from_be_bytes([read_byte(reader)?; 8]));
                }
                return Ok(Tag::LongArray(array));
            }
            _ => {
                return Err(Error::InvalidNBTType);
            }
        }
    }
    #[derive(PartialEq, Clone, Debug)]
    /// Represents a value in a NBT structure.
    pub enum Tag {
        /// A signed byte.
        Byte(i8),
        /// A Java Short.
        Short(i16),
        /// A Java Int.
        Int(i32),
        /// A Java Long.
        Long(i64),
        /// A Java Float.
        Float(f32),
        /// A Java Double.
        Double(f64),
        /// An array of signed bytes.
        ByteArray(Vec<i8>),
        /// A Java modified UTF-8 string.
        String(String),
        /// A list type containing a list of tags without names. All tags will be of the same type.
        List(Vec<Tag>),
        /// A compound type containing a list of named tags.
        Compound(Vec<NamedTag>),
        /// An array of Java Ints.
        IntArray(Vec<i32>),
        /// An array of Java Longs.
        LongArray(Vec<i64>),
        /// Represents the end of a compound or list tag.
        End
    }

    #[derive(PartialEq, Clone, Debug)]
    /// Represents a key-value pair in a NBT structure.
    pub struct NamedTag {
        /// Name of the given tag.
        pub name: String,
        /// Tag of this pair.
        pub tag: Tag
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
    pub fn to_bytes(self) -> Result<Vec<u8>, Error> {
        let xval;
        let yval;
        let zval;
        if self.x < 0 {
            xval = (self.x + (2^26)) as u64;
        }
        else {
            xval = self.x as u64;
        }
        if self.z < 0 {
            zval = (self.x + (2^26)) as u64;
        }
        else {
            zval = self.z as u64;
        }
        if self.y < 0 {
            yval = (self.y + (2^12)) as u64;
        }
        else {
            yval = self.y as u64;
        }
        let u64val: u64 = ((xval & 0x3FFFFFF) << 38) | ((zval & 0x3FFFFFF) << 12) | (yval & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        return Ok(u64bytes.to_vec());
    }
    /// Writes a Position to a Write type.
    pub fn to_writer<W: std::io::Write>(self, writer: &mut W) -> Result<(), Error> {
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

/// `generalized` contains many repetetive and unnecisary functions for reading and writing data.
/// For sake of completion and inclusiveness, all standard types that may be written over the
/// stream, no matter how easy to parse, are included here.
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
    pub fn boolean_from_reader<R: std::io::Read>(reader: &mut R) -> Result<bool, Error> {
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
    pub fn boolean_from_bytes(bytes: &[u8]) -> Result<(bool, usize), Error> {
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
    pub fn boolean_to_writer<W: std::io::Write>(writer: &mut W, data: bool) -> Result<(), Error> {
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
    pub fn boolean_to_bytes(data: bool) -> Result<Vec<u8>, Error> {
        if data {
            return Ok(vec![0x01]);
        }
        else {
            return Ok(vec![0x00]);
        }
    }
    /// Uses a Read type to read a Java Byte from the stream.
    pub fn byte_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i8, Error> {
        let byte = read_byte(reader)?;
        return Ok(i8::from_be_bytes([byte]));
    }
    /// Reads a Java Byte from a list of bytes. Returns the value and number of bytes read.
    pub fn byte_from_bytes(bytes: &[u8]) -> Result<(i8, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        return Ok((i8::from_be_bytes([bytes[0]]), 1));
    }
    /// Writes a Java Byte to a Write type.
    pub fn byte_to_writer<W: std::io::Write>(writer: &mut W, byte: i8) -> Result<(), Error> {
        match writer.write_all(&byte.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Byte as an array of bytes.
    pub fn byte_to_bytes(byte: i8) -> Result<Vec<u8>, Error> {
        return Ok(byte.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read an unsigned Java Byte from the stream.
    pub fn unsigned_byte_from_reader<R: std::io::Read>(reader: &mut R) -> Result<u8, Error> {
        let byte = read_byte(reader)?;
        return Ok(u8::from_be_bytes([byte]));
    }
    /// Reads an unsigned Java Byte from a list of bytes. Returns the value and number of bytes read.
    pub fn unsigned_byte_from_bytes(bytes: &[u8]) -> Result<(u8, usize), Error> {
        if bytes.len() < 1 {
            return Err(Error::MissingData);
        }
        return Ok((u8::from_be_bytes([bytes[0]]), 1));
    }
    /// Writes an unsigned Java Byte to a Write type.
    pub fn unsigned_byte_to_writer<W: std::io::Write>(writer: &mut W, byte: u8) -> Result<(), Error> {
        match writer.write_all(&byte.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns an unsigned Java Byte as an array of bytes.
    pub fn unsigned_byte_to_bytes(byte: u8) -> Result<Vec<u8>, Error> {
        return Ok(byte.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Short from the stream.
    pub fn short_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i16, Error> {
        let bytes = [read_byte(reader)?, read_byte(reader)?];
        return Ok(i16::from_be_bytes(bytes));
    }
    /// Reads a Java Short from a list of bytes. Returns the value and number of bytes read.
    pub fn short_from_bytes(bytes: &[u8]) -> Result<(i16, usize), Error> {
        if bytes.len() < 2 {
            return Err(Error::MissingData);
        }
        return Ok((i16::from_be_bytes([bytes[0], bytes[1]]), 2));
    }
    /// Writes a Java Short to a Write type.
    pub fn short_to_writer<W: std::io::Write>(writer: &mut W, short: i16) -> Result<(), Error> {
        match writer.write_all(&short.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Short as an array of bytes.
    pub fn short_to_bytes(short: i16) -> Result<Vec<u8>, Error> {
        return Ok(short.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read an unsigned Java Short from the stream.
    pub fn unsigned_short_from_reader<R: std::io::Read>(reader: &mut R) -> Result<u16, Error> {
        let bytes = [read_byte(reader)?, read_byte(reader)?];
        return Ok(u16::from_be_bytes(bytes));
    }
    /// Reads an unsigned Java Short from a list of bytes. Returns the value and number of bytes read.
    pub fn unsigned_short_from_bytes(bytes: &[u8]) -> Result<(u16, usize), Error> {
        if bytes.len() < 2 {
            return Err(Error::MissingData);
        }
        return Ok((u16::from_be_bytes([bytes[0], bytes[1]]), 2));
    }
    /// Writes an unsigned Java Short to a Write type.
    pub fn unsigned_short_to_writer<W: std::io::Write>(writer: &mut W, short: u16) -> Result<(), Error> {
        match writer.write_all(&short.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns an unsigned Java Short as an array of bytes.
    pub fn unsigned_short_to_bytes(short: u16) -> Result<Vec<u8>, Error> {
        return Ok(short.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Int from the stream.
    pub fn int_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i32, Error> {
        let bytes = [read_byte(reader)?; 4];
        return Ok(i32::from_be_bytes(bytes));
    }
    /// Reads a Java Int from a list of bytes. Returns the value and number of bytes read.
    pub fn int_from_bytes(bytes: &[u8]) -> Result<(i32, usize), Error> {
        if bytes.len() < 4 {
            return Err(Error::MissingData);
        }
        return Ok((i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 4));
    }
    /// Writes a Java Int to a Write type.
    pub fn int_to_writer<W: std::io::Write>(writer: &mut W, int: i32) -> Result<(), Error> {
        match writer.write_all(&int.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Int as an array of bytes.
    pub fn int_to_bytes(int: i32) -> Result<Vec<u8>, Error> {
        return Ok(int.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Long from the stream.
    pub fn long_from_reader<R: std::io::Read>(reader: &mut R) -> Result<i64, Error> {
        let bytes = [read_byte(reader)?; 8];
        return Ok(i64::from_be_bytes(bytes));
    }
    /// Reads a Java Long from a list of bytes. Returns the value and number of bytes read.
    pub fn long_from_bytes(bytes: &[u8]) -> Result<(i64, usize), Error> {
        if bytes.len() < 8 {
            return Err(Error::MissingData);
        }
        return Ok((i64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]), 8));
    }
    /// Writes a Java Long to a Write type.
    pub fn long_to_writer<W: std::io::Write>(writer: &mut W, long: i64) -> Result<(), Error> {
        match writer.write_all(&long.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Long as an array of bytes.
    pub fn long_to_bytes(long: i64) -> Result<Vec<u8>, Error> {
        return Ok(long.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Float from the stream.
    pub fn float_from_reader<R: std::io::Read>(reader: &mut R) -> Result<f32, Error> {
        let bytes = [read_byte(reader)?; 4];
        return Ok(f32::from_be_bytes(bytes));
    }
    /// Reads a Java Float from a list of bytes. Returns the value and number of bytes read.
    pub fn float_from_bytes(bytes: &[u8]) -> Result<(f32, usize), Error> {
        if bytes.len() < 4 {
            return Err(Error::MissingData);
        }
        return Ok((f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 4));
    }
    /// Writes a Java Float to a Write type.
    pub fn float_to_writer<W: std::io::Write>(writer: &mut W, float: f32) -> Result<(), Error> {
        match writer.write_all(&float.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Float as an array of bytes.
    pub fn float_to_bytes(float: f32) -> Result<Vec<u8>, Error> {
        return Ok(float.to_be_bytes().to_vec());
    }
    /// Uses a Read type to read a Java Double from the stream.
    pub fn double_from_reader<R: std::io::Read>(reader: &mut R) -> Result<f64, Error> {
        let bytes = [read_byte(reader)?; 8];
        return Ok(f64::from_be_bytes(bytes));
    }
    /// Reads a Java Double from a list of bytes. Returns the value and number of bytes read.
    pub fn double_from_bytes(bytes: &[u8]) -> Result<(f64, usize), Error> {
        if bytes.len() < 8 {
            return Err(Error::MissingData);
        }
        return Ok((f64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]), 8));
    }
    /// Writes a Java Double to a Write type.
    pub fn double_to_writer<W: std::io::Write>(writer: &mut W, double: f64) -> Result<(), Error> {
        match writer.write_all(&double.to_be_bytes()) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(Error::WriterError(e));
            }
        }
    }
    /// Returns a Java Double as an array of bytes.
    pub fn double_to_bytes(double: f64) -> Result<Vec<u8>, Error> {
        return Ok(double.to_be_bytes().to_vec());
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
    #[test]
    fn varint_standard_values() -> Result<(), super::Error> {
        use super::VarInt;
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
    fn varlong_standard_values() -> Result<(), super::Error> {
        use super::VarLong;
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
    #[test]
    fn position_standard_values() -> Result<(), super::Error> {
        use super::Position;
        // Create the list of standard values
        let zeroed = Position::from_values(0, 0, 0);
        let max_value = Position::from_values(i32::MAX, i16::MAX, i32::MAX);
        let min_value = Position::from_values(i32::MIN, i16::MIN, i32::MIN);

        // Check that the values are still the same
        assert_eq!(zeroed.get_x(), 0);
        assert_eq!(zeroed.get_y(), 0);
        assert_eq!(zeroed.get_z(), 0);
        assert_eq!(max_value.get_x(), i32::MAX);
        assert_eq!(max_value.get_y(), i16::MAX);
        assert_eq!(max_value.get_z(), i32::MAX);
        assert_eq!(min_value.get_x(), i32::MIN);
        assert_eq!(min_value.get_y(), i16::MIN);
        assert_eq!(min_value.get_z(), i32::MIN);

        // Check that encoding works properly
        assert_eq!(zeroed.to_bytes()?, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(max_value.to_bytes()?, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(min_value.to_bytes()?, [0x00, 0x00, 0x06, 0x00, 0x00, 0x01, 0x80, 0x0E]);
        return Ok(());
    }
}
