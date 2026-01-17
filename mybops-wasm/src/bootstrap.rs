use leptos::{prelude::*, tachys::html::event::MouseEvent};

#[component]
pub fn Accordion(
    children: Children,
    header: String,
    on_toggle: Option<Box<dyn FnMut(MouseEvent) + 'static>>,
    collapsed: ReadSignal<Option<bool>>,
) -> impl IntoView {
    let collapsed = collapsed.read().unwrap_or(true);
    let (collapsed_state, set_collapsed_state) = signal(collapsed);

    let (collapsed, onclick) = if let Some(on_toggle) = on_toggle {
        (collapsed, on_toggle)
    } else {
        (
            collapsed_state.get(),
            Box::new(move |_| set_collapsed_state.set(!collapsed_state.get()))
                as Box<dyn FnMut(MouseEvent) + 'static>,
        )
    };
    let (button_class, body_class) = if collapsed {
        ("accordion-button collapsed", "accordion-collapse collapse")
    } else {
        ("accordion-button", "accordion-collapse collapse show")
    };
    view! {
      <div class="accordion mb-3">
        <div class="accordion-item">
          <h2 class="accordion-header">
            <button class=button_class on:click=onclick>
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
pub fn Collapse(children: Children, collapsed: ReadSignal<bool>) -> impl IntoView {
    let body_class = if collapsed.get() {
        "collapse"
    } else {
        "collapse show"
    };
    view! {
      <div class=body_class>
        <div class="card card-body bg-light">{children()}</div>
      </div>
    }
}

#[component]
pub fn Modal(
    header: String,
    children: Children,
    hide: impl FnMut(MouseEvent) + 'static + Clone,
) -> impl IntoView {
    let hide_copy = hide.clone();
    view! {
      <div>
        <div class="modal d-block" on:click=hide_copy>
          <div class="modal-dialog" on:click=|e: MouseEvent| e.stop_propagation()>
            <div class="modal-content">
              <div class="modal-header">
                <h1 class="modal-title">{header}</h1>
                <button type="button" class="btn-close" on:click=hide></button>
              </div>
              {children()}
            </div>
          </div>
        </div>
        <div class="modal-backdrop show"></div>
      </div>
    }
}
