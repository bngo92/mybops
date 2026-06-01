use crate::{
    base::{Button, SelectWithRef},
    bootstrap::{Accordion, Collapse},
};
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

    let (help, set_help) = signal(logged_in.get());

    crate::nav_content(
        view! {
          <a href="#" class="font-medium">
            {move || if logged_in.get() { "Home" } else { "Demo" }}
          </a>
          <div class="flex gap-4 items-baseline">
            <span class="text-nowrap">"Sort Mode:"</span>
            <SelectWithRef node_ref=select_ref>
              <option>"Tournament"</option>
              <option selected=true>"Random Tournament"</option>
              <option>"Random Matches"</option>
              <option>"Random Rounds"</option>
            </SelectWithRef>
            <Button class="text-white bg-help" on:click=move |_| set_help.set(!help.get())>
              "Help"
            </Button>
          </div>
        },
        move || {
            let Some(lists) = &*lists.read() else {
                return None;
            };
            let disabled = !logged_in.get();
            let column = lists
                .iter()
                .map(|l| {
                    view! { <Widget list=l.clone() select_ref=select_ref /> }
                })
                .collect_view();
            Some(view! {
              <div class="flex flex-col gap-4">
                <Collapse collapsed=help>
                  <p>
                    "mybops is an app that helps you filter your data and remove flops from your life.
                    Use it to gain insights about your favorite songs, TV shows, and even restaurants.
                    mybops makes it easy to rate and/or rank what's important to you."
                  </p>
                  <p>
                    "The data is organized into lists of items and your lists are displayed here on the home page using user-defined widgets.
                    The fastest way to rank your items is with a randomly generated tournament.
                    You can start a tournament for a list by clicking the "
                    <Button style="primary">"Rank"</Button>
                    " button below the list widget. Here is the full list of sort modes:"
                  </p>
                  <ul>
                    <li class="list-disc list-inside">
                      <strong>"Tournament"</strong>
                      " - Sort by choosing between items that are organized using a seeded tournament."
                    </li>
                    <li class="list-disc list-inside">
                      <strong>"Random Tournament"</strong>
                      " - Sort by choosing between items that are organized using a randomly generated tournament."
                    </li>
                    <li class="list-disc list-inside">
                      <strong>"Random Matches"</strong>
                      " - Sort by choosing between randomly selected items."
                    </li>
                    <li class="list-disc list-inside">
                      <strong>"Random Rounds"</strong>
                      " - This mode is similar to Random Matches except every item will be selected before an item is repeated."
                    </li>
                  </ul>
                  <p>
                    "To rate items, go to the item rating page for the list by clicking on the "
                    <Button style="primary">"Rate"</Button>" button."
                  </p>
                  <p>"You can also:"</p>
                  <ul>
                    <li class="list-disc list-inside">
                      "View items in the list by clicking on the widget to expand it."
                    </li>
                    <li class="list-disc list-inside">
                      "Search for data about your ratings and rankings by going to the "
                      <a href="/search">"Search"</a>" page."
                    </li>
                  </ul>
                </Collapse>
                <div class="grid md:grid-cols-2 gap-4">{column}</div>
                <div>
                  <Button
                    class="text-violet-800 bg-purple-200 disabled:bg-purple-200"
                    on:click=move |_| {
                      create.dispatch(());
                    }
                    disabled=disabled
                  >
                    "Create List"
                  </Button>
                </div>
              </div>
            })
        },
    )
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
      <div class="flex flex-col gap-4">
        <Accordion
          header=list.name.clone()
          collapsed=move || Some(collapsed.get())
          on_toggle=Callback::new(move |_| {
            on_toggle.dispatch(());
          })
          px=false
        >
          {move || {
            if let Some(query) = query.get() {
              Either::Left(crate::plot::df_table_view(&query, false).into_any())
            } else {
              Either::Right(view! { <div></div> })
            }
          }}
        </Accordion>
        <div class="flex gap-4">
          <Button
            style="primary"
            on:click=move |_| use_navigate()(&format!("/lists/{}/items", id), Default::default())
            disabled=disabled
          >
            "Rate"
          </Button>
          <Button style="primary" on:click=compare disabled=disabled>
            "Rank"
          </Button>
        </div>
      </div>
    }
}
