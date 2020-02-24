//! routermod - A simple `#`-fragment router for dodrio html templating  
//! This is the generic module. All the specifics for a website are isolated in the  
//! module routerimplmod.  
//! The RootRenderingComponent struct must have the fields:  
//! rrc.web_communication.local_route - fill_rrc_local_route() will fills this field.
//!        It will be the name of the html template file to fetch  
//! rrc.web_communication.html_template - a string where the html_template is
//!        fetched from the server  
//! 2020-02-24 I tried to put away as much boilerplate as I could from routerimplmod
//! , but some code is very difficult to split in Rust.

use crate::*;

use dodrio::VdomWeak;
use wasm_bindgen::{prelude::*, JsCast};
use unwrap::unwrap;

/// trait intended to be added to VdomWeakWrapper
pub trait Routing {
    //region: specific code to be implemented
    fn closure_specific_on_hash_change(
        vdom: dodrio::VdomWeak,
        short_local_route: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static>;
    //endregion: specific code
    //region:generic code (boilerplate)
    /// Start the router.
    fn start_router(&self, vdom: VdomWeak) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
        let on_hash_change = Box::new(move || {
            let location = websysmod::window().location();
            let mut short_local_route = unwrap!(location.hash());
            if short_local_route.is_empty() {
                short_local_route = "index".to_owned();
            }
            // websysmod::debug_write("after .hash");
            wasm_bindgen_futures::spawn_local({
                let vdom = vdom.clone();
                async move {
                    let _ = vdom
                        .with_component({
                            let vdom = vdom.clone();
                            Self::closure_specific_on_hash_change(vdom, short_local_route)
                        })
                        .await;
                }
            });
        });
        self.set_on_hash_change_callback(on_hash_change);
    }

    fn set_on_hash_change_callback(&self, mut on_hash_change: Box<dyn FnMut()>) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
        // Call it once to handle the initial `#` fragment.
        on_hash_change();

        // Now listen for hash changes forever.
        //
        // Note that if we ever intended to unmount our app, we would want to
        // provide a method for removing this router's event listener and cleaning
        // up after ourselves.
        let on_hash_change = Closure::wrap(on_hash_change);
        websysmod::window()
            .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
            .unwrap_throw();
        on_hash_change.forget();
    }
}

/// get the first param after hash in local route after dot
/// example &p03.1234 -> 1234
pub fn get_url_param_in_hash_after_dot(short_local_route: &str) -> &str {
    let mut spl = short_local_route.split('.');
    unwrap!(spl.next());
    unwrap!(spl.next())
}

/// only the html between the <body> </body>
pub fn between_body_tag(resp_body_text: &str) -> String {
    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
    //return
    if pos1 == 0 {
        resp_body_text.to_string()
    } else {
        #[allow(clippy::integer_arithmetic)]
        {
            unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string()
        }
    }
}
