#![feature(iter_intersperse)]
use crate::{app::App, dataframe::DataFrame};
use arrow::array::AsArray;
use js_sys::Uint8Array;
use leptos::{either::Either, prelude::*};
use mybops::{Id, Items, List, Lists, Spotify, User};
use regex::Regex;
use std::{collections::HashSet, io::Cursor};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

mod app;
mod base;
mod bootstrap;
mod dataframe;
mod docs;
mod edit;
mod home;
mod integrations;
mod list;
mod nfl;
mod plot;
mod random;
mod search;
mod settings;
pub mod tournament;

#[derive(Clone)]
pub enum ListsRoute {
    View,
    List,
    Edit,
    Match,
    Tournament,
}

pub fn parse_spotify_source(input: String) -> Option<Spotify> {
    let playlist_re = Regex::new(r"https://open.spotify.com/playlist/([[:alnum:]]*)").unwrap();
    let album_re = Regex::new(r"https://open.spotify.com/album/([[:alnum:]]*)").unwrap();
    let track_re = Regex::new(r"https://open.spotify.com/track/([[:alnum:]]*)").unwrap();
    return if let Some(caps) = playlist_re.captures_iter(&input).next() {
        Some(Spotify::Playlist(Id {
            id: caps[1].to_owned(),
            raw_id: input,
        }))
    } else if let Some(caps) = album_re.captures_iter(&input).next() {
        Some(Spotify::Album(Id {
            id: caps[1].to_owned(),
            raw_id: input,
        }))
    } else if let Some(caps) = track_re.captures_iter(&input).next() {
        Some(Spotify::Track(Id {
            id: caps[1].to_owned(),
            raw_id: input,
        }))
    } else {
        None
    };
}

pub fn parse_setlist_source(input: String) -> Option<Id> {
    let re = Regex::new(r"https://www.setlist.fm/setlist/.*-([[:alnum:]]*).html").unwrap();
    return if let Some(caps) = re.captures_iter(&input).next() {
        Some(Id {
            id: caps[1].to_owned(),
            raw_id: input,
        })
    } else {
        None
    };
}

fn nav_content(nav: impl IntoView, content: impl IntoView) -> impl IntoView {
    view! {
      <nav class="flex gap-4 justify-between items-center p-3 bg-primary">{nav}</nav>
      <div class="p-3 overflow-y-auto bg-pink-50/10">{content}</div>
    }
}

#[component]
fn Content(
    heading: String,
    heading_href: String,
    nav: impl IntoView,
    content: impl IntoView,
) -> impl IntoView {
    let (collapse, set_collapse) = signal(true);

    view! {
      <nav class="flex flex-col lg:flex-row gap-6 lg:gap-8 justify-center lg:items-center p-3 bg-primary">
        <div class="flex justify-between w-full lg:w-auto">
          <a href=heading_href class="w-full lg:w-auto font-medium">
            {heading}
          </a>
          <button
            class="lg:hidden flex-1 s-fit"
            type="button"
            on:click=move |_| set_collapse.set(!collapse.get())
          >
            {move || {
              if collapse.get() {
                Either::Left(
                  view! {
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke-width="2"
                      stroke="currentColor"
                      class="size-4"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M12 4.5v15m7.5-7.5h-15"
                      />
                    </svg>
                  },
                )
              } else {
                Either::Right(
                  view! {
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke-width="2"
                      stroke="currentColor"
                      class="size-4"
                    >
                      <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14" />
                    </svg>
                  },
                )
              }
            }}
          </button>
        </div>
        <div class="flex-1" class=(["hidden", "lg:block"], collapse)>
          {nav}
        </div>
      </nav>
      <div class="p-3 overflow-y-auto bg-pink-50/10">{content}</div>
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
    Ok(())
}

async fn fetch_lists(favorite: bool) -> Result<Vec<List>, JsValue> {
    let window = window();
    let request = query(&format!("/api/lists?favorite={}", favorite), "GET")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    let lists: Lists = serde_wasm_bindgen::from_value(json).unwrap();
    Ok(lists.lists)
}

async fn fetch_list(id: &str) -> Result<Option<List>, JsValue> {
    let window = window();
    let request = query(&format!("/api/lists/{}", id), "GET")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    if resp.status() == 404 {
        return Ok(None);
    }
    let json = JsFuture::from(resp.json()?).await?;
    Ok(Some(serde_wasm_bindgen::from_value(json).unwrap()))
}

async fn create_list(query: Option<String>) -> Result<List, JsValue> {
    let window = window();
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(
        &if let Some(query) = query {
            format!("/api/lists?query={query}")
        } else {
            String::from("/api/lists")
        },
        &opts,
    )?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}

async fn update_list(list: &List) -> Result<(), JsValue> {
    let window = window();
    let opts = RequestInit::new();
    opts.set_method("PUT");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&serde_json::to_string(&list).unwrap()));
    let request = Request::new_with_str_and_init(&format!("/api/lists/{}", list.id), &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(())
}

