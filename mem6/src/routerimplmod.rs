//! routerimplmod
//! Implementation of Router for this special use case with RootRenderingComponent type
//! It routes from short_url (the url hash part) to a
//! html_template file to fetch. The file name is written to rrc.local_route.  
//! Then fetches the file and stores it in rrc.html_template
//! 2020-02-24 I tried to put away as much boilerplate as I could into the library
//! dodrio_templating::routermod, but some code is very difficult to split in Rust.
//! The main problem are Closures that use RootRenderinComponent type. The library does
//! not know that type.

use crate::*;
use unwrap::unwrap;
use dodrio::VdomWeak;
//use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

pub struct Router {}

impl routermod::Routing for Router {
    //I tried to put vdom as a field in Router. But after closures, the vdom
    //is not anymore the same and I cannot use the one in Router.
    // I must pass vdom as parameter, because it comes from Closures
    // and is not anymore the same as self.vdom.

    /// specific code
    fn closure_specific_on_hash_change(
        vdom: VdomWeak,
        short_local_route: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
        Box::new(move |root| {
            let short_local_route = short_local_route.clone();
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            // If the rrc local_route already matches the event's
            // short_local_route, then there is nothing to do (ha). If they
            // don't match, then we need to update the rrc' local_route
            // and re-render.
            if rrc.web_communication.local_route != short_local_route {
                let v2 = vdom.clone();
                //the function that recognizes routes and urls
                fill_rrc_local_route(short_local_route, rrc, v2);
                let url = rrc.web_communication.local_route.to_string();
                // I cannot simply await here because this closure is not async
                let v3 = vdom.clone();
                Router::spawn_local_async_fetch_and_write_to_rrc_html_template(
                    url,
                    v3,
                    Box::new(&Self::closure_fill_html_template),
                );
            }
        })
    }
    fn closure_fill_html_template(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
        Box::new(move |root| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            // only the html inside the <body> </body>
            rrc.web_communication.html_template = routermod::between_body_tag(&resp_body_text);
        })
    }
}

/// the specific code to route short_local_route to actual filenames to download
/// and later dodrio_templating replace.
/// It cannot be a method because it has a parameter RootRenderingComponent, that is unknown in
/// the trait definition (in the library).
pub fn fill_rrc_local_route(local_route: String, rrc: &mut RootRenderingComponent, vdom: VdomWeak) {
    if local_route == "#p02" {
        let vdom = vdom.clone();
        fetchgmod::async_fetch_game_config_request(rrc, &vdom);
        rrc.web_communication.local_route = "p02_start_a_group.html".to_owned();
    } else if local_route.starts_with("#p03") {
        rrc.game_data.my_player_number = 2;
        if local_route.contains('.') {
            let gr = routermod::get_url_param_in_hash_after_dot(&local_route);
            storagemod::save_group_id_string_to_local_storage(rrc, gr.to_string());
        } else {
            storagemod::load_group_id_string(rrc);
        }
        rrc.web_communication.local_route = "p03_join_a_group.html".to_owned();
    } else if local_route == "#p04" {
        statusjoinedmod::on_load_joined(rrc);
        rrc.web_communication.local_route = "p04_wait_to_start.html".to_owned();
    } else if local_route == "#p05" {
        rrc.web_communication.local_route = "p05_choose_game.html".to_owned();
    } else if local_route == "#p06" {
        rrc.web_communication.local_route = "p06_drink.html".to_owned();
    } else if local_route == "#p07" {
        rrc.web_communication.local_route = "p07_do_not_drink.html".to_owned();
    } else if local_route == "#p08" {
        rrc.web_communication.local_route = "p08_instructions.html".to_owned();
    } else if local_route == "#p11" {
        rrc.web_communication.local_route = "p11_gameboard.html".to_owned();
    } else if local_route == "#p21" {
        rrc.web_communication.local_route = "p21_menu.html".to_owned();
    } else if local_route == "#p31" {
        rrc.web_communication.local_route = "p31_debug_text.html".to_owned();
    } else {
        rrc.web_communication.local_route = "p01_start.html".to_owned();
    }
}
