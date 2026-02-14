use std::{collections::HashMap, sync::Arc};

use leptos::{
    html::{Dialog, Div},
    prelude::*,
    tachys::html::event::MouseEvent,
};
use leptos_use::{UseTimeoutFnReturn, on_click_outside, use_timeout_fn};

#[component]
pub fn Accordion(
    children: Children,
    header: String,
    #[prop(optional)] on_toggle: Option<Callback<MouseEvent>>,
    #[prop(into)] collapsed: Signal<Option<bool>>,
) -> impl IntoView {
    let initial = collapsed.read().unwrap_or(true);
    let (collapsed_state, set_collapsed_state) = signal(initial);

    let collapsed = move || {
        if on_toggle.is_some() {
            collapsed.read().unwrap_or(true)
        } else {
            collapsed_state.get()
        }
    };
    let body_class = move || {
        if collapsed() { "tw:hidden" } else { "tw:block" }
    };
    let onclick = if let Some(on_toggle) = on_toggle {
        on_toggle
    } else {
        Callback::new(move |_| set_collapsed_state.set(!collapsed_state.get()))
    };
    view! {
      <div class="tw:mb-3 tw:bg-white tw:rounded-sm tw:border-1 tw:border-gray-200">
        <h2
          class="tw:px-5 tw:py-4 tw:m-0! tw:text-base! tw:border-gray-200"
          class=("tw:border-b", move || !collapsed())
        >
          <button class="tw:flex tw:justify-between tw:w-full" on:click=move |ev| onclick.run(ev)>
            {header}
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="tw:size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d=move || {
                  if collapsed() {
                    "m19.5 8.25-7.5 7.5-7.5-7.5"
                  } else {
                    "m4.5 15.75 7.5-7.5 7.5 7.5"
                  }
                }
              />
            </svg>
          </button>
        </h2>
        <div class=body_class>{children()}</div>
      </div>
    }
}

#[derive(Clone)]
pub struct ToastState {
    result: RwSignal<Option<Result<String, String>>>,
    schedule_hide: Arc<dyn Fn(()) + Send + Sync>,
    cancel_hide: Arc<dyn Fn() + Send + Sync>,
}

#[derive(Clone, Copy)]
pub struct Toast {
    id: &'static str,
    set_toast: WriteSignal<HashMap<&'static str, ToastState>>,
}

impl Toast {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            set_toast: use_context().unwrap(),
        }
    }

    pub fn set(&self, toast: Result<String, String>) {
        self.set_toast.update(|toasts| {
            let entry = toasts.entry(self.id).or_insert_with(|| {
                let hide = {
                    let set_toast = self.set_toast;
                    let id = self.id;
                    move |_| set_toast.update(|toasts| toasts[id].result.set(None))
                };
                let UseTimeoutFnReturn { start, stop, .. } = use_timeout_fn(hide, 5000.);
                ToastState {
                    result: RwSignal::new(None),
                    schedule_hide: Arc::new(start),
                    cancel_hide: Arc::new(stop),
                }
            });
            (entry.cancel_hide)();
            // Don't hide errors
            if toast.is_ok() {
                (entry.schedule_hide)(())
            }
            entry.result.set(Some(toast));
        })
    }
}

#[component]
pub fn Toasts(toasts: ReadSignal<HashMap<&'static str, ToastState>>) -> impl IntoView {
    view! {
      <div class="tw:fixed tw:right-0 tw:bottom-0 tw:p-3 tw:w-full tw:max-w-md">
        <For
          each=move || toasts.read().clone()
          key=|(k, _)| *k
          children=move |(_, toast)| view! { <Alert result=toast.result /> }
        />
      </div>
    }
}

#[component]
fn Alert(result: RwSignal<Option<Result<String, String>>>) -> impl IntoView {
    let hide = move |_| result.set(None);
    move || {
        let result = result.get()?;
        let (alert_class, body) = match result {
            Ok(msg) => (
                "tw:flex tw:justify-between tw:p-4 tw:bg-emerald-100 alert-success",
                msg,
            ),
            Err(msg) => (
                "tw:flex tw:justify-between tw:p-4 tw:bg-red-100 alert-danger",
                msg,
            ),
        };
        Some(view! {
          <div class=alert_class>
            {body} <button type="button" on:click=hide>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="3"
                class="tw:size-4 tw:stroke-gray-500"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M0 24 24 0M0 0l24 24" />
              </svg>
            </button>
          </div>
        })
    }
}

#[component]
pub fn Collapse(children: Children, #[prop(into)] collapsed: Signal<bool>) -> impl IntoView {
    let body_class = move || {
        if collapsed.get() {
            "collapse"
        } else {
            "collapse show"
        }
    };
    view! {
      <div class=body_class>
        <div class="card card-body bg-light">{children()}</div>
      </div>
    }
}

pub enum Direction {
    Up,
    Down,
}

#[component]
pub fn Dropdown(children: Children, id: String, direction: Direction) -> impl IntoView {
    let dropdown = NodeRef::<Div>::new();
    #[allow(unused_must_use)]
    on_click_outside(dropdown, move |_| {
        dropdown.get().unwrap().hide_popover().unwrap();
    });
    view! {
      <div
        id=id
        popover=true
        class="tw:left-[anchor(left)]"
        class=(["tw:inset-auto", "tw:bottom-[anchor(top)]"], matches!(direction, Direction::Up))
        class=("tw:top-[anchor(bottom)]", matches!(direction, Direction::Down))
      >
        <div class="tw:flex tw:flex-col tw:gap-4 tw:p-4 tw:min-w-40">{children()}</div>
      </div>
    }
}

#[component]
pub fn Modal(
    #[prop(into)] header: Signal<String>,
    children: Children,
    modal_ref: NodeRef<Dialog>,
) -> impl IntoView {
    view! {
      <dialog
        class="tw:fixed tw:top-1/3 tw:left-1/2 tw:-translate-1/2 tw:w-full tw:max-w-md"
        closedby="any"
        node_ref=modal_ref
      >
        <div class="tw:flex tw:justify-between tw:p-4">
          <h1 class="">{header}</h1>
          <form method="dialog">
            <button class="">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="tw:size-8"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
              </svg>
            </button>
          </form>
        </div>
        <div class="tw:flex tw:flex-col tw:p-4">{children()}</div>
      </dialog>
    }
}
