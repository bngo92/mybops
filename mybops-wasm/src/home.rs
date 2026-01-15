use std::{collections::HashMap, rc::Rc};

use crate::{
    ListMode, ListsRoute, Route, UserProps,
    bootstrap::{Accordion, Collapse},
};
use mybops::List;
use wasm_bindgen::JsValue;
use web_sys::HtmlSelectElement;
use yew::{
    Callback, Html, HtmlResult, NodeRef, Properties, function_component, html,
    suspense::use_future, use_node_ref, use_state,
};
use yew_router::{hooks::use_navigator, prelude::Link};

#[function_component]
pub fn Home(UserProps { logged_in }: &UserProps) -> HtmlResult {
    let select_ref = use_node_ref();
    let lists = &*use_future(|| async { fetch_lists().await })?;
    let help_collapsed = use_state(|| *logged_in);
    let navigator = use_navigator();

    let toggle_help = {
        let help_collapsed = help_collapsed.clone();
        Callback::from(move |_| {
            help_collapsed.set(!*help_collapsed);
        })
    };
    let create = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let list = crate::create_list(None).await.unwrap();
                navigator.unwrap().push(&ListsRoute::Edit { id: list.id });
            });
        })
    };

    let disabled = !logged_in;
    let mut column = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (l, &r) in lists.iter().zip([false, true].iter().cycle()) {
        column.push(html! {<Widget list={l.clone()} select_ref={select_ref.clone()}/>});
        if r {
            right.push(html! {<Widget list={l.clone()} select_ref={select_ref.clone()}/>});
        } else {
            left.push(html! {<Widget list={l.clone()} select_ref={select_ref.clone()}/>});
        }
    }
    Ok(crate::nav_content(
        html! {
          <>
            <ul class="navbar-nav me-auto">
              <li class="navbar-brand">if disabled { {"Demo"} } else { { "Home" } }</li>
            </ul>
            <div class="d-flex gap-3 align-items-baseline">
              <span class="navbar-text text-nowrap">{"Sort Mode:"}</span>
              <select ref={select_ref.clone()} class="form-select">
                <option>{"Tournament"}</option>
                <option selected=true>{"Random Tournament"}</option>
                <option>{"Random Matches"}</option>
                <option>{"Random Rounds"}</option>
              </select>
              <button class="btn btn-info" onclick={toggle_help}>{"Help"}</button>
            </div>
          </>
        },
        html! {
          <div>
            <Collapse collapsed={*help_collapsed}>
              <p>
              {"mybops is an app that helps you filter your data and remove flops from your life.
                    Use it to gain insights about your favorite songs, TV shows, and even restaurants.
                    mybops makes it easy to rate and/or rank what's important to you."}
              </p>
              <p>
              {"The data is organized into lists of items and your lists are displayed here on the home page using user-defined widgets.
                    The fastest way to rank your items is with a randomly generated tournament.
                    You can start a tournament for a list by clicking the "}<button type="button" class="btn btn-success btn-sm">{"Rank"}</button>
                {" button below the list widget. Here is the full list of sort modes:"}
              </p>
              <ul>
                <li><strong>{"Tournament"}</strong>{" - Sort by choosing between items that are organized using a seeded tournament."}</li>
                <li><strong>{"Random Tournament"}</strong>{" - Sort by choosing between items that are organized using a randomly generated tournament."}</li>
                <li><strong>{"Random Matches"}</strong>{" - Sort by choosing between randomly selected items."}</li>
                <li><strong>{"Random Rounds"}</strong>{" - This mode is similar to Random Matches except every item will be selected before an item is repeated."}</li>
              </ul>
              <p>{"To rate items, go to the item rating page for the list by clicking on the "}<button type="button" class="btn btn-success btn-sm">{"Rate"}</button>{" button."}</p>
              <p>{"You can also:"}</p>
              <ul class="mb-0">
                <li>{"View items in the list by clicking on the widget to expand it."}</li>
                <li>{"Search for data about your ratings and rankings by going to the "}<Link<Route> to={Route::Search}>{"Search"}</Link<Route>>{" page."}</li>
              </ul>
            </Collapse>
            <div class="mt-3">
              <div class="d-md-none">
                {column}
                <button type="button" class="btn btn-primary" onclick={create.clone()} {disabled}>{"Create List"}</button>
              </div>
              <div class="d-none d-md-block">
                <div class="d-grid gap-3" style="grid-template-columns: 1fr 1fr">
                  <div>
                    {left}
                    <button type="button" class="btn btn-primary" onclick={create} {disabled}>{"Create List"}</button>
                  </div>
                  <div>{right}</div>
                </div>
              </div>
            </div>
          </div>
        },
    ))
}

async fn fetch_lists() -> Vec<List> {
    crate::fetch_lists(true).await.unwrap()
}

#[derive(PartialEq, Properties)]
pub struct WidgetProps {
    list: List,
    select_ref: NodeRef,
}

#[function_component]
fn Widget(WidgetProps { list, select_ref }: &WidgetProps) -> Html {
    let list = list.clone();
    let collapsed = use_state(|| true);
    let query = use_state(|| None);

    let on_toggle = {
        let list = list.clone();
        let query = query.clone();
        let collapsed = collapsed.clone();
        Callback::from(move |_| {
            let list = Rc::new(list.clone());
            if query.is_none() {
                let query = query.clone();
                let collapsed = collapsed.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    query.set(crate::query_list(&list, None).await.unwrap());
                    collapsed.set(false);
                });
            } else {
                collapsed.set(!*collapsed);
            }
        })
    };
    let navigator = use_navigator().unwrap();
    let navigator_copy = navigator.clone();
    let id = list.id.clone();
    let compare = {
        let select_ref = select_ref.clone();
        Callback::from(move |_| {
            let id = id.clone();
            let mode = select_ref.cast::<HtmlSelectElement>().unwrap().value();
            match mode.as_ref() {
                "Random Matches" => {
                    navigator_copy.push(&ListsRoute::Match { id });
                }
                "Random Rounds" => {
                    navigator_copy
                        .push_with_query(
                            &ListsRoute::Match { id },
                            &[("mode", "rounds")].into_iter().collect::<HashMap<_, _>>(),
                        )
                        .unwrap();
                }
                "Tournament" => {
                    navigator_copy.push(&ListsRoute::Tournament { id });
                }
                "Random Tournament" => {
                    navigator_copy
                        .push_with_query(
                            &ListsRoute::Tournament { id },
                            &[("mode", "random")].into_iter().collect::<HashMap<_, _>>(),
                        )
                        .unwrap();
                }
                _ => {
                    web_sys::console::log_1(&JsValue::from("Invalid mode"));
                }
            };
        })
    };
    let id = list.id.clone();
    let go = Callback::from(move |_| {
        navigator.push(&ListsRoute::List { id: id.clone() });
    });
    // TODO: support actions on views
    let disabled = matches!(list.mode, ListMode::View(_));
    html! {
        <>
            <Accordion header={list.name.clone()} collapsed={*collapsed} {on_toggle}>
                if let Some(query) = &*query {
                    {crate::plot::df_table_view(query, false)}
                } else {
                    <div></div>
                }
            </Accordion>
            <div class="row mb-3">
                <div class="col-auto">
                    <button type="button" class="btn btn-success" onclick={go} {disabled}>{"Rate"}</button>
                </div>
                <div class="col-auto">
                    <button type="button" class="btn btn-success" onclick={compare} {disabled}>{"Rank"}</button>
                </div>
            </div>
        </>
    }
}
