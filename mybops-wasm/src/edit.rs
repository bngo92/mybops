use leptos::{
    either::Either,
    html::{Input, Select},
    prelude::*,
};
use leptos_router::hooks::use_navigate;
use mybops::{Id, List, ListMode, Source, SourceType, Spotify};

// TODO: need to refresh list after edit
#[component]
pub fn Edit(
    #[prop(into)] logged_in: Signal<bool>,
    #[prop(into)] list: Signal<List>,
) -> impl IntoView {
    let mut list = list.get();
    let (sources, set_sources) = signal({
        list.sources
            .drain(..)
            .enumerate()
            .map(|(i, s)| {
                (
                    i as i32,
                    NodeRef::<Select>::new(),
                    NodeRef::<Input>::new(),
                    Some(s.source_type),
                )
            })
            .collect::<Vec<_>>()
    });
    let (counter, set_counter) = signal(sources.read().len() as i32);
    let (list, set_list) = signal(list);
    let name_ref = NodeRef::<Input>::new();
    let external_ref = NodeRef::<Input>::new();
    let query_ref = NodeRef::<Input>::new();
    let favorite_ref = NodeRef::<Input>::new();
    let public_ref = NodeRef::<Input>::new();

    let add_source = move |_| {
        set_sources.update(|sources| {
            sources.push((counter.get(), NodeRef::default(), NodeRef::default(), None))
        });
    };
    let delete_source = move |i| {
        set_sources.update(|sources| {
            sources.remove(i);
        })
    };
    let save = Action::new_unsync(move |_| async move {
        let mut list_copy = list.get();
        if !matches!(list_copy.mode, ListMode::External) {
            list_copy.name = name_ref.get().unwrap().value();
        }
        if let ListMode::User(external_id) | ListMode::View(external_id) = &mut list_copy.mode {
            let id = external_ref.get().unwrap().value();
            if id.is_empty() {
                *external_id = None;
            } else if let Some(Spotify::Playlist(id)) = crate::parse_spotify_source(id) {
                *external_id = Some(id);
            }
        }
        list_copy.query = query_ref.get().unwrap().value();
        list_copy.favorite = favorite_ref.get().unwrap().checked();
        list_copy.public = public_ref.get().unwrap().checked();
        list_copy.sources.clear();
        for (_, source, id, _) in &*sources.read() {
            let source = source.get().unwrap().value();
            let id = id.get().unwrap().value();
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
        set_list.set(list_copy.clone());
        crate::update_list(&list_copy).await.unwrap();
    });
    let delete = {
        Action::new_unsync(move |_| async move {
            let id = list.read().id.clone();
            if crate::window()
                .confirm_with_message(&format!("Delete {id}?"))
                .unwrap()
            {
                let navigator = use_navigate();
                let navigator = navigator.clone();
                crate::delete_list(&id).await.unwrap();
                navigator("/", Default::default());
            }
        })
    };
    let delete_all = {
        Action::new_unsync(move |_| async move {
            let id = list.read().id.clone();
            let items: Vec<_> = list.read().items.iter().map(|i| i.id.clone()).collect();
            if crate::window()
                .confirm_with_message(&format!("Delete all items in {id} and list?"))
                .unwrap()
            {
                let navigator = use_navigate();
                crate::delete_items(&items).await.unwrap();
                crate::delete_list(&id).await.unwrap();
                navigator("/", Default::default());
            }
        })
    };

    let disabled = move || !logged_in.get();
    let source_html = move || {
        sources
            .read()
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
                let onclick = move |_| delete_source(i);
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
                view! {
                  <div class="row mb-1">
                    <div class="col-4 col-sm-3 col-md-2">
                      <select node_ref=*source_ref class="form-select">
                        <option selected=selected[0]>"Custom"</option>
                        <option selected=selected[1]>"Spotify"</option>
                        <option selected=selected[2]>"Setlist"</option>
                        <option selected=selected[3]>"List Items"</option>
                      </select>
                    </div>
                    <input class="col-9 col-sm-7 col-md-8" node_ref=*id value=value />
                    <div class="col-auto">
                      <button type="button" class="btn btn-danger" on:click=onclick>
                        "Delete"
                      </button>
                    </div>
                  </div>
                }
            })
            .collect_view()
    };
    let mode = match list.read().mode {
        ListMode::User(_) => "User",
        ListMode::External => "External",
        ListMode::View(_) => "View",
    };
    view! {
      <div>
        <h4>"List Settings"</h4>
        <form class="mb-4" style="max-width: 800px">
          <div class="form-floating mb-2">
            {move || {
              if let ListMode::External = &list.read().mode {
                Either::Left(
                  view! {
                    <input
                      type="text"
                      readonly=true
                      class="form-control-plaintext"
                      id="name"
                      value=list.read().name.clone()
                      placeholder=""
                    />
                  },
                )
              } else {
                Either::Right(
                  view! {
                    <input
                      type="text"
                      class="form-control"
                      id="name"
                      node_ref=name_ref
                      value=list.read().name.clone()
                      placeholder=""
                    />
                  },
                )
              }
            }} <label for="name">"List name"</label>
          </div>
          <div class="form-floating mb-2">
            <input
              type="text"
              readonly=true
              class="form-control-plaintext"
              id="mode"
              value=mode
              placeholder=""
            />
            <label for="mode">"List mode"</label>
          </div>
          {move || {
            if let ListMode::User(external_id) | ListMode::View(external_id) = &list.read().mode {
              Some(
                view! {
                  <div class="form-floating mb-3">
                    <input
                      class="form-control"
                      id="externalId"
                      node_ref=external_ref
                      placeholder="External ID"
                      value=external_id.as_ref().map(|i| i.raw_id.clone()).unwrap_or_default()
                    />
                    <label for="externalId">"External ID"</label>
                  </div>
                },
              )
            } else {
              None
            }
          }}
          <div class="form-floating mb-3">
            <input
              class="form-control"
              id="query"
              node_ref=query_ref
              placeholder="External ID"
              value=list.get().query
            />
            <label for="query">"Query"</label>
          </div>
          <div class="form-check">
            <label class="form-check-label" for="favorite">
              "Favorite"
            </label>
            <input
              node_ref=favorite_ref
              class="form-check-input"
              type="checkbox"
              id="favorite"
              checked=list.read().favorite
            />
          </div>
          <div class="form-check">
            <label class="form-check-label" for="public">
              "Public"
            </label>
            <input
              node_ref=public_ref
              class="form-check-input"
              type="checkbox"
              id="public"
              checked=list.read().public
            />
          </div>
        </form>
        <h4>"Data Sources"</h4>
        <div class="mb-3">{source_html}</div>
        <div class="d-flex gap-3">
          <button type="button" class="btn btn-primary" on:click=add_source>
            "Add source"
          </button>
        </div>
        <hr />
        <button
          type="button"
          class="btn btn-success mb-3"
          on:click=move |_| {
            save.dispatch(());
          }
          disabled=disabled
        >
          "Save all settings"
        </button>
        <div class="d-flex gap-3">
          <button
            type="button"
            class="btn btn-danger"
            on:click=move |_| {
              delete.dispatch(());
            }
            disabled=disabled
          >
            "Delete"
          </button>
          <button
            type="button"
            class="btn btn-danger"
            on:click=move |_| {
              delete_all.dispatch(());
            }
            disabled=disabled
          >
            "Delete All"
          </button>
        </div>
      </div>
    }
}
