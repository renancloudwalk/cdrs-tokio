use derive_more::Constructor;
use rand::{thread_rng, Rng};
use std::time::Duration;

const DEFAULT_BASE_DELAY: Duration = Duration::from_secs(1);
const DEFAULT_MAX_DELAY: Duration = Duration::from_secs(60);

/// Determines the time for the next reconnection attempt when trying to reconnect to a node.
pub trait ReconnectionSchedule {
    /// Returns next reconnect delay or `None` if not attempt should be made.
    fn next_delay(&mut self) -> Option<Duration>;
}

/// Creates reconnection schedules when trying to re-establish connections.
pub trait ReconnectionPolicy {
    /// Creates new schedule when a connection needs to be re-established.
    fn new_node_schedule(&self) -> Box<dyn ReconnectionSchedule + Send + Sync>;
}

/// Schedules reconnection at constant interval.
#[derive(Copy, Clone, Constructor)]
pub struct ConstantReconnectionPolicy {
    base_delay: Duration,
}

impl Default for ConstantReconnectionPolicy {
    fn default() -> Self {
        ConstantReconnectionPolicy::new(DEFAULT_BASE_DELAY)
    }
}

impl ReconnectionPolicy for ConstantReconnectionPolicy {
    fn new_node_schedule(&self) -> Box<dyn ReconnectionSchedule + Send + Sync> {
        Box::new(ConstantReconnectionSchedule::new(self.base_delay))
    }
}

#[derive(Constructor)]
struct ConstantReconnectionSchedule {
    base_delay: Duration,
}

impl ReconnectionSchedule for ConstantReconnectionSchedule {
    fn next_delay(&mut self) -> Option<Duration> {
        Some(self.base_delay)
    }
}

/// Never schedules reconnections.
#[derive(Default, Copy, Clone)]
pub struct NeverReconnectionPolicy;

impl ReconnectionPolicy for NeverReconnectionPolicy {
    fn new_node_schedule(&self) -> Box<dyn ReconnectionSchedule + Send + Sync> {
        Box::new(NeverReconnectionSchedule)
    }
}

struct NeverReconnectionSchedule;

impl ReconnectionSchedule for NeverReconnectionSchedule {
    fn next_delay(&mut self) -> Option<Duration> {
        None
    }
}

/// A reconnection policy that waits exponentially longer between each reconnection attempt (but
/// keeps a constant delay once a maximum delay is reached). The delay will increase exponentially,
/// with an added jitter.
#[derive(Copy, Clone, Constructor)]
pub struct ExponentialReconnectionPolicy {
    base_delay: Duration,
    max_delay: Duration,
    max_attempts: usize,
}

impl ReconnectionPolicy for ExponentialReconnectionPolicy {
    fn new_node_schedule(&self) -> Box<dyn ReconnectionSchedule + Send + Sync> {
        Box::new(ExponentialReconnectionSchedule::new(
            self.base_delay,
            self.max_delay,
            self.max_attempts,
        ))
    }
}

impl Default for ExponentialReconnectionPolicy {
    fn default() -> Self {
        let base_delay = DEFAULT_BASE_DELAY.as_millis() as i64;
        let ceil = if (base_delay & (base_delay - 1)) == 0 {
            0
        } else {
            1
        };

        ExponentialReconnectionPolicy::new(
            DEFAULT_BASE_DELAY,
            DEFAULT_MAX_DELAY,
            (64 - (i64::MAX / base_delay).leading_zeros() - ceil) as usize,
        )
    }
}

struct ExponentialReconnectionSchedule {
    base_delay: Duration,
    max_delay: Duration,
    max_attempts: usize,
    attempt: usize,
}

impl ReconnectionSchedule for ExponentialReconnectionSchedule {
    fn next_delay(&mut self) -> Option<Duration> {
        if self.attempt == self.max_attempts {
            return None;
        }

        self.attempt += 1;

        let delay = ((1 << self.attempt) * self.base_delay).min(self.max_delay);
        let jitter = thread_rng().gen_range(85..116);

        Some((jitter * delay / 100).clamp(self.base_delay, self.max_delay))
    }
}

impl ExponentialReconnectionSchedule {
    pub fn new(base_delay: Duration, max_delay: Duration, max_attempts: usize) -> Self {
        ExponentialReconnectionSchedule {
            base_delay,
            max_delay,
            max_attempts,
            attempt: 0,
        }
    }
}
