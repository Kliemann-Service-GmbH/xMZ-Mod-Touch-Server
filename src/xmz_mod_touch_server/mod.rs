/// Server teil der XMZModTouchServer Platform
///
/// Hier werden alle Komponenten des Servers verwaltet.
///
mod xmz_mod_touch_server;
pub mod configuration;
mod zone;

pub use self::xmz_mod_touch_server::XMZModTouchServer;
pub use self::zone::Zone;
