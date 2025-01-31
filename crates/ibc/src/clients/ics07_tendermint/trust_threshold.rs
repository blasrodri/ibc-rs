//! IBC Domain type definition for [`TrustThreshold`]
//! represented as a fraction with valid values in the
//! range `[0, 1)`.

use core::convert::TryFrom;
use core::fmt::{Display, Error as FmtError, Formatter};

use ibc_proto::ibc::lightclients::tendermint::v1::Fraction;
use ibc_proto::Protobuf;
use tendermint::trust_threshold::TrustThresholdFraction;

use crate::core::ics02_client::error::ClientError;

/// [`TrustThreshold`] defines the level of trust that a client has
/// towards a set of validators of a chain.
///
/// A trust threshold is represented as a fraction, i.e., a numerator and
/// and a denominator.
/// A typical trust threshold is 1/3 in practice.
/// This type accepts even a value of 0, (numerator = 0, denominator = 0),
/// which is used in the client state of an upgrading client.
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustThreshold {
    numerator: u64,
    denominator: u64,
}

impl TrustThreshold {
    /// Constant for a trust threshold of 1/3.
    pub const ONE_THIRD: Self = Self {
        numerator: 1,
        denominator: 3,
    };

    /// Constant for a trust threshold of 2/3.
    pub const TWO_THIRDS: Self = Self {
        numerator: 2,
        denominator: 3,
    };

    /// Constant for a trust threshold of 0/0.
    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 0,
    };

    /// Instantiate a TrustThreshold with the given denominator and
    /// numerator.
    ///
    /// The constructor succeeds if long as the resulting fraction
    /// is in the range`[0, 1)`.
    pub fn new(numerator: u64, denominator: u64) -> Result<Self, ClientError> {
        // The two parameters cannot yield a fraction that is bigger or equal to 1
        if (numerator > denominator)
            || (denominator == 0 && numerator != 0)
            || (numerator == denominator && numerator != 0)
        {
            return Err(ClientError::InvalidTrustThreshold {
                numerator,
                denominator,
            });
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// The numerator of the fraction underlying this trust threshold.
    pub fn numerator(&self) -> u64 {
        self.numerator
    }

    /// The denominator of the fraction underlying this trust threshold.
    pub fn denominator(&self) -> u64 {
        self.denominator
    }
}

/// Conversion from Tendermint domain type into
/// IBC domain type.
impl From<TrustThresholdFraction> for TrustThreshold {
    fn from(t: TrustThresholdFraction) -> Self {
        Self {
            numerator: t.numerator(),
            denominator: t.denominator(),
        }
    }
}

/// Conversion from IBC domain type into
/// Tendermint domain type.
impl TryFrom<TrustThreshold> for TrustThresholdFraction {
    type Error = ClientError;

    fn try_from(t: TrustThreshold) -> Result<TrustThresholdFraction, Self::Error> {
        Self::new(t.numerator, t.denominator).map_err(|_| {
            ClientError::FailedTrustThresholdConversion {
                numerator: t.numerator,
                denominator: t.denominator,
            }
        })
    }
}

impl Protobuf<Fraction> for TrustThreshold {}

impl From<TrustThreshold> for Fraction {
    fn from(t: TrustThreshold) -> Self {
        Self {
            numerator: t.numerator,
            denominator: t.denominator,
        }
    }
}

impl TryFrom<Fraction> for TrustThreshold {
    type Error = ClientError;

    fn try_from(value: Fraction) -> Result<Self, Self::Error> {
        Self::new(value.numerator, value.denominator)
    }
}

impl Default for TrustThreshold {
    fn default() -> Self {
        Self::ONE_THIRD
    }
}

impl Display for TrustThreshold {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
