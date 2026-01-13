use mybops::{Id, List, ListMode, Source, SourceType, Spotify};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{Callback, Html, NodeRef, Properties, function_component, html, use_node_ref, use_state};
use yew_router::hooks::use_navigator;

use crate::Route;

// TODO: need to refresh list after edit
#[derive(Eq, PartialEq, Properties)]
pub struct EditProps {
    pub logged_in: bool,
    pub list: List,
}

#[function_component]
pub fn Edit(EditProps { logged_in, list }: &EditProps) -> Html {
    let mut list = list.clone();
    let sources = use_state(|| {
        list.sources
            .drain(..)
            .enumerate()
            .map(|(i, s)| {
                (
                    i as i32,
                    NodeRef::default(),
                    NodeRef::default(),
                    Some(s.source_type),
                )
            })
            .collect::<Vec<_>>()
    });
    let counter = use_state(|| sources.len() as i32);
    let list = use_state(|| list);
    let name_ref = use_node_ref();
    let external_ref = use_node_ref();
    let query_ref = use_node_ref();
    let favorite_ref = use_node_ref();
    let public_ref = use_node_ref();
    let navigator = use_navigator();

    let add_source = {
        let sources = sources.clone();
        Callback::from(move |_| {
            let mut sources_copy = (*sources).clone();
            sources_copy.push((*counter, NodeRef::default(), NodeRef::default(), None));
            sources.set(sources_copy);
            counter.set(*counter + 1);
        })
    };
    let delete_source = {
        let sources = sources.clone();
        Callback::from(move |i| {
            let mut sources_copy = (*sources).clone();
            sources_copy.remove(i);
            sources.set(sources_copy);
        })
    };
    let save = {
        let sources = sources.clone();
        let list = list.clone();
        let name_ref = name_ref.clone();
        let external_ref = external_ref.clone();
        let query_ref = query_ref.clone();
        let favorite_ref = favorite_ref.clone();
        let public_ref = public_ref.clone();
        Callback::from(move |_| {
            let mut list_copy = (*list).clone();
            if !matches!(list_copy.mode, ListMode::External) {
                list_copy.name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            }
            if let ListMode::User(external_id) | ListMode::View(external_id) = &mut list_copy.mode {
                let id = external_ref.cast::<HtmlInputElement>().unwrap().value();
                if id.is_empty() {
                    *external_id = None;
                } else if let Some(Spotify::Playlist(id)) = crate::parse_spotify_source(id) {
                    *external_id = Some(id);
                }
            }
            list_copy.query = query_ref.cast::<HtmlInputElement>().unwrap().value();
            list_copy.favorite = favorite_ref.cast::<HtmlInputElement>().unwrap().checked();
            list_copy.public = public_ref.cast::<HtmlInputElement>().unwrap().checked();
            list_copy.sources.clear();
            for (_, source, id, _) in &*sources {
                let source = source.cast::<HtmlSelectElement>().unwrap().value();
                let id = id.cast::<HtmlInputElement>().unwrap().value();
                match &*source {
                    "Spotify" => {
                        if let Some(source) = crate::parse_spotify_source(id) {
                            list_copy.sources.push(Source {
                                source_type: SourceType::Spotify(source),
                                name: String::new(),
                            });
                        } else {
                            return;
                        }
                    }
                    "Custom" => {
                        if let Ok(json) = serde_json::from_str(&id) {
                            list_copy.sources.push(Source {
                                source_type: SourceType::Custom(json),
                                name: String::new(),
                            });
                        } else {
                            return;
                        }
                    }
                    "Setlist" => {
                        if let Some(id) = crate::parse_setlist_source(id) {
                            list_copy.sources.push(Source {
                                source_type: SourceType::Setlist(id),
                                name: String::new(),
                            });
                        } else {
                            return;
                        }
                    }
                    "List Items" => {
                        list_copy.sources.push(Source {
                            source_type: SourceType::ListItems(id),
                            name: String::new(),
                        });
                    }
                    _ => {
                        return;
                    }
                };
            }
            list.set(list_copy.clone());
            wasm_bindgen_futures::spawn_local(async move {
                crate::update_list(&list_copy).await.unwrap();
            });
        })
    };
    let delete = {
        let list = list.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let id = list.id.clone();
            if crate::window()
                .confirm_with_message(&format!("Delete {id}?"))
                .unwrap()
            {
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    crate::delete_list(&id).await.unwrap();
                    navigator.unwrap().push(&Route::Home);
                });
            }
        })
    };
    let delete_all = {
        let list = list.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let id = list.id.clone();
            let items: Vec<_> = list.items.iter().map(|i| i.id.clone()).collect();
            if crate::window()
                .confirm_with_message(&format!("Delete all items in {id} and list?"))
                .unwrap()
            {
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    crate::delete_items(&items).await.unwrap();
                    crate::delete_list(&id).await.unwrap();
                    navigator.unwrap().push(&Route::Home);
                });
            }
        })
    };

    let disabled = !logged_in;
    let source_html = sources
        .iter()
        .enumerate()
        .map(|(i, (key, source_ref, id, source))| {
            let mut selected = [false; 4];
            match source {
                None => selected[1] = true,
                Some(SourceType::Custom(_)) => selected[0] = true,
                Some(SourceType::Spotify(_)) => selected[1] = true,
                Some(SourceType::Setlist(_)) => selected[2] = true,
                Some(SourceType::ListItems(_)) => selected[3] = true,
            };
            let delete_source = delete_source.clone();
            let onclick = Callback::from(move |_| delete_source.emit(i));
            let value = match source {
                None => String::new(),
                Some(SourceType::Custom(value)) => value.to_string(),
                Some(
                    SourceType::Spotify(Spotify::Playlist(Id { raw_id, .. }))
                    | SourceType::Spotify(Spotify::Album(Id { raw_id, .. }))
                    | SourceType::Spotify(Spotify::Track(Id { raw_id, .. })),
                ) => raw_id.clone(),
                Some(SourceType::Setlist(Id { raw_id, .. })) => raw_id.clone(),
                Some(SourceType::ListItems(id)) => id.clone(),
            };
            html! {
                <div class="row mb-1" key={*key}>
                    <div class="col-4 col-sm-3 col-md-2">
                        <select ref={source_ref} class="form-select">
                            <option selected={selected[0]}>{"Custom"}</option>
                            <option selected={selected[1]}>{"Spotify"}</option>
                            <option selected={selected[2]}>{"Setlist"}</option>
                            <option selected={selected[3]}>{"List Items"}</option>
                        </select>
                    </div>
                    <input class="col-9 col-sm-7 col-md-8" ref={id} {value}/>
                    <div class="col-auto">
                        <button type="button" class="btn btn-danger" {onclick}>{"Delete"}</button>
                    </div>
                </div>
            }
        });
    let mode = match list.mode {
        ListMode::User(_) => "User",
        ListMode::External => "External",
        ListMode::View(_) => "View",
    };
    html! {
        <div>
            <h4>{"List Settings"}</h4>
            <form class="mb-4" style="max-width: 800px">
                <div class="form-floating mb-2">
                    if let ListMode::External = &list.mode {
                        <input type="text" readonly=true class="form-control-plaintext" id="name" value={list.name.clone()} placeholder=""/>
                    } else {
                        <input type="text" class="form-control" id="name" ref={&name_ref} value={list.name.clone()} placeholder=""/>
                    }
                    <label for="name">{"List name"}</label>
                </div>
                <div class="form-floating mb-2">
                    <input type="text" readonly=true class="form-control-plaintext" id="mode" value={mode} placeholder=""/>
                    <label for="mode">{"List mode"}</label>
                </div>
                if let ListMode::User(external_id) | ListMode::View(external_id) = &list.mode {
                    <div class="form-floating mb-3">
                        <input class="form-control" id="externalId" ref={&external_ref} placeholder="External ID" value={external_id.as_ref().map(|i| i.raw_id.clone()).unwrap_or_default()}/>
                        <label for="externalId">{"External ID"}</label>
                    </div>
                }
                <div class="form-floating mb-3">
                    <input class="form-control" id="query" ref={&query_ref} placeholder="External ID" value={list.query.clone()}/>
                    <label for="query">{"Query"}</label>
                </div>
                <div class="form-check">
                    <label class="form-check-label" for="favorite">{"Favorite"}</label>
                    <input ref={&favorite_ref} class="form-check-input" type="checkbox" id="favorite" checked={list.favorite}/>
                </div>
                <div class="form-check">
                    <label class="form-check-label" for="public">{"Public"}</label>
                    <input ref={&public_ref} class="form-check-input" type="checkbox" id="public" checked={list.public}/>
                </div>
            </form>
            <h4>{"Data Sources"}</h4>
            <div class="mb-3">
                {for source_html}
            </div>
            <div class="d-flex gap-3">
                <button type="button" class="btn btn-primary" onclick={add_source}>{"Add source"}</button>
            </div>
            <hr/>
            <button type="button" class="btn btn-success mb-3" onclick={save} {disabled}>{"Save all settings"}</button>
            <div class="d-flex gap-3">
                <button type="button" class="btn btn-danger" onclick={delete} {disabled}>{"Delete"}</button>
                <button type="button" class="btn btn-danger" onclick={delete_all} {disabled}>{"Delete All"}</button>
            </div>
        </div>
    }
}
