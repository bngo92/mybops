use crate::bootstrap::{Accordion, Collapse};
use leptos::{either::Either, html::Select, prelude::*};
use leptos_router::hooks::use_navigate;
use mybops::{List, ListMode};
use wasm_bindgen::JsValue;

#[component]
pub fn Home(logged_in: RwSignal<bool>) -> impl IntoView {
    let select_ref = NodeRef::<Select>::new();
    let lists = LocalResource::new(|| async { fetch_lists().await });

    let create = Action::new_unsync(move |_| async move {
        let list = crate::create_list(None).await.unwrap();
        let navigator = use_navigate();
        navigator(&format!("/lists/{}/edit", list.id), Default::default());
    });

    move || {
        let Some(lists) = &*lists.read() else {
            return None;
        };
        let disabled = !logged_in.get();
        let mut column = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (l, &r) in lists.iter().zip([false, true].iter().cycle()) {
            column.push(view! { <Widget list=l.clone() select_ref=select_ref /> });
            if r {
                right.push(view! { <Widget list=l.clone() select_ref=select_ref /> });
            } else {
                left.push(view! { <Widget list=l.clone() select_ref=select_ref /> });
            }
        }
        Some(crate::nav_content(
        view! {
          <>
            <ul class="navbar-nav me-auto">
              <li class="navbar-brand">{move || if disabled { "Demo" } else { "Home" }}</li>
            </ul>
            <div class="d-flex gap-3 align-items-baseline">
              <span class="navbar-text text-nowrap">"Sort Mode:"</span>
              <select node_ref=select_ref class="form-select">
                <option>"Tournament"</option>
                <option selected=true>"Random Tournament"</option>
                <option>"Random Matches"</option>
                <option>"Random Rounds"</option>
              </select>
              <button class="btn btn-info" on:click=move |_| logged_in.set(!logged_in.get())>
                "Help"
              </button>
            </div>
          </>
        }.into_any(),
        view! {
          <div>
            <Collapse collapsed=logged_in>
              <p>
                "mybops is an app that helps you filter your data and remove flops from your life.
                      Use it to gain insights about your favorite songs, TV shows, and even restaurants.
                      mybops makes it easy to rate and/or rank what's important to you."
              </p>
              <p>
                "The data is organized into lists of items and your lists are displayed here on the home page using user-defined widgets.
                      The fastest way to rank your items is with a randomly generated tournament.
                      You can start a tournament for a list by clicking the "
                <button type="button" class="btn btn-success btn-sm">
                  "Rank"
                </button> " button below the list widget. Here is the full list of sort modes:"
              </p>
              <ul>
                <li>
                  <strong>"Tournament"</strong>
                  " - Sort by choosing between items that are organized using a seeded tournament."
                </li>
                <li>
                  <strong>"Random Tournament"</strong>
                  " - Sort by choosing between items that are organized using a randomly generated tournament."
                </li>
                <li>
                  <strong>"Random Matches"</strong>
                  " - Sort by choosing between randomly selected items."
                </li>
                <li>
                  <strong>"Random Rounds"</strong>
                  " - This mode is similar to Random Matches except every item will be selected before an item is repeated."
                </li>
              </ul>
              <p>
                "To rate items, go to the item rating page for the list by clicking on the "
                <button type="button" class="btn btn-success btn-sm">
                  "Rate"
                </button>" button."
              </p>
              <p>"You can also:"</p>
              <ul class="mb-0">
                <li>"View items in the list by clicking on the widget to expand it."</li>
                <li>
                  "Search for data about your ratings and rankings by going to the "
                  <a href="/search">"Search"</a>" page."
                </li>
              </ul>
            </Collapse>
            <div class="mt-3">
              <div class="d-md-none">
                {column}
                <button
                  type="button"
                  class="btn btn-primary"
                  on:click=move |_| {
                    create.dispatch(());
                  }
                  disabled=disabled
                >
                  "Create List"
                </button>
              </div>
              <div class="d-none d-md-block">
                <div class="d-grid gap-3" style="grid-template-columns: 1fr 1fr">
                  <div>
                    {left}
                    <button
                      type="button"
                      class="btn btn-primary"
                      on:click=move |_| {
                        create.dispatch(());
                      }
                      disabled=disabled
                    >
                      "Create List"
                    </button>
                  </div>
                  <div>{right}</div>
                </div>
              </div>
            </div>
          </div>
        }.into_any(),
    ))
    }
}

async fn fetch_lists() -> Vec<List> {
    crate::fetch_lists(true).await.unwrap()
}

#[component]
fn Widget(list: List, select_ref: NodeRef<Select>) -> impl IntoView {
    let (collapsed, set_collapsed) = signal(true);
    let (query, set_query) = signal(None);

    let on_toggle = {
        let list = list.clone();
        let query = query.clone();
        let collapsed = collapsed.clone();
        Action::new_unsync(move |_| {
            let list = list.clone();
            async move {
                if query.read().is_none() {
                    set_query.set(crate::query_list(&list, None).await.unwrap());
                    set_collapsed.set(false);
                } else {
                    set_collapsed.set(!collapsed.get());
                }
            }
        })
    };
    let navigator = use_navigate();
    let id = list.id.clone();
    let compare = move |_| {
        let id = id.clone();
        let mode = select_ref.get().unwrap().value();
        match mode.as_ref() {
            "Random Matches" => {
                navigator(&format!("/lists/{}/match", id), Default::default());
            }
            "Random Rounds" => {
                navigator(
                    &format!("/lists/{}/match?mode=random", id),
                    Default::default(),
                );
            }
            "Tournament" => {
                navigator(&format!("/lists/{}/tournament", id), Default::default());
            }
            "Random Tournament" => {
                navigator(
                    &format!("/lists/{}/tournament?mode=random", id),
                    Default::default(),
                );
            }
            _ => {
                web_sys::console::log_1(&JsValue::from("Invalid mode"));
            }
        };
    };
    let id = list.id.clone();
    // TODO: support actions on views
    let disabled = matches!(list.mode, ListMode::View(_));
    view! {
      <>
        <Accordion
          header=list.name.clone()
          collapsed=move || Some(collapsed.get())
          on_toggle=Some(
            Box::new(move |_| {
              on_toggle.dispatch(());
            }),
          )
        >
          {move || {
            if let Some(query) = query.get() {
              Either::Left(crate::plot::df_table_view(&query, false).into_any())
            } else {
              Either::Right(view! { <div></div> })
            }
          }}
        </Accordion>
        <div class="row mb-3">
          <div class="col-auto">
            <button
              type="button"
              class="btn btn-success"
              on:click=move |_| use_navigate()(&format!("/lists/{}/items", id), Default::default())
              disabled=disabled
            >
              "Rate"
            </button>
          </div>
          <div class="col-auto">
            <button type="button" class="btn btn-success" on:click=compare disabled=disabled>
              "Rank"
            </button>
          </div>
        </div>
      </>
    }
}
