//! fncallermod  

use crate::*;
use mem6_common::*;
//use qrcode53bytes::*;

use unwrap::unwrap;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use dodrio::{
    RenderContext,
    bumpalo::{self},
    Node,
    builder::*,
};
use typed_html::dodrio;

impl htmltemplatemod::HtmlTemplating for RootRenderingComponent {
    fn get_local_route(&self) -> String {
        self.local_route.to_string()
    }
    fn set_local_route(&mut self, local_route: &str) {
        self.local_route = local_route.to_string();
    }
    fn get_html_template(&self) -> String {
        self.html_template.to_string()
    }
    fn set_html_template(&mut self, html_template: &str) {
        self.html_template = html_template.to_string();
    }
/// html_templating functions that return a String
#[allow(clippy::needless_return, clippy::integer_arithmetic)]
fn call_function_string(&self, sx: &str) -> String {
    //logmod::debug_write(&format!("call_function_string: {}", &sx));
    
    match sx {
        "my_nickname" => self.game_data.my_nickname.to_owned(),
        "blink_or_not_nickname" => divnicknamemod::blink_or_not_nickname(self),
        "blink_or_not_group_id" => blink_or_not_group_id(self),
        "my_ws_uid" => format!("{}", self.game_data.my_ws_uid),
        "players_count" => format!("{} ", self.game_data.players.len() - 1),
        "game_name" => self.game_data.game_name.to_string(),
        "group_id_joined" => group_id_joined(self),
        "url_to_join" => format!("bestia.dev/mem6/#p03.{}", self.game_data.my_ws_uid),
        "cargo_pkg_version" => env!("CARGO_PKG_VERSION").to_string(),
        "debug_text" => sessionstoragemod::get_debug_text(),
        "gameboard_btn" => {
            //different class depend on status
            "btn".to_owned()
        }
        "card_moniker_first" => {
            return unwrap!(unwrap!(self.game_data.game_config.as_ref())
                .card_moniker
                .get(
                    unwrap!(self
                        .game_data
                        .card_grid_data
                        .get(self.game_data.card_index_of_first_click))
                    .card_number_and_img_src
                ))
            .to_string();
        }
        "card_moniker_second" => {
            return unwrap!(unwrap!(self.game_data.game_config.as_ref())
                .card_moniker
                .get(
                    unwrap!(self
                        .game_data
                        .card_grid_data
                        .get(self.game_data.card_index_of_second_click))
                    .card_number_and_img_src
                ))
            .to_string();
        }
        "my_points" => {
            return format!(
                "{} ",
                unwrap!(self
                    .game_data
                    .players
                    .get(self.game_data.my_player_number - 1))
                .points,
            );
        }
        "player_turn" => {
            return unwrap!(self.game_data.players.get(self.game_data.player_turn - 1))
                .nickname
                .to_string();
        }
        _ => {
            let x = format!("Error: Unrecognized call_function_string: {}", sx);
            logmod::debug_write(&x);
            x
        }
    }
}
}

/// html_templating functions for listeners
/// get a clone of the VdomWeak
pub fn call_listener(
    vdom: &dodrio::VdomWeak,
    rrc: &mut RootRenderingComponent,
    sx: &str,
    event: web_sys::Event,
) {
    //logmod::debug_write(&format!("call_listener: {}", &sx));
    match sx {
        "nickname_onkeyup" => {
            divnicknamemod::nickname_onkeyup(rrc, event);
        }
        "group_id_onkeyup" => {
            divnicknamemod::group_id_onkeyup(event);
        }
        "open_youtube1" => {
            open_new_tab("https://www.youtube.com/watch?v=VQdhDw-hE8s");
        }
        "open_youtube2" => {
            open_new_tab("https://www.youtube.com/watch?v=2RT9AzqEfLo");
        }
        "open_menu" => {
            open_new_local_page("#p21");
        }
        "rejoin_resync" => {
            websocketreconnectmod::send_msg_for_resync(rrc);
        }
        "back_to_game" => {
            open_new_local_page("#p11");
        }
        "open_instructions" => {
            open_new_tab("#p08");
        }
        "debug_log" => {
            open_new_tab("#p31");
        }
        "start_a_group_onclick" | "restart_game" => {
            open_new_local_page("#p02");
        }
        "join_a_group_onclick" => {
            open_new_local_page("#p03");
        }
        "choose_a_game_onclick" => {
            open_new_local_page("#p05");
        }
        "start_game_onclick" => {
            statusgamedatainitmod::on_click_start_game(rrc);
            //async fetch all imgs and put them in service worker cache
            fetchallimgsforcachemod::fetch_all_img_for_cache_request(rrc);
            //endregion
            vdom.schedule_render();
            //logmod::debug_write(&format!("start_game_onclick players: {:?}",rrc.game_data.players));
            open_new_local_page("#p11");
        }
        "game_type_right_onclick" => {
            game_type_right_onclick(rrc, vdom);
        }
        "game_type_left_onclick" => {
            game_type_left_onclick(rrc, vdom);
        }
        "join_group_on_click" => {
            //find the group_id input element
            let group_id = get_input_value("input_group_id");
            open_new_local_page(&format!("#p04.{}", group_id));
        }
        "drink_end" => {
            //send a msg to end drinking to all players
            logmod::debug_write(&format!("MsgDrinkEnd send{}", ""));
            websocketcommunicationmod::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::MsgDrinkEnd {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                },
            );
            //if all the cards are permanently up, this is the end of the game
            //logmod::debug_write("if is_all_permanently(rrc)");
            if status2ndcardmod::is_all_permanently(rrc) {
                logmod::debug_write("yes");
                statusgameovermod::on_msg_game_over(rrc);
                //send message
                websocketcommunicationmod::ws_send_msg(
                    &rrc.game_data.ws,
                    &WsMessage::MsgGameOver {
                        my_ws_uid: rrc.game_data.my_ws_uid,
                        players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                    },
                );
            } else {
                statustaketurnmod::on_click_take_turn(rrc, vdom);
            }
            //end the drink dialog
            open_new_local_page("#p11");
        }
        _ => {
            let x = format!("Error: Unrecognized call_listener: {}", sx);
            logmod::debug_write(&x);
        }
    }
}

