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

/// Converts an entire NBT compound into an array of bytes. This must be a full NBT compound.
pub fn to_bytes(root_tag: NamedTag) -> Result<Vec<u8>, Error> {
    let mut final_bytes = vec![];
    // Add start tag
    final_bytes.push(0x0a);
    // Add root tag name
    for byte in root_tag.name.as_bytes() {
        final_bytes.push(*byte);
    }
    // Add root tag components
    if let Tag::Compound(cmptag) = root_tag.tag {
        for tag in cmptag {
            let prefix = tag.tag.clone().tag_prefix();
            final_bytes.push(prefix);
            if prefix == 0 {
                break;
            }
            let name = tag.name.as_bytes();
            for byte in &(name.len() as u16).to_be_bytes() {
                final_bytes.push(*byte);
            }
            for byte in name {
                final_bytes.push(*byte);
            }
            for byte in tag.tag.write_to_bytes()? {
                final_bytes.push(byte);
            }
        }
    }
    else {
        return Err(Error::InvalidRootTag);
    }
    // Add end tag
    final_bytes.push(0x00);
    return Ok(final_bytes);
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

impl Tag {
    fn tag_prefix(self) -> u8 {
        match self {
            Self::End => 0,
            Self::Byte(_) => 1,
            Self::Short(_) => 2,
            Self::Int(_) => 3,
            Self::Long(_) => 4,
            Self::Float(_) => 5,
            Self::Double(_) => 6,
            Self::ByteArray(_) => 7,
            Self::String(_) => 8,
            Self::List(_) => 9,
            Self::Compound(_) => 10,
            Self::IntArray(_) => 11,
            Self::LongArray(_) => 12
        }
    }
    /// Writes this tag to a series of bytes. Does not include the tag's type ID prefix. Does
    /// include list and compound tag's ending byte.
    pub fn write_to_bytes(self) -> Result<Vec<u8>, Error> {
        match self {
            // The end tag has no data.
            Self::End => return Ok(vec![]),
            // It would be great to compact these as they use similar footprints, but the
            // different data types prevent doing this practically.
            Self::Byte(data) => {
                return Ok(data.to_be_bytes().to_vec());
            },
            Self::Short(data) => {
                return Ok(data.to_be_bytes().to_vec());
            },
            Self::Int(data) => {
                return Ok(data.to_be_bytes().to_vec());
            },
            Self::Long(data) => {
                return Ok(data.to_be_bytes().to_vec());
            },
            Self::Float(data) => {
                return Ok(data.to_be_bytes().to_vec());
            },
            Self::Double(data) => {
                return Ok(data.to_be_bytes().to_vec());
            },
            Self::ByteArray(data) => {
                let len_prefix = data.len() as i32;
                let mut final_data = vec![];
                for byte in &len_prefix.to_be_bytes() {
                    final_data.push(*byte);
                }
                for byte in data {
                    final_data.push(byte.to_be_bytes()[0]);
                }
                return Ok(final_data);
            },
            Self::IntArray(data) => {
                let len_prefix = data.len() as i32;
                let mut final_data = vec![];
                for byte in &len_prefix.to_be_bytes() {
                    final_data.push(*byte);
                }
                for chunk in data {
                    for byte in &chunk.to_be_bytes() {
                        final_data.push(*byte);
                    }
                }
                return Ok(final_data);
            },
            Self::LongArray(data) => {
                let len_prefix = data.len() as i32;
                let mut final_data = vec![];
                for byte in &len_prefix.to_be_bytes() {
                    final_data.push(*byte);
                }
                for chunk in data {
                    for byte in &chunk.to_be_bytes() {
                        final_data.push(*byte);
                    }
                }
                return Ok(final_data);
            },
            Self::String(data) => {
                let mut final_data = vec![];
                let strbytes = data.as_bytes();
                for byte in &(strbytes.len() as u16).to_be_bytes() {
                    final_data.push(*byte);
                }
                for byte in strbytes {
                    final_data.push(*byte);
                }
                return Ok(final_data);
            },
            Self::List(data) => {
                let mut final_data = vec![];
                final_data.push(data[0].clone().tag_prefix());
                for byte in &(data.len() as i32).to_be_bytes() {
                    final_data.push(*byte);
                }
                for element in data {
                    for byte in element.write_to_bytes()? {
                        final_data.push(byte);
                    }
                }
                final_data.push(0x00);
                return Ok(final_data);
            },
            Self::Compound(data) => {
                let mut final_data = vec![];
                for named_tag in data {
                    final_data.push(named_tag.tag.clone().tag_prefix());
                    let name_bytes = named_tag.name.as_bytes();
                    for byte in &(name_bytes.len() as u16).to_be_bytes() {
                        final_data.push(*byte);
                    }
                    for byte in name_bytes {
                        final_data.push(*byte);
                    }
                    for byte in named_tag.tag.write_to_bytes()? {
                        final_data.push(byte);
                    }
                }
                final_data.push(0x00);
                return Ok(final_data);
            }
        }
    }
}

 #[derive(PartialEq, Clone, Debug)]
/// Represents a key-value pair in a NBT structure.
pub struct NamedTag {
    /// Name of the given tag.
    pub name: String,
    /// Tag of this pair.
    pub tag: Tag
}
