use serde::Serialize;

/// <https://xiexe.github.io/XSOverlayDocumentation/#/NotificationsAPI?id=xsoverlay-message-object>
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// 1 = Notification Popup, 2 = MediaPlayer Information, will be extended later on.
    pub message_type: u32,
    /// Only used for Media Player, changes the icon on the wrist.
    pub index: u32,
    /// How long the notification will stay on screen for in seconds
    pub timeout: f32,
    /// Height notification will expand to if it has content other than a title. Default is 175
    pub height: f32,
    /// Opacity of the notification, to make it less intrusive. Setting to 0 will set to 1.
    pub opacity: f32,
    /// Notification sound volume.
    pub volume: f32,
    /// File path to .ogg audio file. Can be "default", "error", or "warning". Notification will be silent if left empty.
    pub audio_path: String,
    /// Notification title, supports Rich Text Formatting
    pub title: String,
    /// Notification content, supports Rich Text Formatting, if left empty, notification will be small.
    pub content: String,
    /// Set to true if using Base64 for the icon image
    pub use_base64_icon: bool,
    /// Base64 Encoded image, or file path to image. Can also be "default", "error", or "warning"
    pub icon: String,
    /// Somewhere to put your app name for debugging purposes
    pub source_app: String,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            message_type: 0,
            index: 0,
            timeout: 0.5,
            height: 175.0,
            opacity: 1.0,
            volume: 0.7,
            audio_path: String::new(),
            title: String::new(),
            content: String::new(),
            use_base64_icon: false,
            icon: String::new(),
            source_app: String::new(),
        }
    }
}
