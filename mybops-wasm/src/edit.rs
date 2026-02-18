use leptos::{
    either::Either,
    html::{Input, Select},
    prelude::*,
};
use leptos_router::hooks::use_navigate;
use mybops::{Id, List, ListMode, Source, SourceType, Spotify};

use crate::base::{Button, INPUT_STYLE, READONLY_INPUT_STYLE, SelectWithRef};

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
                  <SelectWithRef node_ref=*source_ref>
                    <option selected=selected[0]>"Custom"</option>
                    <option selected=selected[1]>"Spotify"</option>
                    <option selected=selected[2]>"Setlist"</option>
                    <option selected=selected[3]>"List Items"</option>
                  </SelectWithRef>
                  <input class=INPUT_STYLE node_ref=*id value=value />
                  <Button class="tw:text-white tw:bg-red-500" on:click=onclick>
                    "Delete"
                  </Button>
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
        <div class="tw:flex tw:flex-col tw:gap-6">
          <form class="tw:flex tw:flex-col tw:gap-4 tw:max-w-3xl">
            <FormInput
              id="name"
              label="List name"
              input=move || {
                if let ListMode::External = &list.read().mode {
                  Either::Left(
                    view! {
                      <input
                        type="text"
                        readonly=true
                        class=READONLY_INPUT_STYLE
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
                        class=INPUT_STYLE
                        node_ref=name_ref
                        value=list.read().name.clone()
                        placeholder=""
                      />
                    },
                  )
                }
              }
            />
            <FormInput
              id="mode"
              label="List mode"
              input=view! {
                <input
                  type="text"
                  readonly=true
                  class=READONLY_INPUT_STYLE
                  value=mode
                  placeholder=""
                />
              }
            />
            {move || {
              if let ListMode::User(external_id) | ListMode::View(external_id) = &list.read().mode {
                Some(
                  view! {
                    <FormInput
                      id="externalId"
                      label="External ID"
                      input=view! {
                        <input
                          class=INPUT_STYLE
                          node_ref=external_ref
                          placeholder=""
                          value=external_id.as_ref().map(|i| i.raw_id.clone()).unwrap_or_default()
                        />
                      }
                    />
                  },
                )
              } else {
                None
              }
            }}
            <FormInput
              id="query"
              label="Query"
              input=view! {
                <input class=INPUT_STYLE node_ref=query_ref placeholder="" value=list.get().query />
              }
            />
            <div>
              <div class="tw:flex tw:items-center tw:gap-2">
                <input
                  node_ref=favorite_ref
                  class="tw:my-1! tw:size-4"
                  type="checkbox"
                  id="favorite"
                  checked=list.read().favorite
                />
                <label for="favorite">"Favorite"</label>
              </div>
              <div class="tw:flex tw:items-center tw:gap-2">
                <input
                  node_ref=public_ref
                  class="tw:my-1! tw:size-4"
                  type="checkbox"
                  id="public"
                  checked=list.read().public
                />
                <label for="public">"Public"</label>
              </div>
            </div>
          </form>
          <div>
            <h5>"Data Sources"</h5>
            <div class="tw:grid tw:grid-cols-[10rem_minmax(auto,calc(var(--tw-container-3xl)-10rem-1rem))_min-content] tw:gap-4 mb-3">
              {source_html}
            </div>
            <Button class="tw:text-white tw:bg-blue-500" on:click=add_source>
              "Add source"
            </Button>
          </div>
        </div>
        <hr />
        <Button
          class="tw:text-white tw:bg-primary tw:mb-3!"
          on:click=move |_| {
            save.dispatch(());
          }
          disabled=disabled
        >
          "Save all settings"
        </Button>
        <div class="tw:flex tw:gap-4">
          <Button
            class="tw:text-white tw:bg-red-500"
            on:click=move |_| {
              delete.dispatch(());
            }
            disabled=disabled
          >
            "Delete"
          </Button>
          <Button
            class="tw:text-white tw:bg-red-500"
            on:click=move |_| {
              delete_all.dispatch(());
            }
            disabled=disabled
          >
            "Delete All"
          </Button>
        </div>
      </div>
    }
}

#[component]
fn FormInput(id: &'static str, label: &'static str, input: impl IntoView) -> impl IntoView {
    view! {
      <div class="tw:flex tw:flex-wrap tw:items-baseline tw:gap-x-4">
        <label class="tw:basis-[10rem]" for=id>
          {label}
        </label>
        <div class="tw:basis-[calc(var(--tw-container-3xl)-10rem-1rem)] tw:flex-1">
          <FormInputInner input=input prop:id=id />
        </div>
      </div>
    }
}

#[component]
fn FormInputInner(input: impl IntoView) -> impl IntoView {
    input
}
