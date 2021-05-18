/// Return value of timelines that includes if the timeline existed at the relevant simulation
/// [Instant](crate::instant::Instant).
pub enum Liveness<T> {
    /// The timeline has not begun.
    BeforeBegin,

    /// The timeline is alive, and includes the relevant value.
    Alive(T),

    /// The timeline has already finished.
    AfterFinish,
}
