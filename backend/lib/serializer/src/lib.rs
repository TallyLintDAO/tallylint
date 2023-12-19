use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::error::Error;
use std::io::{Read, Write};
pub fn serialize<T, W>(value: T, writer: W) -> Result<(), impl Error>
where
  T: Serialize,
  W: Write,
{
  // Rust MessagePack library
  // MessagePack is a binary serialization format
  // MessagePack is diff with CBOR(Concise Binary Object Representation,base on
  // json data model),which is more standard and more functions.
  // let mut serializer = rmp_serde::Serializer::new(writer).with_struct_map();
  // with_struct_map(), the serializer will instead encode Rust structs as
  // MessagePack map types. This means that the serialized data will include
  // both the names and the values of the struct's fields. This is slightly less
  // compact, but it means that the data can be deserialized even if the
  // deserializer doesn't know the exact structure of the struct.

  let mut serializer = serde_json::Serializer::new(writer);

  value.serialize(&mut serializer).map(|_| ())
  // it's ignoring the Ok value (represented by _ in the closure |_|) and just
  // returning () the unit type. This effectively discards the Ok value and
  // only leaves the Err value if there was an error.
}

pub fn deserialize<T, R>(reader: R) -> Result<T, impl Error>
where
  T: DeserializeOwned,
  R: Read,
{
  // let mut deserializer = rmp_serde::Deserializer::new(reader);
  // let mut deserializer = serde_json::Deserializer::new(rd);
  serde_json::from_reader(reader)
  // T::deserialize(&mut deserializer)
}
