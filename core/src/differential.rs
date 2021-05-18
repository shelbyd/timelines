use crate::instant::*;
use crate::liveness::*;

/// A timeline with a value across any range of time where it is alive.
pub trait Differential {
    /// The type of the value the differential takes on.
    type Value;

    /// The value within the [Instant] [range](core::ops::RangeBounds).
    fn within(&self, range: impl core::ops::RangeBounds<Instant>) -> Liveness<Self::Value>;
}
