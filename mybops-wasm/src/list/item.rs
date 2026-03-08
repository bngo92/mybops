use crate::{
    base::{Button, INPUT_STYLE, SelectWithCallback},
    bootstrap::{Modal, Toast},
};
use arrow::{array::AsArray, datatypes::UInt64Type};
use js_sys::Error;
use leptos::{
    either::EitherOf3,
    html::{Dialog, Input},
    prelude::*,
};
use mybops::{Id, ItemMetadata, List, ListMode, SourceType, Spotify, User};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AbortSignal, Request, RequestInit, RequestMode, Response, Url};

#[derive(Clone)]
struct ListItem {
    item: ItemMetadata,
    hidden_ref: NodeRef<Input>,
    note_ref: NodeRef<Input>,
}

#[derive(Default)]
struct State {
    rating: RwSignal<Option<u64>>,
    hidden: bool,
    note: String,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            // RwSignal clones shallowly
            rating: RwSignal::new(self.rating.get()),
            hidden: self.hidden,
            note: self.note.clone(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ItemMode {
    View,
    Update,
    Delete,
}

#[component]
pub fn ListItems(
    user: Signal<Option<User>>,
    #[prop(into)] list: Signal<List>,
    mode: ReadSignal<ItemMode>,
) -> impl IntoView {
    let (items, set_items) = signal(
        list.read()
            .items
            .iter()
            .map(|i| ListItem {
                item: i.clone(),
                hidden_ref: NodeRef::<Input>::new(),
                note_ref: NodeRef::<Input>::new(),
            })
            .collect::<Vec<_>>(),
    );
    let (prev_state, set_prev_state) = signal(None);
    let (state, set_state) = signal(None);
    let (modal, set_modal) = signal(0);

    LocalResource::new(move || async move {
        if !matches!(list.read().mode, ListMode::View(_)) {
            let query = crate::query_list(
                &list.read(),
                Some("SELECT id, rating, hidden, note FROM item".to_owned()),
            )
            .await
            .unwrap();
            if let Some(query) = query {
                let index: HashMap<_, _> = items
                    .read()
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (item.item.id.as_str().to_owned(), i))
                    .collect();
                let ids = query.column("id").unwrap().as_string::<i64>();
                // NullArray does not cast correctly
                let ratings = if let Some(ratings) = query
                    .column("rating")
                    .unwrap()
                    .as_primitive_opt::<UInt64Type>()
                {
                    ratings.into_iter().collect()
                } else {
                    vec![None; query.column("rating").unwrap().len()]
                };
                let hidden = query.column("hidden").unwrap().as_boolean();
                let note = query.column("note").unwrap().as_string::<i64>();
                let mut new_state = vec![State::default(); items.read().len()];
                for (((id, &rating), hidden), note) in ids
                    .iter()
                    .zip(ratings.iter())
                    .zip(hidden.iter())
                    .zip(note.iter())
                {
                    new_state[index[id.unwrap()]] = State {
                        rating: RwSignal::new(rating),
                        hidden: hidden.unwrap(),
                        note: note.unwrap().to_owned(),
                    };
                }
                set_prev_state.set(Some(new_state.clone()));
                set_state.set(Some(new_state.clone()));
            }
        }
    });

    let update_rating = move |(i, rating): (usize, Option<u64>)| {
        set_state.update(|state| state.as_mut().unwrap()[i].rating.set(rating))
    };
    let toast = Toast::new("list-items-alert");
    let save = Action::new_unsync(move |_| async move {
        let mut update_ids = HashMap::new();
        let mut update_indexes = Vec::new();
        for (
            i,
            (
                ListItem {
                    item,
                    hidden_ref,
                    note_ref,
                },
                rating_hidden,
            ),
        ) in items
            .read()
            .iter()
            .zip(state.read().as_ref().unwrap().iter())
            .enumerate()
        {
            let State { rating, hidden, .. } = rating_hidden;
            let mut updates = HashMap::new();
            if prev_state.read().as_ref().unwrap()[i].rating != *rating {
                updates.insert(String::from("rating"), rating.get().into());
            }
            let value = Value::Bool(hidden_ref.get().unwrap().checked());
            #[allow(clippy::cmp_owned)]
            if value != *hidden {
                updates.insert(String::from("hidden"), value);
            }
            let value = note_ref.get().unwrap().value();
            if prev_state.read().as_ref().unwrap()[i].note != value {
                updates.insert(String::from("note"), value.clone().into());
            }
            if !updates.is_empty() {
                update_ids.insert(item.id.clone(), updates.clone());
                update_indexes.push((i, updates));
            }
        }
        if !update_ids.is_empty() {
            let window = web_sys::window().expect("no global `window` exists");
            let opts = RequestInit::new();
            opts.set_method("POST");
            opts.set_mode(RequestMode::Cors);
            opts.set_signal(Some(&AbortSignal::timeout_with_u32(1000)));
            let updates = JsValue::from_str(&serde_json::to_string(&update_ids).unwrap());
            opts.set_body(&updates);
            let request =
                Request::new_with_str_and_init("/api/?action=updateItems", &opts).unwrap();
            request
                .headers()
                .set("Content-Type", "application/json")
                .unwrap();
            match JsFuture::from(window.fetch_with_request(&request)).await {
                Ok(resp) => {
                    let resp_value: Response = resp.dyn_into().unwrap();
                    if resp_value.status() >= 400 {
                        let alert = Err(JsFuture::from(resp_value.text().unwrap())
                            .await
                            .unwrap()
                            .as_string()
                            .unwrap());
                        toast.set(alert);
                    } else {
                        // Update the rating and hidden state values if the save request is successful.
                        set_state.update(|state| {
                            for (i, update) in update_indexes {
                                for (k, v) in update {
                                    let State {
                                        rating,
                                        hidden,
                                        note,
                                    } = state.as_mut().unwrap().get_mut(i).unwrap();
                                    match k.as_str() {
                                        "rating" => {
                                            rating.set(v.as_u64());
                                        }
                                        "hidden" => {
                                            *hidden = v.as_bool().unwrap();
                                        }
                                        "note" => {
                                            *note = v.as_str().unwrap().to_owned();
                                        }
                                        _ => unimplemented!(),
                                    }
                                }
                            }
                        });
                        set_prev_state.set(state.get());
                        toast.set(Ok("Save successful".to_owned()));
                    }
                }
                Err(e) => {
                    toast.set(Err(e.dyn_into::<Error>().unwrap().to_string().into()));
                }
            }
        }
    });
    let push = Action::new_unsync(move |_| async move {
        crate::push_list(&list.read().id).await.unwrap();
    });
    let modal_ref = NodeRef::<Dialog>::new();
    let open = move |i| {
        set_modal.set(i);
        modal_ref.get().unwrap().show_modal().unwrap()
    };
    let modal_back = move |_| {
        let i = modal.get();
        set_modal.set(if i == 0 {
            items.read().len() - 1
        } else {
            i - 1
        })
    };
    let modal_forward = move |_| {
        let i = modal.get();
        set_modal.set(if i == items.read().len() - 1 {
            0
        } else {
            i + 1
        })
    };
    let delete = Action::new_unsync(move |(i, id): &(usize, String)| {
        let i = *i;
        let id = id.clone();
        async move {
            crate::delete_items(&[id]).await.unwrap();
            set_items.update(|items| {
                items.remove(i);
            });
            set_state.update(|state| {
                state.as_mut().map(|state| state.remove(i));
            });
        }
    });

    let disabled = move || user.read().is_none() || !crate::user_list(&list.read(), &user.read());
    let modal_html = view! {
      <Modal
        header=move || {
          let i = modal.get();
          items.read()[i].item.name.clone()
        }
        modal_ref=modal_ref
      >
        <div class="tw:relative">
          {move || {
            let i = modal.get();
            items
              .read()[i]
              .item
              .iframe
              .clone()
              .map(|iframe| {
                view! {
                  <iframe width="100%" height="380" prop:frameborder="0" src=iframe></iframe>
                }
              })
          }}
          <button
            class="tw:absolute tw:flex tw:justify-center tw:items-center tw:w-[15%] tw:text-gray-300"
            type="button"
            on:click=modal_back
            style="top: 56px; bottom: auto; height: 137px"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="tw:size-8"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 19.5 8.25 12l7.5-7.5"
              />
            </svg>
          </button>
          <button
            class="tw:absolute tw:flex tw:right-0 tw:justify-center tw:items-center tw:w-[15%] tw:text-gray-300"
            type="button"
            on:click=modal_forward
            style="top: 56px; bottom: auto; height: 137px"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="tw:size-8"
            >
              <path stroke-linecap="round" stroke-linejoin="round" d="m8.25 4.5 7.5 7.5-7.5 7.5" />
            </svg>
          </button>
        </div>
        {move || {
          let i = modal.get();
          let onchange = move |rating| update_rating((i, rating));
          state
            .read()
            .as_ref()
            .map(|state| {
              view! { <Rating rating=state[i].rating onchange=onchange /> }
            })
        }}
      </Modal>
    };
    let source_html = list
        .read()
        .sources
        .iter()
        .map(|source| {
            let raw_id = match &source.source_type {
                SourceType::Spotify(Spotify::Playlist(Id { raw_id, .. }))
                | SourceType::Spotify(Spotify::Album(Id { raw_id, .. }))
                | SourceType::Setlist(Id { raw_id, .. })
                    if Url::new(raw_id).is_ok() =>
                {
                    Some(raw_id.clone())
                }
                _ => None,
            };
            if let SourceType::ListItems(id) = &source.source_type {
                EitherOf3::A(view! { <a href=format!("/lists/{}", id)>{source.name.clone()}</a> })
            } else if let Some(href) = raw_id {
                EitherOf3::B(view! { <a href=href>{source.name.clone()}</a> })
            } else {
                EitherOf3::C(view! { <p>{source.name.clone()}</p> })
            }
        })
        .collect_view();
    let (style, grid) = match mode.get() {
        ItemMode::View => ("", "max-height: 800px"),
        ItemMode::Update => (
            "grid-template-columns: auto max-content max-content max-content",
            "max-height: 800px; grid-template-columns: subgrid; grid-column: span 4",
        ),
        ItemMode::Delete => (
            "grid-template-columns: auto max-content",
            "max-height: 800px; grid-template-columns: subgrid; grid-column: span 2",
        ),
    };
    let html = move || match mode.get() {
        ItemMode::View => items
            .read()
            .iter()
            .enumerate()
            .map(|(i, ListItem { item, .. })| {
                view! {
                  <label>
                    <a href="#" on:click=move |_| open(i)>
                      {item.name.clone()}
                    </a>
                  </label>
                }
                .into_any()
            })
            .collect_view(),
        ItemMode::Update => items
            .read()
            .iter()
            .enumerate()
            .map(
                |(
                    i,
                    ListItem {
                        item,
                        hidden_ref,
                        note_ref,
                    },
                )| {
                    let label = view! {
                      <label>
                        <a href="#" on:click=move |_| open(i)>
                          {item.name.clone()}
                        </a>
                      </label>
                    };
                    if let Some(ref state_copy) = *state.read() {
                        let State {
                            rating,
                            hidden,
                            note,
                        } = state_copy[i].clone();
                        view! {
                          <>
                            {label} <div>
                              <Rating
                                rating=rating
                                onchange=move |rating| update_rating((i, rating))
                              />
                            </div> <div class="tw:flex tw:justify-center tw:items-center">
                              <input
                                node_ref=*hidden_ref
                                class="tw:my-1! tw:size-4"
                                type="checkbox"
                                checked=hidden
                                disabled=disabled
                              />
                            </div> <div>
                              <input
                                node_ref=*note_ref
                                class=INPUT_STYLE
                                value=Some(note.clone())
                                disabled=disabled
                              />
                            </div>
                          </>
                        }
                        .into_any()
                    } else {
                        view! { <>{label} <div></div> <div></div> <div></div></> }.into_any()
                    }
                },
            )
            .collect_view(),
        ItemMode::Delete => items
            .read()
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, ListItem { item, .. })| {
                view! {
                  <>
                    <label>
                      <a href="#" on:click=move |_| open(i)>
                        {item.name.clone()}
                      </a>
                    </label>
                    <Button
                      style="danger"
                      on:click=move |_| {
                        delete.dispatch((i, item.id.clone()));
                      }
                      disabled=disabled
                    >
                      "Delete"
                    </Button>
                  </>
                }
                .into_any()
            })
            .collect_view(),
    };
    let push_available = move || {
        if let Some(user) = &*user.read() {
            if let Ok((Some(source), _)) = list.read().get_unique_source() {
                source == "spotify" && user.spotify_user.is_some()
            } else {
                false
            }
        } else {
            false
        }
    };
    view! {
      <div class="tw:flex tw:flex-col tw:gap-4">
        <div class="tw:flex tw:flex-row-reverse tw:flex-wrap tw:gap-4 tw:justify-end">
          {modal_html}
          {move || {
            list
              .read()
              .iframe
              .clone()
              .map(|src| {
                view! {
                  <iframe
                    width="100%"
                    height="380"
                    prop:frameborder="0"
                    src=src
                    style="flex-basis: 600px"
                  ></iframe>
                }
              })
          }} <form style="flex-basis: 750px">
            <div class="tw:grid tw:gap-x-4" style=style>
              {move || {
                if let ItemMode::Update = mode.get() {
                  Some(
                    view! {
                      <div></div>
                      <p>
                        <strong>"Rating"</strong>
                      </p>
                      <p>
                        <strong>"Hidden"</strong>
                      </p>
                      <p>
                        <strong>"Note"</strong>
                      </p>
                    },
                  )
                } else {
                  None
                }
              }}
              <div class="tw:grid tw:overflow-y-auto tw:gap-y-2 tw:items-baseline" style=grid>
                {html}
              </div>
            </div>
            <Button
              style="primary"
              on:click=move |_| {
                save.dispatch(());
              }
              disabled=disabled
            >
              "Save"
            </Button>
          </form>
        </div>
        <hr class="tw:text-gray-300" />
        <h4>"Data Sources"</h4>
        {source_html}
        {move || {
          if matches!(list.read().mode, ListMode::External) {
            None
          } else {
            Some(
              view! {
                <div>
                  <Button
                    style="primary"
                    on:click=move |_| {
                      push.dispatch(());
                    }
                    disabled=move || !push_available()
                  >
                    "Push"
                  </Button>
                </div>
              },
            )
          }
        }}
      </div>
    }
}

#[component]
fn Rating(
    rating: RwSignal<Option<u64>>,
    mut onchange: impl FnMut(Option<u64>) + 'static,
) -> impl IntoView {
    let rating = rating.get();
    view! {
      <SelectWithCallback on_change=move |ev| {
        onchange(ev.target().value().parse().ok());
      }>
        <option selected=move || rating.is_none()></option>
        <option selected=move || rating == Some(0)>"0"</option>
        <option selected=move || rating == Some(1)>"1"</option>
        <option selected=move || rating == Some(2)>"2"</option>
        <option selected=move || rating == Some(3)>"3"</option>
        <option selected=move || rating == Some(4)>"4"</option>
        <option selected=move || rating == Some(5)>"5"</option>
        <option selected=move || rating == Some(6)>"6"</option>
        <option selected=move || rating == Some(7)>"7"</option>
        <option selected=move || rating == Some(8)>"8"</option>
        <option selected=move || rating == Some(9)>"9"</option>
        <option selected=move || rating == Some(10)>"10"</option>
      </SelectWithCallback>
    }
}
