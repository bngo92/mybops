use yew::{Callback, Children, Html, MouseEvent, Properties, function_component, html, use_state};

#[derive(Clone, PartialEq, Properties)]
pub struct AccordionProps {
    pub children: Children,
    pub header: String,
    pub on_toggle: Option<Callback<MouseEvent>>,
    pub collapsed: Option<bool>,
}

#[function_component]
pub fn Accordion(
    AccordionProps {
        children,
        header,
        on_toggle,
        collapsed,
    }: &AccordionProps,
) -> Html {
    let collapsed = collapsed.unwrap_or(true);
    let collapsed_state = use_state(|| collapsed);

    let toggle = {
        let collapsed_state = collapsed_state.clone();
        Callback::from(move |_| {
            collapsed_state.set(!*collapsed_state);
        })
    };

    let (collapsed, onclick) = if let Some(on_toggle) = on_toggle {
        (collapsed, on_toggle.clone())
    } else {
        (*collapsed_state, toggle)
    };
    let (button_class, body_class) = if collapsed {
        ("accordion-button collapsed", "accordion-collapse collapse")
    } else {
        ("accordion-button", "accordion-collapse collapse show")
    };
    html! {
        <div class="accordion mb-3">
            <div class="accordion-item">
                <h2 class="accordion-header">
                    <button class={button_class} {onclick}>{header}</button>
                </h2>
                <div class={body_class}>
                {for children.iter() }
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct AlertProps {
    pub result: Result<String, String>,
    pub hide: Callback<MouseEvent>,
}

#[function_component]
pub fn Alert(AlertProps { result, hide }: &AlertProps) -> Html {
    let (alert_class, body) = match result {
        Ok(msg) => ("alert alert-success alert-dismissible", msg),
        Err(msg) => ("alert alert-danger alert-dismissible", msg),
    };
    html! {
        <div class={alert_class}>
            {body}
            <button type="button" class="btn-close" onclick={hide}></button>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CollapseProps {
    pub children: Children,
    pub collapsed: bool,
}

#[function_component]
pub fn Collapse(
    CollapseProps {
        children,
        collapsed,
    }: &CollapseProps,
) -> Html {
    let body_class = if *collapsed {
        "collapse"
    } else {
        "collapse show"
    };
    html! {
        <div class={body_class}>
            <div class="card card-body bg-light">
            {for children.iter() }
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ModalProps {
    pub header: String,
    pub children: Children,
    pub hide: Callback<MouseEvent>,
}

#[function_component]
pub fn Modal(
    ModalProps {
        header,
        children,
        hide,
    }: &ModalProps,
) -> Html {
    let onclick = hide.clone();
    html! {
        <div>
            <div class="modal d-block" onclick={onclick.clone()}>
                <div class="modal-dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title">{header}</h1>
                            <button type="button" class="btn-close" {onclick}></button>
                        </div>
                        {for children.iter()}
                    </div>
                </div>
            </div>
            <div class="modal-backdrop show"></div>
        </div>
    }
}
