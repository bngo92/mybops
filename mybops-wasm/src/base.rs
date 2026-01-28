use leptos::{html, prelude::*};
use mybops::ItemMetadata;
use std::borrow::Cow;
use web_sys::MouseEvent;

#[component]
pub fn Button(
    children: Children,
    #[prop(default = "button")] r#type: &'static str,
    class: &'static str,
) -> impl IntoView {
    view! {
      <button
        type=r#type
        class=class
        class=(["tw:rounded-sm!"], true)
        class=(["tw:px-4"], true)
        class=(["tw:py-2"], true)
      >
        {children()}
      </button>
    }
}

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
    view! {
      <div class="tw:grid tw:lg:hidden tw:grid-cols-2 tw:mb-px tw:text-center">
        <a
          class="tw:h-full tw:text-gray-700! tw:no-underline! tw:border-b-2"
          class=("tw:border-purple-500", move || matches!(*flag.read(), IframeCompareMsg::Left))
          class=("tw:border-gray-300", move || matches!(*flag.read(), IframeCompareMsg::Right))
          aria-label="Show left item"
          href="#"
          on:click=move |_| set_flag.set(IframeCompareMsg::Left)
        >
          {left.name.clone()}
        </a>
        <a
          class="tw:h-full tw:text-gray-700! tw:no-underline! tw:border-b-2"
          class=("tw:border-gray-300", move || matches!(*flag.read(), IframeCompareMsg::Left))
          class=("tw:border-purple-500", move || matches!(*flag.read(), IframeCompareMsg::Right))
          href="#"
          on:click=move |_| set_flag.set(IframeCompareMsg::Right)
        >
          {right.name.clone()}
        </a>
      </div>
      <div class="tw:grid tw:grid-cols-2 tw:gap-x-6">
        <div class="tw:col-span-full tw:lg:hidden">
          <iframe
            width="100%"
            height="380"
            prop:frameborder="0"
            src={
              let left = left.iframe.clone();
              let right = right.iframe.clone();
              move || match *flag.read() {
                IframeCompareMsg::Left => left.clone(),
                IframeCompareMsg::Right => right.clone(),
              }
            }
          ></iframe>
        </div>
        <div class="tw:hidden tw:lg:block">
          <iframe width="100%" height="380" prop:frameborder="0" src=left.iframe.clone()></iframe>
        </div>
        <div class="tw:hidden tw:lg:block">
          <iframe width="100%" height="380" prop:frameborder="0" src=right.iframe.clone()></iframe>
        </div>
        <Button
          class="tw:py-2 tw:w-full tw:truncate tw:text-white tw:bg-violet-400"
          on:click=on_left_select
        >
          {left.name}
        </Button>
        <Button
          class="tw:py-2 tw:w-full tw:truncate tw:text-white tw:bg-purple-400"
          on:click=on_right_select
        >
          {right.name}
        </Button>
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
    view! {
      <div class="tw:flex tw:gap-2">
        <div style="flex-basis: 800px">
          // Copy only the styles from .form-control that are needed for sizing
          <input
            class="tw:bg-white"
            node_ref=input_ref
            type="text"
            style="padding: .5rem 1rem; font-size: .875rem; border-width: 1px; min-width: 100%"
            placeholder=default
            value=value.clone()
            disabled=disabled
          />
          {move || {
            error.get().map(|error| view! { <div class="tw:text-sm tw:text-red-500">{error}</div> })
          }}
        </div>
        <div>
          <Button class="tw:text-white tw:bg-purple-400" {..} on:click=onclick disabled=disabled>
            "Search"
          </Button>
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
      <div class="tw:grid tw:grid-cols-2 tw:gap-x-6">
        <div class="tw:hidden tw:lg:block">{table_view(header, left_items)}</div>
        <div class="tw:hidden tw:lg:block">{table_view(header, right_items)}</div>
        <div class="tw:lg:hidden tw:col-span-full">{table_view(header, items.into_iter())}</div>
      </div>
    }
}

pub fn table_view<'a>(
    header: &[&str],
    items: impl Iterator<Item = Option<(i32, Cow<'a, [String]>)>>,
) -> impl IntoView {
    view! {
      <div class="tw:overflow-x-auto">
        <table class="tw:w-full">
          <thead>
            <tr>
              <th class="tw:p-4">"#"</th>
              {header
                .iter()
                .map(|item| view! { <th class="tw:p-4">{item.to_owned()}</th> })
                .collect_view()}
            </tr>
          </thead>
          <tbody>{items.map(|item| item_view(item, header.len())).collect_view()}</tbody>
        </table>
      </div>
    }
}

fn item_view(item: Option<(i32, Cow<[String]>)>, len: usize) -> impl IntoView {
    item.map(|(i, item)| {
        view! {
          <tr>
            <th class="tw:p-4">{i}</th>
            {item
              .iter()
              .take(len)
              .map(|item| {
                view! { <td class="tw:p-4 tw:max-w-[424px] tw:truncate">{item.clone()}</td> }
              })
              .collect_view()}
          </tr>
        }
    })
}
