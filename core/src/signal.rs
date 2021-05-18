use crate::instant::*;
use crate::liveness::*;

/// A timeline with a value at every point when the timeline is alive.
pub trait Signal {
    /// The type of the value the signal takes on.
    type Value;

    /// The value at a specific simulation [Instant].
    /// Wrapped in [Liveness] to communicate that the signal may not be alive at the requested
    /// instant.
    fn at(&self, instant: Instant) -> Liveness<Self::Value>;
}
