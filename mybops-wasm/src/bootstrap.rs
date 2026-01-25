use leptos::{
    html::{Dialog, Div},
    prelude::*,
    tachys::html::event::MouseEvent,
};
use leptos_use::on_click_outside;

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
    let button_class = move || {
        if collapsed() {
            "accordion-button collapsed"
        } else {
            "accordion-button"
        }
    };
    let body_class = move || {
        if collapsed() {
            "accordion-collapse collapse"
        } else {
            "accordion-collapse collapse show"
        }
    };
    let onclick = if let Some(on_toggle) = on_toggle {
        on_toggle
    } else {
        Callback::new(move |_| set_collapsed_state.set(!collapsed_state.get()))
    };
    view! {
      <div class="accordion mb-3">
        <div class="accordion-item">
          <h2 class="accordion-header">
            <button class=button_class on:click=move |ev| onclick.run(ev)>
              {header}
            </button>
          </h2>
          <div class=body_class>{children()}</div>
        </div>
      </div>
    }
}

#[component]
pub fn Alert(
    result: Result<String, String>,
    hide: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let (alert_class, body) = match result {
        Ok(msg) => ("alert alert-success alert-dismissible", msg),
        Err(msg) => ("alert alert-danger alert-dismissible", msg),
    };
    view! {
      <div class=alert_class>
        {body} <button type="button" class="btn-close" on:click=hide></button>
      </div>
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
