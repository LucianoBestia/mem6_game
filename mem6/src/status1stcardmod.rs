// status1stcardmod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::*;

use mem6_common::*;

use unwrap::unwrap;
use dodrio::{
    builder::{ElementBuilder, text},
    bumpalo::{self, Bump},
    Node,
};
use wasm_bindgen::JsCast;
//endregion

/// on click
pub fn on_click_1st_card(
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
    this_click_card_index: usize,
) {
    // websysmod::debug_write("on_click_1st_card");
    flip_back(rrc);
    // change card status and game status
    rrc.game_data.card_index_of_first_click = this_click_card_index;

    let msg_id = ackmsgmod::prepare_for_ack_msg_waiting(rrc, vdom);
    let msg = WsMessage::MsgClick1stCard {
        my_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers: rrc.web_data.msg_receivers.to_string(),
        card_index_of_first_click: this_click_card_index,
        msg_id,
    };
    ackmsgmod::send_msg_and_write_in_queue(rrc, &msg, msg_id);
    // websysmod::debug_write(&format!("send_msg_and_write_in_queue: {}", msg_id));
    divgridcontainermod::play_sound(rrc, this_click_card_index);
    // after ack for this message call on_msg_click_1st_card(rrc, this_click_card_index);
}

/// flip back any not permanent card
pub fn flip_back(rrc: &mut RootRenderingComponent) {
    for x in &mut rrc.game_data.card_grid_data {
        if let CardStatusCardFace::UpTemporary = x.status {
            x.status = CardStatusCardFace::Down;
        }
    }
    rrc.game_data.card_index_of_first_click = 0;
    rrc.game_data.card_index_of_second_click = 0;
}

/// on msg
pub fn on_msg_click_1st_card(
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
    msg_sender_ws_uid: usize,
    card_index_of_first_click: usize,
    msg_id: usize,
) {
    flip_back(rrc);
    ackmsgmod::send_ack(rrc, msg_sender_ws_uid, msg_id, MsgAckKind::MsgClick1stCard);
    // it can happen that 2 smartphones send the msg click1st simultaneously.
    // This is a conflict.
    // Only one Player can be the judge and I chosen the Player 1 to resolve it.
    if rrc.game_data.my_player_number == 1 && GameStatus::Status1stCard != rrc.game_data.game_status
    {
        websysmod::debug_write("CONFLICT on_msg_click_1st_card");
        // do the whole click1st process
        on_click_1st_card(rrc, vdom, rrc.game_data.card_index_of_first_click);
        // do the whole click2nd process
        status2ndcardmod::on_click_2nd_card(rrc, vdom, card_index_of_first_click)
    } else {
        // websysmod::debug_write("on_msg_click_1st_card");
        rrc.game_data.card_index_of_first_click = card_index_of_first_click;
        update_on_1st_card(rrc);
    }
}

/// on msg ack
pub fn on_msg_ack_click_1st_card(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) {
    // websysmod::debug_write("on_msg_ack_click_1st_card");
    // websysmod::debug_write(&format!("remove_ack_msg_from_queue: {} {}",player_ws_uid, msg_id));
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        // websysmod::debug_write("update_on_1st_card (rrc)");
        update_on_1st_card(rrc);
    }
    // TODO: timer if after 3 seconds the ack is not received resend the msg
    // do this 3 times and then hard error
}

/// update game data
pub fn update_on_1st_card(rrc: &mut RootRenderingComponent) {
    websysmod::debug_write("update_on_1st_card");
    // flip the card up
    unwrap!(rrc
        .game_data
        .card_grid_data
        .get_mut(rrc.game_data.card_index_of_first_click))
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::Status2ndCard;
}

/// render div
#[allow(clippy::integer_arithmetic)]
pub fn div_on_1st_card<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    if rrc.game_data.is_my_turn() {
        ElementBuilder::new(bump, "div")
            .children([ElementBuilder::new(bump, "h2")
                .attr("class", "h2_must_do_something")
                .children([text(
                    bumpalo::format!(in bump,
                        "Play {}",
                        rrc.game_data.player_turn_now().nickname
                    )
                    .into_bump_str(),
                )])
                .finish()])
            .finish()
    } else {
        // return wait for the other player
        ElementBuilder::new(bump, "div")
            .children([ElementBuilder::new(bump, "h2")
                .attr("class", "h2_user_must_wait")
                .children([text(
                    bumpalo::format!(in bump,
                        "Wait for {}",
                        rrc.game_data.player_turn_now().nickname
                    )
                    .into_bump_str(),
                )])
                .finish()])
            .finish()
    }
}

/// on click for image in status 1s
pub fn on_click_img_status1st(
    root: &mut dyn dodrio::RootRender,
    vdom: &dodrio::VdomWeak,
    event: &web_sys::Event,
) {
    // websysmod::debug_write("img click");
    let rrc = root.unwrap_mut::<RootRenderingComponent>();
    // If the event's target is our image...
    let img = match event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
    {
        None => return,
        // ?? Don't understand what this does. The original was written for Input element.
        Some(input) => input,
    };
    // id attribute of image html element is prefixed with img ex. "img12"
    let this_click_card_index = unwrap!(
        (unwrap!(img.id().get(3..), "error slicing")).parse::<usize>(),
        "error parse img id to usize"
    );
    // click is useful only on facedown cards
    if unwrap!(
        rrc.game_data.card_grid_data.get(this_click_card_index),
        "error this_click_card_index"
    )
    .status
    .as_ref()
        == CardStatusCardFace::Down.as_ref()
    {
        status1stcardmod::on_click_1st_card(rrc, vdom, this_click_card_index);
        // Finally, re-render the component on the next animation frame.
        vdom.schedule_render();
    }
}
//div_grid_container() is in divgridcontainermod.rs
