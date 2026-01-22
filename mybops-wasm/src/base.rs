use leptos::{either::Either, html, prelude::*};
use mybops::ItemMetadata;
use std::borrow::Cow;
use web_sys::MouseEvent;

pub enum IframeCompareMsg {
    Left,
    Right,
}

#[component]
pub fn IframeCompare(
    left: ItemMetadata,
    on_left_select: impl FnMut(MouseEvent) + 'static,
    right: ItemMetadata,
    on_right_select: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let (flag, set_flag) = signal(IframeCompareMsg::Left);
    let (left_class, right_class, src) = match *flag.read() {
        IframeCompareMsg::Left => ("nav-link active", "nav-link", left.iframe.clone()),
        IframeCompareMsg::Right => ("nav-link", "nav-link active", right.iframe.clone()),
    };
    view! {
      <div class="row">
        <div class="col-12 d-lg-none">
          <ul class="nav nav-tabs nav-justified">
            <li class="nav-item">
              <a
                class=left_class
                aria-label="Show left item"
                href="#"
                on:click=move |_| set_flag.set(IframeCompareMsg::Left)
              >
                {left.name.clone()}
              </a>
            </li>
            <li class="nav-item">
              <a class=right_class href="#" onclick=move || set_flag.set(IframeCompareMsg::Right)>
                {right.name.clone()}
              </a>
            </li>
          </ul>
          <iframe width="100%" height="380" prop:frameborder="0" src=src></iframe>
        </div>
        <div class="col-md-6 d-none d-lg-block">
          <iframe width="100%" height="380" prop:frameborder="0" src=left.iframe.clone()></iframe>
        </div>
        <div class="col-md-6 d-none d-lg-block">
          <iframe width="100%" height="380" prop:frameborder="0" src=right.iframe.clone()></iframe>
        </div>
        <div class="col-6">
          <button type="button" class="btn btn-info text-truncate w-100" on:click=on_left_select>
            {left.name}
          </button>
        </div>
        <div class="col-6">
          <button
            type="button"
            class="btn btn-warning text-truncate w-100"
            on:click=on_right_select
          >
            {right.name}
          </button>
        </div>
      </div>
    }
}

#[component]
pub fn Input(
    input_ref: NodeRef<html::Input>,
    #[prop(optional)] default: Option<&'static str>,
    value: Option<String>,
    onclick: impl FnMut(MouseEvent) + 'static,
    error: ReadSignal<Option<String>>,
    disabled: bool,
) -> impl IntoView {
    let class = move || {
        if error.read().is_some() {
            "is-invalid"
        } else {
            ""
        }
    };
    view! {
      <div class="d-flex gap-2">
        <div style="flex-basis: 800px">
          // Copy only the styles from .form-control that are needed for sizing
          <input
            node_ref=input_ref
            type="text"
            class=class
            style="padding: .5rem 1rem; font-size: .875rem; border-width: 1px; min-width: 100%"
            placeholder=default
            value=value.clone()
            disabled=disabled
          />
          {move || error.get().map(|error| view! { <div class="invalid-feedback">{error}</div> })}
        </div>
        <div>
          <button type="button" class="btn btn-success" on:click=onclick disabled=disabled>
            "Search"
          </button>
        </div>
      </div>
    }
}

pub fn responsive_table_view(
    header: &[&str],
    items: Vec<Option<(i32, Cow<'_, [String]>)>>,
) -> impl IntoView {
    let (left_items, right_items): (Vec<_>, Vec<_>) = items
        .iter()
        .cloned()
        .zip(1..)
        .partition(|(_, i)| i % 2 == 1);
    let left_items = left_items.into_iter().map(|(item, _)| item);
    let right_items = right_items.into_iter().map(|(item, _)| item);
    view! {
      <div class="row">
        <div class="col-md-6 d-none d-lg-block">{table_view(header, left_items)}</div>
        <div class="col-md-6 d-none d-lg-block">{table_view(header, right_items)}</div>
        <div class="col-12 d-lg-none">{table_view(header, items.into_iter())}</div>
      </div>
    }
}

pub fn table_view<'a>(
    header: &[&str],
    items: impl Iterator<Item = Option<(i32, Cow<'a, [String]>)>>,
) -> impl IntoView {
    view! {
      <div class="table-responsive">
        <table class="table table-striped mb-0">
          <thead>
            <tr>
              <th>"#"</th>
              {header.iter().map(|item| view! { <th>{item.to_owned()}</th> }).collect_view()}
            </tr>
          </thead>
          <tbody>{items.map(|item| item_view(item, header.len())).collect_view()}</tbody>
        </table>
      </div>
    }
}

fn item_view(item: Option<(i32, Cow<[String]>)>, len: usize) -> impl IntoView {
    if let Some((i, item)) = item {
        Either::Left(view! {
          <tr>
            <th>{i}</th>
            {item
              .iter()
              .take(len)
              .map(|item| view! { <td class="text-truncate max-width">{item.clone()}</td> })
              .collect_view()}
          </tr>
        })
    } else {
        Either::Right(view! {
          <tr style="height: 41.5px">
            <th></th>
            <td class="td"></td>
            <td></td>
            <td></td>
          </tr>
        })
    }
}
