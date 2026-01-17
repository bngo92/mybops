use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

pub mod item;

#[component]
pub fn Lists(logged_in: ReadSignal<bool>) -> impl IntoView {
    let lists = LocalResource::new(|| async { crate::fetch_lists(false).await.unwrap() });

    let create = Action::new_unsync(|_| async {
        let list = crate::create_list(None).await.unwrap();
        let navigator = use_navigate();
        navigator(&format!("/lists/{}/edit", list.id), Default::default());
    });

    let list_html = || {
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
          <ul class="navbar-nav me-auto">
            <li class="navbar-brand">"All Lists"</li>
          </ul>
        }
        .into_any(),
        view! {
          <div>
            <div class="row mt-3">{list_html()}</div>
            <button
              type="button"
              class="btn btn-primary"
              on:click=move |_| {
                create.dispatch(());
              }
              disabled=move || !logged_in.get()
            >
              "Create List"
            </button>
          </div>
        }
        .into_any(),
    )
}
