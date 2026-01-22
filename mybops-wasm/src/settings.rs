use leptos::prelude::*;
use mybops::User;

#[component]
pub fn Settings(#[prop(into)] user: Signal<User>) -> impl IntoView {
    let origin = location().origin().unwrap();
    // TODO: let you remove integrations
    // Should we link to Google profile?
    crate::nav_content(
        view! {
          <ul class="navbar-nav me-auto">
            <li class="navbar-brand">"Settings"</li>
          </ul>
        },
        view! {
          <div>
            <h1>"Integrations"</h1>
            <h2>"Spotify"</h2>
            {
              let origin = origin.clone();
              move || {
                let user = user.get();
                if let (Some(url), Some(user)) = (user.spotify_url, user.spotify_user) {
                  view! { <a href=url.clone()>{user}</a> }.into_any()
                } else {
                  view! {
                    <a
                      class="btn btn-success"
                      href=format!(
                        "https://accounts.spotify.com/authorize?client_id=ee3d1b4f8d80477ea48743a511ef3018&redirect_uri={}/api/login&response_type=code&scope=playlist-modify-public playlist-modify-private user-read-recently-played playlist-read-private",
                        origin.as_str(),
                      )
                    >
                      "Log in with Spotify"
                    </a>
                  }
                    .into_any()
                }
              }
            }
            <h2>"Google"</h2>
            {move || {
              if let Some(google_email) = user.get().google_email {
                view! { <p>{google_email}</p> }.into_any()
              } else {
                view! {
                  <a
                    class="btn btn-success"
                    href=format!(
                      "https://accounts.google.com/o/oauth2/v2/auth?client_id=1038220726403-n55jha2cvprd8kdb4akdfvo0uiok4p5u.apps.googleusercontent.com&redirect_uri={}/api/login/google&response_type=code&scope=email",
                      origin.as_str(),
                    )
                  >
                    "Log in with Google"
                  </a>
                }
                  .into_any()
              }
            }}
          </div>
        },
    )
}
