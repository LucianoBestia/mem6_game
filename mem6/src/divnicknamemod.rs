// divnicknamemod.rs
//! load and save nickname

#![allow(clippy::panic)]

//region: use
use crate::*;

//use unwrap::unwrap;
use unwrap::unwrap;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
                          //endregion

///save nickname from html input elements to local storage and rrc
pub fn save_nickname_to_localstorage(rrc: &mut RootRenderingComponent) {
    let document = unwrap!(windowmod::window().document(), "document");

    //logmod::debug_write("before get_element_by_id");
    let input_nickname = unwrap!(document.get_element_by_id("input_nickname"));
    //logmod::debug_write("before dyn_into");
    let input_html_element_nickname = unwrap!(
        input_nickname.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    //logmod::debug_write("before value()");
    let nickname_string = input_html_element_nickname.value();
    //logmod::debug_write("before as_str");
    let nickname = nickname_string.as_str();
    //logmod::debug_write(nickname);

    let ls = unwrap!(unwrap!(windowmod::window().local_storage()));
    let _x = ls.set_item("nickname", nickname);

    //To change the data in rrc I must use the future `vdom.with_component`
    //it will be executed at the next tick to avoid concurrent data races.
    rrc.game_data.my_nickname = nickname_string.clone();
    //change it also in players
    if rrc.game_data.my_player_number == 1 {
        unwrap!(rrc
            .game_data
            .players
            .get_mut(unwrap!(rrc.game_data.my_player_number.checked_sub(1))))
        .nickname = nickname_string;
    }
}

///save group_id from html input elements to local storage and rrc
pub fn save_group_id_to_localstorage(rrc: &mut RootRenderingComponent) {
    let document = unwrap!(windowmod::window().document(), "document");

    //logmod::debug_write("before get_element_by_id");
    let input_group_id = unwrap!(document.get_element_by_id("input_group_id"));
    //logmod::debug_write("before dyn_into");
    let input_html_element_group_id = unwrap!(
        input_group_id.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    //logmod::debug_write("before value()");
    let group_id_string = input_html_element_group_id.value();
    save_group_id_string_to_localstorage(rrc, group_id_string);
}

///save group_id from html input elements to local storage and rrc
pub fn save_group_id_string_to_localstorage(
    rrc: &mut RootRenderingComponent,
    group_id_string: String,
) {
    //logmod::debug_write("before as_str");
    let group_id = group_id_string.as_str();
    //logmod::debug_write(nickname);

    let ls = unwrap!(unwrap!(windowmod::window().local_storage()));
    let _x = ls.set_item("group_id", group_id);

    //change it also in players[0]
    unwrap!(rrc.game_data.players.get_mut(0)).ws_uid = unwrap!(group_id_string.parse::<usize>());
}

///load nickname from local storage
pub fn load_nickname() -> String {
    let ls = unwrap!(unwrap!(windowmod::window().local_storage()));
    let empty1 = "".to_string();
    //return nickname
    unwrap!(ls.get_item("nickname")).unwrap_or(empty1)
}

///load nickname from local storage
pub fn load_group_id() -> String {
    let ls = unwrap!(unwrap!(windowmod::window().local_storage()));
    let empty1 = "".to_string();
    //return nickname
    unwrap!(ls.get_item("group_id")).unwrap_or(empty1)
}

/// if there is already a nickname don't blink
pub fn blink_or_not_nickname(rrc: &RootRenderingComponent) -> String {
    if rrc.game_data.my_nickname.is_empty() {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}

/// save on every key stroke
pub fn nickname_onkeyup(rrc: &mut RootRenderingComponent, event: web_sys::Event) {
    //logmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<web_sys::KeyboardEvent>());
    //logmod::debug_write(&keyboard_event.key());
    if keyboard_event.key() == "Enter" {
        //open page start group
        htmltemplateimplmod::open_new_local_page("#p02");
    } else {
        save_nickname_to_localstorage(rrc);
    }
    //vdom.schedule_render();
}

/// group id key stroke
pub fn group_id_onkeyup(rrc: &mut RootRenderingComponent, event: web_sys::Event) {
    //logmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<web_sys::KeyboardEvent>());
    //logmod::debug_write(&keyboard_event.key());
    if keyboard_event.key() == "Enter" {
        //open page start group
        let group_id = htmltemplateimplmod::get_input_value("input_group_id");
        htmltemplateimplmod::open_new_local_page(&format!("#p04.{}", group_id));
    } else {
        save_group_id_to_localstorage(rrc);
    }
}
