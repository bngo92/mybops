use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::base::Button;

pub mod item;

#[component]
pub fn Lists(#[prop(into)] logged_in: Signal<bool>) -> impl IntoView {
    let lists = LocalResource::new(|| async { crate::fetch_lists(false).await.unwrap() });

    let create = Action::new_unsync(|_| async {
        let list = crate::create_list(None).await.unwrap();
        let navigator = use_navigate();
        navigator(&format!("/lists/{}/edit", list.id), Default::default());
    });

    let list_html = move || {
        lists
            .read()
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|l| {
                view! {
                  <div class="col-12 col-md-6 mb-4">
                    <div class="card">
                      <div class="card-body">
                        <a href=format!("/lists/{}", l.id)>{l.name.clone()}</a>
                      </div>
                    </div>
                  </div>
                }
            })
            .collect_view()
    };
    crate::nav_content(
        view! {
          <a href="#" class="tw:text-lg tw:font-medium tw:text-black! tw:no-underline!">
            "All Lists"
          </a>
        }
        .into_any(),
        (move || {
            view! {
              <div>
                <div class="row mt-3">{list_html()}</div>
                <Button
                  class="tw:text-white tw:bg-blue-500"
                  on:click=move |_| {
                    create.dispatch(());
                  }
                  disabled=move || !logged_in.get()
                >
                  "Create List"
                </Button>
              </div>
            }
        })
        .into_any(),
    )
}
