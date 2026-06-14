use iced::Task;
use crate::app::model::Model;
use crate::app::message::UiMessage;

mod auth;
mod chat;
mod network;
mod profile;
mod ui;

pub fn update(model: &mut Model, msg: UiMessage) -> Task<UiMessage> {
    match msg {
        // Сеть
        UiMessage::Tick => network::poll(model),
        UiMessage::Network(ev) => network::handle(model, ev),
        
        // Аутентификация
        UiMessage::AuthNicknameChanged(v) => auth::nickname_changed(model, v),
        UiMessage::AuthUsernameChanged(v) => auth::username_changed(model, v),
        UiMessage::AuthPasswordChanged(v) => auth::password_changed(model, v),
        UiMessage::AuthSubmit => auth::submit(model),
        UiMessage::AuthTogglePasswordVisibility => auth::toggle_password(model),
        UiMessage::AuthBack => auth::back(model),
        UiMessage::MainMenuSelect(idx) => auth::main_menu_select(model, idx),

        // Чаты
        UiMessage::ChatSelected(t) => chat::select(model, t),
        UiMessage::ChatViewInputChanged(v) => chat::input_changed(model, v),
        UiMessage::ChatViewSend => chat::send(model),
        UiMessage::OpenNewChatScreen => chat::open_new(model),
        UiMessage::NewChatInputChanged(v) => chat::new_input_changed(model, v),
        UiMessage::NewChatSubmit => chat::new_submit(model),
        UiMessage::NewChatCancel => chat::new_cancel(model),
        UiMessage::ReadyToSendImage(mime, data) => chat::send_image(model, mime, data),
        UiMessage::FetchKeysFor(t) => chat::fetch_keys(model, t),
        UiMessage::KeysReceived { target, ed_public, x25519_public } => chat::keys_received(model, target, ed_public, x25519_public),
        UiMessage::SearchInputChanged(v) => chat::search_input_changed(model, v),
        UiMessage::SearchSubmit => chat::search_submit(model),
        UiMessage::SearchResultsReceived(matches) => chat::search_results(model, matches),
        UiMessage::SearchResultSelected(name) => chat::search_result_selected(model, name),
        UiMessage::SearchResult { username, exists } => chat::search_result(model, username, exists),

        // Профиль
        UiMessage::ViewProfile(username) => profile::view(model, username),
        UiMessage::ProfileReceived(user) => profile::received(model, user),
        UiMessage::CloseProfile => profile::close(model),
        UiMessage::OpenEditProfile => profile::open_edit(model),
        UiMessage::EditProfileBioChanged(v) => profile::bio_changed(model, v),
        UiMessage::EditProfileAvatarPicked(path) => profile::avatar_picked(model, path),
        UiMessage::EditProfileAvatarReady(b64) => profile::avatar_ready(model, b64),
        UiMessage::EditProfileSave => profile::save(model),
        UiMessage::ProfileUpdated(user) => profile::updated(model, user),

        // UI и прочее
        UiMessage::ToggleTheme => ui::toggle_theme(model),
        UiMessage::ToggleHamburgerMenu => ui::toggle_hamburger(model),
        UiMessage::ToggleSettings => ui::toggle_settings(model),
        UiMessage::ServerAddressChanged(v) => ui::server_address_changed(model, v),
        UiMessage::SaveServerAddress => ui::save_server_address(model),
        UiMessage::Logout => ui::logout(model),
        UiMessage::PickImage => ui::pick_image(model),
        UiMessage::ImagePicked(path) => ui::image_picked(model, path),
        UiMessage::ExpandImage(id) => ui::expand_image(model, id),
        UiMessage::CloseExpandedImage => ui::close_expanded_image(model),
        UiMessage::StatusUpdate(status) => ui::status_update(model, status),
    }
}