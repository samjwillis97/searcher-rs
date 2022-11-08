use strum_macros::{AsRefStr, Display};

#[derive(AsRefStr, Display)]
pub enum ClientEvent {
    ClearSearch,
    SetService,
    FocusSearch,
    Navigate,
    RefreshLensManager,
    RefreshPluginManager,
    RefreshSearchResults,
    StartupProgress,
    UpdateLensFinished,
}
