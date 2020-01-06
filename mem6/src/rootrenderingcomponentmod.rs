// rootrenderingcomponentmod.rs
//! renders the web page

//region: use, const

use crate::divgridcontainermod;

use crate::divplayersandscoresmod;
use crate::divrulesanddescriptionmod;
use crate::gamedatamod;
use crate::page01nicknamemod;
//use crate::page02groupmod;
use crate::page03gamemod;
//use crate::page04instructionsmod;
use crate::page05errormod;

use mem6_common::GameStatus;

//use unwrap::unwrap;
//use dodrio::builder::text;
use dodrio::{Cached, Node, Render, RenderContext};
use web_sys::WebSocket;
//endregion

/// Root Rendering Component has all
/// the data needed for play logic and rendering
pub struct RootRenderingComponent {
    ///game data will be inside of Root
    pub game_data: gamedatamod::GameData,
    ///subComponent 1: players and scores. The data is a cached copy of GameData.
    pub cached_players_and_scores: Cached<divplayersandscoresmod::PlayersAndScores>,
    ///subComponent 2: the static parts can be cached.
    pub cached_rules_and_description: Cached<divrulesanddescriptionmod::RulesAndDescription>,
}

///methods
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` at the beginning only once.
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let game_data = gamedatamod::GameData::new(ws, my_ws_uid);

        let cached_rules_and_description =
            Cached::new(divrulesanddescriptionmod::RulesAndDescription::new());
        let cached_players_and_scores =
            Cached::new(divplayersandscoresmod::PlayersAndScores::new(my_ws_uid));

        RootRenderingComponent {
            game_data,
            cached_players_and_scores,
            cached_rules_and_description,
        }
    }
    ///check invalidate render cache for all sub components
    pub fn check_invalidate_for_all_components(&mut self) {
        if self
            .cached_players_and_scores
            .update_intern_cache(&self.game_data)
        {
            Cached::invalidate(&self.cached_players_and_scores);
        }
        if self
            .cached_rules_and_description
            .update_intern_cache(&self.game_data)
        {
            Cached::invalidate(&self.cached_rules_and_description);
        }
    }
    ///reset the data to replay the game
    pub fn reset(&mut self) {
        self.game_data.card_grid_data = gamedatamod::GameData::prepare_for_empty();
        self.game_data.card_index_of_first_click = 0;
        self.game_data.card_index_of_second_click = 0;
        self.game_data.players.clear();
        self.game_data.game_status = GameStatus::StatusStartPage;
        self.game_data.game_name = "alphabet".to_string();
        self.game_data.asked_game_name = "".to_string();
        self.game_data.my_player_number = 1;
        self.game_data.player_turn = 0;
        self.game_data.game_config = None;

        self.check_invalidate_for_all_components();
    }
}
//endregion

//region: `Render` trait implementation on RootRenderingComponent struct
///It is called for every Dodrio animation frame to render the vdom.
///Only when render is scheduled after aomw change id the game data.
impl Render for RootRenderingComponent {
    #[inline]
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        //the card grid is a html css grid object (like a table) with <img> inside
        //other html elements are pretty simple.

        //region: create the whole virtual dom. The verbose stuff is in private functions
        //the UI has different 'pages' for playing or errors
        if self.game_data.error_text == "" {
            let xmax_grid_size = divgridcontainermod::max_grid_size(self);
            //the UI has 2 different 'pages', depends on the status
            if self.game_data.is_status_for_grid_container() {
                page03gamemod::page_render(self, cx, &xmax_grid_size)
            } else {
                page01nicknamemod::page_render(self, cx)
            }
        } else {
            page05errormod::page_render(self, cx)
        }
        //endregion
    }
}
//endregion
