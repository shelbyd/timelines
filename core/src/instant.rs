/// A moment in the simulation.
///
/// Not necessarily related to the machine's time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    since_start: core::time::Duration,
}
