pub use nightraven_macros::*;

/// Trait to be derived by `nightraven-derive` macro.
pub trait NightRaven {
    fn list_names(&self) -> &'static [&'static str];
    // fn concatenated_names(&self) -> Option<&'static str>;
}
