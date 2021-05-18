use crate::instant::*;

/// A timeline with values only at specific [Instant]s.
///
/// Streams do not begin or end, they simply have no events outside their effective liveness.
pub trait Stream {
    /// The type of the event.
    type Event;

    /// Core method to return the events that occurred in the range.
    fn in_range(&self, range: impl core::ops::RangeBounds<Instant>)
        -> Vec<(Instant, &Self::Event)>;
}
