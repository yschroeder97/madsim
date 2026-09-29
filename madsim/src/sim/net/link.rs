//! Pluggable per-link packet models.

use rand::RngCore;
use std::time::Duration;

/// Decides the fate of every packet sent over one directed link.
///
/// `transit` is called once per send, in simulation-time order, with the
/// send time since the simulation started. It returns the one-way delay,
/// or `None` when the packet is lost. The model keeps whatever state it
/// needs between calls (queues, FIFO fronts, regime cursors).
pub trait LinkModel: Send + 'static {
    /// One-way delay of a packet sent at `now`, or `None` if it is lost.
    fn transit(&mut self, now: Duration, rng: &mut dyn RngCore) -> Option<Duration>;
}

impl<F> LinkModel for F
where
    F: FnMut(Duration, &mut dyn RngCore) -> Option<Duration> + Send + 'static,
{
    fn transit(&mut self, now: Duration, rng: &mut dyn RngCore) -> Option<Duration> {
        self(now, rng)
    }
}
