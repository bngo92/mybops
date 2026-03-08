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
                  <div class="px-4 py-2 rounded-sm border border-gray-300">
                    <a href=format!("/lists/{}", l.id)>{l.name.clone()}</a>
                  </div>
                }
            })
            .collect_view()
    };
    crate::nav_content(
        view! {
          <a href="#" class="text-lg font-medium text-black">
            "All Lists"
          </a>
        }
        .into_any(),
        (move || {
            view! {
              <div class="flex flex-col gap-4">
                <div class="grid md:grid-cols-2 gap-4">{list_html()}</div>
                <div>
                  <Button
                    class="text-white bg-blue-500 disabled:bg-blue-500/65"
                    on:click=move |_| {
                      create.dispatch(());
                    }
                    disabled=move || !logged_in.get()
                  >
                    "Create List"
                  </Button>
                </div>
              </div>
            }
        })
        .into_any(),
    )
}
