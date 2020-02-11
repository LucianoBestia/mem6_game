// statusjoinedmod.rs
//! code flow for this status

#![allow(clippy::panic)]

//region: use
use crate::*;

use mem6_common::*;

use unwrap::unwrap;

//endregion

/// group_id is the ws_uid of the first player
pub fn on_load_joined(rrc: &mut RootRenderingComponent) {
    rrc.game_data.game_status = GameStatus::StatusJoined;
    let _group_id = format!("{}", unwrap!(rrc.game_data.players.get(0)).ws_uid);
    logmod::debug_write(&format!(
        "StatusJoined send {}",
        rrc.game_data.players_ws_uid
    ));
    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgJoin {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
            my_nickname: rrc.game_data.my_nickname.clone(),
        },
    );
}

///msg joined
pub fn on_msg_joined(rrc: &mut RootRenderingComponent, his_ws_uid: usize, his_nickname: String) {
    //logmod::debug_write(&format!("on_msg_joined {}",his_ws_uid));
    if rrc.game_data.my_player_number == 1 {
        //push if not exists
        let mut ws_uid_exists = false;
        for x in &rrc.game_data.players {
            if x.ws_uid == his_ws_uid {
                ws_uid_exists = true;
                break;
            }
        }
        if !ws_uid_exists {
            rrc.game_data.players.push(Player {
                ws_uid: his_ws_uid,
                nickname: his_nickname,
                points: 0,
            });
            rrc.game_data.players_ws_uid =
                gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);
        }
    }
}
