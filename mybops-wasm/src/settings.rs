use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use mybops::User;

use crate::base::Button;

#[component]
pub fn Settings(#[prop(into)] user: Signal<User>) -> impl IntoView {
    let origin = location().origin().unwrap();
    // TODO: let you remove integrations
    // Should we link to Google profile?
    crate::nav_content(
        view! {
          <a href="#" class="text-lg font-medium text-black">
            "Settings"
          </a>
        },
        view! {
          <div>
            <h1>"Integrations"</h1>
            <h2>"Spotify"</h2>
            {
              let origin = origin.clone();
              move || {
                let user = user.get();
                let origin = origin.clone();
                if let (Some(url), Some(user)) = (user.spotify_url, user.spotify_user) {
                  view! { <a href=url.clone()>{user}</a> }.into_any()
                } else {
                  view! {
                    <Button
                      style="primary"
                      on:click=move |_| {
                        let navigate = use_navigate();
                        navigate(
                          &format!(
                            "https://accounts.spotify.com/authorize?client_id=ee3d1b4f8d80477ea48743a511ef3018&redirect_uri={}/api/login&response_type=code&scope=playlist-modify-public playlist-modify-private user-read-recently-played playlist-read-private",
                            origin.as_str(),
                          ),
                          Default::default(),
                        )
                      }
                    >
                      "Log in with Spotify"
                    </Button>
                  }
                    .into_any()
                }
              }
            }
            <h2>"Google"</h2>
            {move || {
              let origin = origin.clone();
              if let Some(google_email) = user.get().google_email {
                view! { <p>{google_email}</p> }.into_any()
              } else {
                view! {
                  <Button
                    style="primary"
                    on:click=move |_| {
                      let navigate = use_navigate();
                      navigate(
                        &format!(
                          "https://accounts.google.com/o/oauth2/v2/auth?client_id=1038220726403-n55jha2cvprd8kdb4akdfvo0uiok4p5u.apps.googleusercontent.com&redirect_uri={}/api/login/google&response_type=code&scope=email",
                          origin.as_str(),
                        ),
                        Default::default(),
                      )
                    }
                  >
                    "Log in with Google"
                  </Button>
                }
                  .into_any()
              }
            }}
          </div>
        },
    )
}
