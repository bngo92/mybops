use crate::{
    base::{Button, INPUT_STYLE},
    bootstrap::Accordion,
};
use leptos::{html::Input, prelude::*};
use mybops::{
    Spotify,
    spotify::{Playlists, RecentTracks},
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[component]
pub fn SpotifyIntegration(#[prop(into)] logged_in: Signal<bool>) -> impl IntoView {
    let import_ref = NodeRef::<Input>::new();
    let recent_tracks = LocalResource::new(move || async move {
        if logged_in.get() {
            Some(get_recent_tracks().await.unwrap())
        } else {
            None
        }
    });
    let playlists = LocalResource::new(move || async move {
        if logged_in.get() {
            Some(get_playlists().await.unwrap())
        } else {
            None
        }
    });

    let import = {
        Action::new_unsync(move |_: &()| {
            let input = import_ref.get().unwrap().value();
            // TODO: handle bad input
            let source = match crate::parse_spotify_source(input) {
                Some(Spotify::Playlist(id)) => Some(("spotify:playlist", id)),
                Some(Spotify::Album(id)) => Some(("spotify:album", id)),
                Some(Spotify::Track(id)) => Some(("spotify:track", id)),
                None => None,
            };
            async move {
                if let Some((source, id)) = source {
                    crate::import_list(source, &id.id).await.unwrap()
                }
            }
        })
    };
    let import_track = Action::new_unsync(|input: &String| {
        // TODO: handle bad input
        let source = match crate::parse_spotify_source(input.clone()) {
            Some(Spotify::Track(id)) => Some(("spotify:track", id)),
            _ => None,
        };
        async move {
            if let Some((source, id)) = source {
                crate::import_list(source, &id.id).await.unwrap();
                // TODO: refresh row
            }
        }
    });

    let default_import =
        "https://open.spotify.com/playlist/5MztFbRbMpyxbVYuOSfQV9?si=9db089ab25274efa";
    let track_html = move || {
        if let Some(Some(tracks)) = recent_tracks.get() {
            tracks
                .tracks
                .into_iter()
                .map(|i| {
                    view! {
                      <div>
                        <a href=i.url.clone()>{i.name.clone()}</a>
                        {move || {
                          i.user_score
                            .is_none()
                            .then(|| {
                              view! {
                                <Button
                                  class="text-white bg-primary"
                                  on:click={
                                    let url = i.url.clone();
                                    move |_| {
                                      import_track.dispatch(url.clone());
                                    }
                                  }
                                >
                                  "Import"
                                </Button>
                              }
                            })
                        }}
                      </div>
                      <div>{i.rating}</div>
                      <div>{i.user_score}</div>
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    };
    crate::nav_content(
        view! {
          <a href="#" class="font-medium">
            "Spotify"
          </a>
        },
        view! {
          <div>
            <Accordion header="Recent Tracks".to_owned() collapsed=false>
              {move || {
                if logged_in.get() {
                  view! {
                    <div class="grid grid-cols-[1fr_6rem_6rem] gap-1">
                      <div></div>
                      <strong>"Rating"</strong>
                      <strong>"User Score"</strong>
                      {track_html}
                    </div>
                  }
                    .into_any()
                } else {
                  view! {
                    <p>
                      "Create an account to view and import tracks that were recently played in Spotify"
                    </p>
                  }
                    .into_any()
                }
              }}
            </Accordion>
            <Accordion header="Saved Playlists".to_owned() collapsed=false>
              {move || {
                if let Some(Some(ref playlists)) = *playlists.read() {
                  {
                    playlists
                      .items
                      .iter()
                      .map(|i| {
                        view! {
                          <div>
                            <a href=i.external_urls["spotify"].clone()>{i.name.clone()}</a>
                          </div>
                        }
                      })
                      .collect_view()
                      .into_any()
                  }
                } else {
                  view! { <p>"Create an account to import playlists from Spotify"</p> }.into_any()
                }
              }}
            </Accordion>
            <h2 class="text-xl font-medium">"Import from Spotify link"</h2>
            <form>
              <div class="flex gap-2">
                <div class="basis-3xl">
                  <input node_ref=import_ref type="text" class=INPUT_STYLE value=default_import />
                </div>
                <Button
                  class="text-white bg-primary"
                  on:click=move |_| {
                    import.dispatch(());
                  }
                  disabled=logged_in
                >
                  "Import"
                </Button>
              </div>
            </form>
          </div>
        },
    )
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
