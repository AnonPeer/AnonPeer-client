use crate::network::ServerEvent;

#[derive(Debug, Clone)]
pub enum UiMessage {
    Network(ServerEvent),
    Tick,

    MainMenuSelect(usize),
    AuthUsernameChanged(String),
    AuthPasswordChanged(String),
    AuthSubmit,
    AuthTogglePasswordVisibility,
    AuthBack,

    ChatSelected(String),
    OpenNewChatScreen,
    NewChatInputChanged(String),
    NewChatSubmit,
    NewChatCancel,

    ChatViewInputChanged(String),
    ChatViewSend,
    Logout,

    PickImage,
    ImagePicked(String), 
    ReadyToSendImage(String, String), 
    StatusUpdate(String),

    SearchInputChanged(String),
    SearchSubmit,
    SearchResult { username: String, exists: bool },

    SearchResultSelected(String),
    SearchResultsReceived(Vec<String>),

    ExpandImage(uuid::Uuid),
    CloseExpandedImage,

    FetchKeysFor(String),
    KeysReceived { target: String, ed_public: Vec<u8>, x25519_public: Vec<u8> },
    ToggleTheme,
    ToggleHamburgerMenu,
    ToggleSettings,
    ServerAddressChanged(String),
    SaveServerAddress,
}