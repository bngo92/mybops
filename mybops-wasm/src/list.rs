use crate::{ListsRoute, UserProps};
use yew::{Callback, HtmlResult, function_component, html, suspense::use_future};
use yew_router::{hooks::use_navigator, prelude::Link};

pub mod item;

#[function_component]
pub fn Lists(UserProps { logged_in }: &UserProps) -> HtmlResult {
    let lists = &*use_future(|| async move { crate::fetch_lists(false).await.unwrap() })?;
    let navigator = use_navigator().unwrap();

    let create = Callback::from(move |_| {
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let list = crate::create_list(None).await.unwrap();
            navigator.push(&ListsRoute::Edit { id: list.id });
        });
    });

    let list_html = lists.iter().map(|l| {
        html! {
            <div class="col-12 col-md-6 mb-4">
                <div class="card">
                    <div class="card-body">
                        <Link<ListsRoute> to={ListsRoute::View{id: l.id.clone()}}>{&l.name}</Link<ListsRoute>>
                    </div>
                </div>
            </div>
        }
    });
    let disabled = !logged_in;
    Ok(crate::nav_content(
        html! {
          <ul class="navbar-nav me-auto">
            <li class="navbar-brand">{"All Lists"}</li>
          </ul>
        },
        html! {
          <div>
            <div class="row mt-3">
              {for list_html}
            </div>
            <button type="button" class="btn btn-primary" onclick={create} {disabled}>{"Create List"}</button>
          </div>
        },
    ))
}
