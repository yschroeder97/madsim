//! Pluggable per-link packet models.

use rand::RngCore;
use std::time::Duration;

/// Decides the fate of every packet sent over one directed link.
///
/// `transit` is called once per send, in simulation-time order, with the
/// send time since the simulation started and the size in bytes of the IP
/// packet: payload plus transport and IP headers (UDP/TCP, IPv4/IPv6), for
/// the data of `Endpoint::send_to` and of TCP writes; a connection setup is
/// a header-only packet. 0 for opaque payloads (raw messages, RPCs), whose
/// size is unknown. Link-layer framing is the model's to add.
/// It returns the one-way delay, or `None` when the packet is lost. The model keeps whatever state it
/// needs between calls (queues, FIFO fronts, regime cursors).
pub trait LinkModel: Send + 'static {
    /// One-way delay of a `len`-byte IP packet sent at `now`, or `None` if it is lost.
    fn transit(&mut self, now: Duration, len: usize, rng: &mut dyn RngCore) -> Option<Duration>;
}

impl<F> LinkModel for F
where
    F: FnMut(Duration, usize, &mut dyn RngCore) -> Option<Duration> + Send + 'static,
{
    fn transit(&mut self, now: Duration, len: usize, rng: &mut dyn RngCore) -> Option<Duration> {
        self(now, len, rng)
    }
}
