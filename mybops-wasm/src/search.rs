use crate::{
    base::{Button, Input, SelectWithCallback},
    bootstrap::Collapse,
    plot::{DataView, DataViewRender},
};
use leptos::{either::Either, html, prelude::*};
use web_sys::KeyboardEvent;

#[component]
pub fn Search(#[prop(into)] logged_in: RwSignal<bool>) -> impl IntoView {
    let (split_view, set_split_view) = signal(false);

    let button_text = {
        move || {
            if split_view.get() {
                "Single View"
            } else {
                "Split View"
            }
        }
    };
    crate::nav_content(
        view! {
          <>
            <a href="#" class="tw:text-lg tw:font-medium tw:text-black! tw:no-underline!">
              "Query"
            </a>
            <div class="tw:flex tw:gap-4">
              <Button
                class="tw:w-32 tw:text-white tw:bg-purple-500/80"
                on:click=move |_| set_split_view.set(!split_view.get())
              >
                {button_text}
              </Button>
              <Button
                class="tw:text-white tw:bg-purple-500/80"
                on:click=move |_| logged_in.set(!logged_in.get())
              >
                "Help"
              </Button>
            </div>
          </>
        },
        view! {
          <div class="tw:flex tw:flex-col tw:gap-4">
            <Collapse collapsed=logged_in>
              <p>
                "Run SQL queries to transform your data into insights.
                All queries should run against the \"c\" table."
              </p>
              <p>
                <strong>"Example Queries"</strong>
              </p>
              <p>"Get names of songs that have more tournament and match wins than losses:"</p>
              <code>
                "SELECT name, user_wins, user_losses FROM item WHERE type='track' AND user_wins > user_losses"
              </code>
              <p>"Get names of songs ordered by your scores:"</p>
              <code>
                "SELECT name, user_score FROM item WHERE type='track' ORDER BY user_score DESC"
              </code>
              <p>"Count how many songs were performed by each distinct group of artists:"</p>
              <code>"SELECT artists, COUNT(1) FROM item WHERE type='track' GROUP BY artists"</code>
              <p>"Get songs performed by Troy:"</p>
              <code>
                "SELECT name, artists FROM item, json_each(metadata->'artists') WHERE json_each.value='Troy'"
              </code>
              <p>"Get your average score for each group of artists:"</p>
              <code>
                "SELECT artists, AVG(user_score) FROM item WHERE type='track' GROUP BY artists"
              </code>
              <p>
                <strong>"Fields"</strong>
              </p>
              <p>
                "The fields you can query on are listed below.
                Here is the list of fields that are available for all items:"
              </p>
              <ul>
                <li class="tw:list-disc tw:list-inside">"type: string - The type of item"</li>
                <li class="tw:list-disc tw:list-inside">"name: string - The name of the item"</li>
                <li class="tw:list-disc tw:list-inside">
                  "rating: number - The rating that you gave the item"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "user_score: number - Score computed from tournaments and matches"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "user_wins: number - Tournament and match wins"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "user_losses: number - Tournament and match losses"
                </li>
                <li class="tw:list-disc tw:list-inside">"hidden: boolean - The item was hidden"</li>
              </ul>
              <p>"There are also fields that are specific to a single item type."</p>
              <p>
                <em>"Spotify Item Fields"</em>
              </p>
              <p>"Type is set to 'track' for Spotify items"</p>
              <ul>
                <li class="tw:list-disc tw:list-inside">
                  "album: string - The name of the album that the track appears on"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "artists: array of string - The names of the artists who performed the track"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "duration_ms: number - The track length in milliseconds"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "popularity - Spotify popularity of the track"
                </li>
                <li class="tw:list-disc tw:list-inside">
                  "track_number - The number of the track"
                </li>
              </ul>
            </Collapse>
            {move || {
              if split_view.get() {
                Either::Left(
                  view! {
                    <div class="tw:flex tw:gap-4">
                      <SearchPane />
                      <SearchPane />
                    </div>
                  },
                )
              } else {
                Either::Right(
                  view! {
                    <div style="max-width: 1000px">
                      <SearchPane />
                    </div>
                  },
                )
              }
            }}
          </div>
        },
    )
}

#[component]
pub fn SearchPane() -> impl IntoView {
    let search_ref = NodeRef::<html::Input>::new();
    let (query, set_query) = signal(None);
    let (error, set_error) = signal(None);
    let (view, set_view) = signal(DataView::Table);

    let search = Action::new_unsync(move |_| {
        let input = search_ref.get().unwrap().value();
        async move {
            match crate::find_items(&input).await {
                Ok(q) => {
                    set_query.set(q);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e.as_string().unwrap()));
                }
            }
        }
    });
    let create = Action::new_unsync(move |_| {
        let input = search_ref.get().unwrap().value();
        async move {
            if let Err(e) = crate::create_list(Some(input)).await {
                set_error.set(Some(e.as_string().unwrap()));
            }
        }
    });

    let default_search = "SELECT name, user_score FROM item";
    let onkeydown = |event: KeyboardEvent| {
        if event.key_code() == 13 {
            event.prevent_default();
        }
    };
    view! {
      <div class="tw:flex tw:flex-col tw:gap-4 tw:w-full">
        <div class="tw:w-fit">
          <SelectWithCallback on_change=move |ev| {
            set_view
              .set(
                match ev.target().value().as_str() {
                  "Table" => DataView::Table,
                  "Column Graph" => DataView::ColumnGraph,
                  "Line Graph" => DataView::LineGraph,
                  "Scatter Plot" => DataView::ScatterPlot,
                  "Cumulative Line Graph" => DataView::CumLineGraph,
                  "CSV" => DataView::Csv,
                  _ => unreachable!(),
                },
              )
          }>
            <option selected=true>"Table"</option>
            <option>"Column Graph"</option>
            <option>"Line Graph"</option>
            <option>"Scatter Plot"</option>
            <option>"Cumulative Line Graph"</option>
            <option>"CSV"</option>
          </SelectWithCallback>
        </div>
        <form on:keydown=onkeydown>
          <div class="tw:flex tw:gap-2">
            <div class="tw:flex-1">
              <Input
                input_ref=search_ref
                default=default_search
                value=Some(String::new())
                onclick=move |_| {
                  search.dispatch(());
                }
                error=error
                disabled=false
              />
            </div>
            <Button
              style="primary"
              on:click=move |_| {
                create.dispatch(());
              }
              prop:style="height: fit-content"
              disabled=move || query.read().is_none()
            >
              "Create List"
            </Button>
          </div>
        </form>
        {move || {
          query
            .get()
            .map(|query| {
              view! { <DataViewRender view=view df=move || query.clone() set_error=set_error /> }
            })
        }}
      </div>
    }
}
