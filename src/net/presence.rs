use std::time::Duration;

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);
pub const INACTIVE_TIMEOUT: Duration = Duration::from_secs(6);
pub const OFFLINE_TIMEOUT: Duration = Duration::from_secs(12);

pub enum UpdatePresenceKind {
    Boring,
    JoinOrReconnect,
    UsernameChange { previous_username: String },
}
