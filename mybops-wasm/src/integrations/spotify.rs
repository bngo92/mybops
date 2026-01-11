use crate::{UserProps, bootstrap::Accordion};
use mybops::{
    Spotify,
    spotify::{Playlists, RecentTracks},
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlSelectElement, Response};
use yew::{Callback, HtmlResult, function_component, html, suspense::use_future, use_node_ref};

#[function_component]
pub fn SpotifyIntegration(UserProps { logged_in }: &UserProps) -> HtmlResult {
    let logged_in = *logged_in;
    let import_ref = use_node_ref();
    let (ref recent_tracks, ref playlists) = *use_future(|| async move {
        if logged_in {
            (
                Some(get_recent_tracks().await.unwrap()),
                Some(get_playlists().await.unwrap()),
            )
        } else {
            (None, None)
        }
    })?;

    let import = {
        let import_ref = import_ref.clone();
        Callback::from(move |_| {
            let input = import_ref.cast::<HtmlSelectElement>().unwrap().value();
            // TODO: handle bad input
            let source = match crate::parse_spotify_source(input) {
                Some(Spotify::Playlist(id)) => Some(("spotify:playlist", id)),
                Some(Spotify::Album(id)) => Some(("spotify:album", id)),
                Some(Spotify::Track(id)) => Some(("spotify:track", id)),
                None => None,
            };
            if let Some((source, id)) = source {
                wasm_bindgen_futures::spawn_local(async move {
                    crate::import_list(source, &id.id).await.unwrap();
                });
            }
        })
    };
    let import_track = Callback::from(|input| {
        // TODO: handle bad input
        let source = match crate::parse_spotify_source(input) {
            Some(Spotify::Track(id)) => Some(("spotify:track", id)),
            _ => None,
        };
        if let Some((source, id)) = source {
            wasm_bindgen_futures::spawn_local(async move {
                crate::import_list(source, &id.id).await.unwrap();
                // TODO: refresh row
            });
        }
    });

    let default_import =
        "https://open.spotify.com/playlist/5MztFbRbMpyxbVYuOSfQV9?si=9db089ab25274efa";
    let track_html = if let Some(tracks) = recent_tracks {
        tracks
            .tracks
            .iter()
            .map(|i| {
                let import_track = {
                    let url = i.url.clone();
                    let import_track = import_track.clone();
                    Callback::from(move |_| {
                        let url = url.clone();
                        import_track.emit(url)
                    })
                };
                html! {
                    <div class="row">
                        <div class="col">
                             <a href={i.url.clone()}>{&i.name}</a>
                             if i.user_score.is_none() {
                                 <button type="button" class="btn btn-success" onclick={import_track}>{"Import"}</button>
                             }
                        </div>
                        <div class="col-1">{i.rating}</div>
                        <div class="col-1">{i.user_score}</div>
                    </div>
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    Ok(crate::nav_content(
        html! {
          <ul class="navbar-nav me-auto">
            <li class="navbar-brand">{"Spotify"}</li>
          </ul>
        },
        html! {
          <div>
            <Accordion header={"Recent Tracks"} collapsed={false}>
              if logged_in {
                <div class="row">
                  <div class="col"></div>
                  <div class="col-1"><strong>{"Rating"}</strong></div>
                  <div class="col-1"><strong>{"User Score"}</strong></div>
                </div>
                {for track_html}
              } else {
                <p>{"Create an account to view and import tracks that were recently played in Spotify"}</p>
              }
            </Accordion>
            <Accordion header={"Saved Playlists"} collapsed={false}>
              if let Some(playlists) = playlists {
                {for playlists.items.iter().map(|i| html! {<div><a href={i.external_urls["spotify"].clone()}>{&i.name}</a></div>})}
              } else {
                <p>{"Create an account to import playlists from Spotify"}</p>
              }
            </Accordion>
            <h2>{"Import from Spotify link"}</h2>
            <form>
              <div class="row">
                <div class="col-12 col-md-8 col-lg-9">
                  <input ref={import_ref.clone()} type="text" class="w-100 h-100" value={default_import}/>
                </div>
                <div class="col-2 col-lg-1 pe-2">
                  <button type="button" class="btn btn-success" onclick={import} disabled={logged_in}>{"Import"}</button>
                </div>
              </div>
            </form>
          </div>
        },
    ))
}

async fn get_recent_tracks() -> Result<RecentTracks, JsValue> {
    let window = crate::window();
    let request = crate::query("/api/spotify/recentTracks", "GET").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}

async fn get_playlists() -> Result<Playlists, JsValue> {
    let window = crate::window();
    let request = crate::query("/api/spotify/playlists", "GET").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}