async fn delete_list(id: &str) -> Result<(), JsValue> {
    let window = window();
    let opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&format!("/api/lists/{}", id), &opts)?;
    JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(())
}

async fn query_list(list: &List, qs: Option<String>) -> Result<Option<DataFrame>, JsValue> {
    let window = window();
    let url = if let Some(qs) = qs {
        format!("/api/lists/{}/query?query={}", list.id, qs)
    } else {
        format!("/api/lists/{}/query", list.id)
    };
    let request = query(&url, "GET").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    if [400, 500].contains(&resp.status()) {
        return Err(JsFuture::from(resp.text()?).await?);
    }
    Ok(serialize_into_df(resp).await?.map(|mut items| {
        if let Some(id_col) = items.column("id") {
            let ids: HashSet<_> = list.items.iter().map(|i| i.id.as_str()).collect();
            // inner join
            items.remove(
                id_col
                    .as_string::<i64>()
                    .iter()
                    .map(|id| ids.contains(id.unwrap()))
                    .collect(),
            );
        }
        items
    }))
}

async fn get_items(id: &str) -> Result<Items, JsValue> {
    let window = window();
    let request = query(&format!("/api/lists/{}/items", id), "GET").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}

async fn update_stats(list: &str, win: &str, lose: &str) -> Result<(), JsValue> {
    let window = window();
    let request = query(
        &format!(
            "/api/?action=update&list={}&win={}&lose={}",
            list, win, lose
        ),
        "POST",
    )?;
    JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(())
}

async fn push_list(id: &str) -> Result<(), JsValue> {
    let window = window();
    let request = query(&format!("/api/?action=push&list={}", id), "POST")?;
    JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(())
}

async fn import_list(source: &str, id: &str) -> Result<(), JsValue> {
    let window = window();
    let request = query(
        &format!("/api/?action=import&source={source}&id={id}"),
        "POST",
    )?;
    JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(())
}

async fn find_items(search: &str) -> Result<Option<DataFrame>, JsValue> {
    let window = window();
    let request = query(&format!("/api/items?q=search&query={}", search), "GET")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    if [400, 500].contains(&resp.status()) {
        return Err(JsFuture::from(resp.text()?).await?);
    }
    serialize_into_df(resp).await
}

async fn serialize_into_df(resp: Response) -> Result<Option<DataFrame>, JsValue> {
    let buf = Uint8Array::new(&JsFuture::from(resp.array_buffer()?).await?).to_vec();
    if buf.is_empty() {
        return Ok(None);
    }
    let mut buf = Cursor::new(buf);
    Ok(Some(DataFrame::from(&mut buf)))
}

async fn delete_items(ids: &[String]) -> Result<(), JsValue> {
    let window = window();
    let request = query(&format!("/api/items?ids={}", ids.join(",")), "DELETE")?;
    JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(())
}

fn query(url: &str, method: &str) -> Result<Request, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(RequestMode::Cors);
    Request::new_with_str_and_init(url, &opts)
}

async fn get_user() -> Result<User, JsValue> {
    let window = window();
    let request = query("/api/user", "GET")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    Ok(serde_wasm_bindgen::from_value(json).unwrap())
}

fn user_list(list: &List, user: &Option<User>) -> bool {
    Some(&list.user_id) == user.as_ref().as_ref().map(|u| &u.user_id)
        || (user.is_none() && list.user_id == "demo")
}

fn not_found() -> impl IntoView {
    view! { <h1>"Not found"</h1> }
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}
