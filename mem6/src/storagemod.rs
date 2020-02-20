//! storagemod.rs
//! local_storage for nickname and group_id
//! session_storage for ws_uid and debug_text

use crate::*;
use wasm_bindgen::JsCast; // don't remove this. It is needed for dyn_into.
use unwrap::unwrap;

//region: debug_text
/// add to begin of debug text
pub fn add_to_begin_of_debug_text(text: &str) {
    let mut debug_text = format!("{}: {}\n{}", logmod::now_string(), text, get_debug_text());
    utf8_truncate(&mut debug_text, 800);
    windowmod::save_string_to_session_storage("debug_text", &debug_text);
}

/// utf8 truncate
fn utf8_truncate(input: &mut String, maxsize: usize) {
    let mut utf8_maxsize = input.len();
    if utf8_maxsize >= maxsize {
        {
            let mut char_iter = input.char_indices();
            while utf8_maxsize >= maxsize {
                utf8_maxsize = match char_iter.next_back() {
                    Some((index, _)) => index,
                    None => 0,
                };
            }
        } // Extra {} wrap to limit the immutable borrow of char_indices()
        input.truncate(utf8_maxsize);
    }
}

/// get debug text from session storage
pub fn get_debug_text() -> String {
    windowmod::load_string_from_session_storage("debug_text", "")
}
//endregion debug_text

//region: my_ws_uid
/// save my_ws_uid to session storage so we can restart the game and preserve the ws_uid
pub fn save_my_ws_uid(my_ws_uid: usize) {
    windowmod::save_string_to_session_storage("my_ws_uid", &format!("{}", my_ws_uid))
}

/// load my_ws_uid from session storage
pub fn load_my_ws_uid() -> usize {
    // session_storage saves only strings
    let str_uid = windowmod::load_string_from_session_storage("my_ws_uid", "0");
    let my_ws_uid = unwrap!(str_uid.parse::<usize>());
    // return my_ws_uid
    my_ws_uid
}
//endregion: my_ws_uid

//region: nickname
/// save on every key stroke
pub fn nickname_onkeyup(rrc: &mut RootRenderingComponent, event: web_sys::Event) {
    // logmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<web_sys::KeyboardEvent>());
    // logmod::debug_write(&keyboard_event.key());
    if keyboard_event.key() == "Enter" {
        // open page start group
        htmltemplateimplmod::open_new_local_page("#p02");
    } else {
        save_nickname_to_local_storage(rrc);
    }
    // vdom.schedule_render();
}

/// save nickname from html input elements to local storage and rrc
pub fn save_nickname_to_local_storage(rrc: &mut RootRenderingComponent) {
    let nickname = windowmod::get_input_element_value_string_by_id("input_nickname");
    windowmod::save_to_local_storage("nickname", &nickname);

    rrc.game_data.my_nickname = nickname.clone();
    // change it also in players, if the player exists
    if rrc.game_data.my_player_number < rrc.game_data.players.len() {
        unwrap!(rrc
            .game_data
            .players
            .get_mut(unwrap!(rrc.game_data.my_player_number.checked_sub(1))))
        .nickname = nickname;
    }
}

/// load nickname from local storage
pub fn load_nickname() -> String {
    // return nickname
    windowmod::load_string_from_local_storage("nickname", "")
}

/// if there is already a nickname don't blink
pub fn blink_or_not_nickname(rrc: &RootRenderingComponent) -> String {
    if rrc.game_data.my_nickname.is_empty() {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}

//endregion: nickname

//region: group_id
/// group id key stroke
pub fn group_id_onkeyup(rrc: &mut RootRenderingComponent, event: web_sys::Event) {
    // logmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<web_sys::KeyboardEvent>());
    // logmod::debug_write(&keyboard_event.key());
    if keyboard_event.key() == "Enter" {
        // open page start group
        htmltemplateimplmod::open_new_local_page("#p04");
    } else {
        save_group_id_to_local_storage(rrc);
    }
}

/// save group_id from html input elements to local storage and rrc
pub fn save_group_id_to_local_storage(rrc: &mut RootRenderingComponent) {
    let group_id_string = windowmod::get_input_element_value_string_by_id("input_group_id");
    save_group_id_string_to_local_storage(rrc, group_id_string);
}

/// save group_id from html input elements to local storage and rrc
pub fn save_group_id_string_to_local_storage(
    rrc: &mut RootRenderingComponent,
    group_id_string: String,
) {
    set_group_id(rrc, &group_id_string);
    windowmod::save_to_local_storage("group_id", &group_id_string);
}

/// load group_id from local storage
pub fn load_group_id_string(rrc: &mut RootRenderingComponent) -> String {
    let group_id_string = windowmod::load_string_from_local_storage("group_id", "");
    set_group_id(rrc, &group_id_string);
    // return
    group_id_string
}

/// there are 3 places that must be managed (plus the local_storage)
///
pub fn set_group_id(rrc: &mut RootRenderingComponent, group_id_string: &str) {
    rrc.game_data.group_id = unwrap!(group_id_string.parse::<usize>());
    // change it also in players[0]
    unwrap!(rrc.game_data.players.get_mut(0)).ws_uid = unwrap!(group_id_string.parse::<usize>());
    // on any change in players the players_ws_uid must be constructed
    rrc.game_data.players_ws_uid = gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);
}
//endregion: group_id
