use schemars::JsonSchema;
use validator::Validate;

use crate::json::{type_name_pretty, schema::{u8_schema, i16_schema, i8_schema, u16_schema, double_schema}};
use crate::json::validation::out_of_range_err;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Parameter(pub i16); // 12768-52768 (-20000 - +20000)

impl Parameter {
    const ZERO: u16 = 32768;
    pub const MIN: i16 = -20000;
    pub const MAX: i16 = 20000;
}

impl From<u16> for Parameter {
    fn from(value: u16) -> Self {
        if value >= Self::ZERO {
            Self((value - Self::ZERO) as i16)
        } else {
            Self(-1 * (Self::ZERO - value) as i16)
        }
    }
}

impl Into<u16> for Parameter {
    fn into(self) -> u16 {
        if self.0 >= 0 {
            self.0 as u16 + Self::ZERO
        } else {
            Self::ZERO - self.0.abs() as u16
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Self::from(Self::ZERO)
    }
}

impl JsonSchema for Parameter {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        i16_schema(Self::MIN, Self::MAX)
    }
}

impl Validate for Parameter {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0 < Self::MIN || self.0 > Self::MAX {
            Err(out_of_range_err("0", &Self::MIN, &Self::MAX))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct OffsetU8<const OFFSET: u8, const MIN: u8, const MAX: u8>(i8); // MIN(0)-MAX(255) (MIN-OFFSET - MAX-OFFSET)

impl<const O: u8, const L: u8, const H: u8> OffsetU8<O, L, H> {
    pub const ZERO: u8 = O;
}

impl<const O: u8, const L: u8, const H: u8> From<u8> for OffsetU8<O, L, H> {
    fn from(value: u8) -> Self {
        if value >= Self::ZERO {
            Self((value - Self::ZERO) as i8)
        } else {
            Self(-1 * (Self::ZERO - value) as i8)
        }
    }
}

impl<const O: u8, const L: u8, const H: u8> Into<u8> for OffsetU8<O, L, H> {
    fn into(self) -> u8 {
        if self.0 >= 0 {
            self.0 as u8 + Self::ZERO
        } else {
            Self::ZERO - self.0.abs() as u8
        }
    }
}

impl<const O: u8, const L: u8, const H: u8> Default for OffsetU8<O, L, H> {
    fn default() -> Self {
        Self::from(Self::ZERO)
    }
}

impl<const O: u8, const L: u8, const H: u8> JsonSchema for OffsetU8<O, L, H> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        i8_schema(Self::from(L).0, Self::from(H).0)
    }
}

impl<const O: u8, const L: u8, const H: u8> Validate for OffsetU8<O, L, H> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let min = Self::from(L);
        let max = Self::from(H);
        if self.0 < min.0 || self.0 > max.0 {
            Err(out_of_range_err("0", &min.0, &max.0))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct OneIndexedU16(u16); // 0-299 (1-300)

impl OneIndexedU16 {
    const MAX: u16 = 300; // this is technically arbitrary and could be a generic parameter, but given this type is currently only used with a max of 300 it is hard coded for now
}

impl From<u16> for OneIndexedU16 {
    fn from(value: u16) -> Self {
        Self(value + 1)
    }
}

impl Into<u16> for OneIndexedU16 {
    fn into(self) -> u16 {
        self.0 - 1
    }
}

impl Default for OneIndexedU16 {
    fn default() -> Self {
        Self::from(0)
    }
}

impl JsonSchema for OneIndexedU16 {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        u16_schema(1, Self::MAX)
    }
}

impl Validate for OneIndexedU16 {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0 < 1 || self.0 > Self::MAX {
            Err(out_of_range_err("0", &1, &Self::MAX))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct OneIndexedU8<const MAX: u8>(u8); // 0- MAX-1 (1-MAX)

impl<const M: u8> OneIndexedU8<M> {
    const MAX: u8 = M;
}

impl<const M: u8> From<u8> for OneIndexedU8<M> {
    fn from(value: u8) -> Self {
        Self(value + 1)
    }
}

impl<const M: u8> Into<u8> for OneIndexedU8<M> {
    fn into(self) -> u8 {
        self.0 - 1
    }
}

impl<const M: u8> Default for OneIndexedU8<M> {
    fn default() -> Self {
        Self::from(0)
    }
}

impl<const M: u8> JsonSchema for OneIndexedU8<M> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        u8_schema(1, Self::MAX)
    }
}

impl<const M: u8> Validate for OneIndexedU8<M> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0 < 1 || self.0 > Self::MAX {
            Err(out_of_range_err("0", &1, &Self::MAX))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Offset1Dp<const OFFSET: u16, const MIN: u16, const MAX: u16>(f64); // MIN(0)-MAX(65536) ((MIN-OFFSET)/10 - (MAX-OFFSET)/10)

impl<const O: u16, const L: u16, const H: u16> Offset1Dp<O, L, H> {
    const ZERO: u16 = O;
}

impl<const O: u16, const L: u16, const H: u16> From<u16> for Offset1Dp<O, L, H> {
    fn from(value: u16) -> Self {
        Self((value as f64 - Self::ZERO as f64) / 10.0)
    }
}

impl<const O: u16, const L: u16, const H: u16> Into<u16> for Offset1Dp<O, L, H> {
    fn into(self) -> u16 {
        ((self.0 * 10.0) + Self::ZERO as f64) as u16
    }
}

impl<const O: u16, const L: u16, const H: u16> Default for Offset1Dp<O, L, H> {
    fn default() -> Self {
        Self::from(Self::ZERO)
    }
}

impl<const O: u16, const L: u16, const H: u16> JsonSchema for Offset1Dp<O, L, H> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        double_schema(Self::from(L).0, Self::from(H).0, 0.1)
    }
}

impl<const O: u16, const L: u16, const H: u16> Validate for Offset1Dp<O, L, H> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let min = Self::from(L);
        let max = Self::from(H);
        if self.0 < min.0 || self.0 > max.0 {
            Err(out_of_range_err("0", &min.0, &max.0))
        } else {
            Ok(())
        }
    }
}
