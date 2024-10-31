//! Define the formats of point arrangements.

use crate::{
    packet::{ProductID, ReturnMode},
    traits::{AzimuthRange, PointField},
};
use eyre::Result;
use measurements::Angle;
use std::ops::Range;

/// An enumeration of point arrangement formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Single16,
    Single32,
    Dual16,
    Dual32,
}

impl Format {
    pub fn try_new(num_lines: usize, return_mode: ReturnMode) -> Option<Format> {
        use Format::*;
        use ReturnMode::*;

        Some(match (num_lines, return_mode) {
            (16, Strongest | Last) => Single16,
            (16, Dual) => Dual16,
            (32, Strongest | Last) => Single32,
            (32, Dual) => Dual32,
            _ => return None,
        })
    }

    pub fn new(num_lines: usize, return_mode: ReturnMode) -> Format {
        Self::try_new(num_lines, return_mode).unwrap()
    }

    pub fn try_from_model(product_id: ProductID, return_mode: ReturnMode) -> Option<Format> {
        Self::try_new(product_id.num_lines(), return_mode)
    }

    pub fn from_model(product_id: ProductID, return_mode: ReturnMode) -> Format {
        Self::try_from_model(product_id, return_mode).unwrap()
    }
}

/// An enumeration of point arrangement formats with additional generics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatKind<S16, S32, D16, D32> {
    Single16(S16),
    Single32(S32),
    Dual16(D16),
    Dual32(D32),
}

impl<S16, S32, D16, D32> FormatKind<S16, S32, D16, D32> {
    pub fn from_format_default(format: Format) -> Self
    where
        S16: Default,
        S32: Default,
        D16: Default,
        D32: Default,
    {
        match format {
            Format::Single16 => Self::Single16(S16::default()),
            Format::Single32 => Self::Single32(S32::default()),
            Format::Dual16 => Self::Dual16(D16::default()),
            Format::Dual32 => Self::Dual32(D32::default()),
        }
    }

    pub fn format(&self) -> Format {
        match self {
            FormatKind::Single16(_) => Format::Single16,
            FormatKind::Single32(_) => Format::Single32,
            FormatKind::Dual16(_) => Format::Dual16,
            FormatKind::Dual32(_) => Format::Dual32,
        }
    }

    pub fn from_s16(from: S16) -> Self {
        Self::Single16(from)
    }

    pub fn from_s32(from: S32) -> Self {
        Self::Single32(from)
    }

    pub fn from_d16(from: D16) -> Self {
        Self::Dual16(from)
    }

    pub fn from_d32(from: D32) -> Self {
        Self::Dual32(from)
    }

    pub fn try_into_s16(self) -> Result<S16, Self> {
        match self {
            Self::Single16(s16) => Ok(s16),
            _ => Err(self),
        }
    }

    pub fn try_into_s32(self) -> Result<S32, Self> {
        match self {
            Self::Single32(s16) => Ok(s16),
            _ => Err(self),
        }
    }

    pub fn try_into_d16(self) -> Result<D16, Self> {
        match self {
            Self::Dual16(s16) => Ok(s16),
            _ => Err(self),
        }
    }

    pub fn try_into_d32(self) -> Result<D32, Self> {
        match self {
            Self::Dual32(s16) => Ok(s16),
            _ => Err(self),
        }
    }

    pub fn as_s16(&self) -> Option<&S16> {
        match self {
            Self::Single16(s16) => Some(s16),
            _ => None,
        }
    }

    pub fn as_s32(&self) -> Option<&S32> {
        match self {
            Self::Single32(s32) => Some(s32),
            _ => None,
        }
    }

    pub fn as_d16(&self) -> Option<&D16> {
        match self {
            Self::Dual16(d16) => Some(d16),
            _ => None,
        }
    }

    pub fn as_d32(&self) -> Option<&D32> {
        match self {
            Self::Dual32(d32) => Some(d32),
            _ => None,
        }
    }

    pub fn as_s16_mut(&mut self) -> Option<&mut S16> {
        match self {
            Self::Single16(s16) => Some(s16),
            _ => None,
        }
    }

    pub fn as_s32_mut(&mut self) -> Option<&mut S32> {
        match self {
            Self::Single32(s32) => Some(s32),
            _ => None,
        }
    }

    pub fn as_d16_mut(&mut self) -> Option<&mut D16> {
        match self {
            Self::Dual16(d16) => Some(d16),
            _ => None,
        }
    }

    pub fn as_d32_mut(&mut self) -> Option<&mut D32> {
        match self {
            Self::Dual32(d32) => Some(d32),
            _ => None,
        }
    }
}

impl<S16, S32, D16, D32> AzimuthRange for FormatKind<S16, S32, D16, D32>
where
    S16: AzimuthRange,
    S32: AzimuthRange,
    D16: AzimuthRange,
    D32: AzimuthRange,
{
    fn azimuth_range(&self) -> Range<Angle> {
        match self {
            FormatKind::Single16(inner) => inner.azimuth_range(),
            FormatKind::Single32(inner) => inner.azimuth_range(),
            FormatKind::Dual16(inner) => inner.azimuth_range(),
            FormatKind::Dual32(inner) => inner.azimuth_range(),
        }
    }
}

impl<S16, S32, D16, D32> Iterator for FormatKind<S16, S32, D16, D32>
where
    S16: Iterator,
    S32: Iterator,
    D16: Iterator,
    D32: Iterator,
{
    type Item = FormatKind<S16::Item, S32::Item, D16::Item, D32::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self {
            FormatKind::Single16(iter) => FormatKind::from_s16(iter.next()?),
            FormatKind::Single32(iter) => FormatKind::from_s32(iter.next()?),
            FormatKind::Dual16(iter) => FormatKind::from_d16(iter.next()?),
            FormatKind::Dual32(iter) => FormatKind::from_d32(iter.next()?),
        };
        Some(item)
    }
}

impl<S16, S32, D16, D32> PointField for FormatKind<S16, S32, D16, D32>
where
    S16: PointField,
    S32: PointField,
    D16: PointField,
    D32: PointField,
{
    type Point<'a>
        = FormatKind<S16::Point<'a>, S32::Point<'a>, D16::Point<'a>, D32::Point<'a>>
    where
        S16: 'a,
        S32: 'a,
        D16: 'a,
        D32: 'a;

    fn nrows(&self) -> usize {
        match self {
            FormatKind::Single16(inner) => inner.nrows(),
            FormatKind::Single32(inner) => inner.nrows(),
            FormatKind::Dual16(inner) => inner.nrows(),
            FormatKind::Dual32(inner) => inner.nrows(),
        }
    }

    fn ncols(&self) -> usize {
        match self {
            FormatKind::Single16(inner) => inner.ncols(),
            FormatKind::Single32(inner) => inner.ncols(),
            FormatKind::Dual16(inner) => inner.ncols(),
            FormatKind::Dual32(inner) => inner.ncols(),
        }
    }

    fn point_at(&self, row: usize, col: usize) -> Option<Self::Point<'_>> {
        let point = match self {
            FormatKind::Single16(inner) => FormatKind::from_s16(inner.point_at(row, col)?),
            FormatKind::Single32(inner) => FormatKind::from_s32(inner.point_at(row, col)?),
            FormatKind::Dual16(inner) => FormatKind::from_d16(inner.point_at(row, col)?),
            FormatKind::Dual32(inner) => FormatKind::from_d32(inner.point_at(row, col)?),
        };
        Some(point)
    }
}
