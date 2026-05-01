//! Shared protobuf encoding/decoding utilities for providers
//!
//! This module provides common protobuf encoding and decoding functions
//! used by multiple providers (e.g., cursor, windsurf) that communicate
//! via protobuf-based gRPC protocols.

use anyhow::{bail, Result};

/// Encode a number as a varint (variable-length integer)
pub fn encode_varint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    while value >= 0x80 {
        bytes.push(((value as u8) & 0x7f) | 0x80);
        value >>= 7;
    }
    bytes.push(value as u8);
    bytes
}

/// Encode a protobuf field with field number, wire type, and value
pub fn encode_field(field_number: u64, wire_type: u8, value: Vec<u8>) -> Vec<u8> {
    let mut bytes = encode_varint((field_number << 3) | u64::from(wire_type));
    match wire_type {
        0 => bytes.extend(value),
        2 => {
            bytes.extend(encode_varint(value.len() as u64));
            bytes.extend(value);
        }
        _ => unreachable!("unsupported wire type"),
    }
    bytes
}

/// Encode a string field (wire type 2: length-delimited)
pub fn encode_string(field_number: u64, str: &str) -> Vec<u8> {
    encode_field(field_number, 2, str.as_bytes().to_vec())
}

/// Encode a nested message field (wire type 2: length-delimited)
pub fn encode_message(field_number: u64, data: &[u8]) -> Vec<u8> {
    encode_field(field_number, 2, data.to_vec())
}

/// Encode a varint field (wire type 0)
pub fn encode_varint_field(field_number: u64, value: u64) -> Vec<u8> {
    encode_field(field_number, 0, encode_varint(value))
}

/// Decode a varint from a buffer starting at offset
/// Returns (value, bytes_consumed)
pub fn decode_varint(bytes: &[u8], index: &mut usize) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0u32;

    loop {
        let byte = bytes.get(*index)
            .ok_or_else(|| anyhow::anyhow!("Unexpected EOF while decoding protobuf varint"))?;
        *index += 1;
        result |= ((byte & 0x7f) as u64) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            bail!("Protobuf varint too large");
        }
    }

    Ok(result)
}

/// Protobuf field value types
#[derive(Debug)]
pub enum FieldValue {
    Varint(u64),
    Bytes(Vec<u8>),
    Fixed32([u8; 4]),
    Fixed64([u8; 8]),
}

/// A parsed protobuf field
#[derive(Debug)]
pub struct ProtobufField {
    pub number: u64,
    pub value: FieldValue,
}

/// Parse protobuf fields from a buffer
pub fn parse_fields(bytes: &[u8]) -> Result<Vec<ProtobufField>> {
    let mut fields = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        let tag = decode_varint(bytes, &mut index)?;
        let field_number = tag >> 3;
        let wire_type = (tag & 0x07) as u8;

        let value = match wire_type {
            0 => FieldValue::Varint(decode_varint(bytes, &mut index)?),
            1 => {
                let mut arr = [0u8; 8];
                for i in 0..8 {
                    arr[i] = bytes.get(index + i)
                        .ok_or_else(|| anyhow::anyhow!("Truncated fixed64 protobuf field"))?
                        .clone();
                }
                index += 8;
                FieldValue::Fixed64(arr)
            }
            2 => {
                let len = decode_varint(bytes, &mut index)? as usize;
                let data = bytes.get(index..index + len)
                    .ok_or_else(|| anyhow::anyhow!("Truncated length-delimited protobuf field"))?
                    .to_vec();
                index += len;
                FieldValue::Bytes(data)
            }
            5 => {
                let mut arr = [0u8; 4];
                for i in 0..4 {
                    arr[i] = bytes.get(index + i)
                        .ok_or_else(|| anyhow::anyhow!("Truncated fixed32 protobuf field"))?
                        .clone();
                }
                index += 4;
                FieldValue::Fixed32(arr)
            }
            _ => bail!("Unsupported protobuf wire type {}", wire_type),
        };

        fields.push(ProtobufField {
            number: field_number,
            value,
        });
    }

    Ok(fields)
}