/// html_templating functions that return a Node
#[allow(clippy::needless_return)]
pub fn call_function_node<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
    sx: &str,
) -> Node<'a> {
    let bump = cx.bump;
    //logmod::debug_write(&format!("call_function_node: {}", &sx));
    match sx {
        "div_grid_container" => {
            //what is the game_status now?
            //logmod::debug_write(&format!("game status: {}", rrc.game_data.game_status));
            let max_grid_size = divgridcontainermod::max_grid_size(rrc);
            return divgridcontainermod::div_grid_container(rrc, bump, &max_grid_size);
        }
        "div_player_action" => {
            let node = divplayeractionsmod::div_player_actions_from_game_status(rrc, bump);
            return node;
        }
        "svg_qrcode" => {
            return svg_qrcode_to_node(rrc, cx);
        }
        _ => {
            let node = dodrio!(bump,
            <h2  >
                {vec![text(bumpalo::format!(in bump, "Error: Unrecognized call_function_node: {}", sx).into_bump_str())]}
            </h2>
            );
            return node;
        }
    }
}

/// qrcode svg
pub fn svg_qrcode_to_node<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    let link = format!("https://bestia.dev/mem6/#p03.{}", rrc.game_data.my_ws_uid);
    let qr = unwrap!(qrcode53bytes::Qr::new(&link));
    let svg_template = qrcode53bytes::SvgDodrioRenderer::new(222, 222).render(&qr);

    unwrap!(htmltemplatemod::get_root_element(
        rrc,
        cx,
        &svg_template,
        htmltemplatemod::HtmlOrSvg::Svg,
        &fncallermod::call_function_node,
        &fncallermod::call_listener
    ))
}

/// the arrow to the right
pub fn game_type_right_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
    let mut last_name = unwrap!(gmd.last()).name.to_string();
    for x in gmd {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
    fetchgameconfigmod::async_fetch_game_config_request(rrc, vdom);
}
/// left arrow button
pub fn game_type_left_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
    let mut last_name = unwrap!(gmd.first()).name.to_string();
    for x in gmd.iter().rev() {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
    fetchgameconfigmod::async_fetch_game_config_request(rrc, vdom);
}

/// get value form input html element by id
pub fn get_input_value(id: &str) -> String {
    let document = unwrap!(utilsmod::window().document(), "document");
    //logmod::debug_write(&format!("before get_element_by_id: {}", id));
    let input_el = unwrap!(document.get_element_by_id(id));
    //logmod::debug_write("before dyn_into");
    let input_html_element = unwrap!(input_el.dyn_into::<web_sys::HtmlInputElement>(), "dyn_into");
    //return
    input_html_element.value()
}

/// fn open new local page with # window.location.set_hash
pub fn open_new_local_page(hash: &str) {
    let (_location_href, href_hash) = get_url_and_hash();
    if href_hash.is_empty() || href_hash.starts_with("#p03.") {
        //put the first url in the history
        //the first player first url is without hash
        //the joined players first url is #p03.xxxx
        let _x = utilsmod::window().location().assign(hash);
    } else {
        //don't put other url in history
        let _x = utilsmod::window().location().replace(hash);
    }
}

/// fn open new tab
pub fn open_new_tab(url: &str) {
    let _w = utilsmod::window().open_with_url_and_target(url, "_blank");
}

/// return the text for html template replace
/// empty if there is no group_id
pub fn group_id_joined(rrc: &RootRenderingComponent) -> String {
    //if the first players is not yet pushed
    let group_id = match rrc.game_data.players.get(0) {
        Some(ggg) => ggg.ws_uid,
        None => 0,
    };
    //the  first player cannot join himself
    if group_id == 0 || group_id == rrc.game_data.my_ws_uid {
        String::from("")
    } else {
        return format!("{}", group_id);
    }
}

/// if there is already a group_id don't blink
pub fn blink_or_not_group_id(rrc: &RootRenderingComponent) -> String {
    if group_id_joined(rrc) == "" {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}
