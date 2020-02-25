// fetchmod.rs
//! fetch game_config, game metadata, imgs

//region: use
use crate::*;
use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
//endregion

/// async fetch for gameconfig.json and update rrc
pub fn async_fetch_game_config_and_update(
    rrc: &mut RootRenderingComponent,
    vdom: dodrio::VdomWeak,
) {
    let url_config = format!(
        "{}/content/{}/game_config.json",
        rrc.web_data.href, rrc.game_data.game_name
    );
    spawn_local(async move {
        let respbody = websysmod::fetch_response(url_config).await;
        let json = unwrap!(serde_json::from_str(respbody.as_str()));
        // websysmod::debug_write(format!("respbody: {}", respbody).as_str());
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    rrc.game_data.game_config = json;
                }
            })
            .await
        );
    });
}

/// async fetch for gamesmetadata.json and update rrc
pub fn fetch_games_metadata_and_update(href: &str, vdom: dodrio::VdomWeak) {
    let url_config = format!("{}/content/gamesmetadata.json", href);
    spawn_local(async move {
        // websysmod::debug_write(format!("respbody: {}", respbody).as_str());
        let respbody = websysmod::fetch_response(url_config).await;
        let v: gamedatamod::GamesMetadata = unwrap!(serde_json::from_str(&respbody));
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // fill the vector
                    rrc.game_data.content_folders.clear();
                    for x in &v.vec_game_metadata {
                        rrc.game_data.content_folders.push(x.folder.clone());
                    }
                    rrc.game_data.games_metadata = Some(v);
                }
            })
            .await
        );
    });
}

/// async fetch for videos.json and update rrc
pub fn fetch_videos_and_update(href: &str, vdom: dodrio::VdomWeak) {
    let url = format!("{}/content/videos.json", href);
    spawn_local(async move {
        let respbody = websysmod::fetch_response(url).await;
        let v: gamedatamod::Videos = unwrap!(serde_json::from_str(&respbody));
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // fill the vector
                    rrc.videos = v.videos;
                }
            })
            .await
        );
    });
}

/// fetch all imgs for the cache
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_all_img_for_cache_request(rrc: &mut RootRenderingComponent) {
    for x in &rrc.game_data.card_grid_data {
        if x.card_index_and_id != 0 {
            let url_img = format!(
                "content/{}/img/{}",
                rrc.game_data.game_name,
                unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                    .img_filename
                    .get(x.card_number_and_img_src))
            );
            // websysmod::debug_write(&url_img);
            // this is async, so I don't care how much it takes
            spawn_local(websysmod::fetch_only(url_img));
        }
    }
}
