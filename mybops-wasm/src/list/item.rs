use crate::{
    ListsRoute,
    bootstrap::{Alert, Modal},
};
use arrow::{array::AsArray, datatypes::UInt64Type};
use js_sys::Error;
use mybops::{Id, ItemMetadata, List, ListMode, SourceType, Spotify, User};
use serde_json::Value;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    HtmlInputElement, HtmlSelectElement, Request, RequestInit, RequestMode, Response, Url,
};
use yew::{
    Callback, Html, HtmlResult, NodeRef, Properties, function_component, html,
    suspense::use_future, use_node_ref, use_state,
};
use yew_router::prelude::Link;

#[derive(PartialEq, Properties)]
pub struct ListProps {
    pub user: Rc<Option<User>>,
    pub list: List,
    pub mode: ItemMode,
}

#[derive(Clone)]
struct ListItem {
    item: ItemMetadata,
    hidden_ref: NodeRef,
    note_ref: NodeRef,
}

#[derive(Clone, Default)]
struct State {
    rating: Option<u64>,
    hidden: bool,
    note: String,
}

#[derive(Clone, PartialEq)]
pub enum ItemMode {
    View,
    Update,
    Delete,
}

#[function_component]
pub fn ListItems(ListProps { user, list, mode }: &ListProps) -> HtmlResult {
    let list_copy = list.clone();
    let items = use_state(|| {
        list.items
            .iter()
            .map(|i| ListItem {
                item: i.clone(),
                hidden_ref: NodeRef::default(),
                note_ref: NodeRef::default(),
            })
            .collect::<Vec<_>>()
    });
    let prev_state = use_state(|| None);
    let state = use_state(|| None);
    let alert = use_state(|| None);
    let modal = use_state(|| None);
    {
        let items = items.clone();
        let prev_state = prev_state.clone();
        let state = state.clone();
        use_future(|| async move {
            if !matches!(list_copy.mode, ListMode::View(_)) {
                let query = crate::query_list(
                    &list_copy,
                    Some("SELECT id, rating, hidden, note FROM item".to_owned()),
                )
                .await
                .unwrap();
                if let Some(query) = query {
                    let index: HashMap<_, _> = items
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
                    let mut new_state = vec![State::default(); items.len()];
                    for (((id, &rating), hidden), note) in ids
                        .iter()
                        .zip(ratings.iter())
                        .zip(hidden.iter())
                        .zip(note.iter())
                    {
                        new_state[index[id.unwrap()]] = State {
                            rating,
                            hidden: hidden.unwrap(),
                            note: note.unwrap().to_owned(),
                        };
                    }
                    prev_state.set(Some(new_state.clone()));
                    state.set(Some(new_state.clone()));
                }
            }
        })?;
    }

    let update_rating = {
        let state = state.clone();
        Callback::from(move |(i, rating): (usize, Option<u64>)| {
            let mut state_copy = (*state).clone();
            state_copy.as_mut().unwrap()[i].rating = rating;
            state.set(state_copy);
        })
    };
    let save = {
        let items = items.clone();
        let prev_state = prev_state.clone();
        let state = state.clone();
        let alert = alert.clone();
        Callback::from(move |_| {
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
            ) in items.iter().zip(state.as_ref().unwrap().iter()).enumerate()
            {
                let State { rating, hidden, .. } = rating_hidden;
                let mut updates = HashMap::new();
                if prev_state.as_ref().unwrap()[i].rating != *rating {
                    updates.insert(String::from("rating"), (*rating).into());
                }
                let value = Value::Bool(hidden_ref.cast::<HtmlInputElement>().unwrap().checked());
                #[allow(clippy::cmp_owned)]
                if value != *hidden {
                    updates.insert(String::from("hidden"), value);
                }
                let value = note_ref.cast::<HtmlInputElement>().unwrap().value();
                if prev_state.as_ref().unwrap()[i].note != value {
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
                let updates = JsValue::from_str(&serde_json::to_string(&update_ids).unwrap());
                opts.set_body(&updates);
                let request =
                    Request::new_with_str_and_init("/api/?action=updateItems", &opts).unwrap();
                request
                    .headers()
                    .set("Content-Type", "application/json")
                    .unwrap();
                let state = state.clone();
                let prev_state = prev_state.clone();
                let alert = alert.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(window.fetch_with_request(&request)).await {
                        Ok(resp) => {
                            let resp_value: Response = resp.dyn_into().unwrap();
                            if resp_value.status() >= 400 {
                                alert.set(Some(Err(JsFuture::from(resp_value.text().unwrap())
                                    .await
                                    .unwrap()
                                    .as_string()
                                    .unwrap())));
                            } else {
                                // Update the rating and hidden state values if the save request is successful.
                                let mut state_copy = (*state).clone();
                                for (i, update) in update_indexes {
                                    for (k, v) in update {
                                        let State {
                                            rating,
                                            hidden,
                                            note,
                                        } = state_copy.as_mut().unwrap().get_mut(i).unwrap();
                                        match k.as_str() {
                                            "rating" => {
                                                *rating = v.as_u64();
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
                                state.set(state_copy.clone());
                                prev_state.set(state_copy);
                                alert.set(Some(Ok("Save successful".to_owned())));
                            }
                        }
                        Err(e) => {
                            alert.set(Some(Err(e.dyn_into::<Error>().unwrap().to_string().into())))
                        }
                    }
                });
            }
        })
    };
    let push = {
        let id = list.id.clone();
        Callback::from(move |_| {
            let id = id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                crate::push_list(&id).await.unwrap();
            });
        })
    };
    let open = {
        let modal = modal.clone();
        Callback::from(move |i| modal.set(Some(i)))
    };
    let modal_back = {
        let items = items.len();
        let modal = modal.clone();
        Callback::from(move |i| modal.set(Some(if i == 0 { items - 1 } else { i - 1 })))
    };
    let modal_forward = {
        let items = items.len();
        let modal = modal.clone();
        Callback::from(move |i| modal.set(Some(if i == items - 1 { 0 } else { i + 1 })))
    };
    let hide_modal = {
        let modal = modal.clone();
        Callback::from(move |_| modal.set(None))
    };
    let delete = {
        let delete_success = {
            let items = items.clone();
            let state = state.clone();
            move |i| {
                let mut items_copy = (*items).clone();
                items_copy.remove(i);
                items.set(items_copy);
                let mut state_copy = (*state).clone();
                if let Some(state) = state_copy.as_mut() {
                    state.remove(i);
                }
                state.set(state_copy);
            }
        };
        Callback::from(move |(i, id): (usize, String)| {
            let id = id.clone();
            let delete_success = delete_success.clone();
            wasm_bindgen_futures::spawn_local(async move {
                crate::delete_items(&[id]).await.unwrap();
                delete_success(i)
            });
        })
    };

    let disabled = user.is_none() || !crate::user_list(list, user);
    let modal_html = if let Some(i) = *modal {
        let item = &items[i];
        let onchange = {
            let update_rating = update_rating.clone();
            Callback::from(move |rating| update_rating.emit((i, rating)))
        };
        html! {
          <Modal header={item.item.name.clone()} hide={hide_modal}>
            <div class="carousel slide">
              <div class="carousel-item active">
                if let Some(iframe) = &item.item.iframe {
                  <iframe width="100%" height="380" frameborder="0" src={iframe.clone()}></iframe>
                }
              </div>
              <button class="carousel-control-prev" type="button" onclick={Callback::from(move |_| modal_back.emit(i))} style="top: 56px; bottom: auto; height: 137px">
                <span class="carousel-control-prev-icon"></span>
              </button>
              <button class="carousel-control-next" type="button" onclick={Callback::from(move |_| modal_forward.emit(i))} style="top: 56px; bottom: auto; height: 137px">
                <span class="carousel-control-next-icon"></span>
              </button>
            </div>
            if let Some(state) = state.as_ref() {
              <div class="col-2">
                <Rating rating={state[i].rating} {onchange}/>
              </div>
            }
          </Modal>
        }
    } else {
        html! {}
    };
    let source_html = list.sources.iter().map(|source| {
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
        html! {
            if let SourceType::ListItems(id) = &source.source_type {
                <div class="mb-2"><Link<ListsRoute> to={ListsRoute::View { id: id.clone() }}>{&source.name}</Link<ListsRoute>></div>
            } else if let Some(href) = raw_id {
                <div class="mb-2"><a {href}>{&source.name}</a></div>
            } else {
                <p class="mb-2">{&source.name}</p>
            }
        }
    });
    let (style, grid) = match mode {
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
    let html: Html = match mode {
        ItemMode::View => items
            .iter()
            .enumerate()
            .map(|(i, ListItem { item, .. })| {
                let open = open.clone();
                html! {
                    <label class="col-form-label"><a href="#" onclick={Callback::from(move |_| open.emit(i))}>{&item.name}</a></label>
                }
            })
            .collect(),
        ItemMode::Update => items
            .iter()
            .enumerate()
            .map(
                |(i, ListItem {
                    item,
                    hidden_ref,
                    note_ref,
                })| {
                    let open = open.clone();
                    let label = html! {
                        <label class="col-form-label"><a href="#" onclick={Callback::from(move |_| open.emit(i))}>{&item.name}</a></label>
                    };
                    if let Some(ref state_copy) = *state {
                        let State { rating, hidden, note } = state_copy[i].clone();
                        let update_rating = update_rating.clone();
                        html! {
                            <>
                                {label}
                                <div>
                                    <Rating {rating} onchange={Callback::from(move |rating| update_rating.emit((i, rating)))}/>
                                </div>
                                <div class="d-flex justify-content-center">
                                    <input ref={hidden_ref} class="form-check-input mt-2" type="checkbox" checked={hidden} {disabled}/>
                                </div>
                                <div>
                                    <input ref={note_ref} class="form-control" value={Some(note.clone())} {disabled}/>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                {label}
                                <div></div>
                                <div></div>
                                <div></div>
                            </>
                        }
                    }
                },
            )
            .collect(),
        ItemMode::Delete => items
            .iter()
            .enumerate()
            .map(|(i, ListItem { item, .. })| {
                let open = open.clone();
                let delete = {
                    let delete = delete.clone();
                    let id = item.id.clone();
                    Callback::from(move |_| delete.emit((i, id.clone())))
                };
                html! {
                    <>
                        <label class="col-form-label"><a href="#" onclick={Callback::from(move |_| open.emit(i))}>{&item.name}</a></label>
                        <button type="button" class="btn btn-danger" onclick={delete} {disabled}>{"Delete"}</button>
                    </>
                }
            })
            .collect(),
        };
    let push_available = if let Some(user) = &**user {
        if let Ok((Some(source), _)) = list.get_unique_source() {
            source == "spotify" && user.spotify_user.is_some()
        } else {
            false
        }
    } else {
        false
    };
    let hide = {
        let alert = alert.clone();
        Callback::from(move |_| alert.set(None))
    };
    Ok(html! {
        <div>
            <div class="d-flex flex-row-reverse flex-wrap justify-content-end row-gap-3 column-gap-5">
                {modal_html}
                if let Some(src) = list.iframe.clone() {
                    <iframe width="100%" height="380" frameborder="0" {src} style="flex-basis: 600px"></iframe>
                }
                <form style="flex-basis: 750px">
                    <div class="d-grid row-gap-1 column-gap-3 mb-3" {style}>
                        if let ItemMode::Update = mode {
                            <div></div>
                            <p><strong>{"Rating"}</strong></p>
                            <p><strong>{"Hidden"}</strong></p>
                            <p><strong>{"Note"}</strong></p>
                        }
                        <div class="d-grid row-gap-1 overflow-y-auto" style={grid}>
                            {html}
                        </div>
                    </div>
                    if let Some(result) = (*alert).clone() {
                        <button type="button" class="btn btn-success mb-3" onclick={save} {disabled}>{"Save"}</button>
                        <Alert {result} {hide}/>
                    } else {
                        <button type="button" class="btn btn-success" onclick={save} {disabled}>{"Save"}</button>
                    }
                </form>
            </div>
            <hr/>
            <h4>{"Data Sources"}</h4>
            {for source_html}
            if !matches!(list.mode, ListMode::External) {
                <button type="button" class="btn btn-success" onclick={push} disabled={!push_available}>{"Push"}</button>
            }
        </div>
    })
}

#[derive(PartialEq, Properties)]
struct RatingProps {
    rating: Option<u64>,
    onchange: Callback<Option<u64>>,
}

#[function_component]
fn Rating(RatingProps { rating, onchange }: &RatingProps) -> Html {
    let select_ref = use_node_ref();
    let onchange = {
        let onchange = onchange.clone();
        let select_ref = select_ref.clone();
        Callback::from(move |_| {
            onchange.emit(
                select_ref
                    .cast::<HtmlSelectElement>()
                    .unwrap()
                    .value()
                    .parse()
                    .ok(),
            )
        })
    };
    html! {
        <select ref={select_ref} {onchange} class="form-select">
            <option selected={rating.is_none()}></option>
            <option selected={*rating == Some(0)}>{"0"}</option>
            <option selected={*rating == Some(1)}>{"1"}</option>
            <option selected={*rating == Some(2)}>{"2"}</option>
            <option selected={*rating == Some(3)}>{"3"}</option>
            <option selected={*rating == Some(4)}>{"4"}</option>
            <option selected={*rating == Some(5)}>{"5"}</option>
            <option selected={*rating == Some(6)}>{"6"}</option>
            <option selected={*rating == Some(7)}>{"7"}</option>
            <option selected={*rating == Some(8)}>{"8"}</option>
            <option selected={*rating == Some(9)}>{"9"}</option>
            <option selected={*rating == Some(10)}>{"10"}</option>
        </select>
    }
}
