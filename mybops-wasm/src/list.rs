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
                  <div class="tw:px-4 tw:py-2 tw:rounded-sm tw:border tw:border-gray-300">
                    <a href=format!("/lists/{}", l.id)>{l.name.clone()}</a>
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
              <div class="tw:flex tw:flex-col tw:gap-4">
                <div class="tw:grid tw:md:grid-cols-2 tw:gap-4">{list_html()}</div>
                <div>
                  <Button
                    class="tw:text-white tw:bg-blue-500 tw:disabled:bg-blue-500/65"
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
