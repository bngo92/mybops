use std::collections::HashMap;

use crate::{
    Content, ListsRoute,
    base::{Button, Input, SelectWithCallback},
    bootstrap::{Direction, Dropdown, Modal, Toasts},
    docs,
    edit::Edit,
    home::Home,
    integrations::spotify::SpotifyIntegration,
    list::{
        self,
        item::{ItemMode, ListItems},
    },
    nfl::Nfl,
    plot::{DataView, DataViewRender},
    random::{RandomMatches, RandomRounds},
    search::Search,
    settings::Settings,
    tournament::{RandomTournamentLoader, TournamentLoader},
};
use leptos::{
    either::Either,
    html::{self, Dialog},
    prelude::*,
};
use leptos_router::{
    components::*,
    hooks::{use_navigate, use_params, use_query_map},
    params::Params,
    path,
};
use mybops::{List, ListMode, User};

pub enum ListPage {
    View,
    List,
    Edit,
    RandomMatches,
    RandomRounds,
    Tournament,
    RandomTournament,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Transition>
        <Router>
          <AppImpl />
        </Router>
      </Transition>
    }
}

#[component]
fn AppImpl() -> impl IntoView {
    let logged_in = RwSignal::new(false);
    let user = LocalResource::new(move || async move {
        let user = crate::get_user().await.ok();
        logged_in.set(user.is_some());
        user
    });
    let (sidebar, set_sidebar) = signal(false);
    let (toasts, set_toasts) = signal(HashMap::new());
    provide_context(set_toasts);

    /*Msg::Logout => {
        ctx.link().clone().send_future(async move {
            let window = web_sys::window().expect("no global `window` exists");
            let request = query("/api/logout", "POST").unwrap();
            JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap();
            Msg::Reload
        });
        false
    }
    Msg::Reload => true,*/

    //let onclick = ctx.link().callback(|_| Msg::Logout);
    // TODO: make anchors active if active
    let search = /*if location.pathname().unwrap() == "/search" {
        ""
    } else */{
        "text-white"
    };
    let origin = location().origin().unwrap();
    let modal_ref = NodeRef::<Dialog>::new();
    view! {
      <div class="flex h-dvh">
        <nav
          class="flex fixed md:static md:visible flex-col gap-4 p-4 h-full text-white bg-mist-800"
          class=("invisible", move || !sidebar.get())
          style="width: 200px;"
        >
          <div class="flex gap-4">
            <a class="text-xl" href="/">
              "mybops"
            </a>
            <button type="button" class="md:hidden" on:click=move |_| set_sidebar.set(false)>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-7"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <hr class="text-gray-600" />
          <div class="flex flex-col flex-1 gap-4">
            <a class=search href="/lists">
              "Lists"
            </a>
            <a class=search href="/search">
              "Query"
            </a>
            <button popovertarget="integrations-dropdown" class="flex gap-1 items-baseline">
              "Integrations"
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                fill="currentColor"
                class="size-2"
              >
                <polygon points="0,0 20,0 10,10" />
              </svg>
            </button>
            <Dropdown id="integrations-dropdown".to_owned() direction=Direction::Down>
              <a class="text-black" href="/integrations/spotify">
                "Spotify"
              </a>
            </Dropdown>
            <a class=search href="/docs">
              "Docs"
            </a>
          </div>
          <hr class="text-gray-600" />
          {move || {
            if let Some(user) = user.get().flatten() {
              Either::Left(
                view! {
                  <button popovertarget="login-dropdown" class="flex gap-1 items-baseline">
                    {user.user_id}
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                      class="size-2"
                    >
                      <polygon points="0,0 20,0 10,10" />
                    </svg>
                  </button>
                  <Dropdown id="login-dropdown".to_owned() direction=Direction::Up>
                    <a class="text-black" href="/settings">
                      "Settings"
                    </a>
                    <a class="text-black" href="/api/logout" rel="external">
                      "Log out"
                    </a>
                  </Dropdown>
                },
              )
            } else {
              Either::Right(
                view! {
                  <a
                    class=search
                    href="#"
                    on:click=move |_| modal_ref.get().unwrap().show_modal().unwrap()
                  >
                    "Log in"
                  </a>
                },
              )
            }
          }}
        </nav>
        <div class="flex flex-col flex-1">
          <nav class="md:hidden bg-mist-800">
            <div class="flex gap-4 p-3">
              <button
                type="button"
                style="background-color: transparent; color: rgba(255,255,255,0.85)"
                on:click=move |_| set_sidebar.set(true)
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  class="size-6"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                  />
                </svg>
              </button>
              <a class="text-xl text-white" href="/">
                "mybops"
              </a>
            </div>
          </nav>
          <Routes fallback=crate::not_found>
            <Route path=path!("/") view=move || view! { <Home logged_in=logged_in /> } />
            <Route path=path!("/docs") view=docs::docs />
            <ParentRoute path=path!("/lists") view=Outlet>
              <Route
                path=path!("")
                view=move || {
                  view! { <list::Lists logged_in=logged_in /> }
                }
              />
              <Route
                path=path!(":id")
                view=move || {
                  view! {
                    <ListComponent view=ListsRoute::View user=move || user.get().flatten() />
                  }
                }
              />
              <Route
                path=path!(":id/items")
                view=move || {
                  view! {
                    <ListComponent view=ListsRoute::List user=move || user.get().flatten() />
                  }
                }
              />
              <Route
                path=path!(":id/edit")
                view=move || {
                  view! {
                    <ListComponent view=ListsRoute::Edit user=move || user.get().flatten() />
                  }
                }
              />
              <Route
                path=path!(":id/match")
                view=move || {
                  view! {
                    <ListComponent view=ListsRoute::Match user=move || user.get().flatten() />
                  }
                }
              />
              <Route
                path=path!(":id/tournament")
                view=move || {
                  view! {
                    <ListComponent view=ListsRoute::Tournament user=move || user.get().flatten() />
                  }
                }
              />
            </ParentRoute>
            <Route path=path!("/search") view=move || view! { <Search logged_in=logged_in /> } />
            <Route
              path=path!("/settings")
              view=move || {
                if user.read().as_ref().flatten().is_some() {
                  view! { <Settings user=move || user.get().flatten().unwrap() /> }.into_any()
                } else {
                  crate::not_found().into_any()
                }
              }
            />
            <Route
              path=path!("/integrations/spotify")
              view=move || view! { <SpotifyIntegration logged_in=logged_in /> }
            />
            <Route path=path!("/nfl") view=Nfl />
          </Routes>
        </div>
        <Modal header="Log in".to_owned() modal_ref=modal_ref>
          <div class="flex flex-col gap-4">
            <Button
              class="text-white bg-primary"
              on:click={
                let origin = origin.clone();
                move |_| {
                  let navigate = use_navigate();
                  navigate(
                    &format!(
                      "https://accounts.spotify.com/authorize?client_id=ee3d1b4f8d80477ea48743a511ef3018&redirect_uri={}/api/login&response_type=code&scope=playlist-modify-public playlist-modify-private user-read-recently-played playlist-read-private",
                      origin.as_str(),
                    ),
                    Default::default(),
                  )
                }
              }
            >
              "Log in with Spotify"
            </Button>
            <Button
              class="text-white bg-primary"
              on:click=move |_| {
                let navigate = use_navigate();
                navigate(
                  &format!(
                    "https://accounts.google.com/o/oauth2/v2/auth?client_id=1038220726403-n55jha2cvprd8kdb4akdfvo0uiok4p5u.apps.googleusercontent.com&redirect_uri={}/api/login/google&response_type=code&scope=email",
                    origin.as_str(),
                  ),
                  Default::default(),
                )
              }
            >
              "Log in with Google"
            </Button>
          </div>
        </Modal>
        <Toasts toasts=toasts />
      </div>
    }
}

#[component]
fn ListView(#[prop(into)] list: Signal<List>) -> impl IntoView {
    let (view, set_view) = signal(DataView::Table);
    let (query, set_query) = signal(None);
    let (error, set_error) = signal(None);
    let query_ref = NodeRef::<html::Input>::new();

    let data = LocalResource::new(move || async move {
        match crate::query_list(&list.read(), query.get()).await {
            Ok(None) => None,
            Ok(Some(mut data)) => {
                set_error.set(None);
                data.drop_in_place("id");
                Some(data)
            }
            Err(e) => {
                set_error.set(e.as_string());
                None
            }
        }
    });

    view! {
      <div class="flex flex-col gap-4">
        <SelectWithCallback on_change=move |ev| {
          let view = match ev.target().value().as_str() {
            "Table" => DataView::Table,
            "Column Graph" => DataView::ColumnGraph,
            "Line Graph" => DataView::LineGraph,
            "Scatter Plot" => DataView::ScatterPlot,
            "Cumulative Line Graph" => DataView::CumLineGraph,
            "CSV" => DataView::Csv,
            _ => unreachable!(),
          };
          set_view.set(view)
        }>
          <option selected=true>"Table"</option>
          <option>"Column Graph"</option>
          <option>"Line Graph"</option>
          <option>"Scatter Plot"</option>
          <option>"Cumulative Line Graph"</option>
          <option>"CSV"</option>
        </SelectWithCallback>
        <Input
          input_ref=query_ref
          value=Some(list.get().query)
          onclick=move |_| set_query.set(query_ref.get().map(|query| query.value()))
          error=error
          disabled=matches!(list.read().mode, ListMode::View(_))
        />
        {move || {
          data
            .get()
            .flatten()
            .map(|data| {
              view! { <DataViewRender view=view df=move || data.clone() set_error=set_error /> }
            })
        }}
      </div>
    }
}

#[derive(Params, PartialEq)]
struct ListParams {
    id: Option<String>,
}

enum ListState {
    Success(Box<List>),
    NotFound,
}

#[component]
pub fn ListComponent(view: ListsRoute, #[prop(into)] user: Signal<Option<User>>) -> impl IntoView {
    let params = use_params::<ListParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };
    let (mode, set_mode) = signal(ItemMode::View);
    let state = LocalResource::new(move || async move {
        if let Some(list) = crate::fetch_list(&id()).await.unwrap() {
            set_mode.set(if let ListMode::View(_) = &list.mode {
                ItemMode::View
            } else {
                ItemMode::Update
            });
            ListState::Success(Box::new(list))
        } else {
            ListState::NotFound
        }
    });

    let query = use_query_map();
    let view = move || match view {
        ListsRoute::View => ListPage::View,
        ListsRoute::List => ListPage::List,
        ListsRoute::Edit => ListPage::Edit,
        ListsRoute::Tournament => {
            if query.read().get("mode").as_deref() == Some("random") {
                ListPage::RandomTournament
            } else {
                ListPage::Tournament
            }
        }
        ListsRoute::Match => {
            if query.read().get("mode").as_deref() == Some("rounds") {
                ListPage::RandomRounds
            } else {
                ListPage::RandomMatches
            }
        }
    };
    let dropdown_html = {
        let view = view.clone();
        move || {
            let list = match &*state.read() {
                None => return None,
                Some(ListState::NotFound) => return None,
                Some(ListState::Success(list)) => list.clone(),
            };
            let toggle = match view() {
                ListPage::RandomMatches => "Random Matches",
                ListPage::RandomRounds => "Random Rounds",
                ListPage::Tournament => "Tournament",
                ListPage::RandomTournament => "Random Tournament",
                _ => "Rank",
            };
            // TODO: handle GROUP BY queries
            if let ListMode::View(_) = list.mode {
                None
            } else {
                Some(view! {
                  <button
                    popovertarget="list-dropdown"
                    class="flex gap-1 items-baseline text-black"
                  >
                    {toggle}
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                      class="size-2"
                    >
                      <polygon points="0,0 20,0 10,10" />
                    </svg>
                  </button>
                  <Dropdown id="list-dropdown".to_owned() direction=Direction::Down>
                    <a class="text-black" href=format!("/lists/{}/tournament", list.id)>
                      "Tournament"
                    </a>
                    <a class="text-black" href=format!("/lists/{}/tournament?mode=random", list.id)>
                      "Random Tournament"
                    </a>
                    <a class="text-black" href=format!("/lists/{}/match", list.id)>
                      "Random Matches"
                    </a>
                    <a class="text-black" href=format!("/lists/{}/match?mode=rounds", list.id)>
                      "Random Rounds"
                    </a>
                  </Dropdown>
                })
            }
        }
    };
    move || {
        let dropdown_html = dropdown_html.clone();
        let list = match &*state.read() {
            None => return None,
            Some(ListState::NotFound) => return None,
            Some(ListState::Success(list)) => list.clone(),
        };
        let list_signal = move || match &*state.read() {
            None => unreachable!(),
            Some(ListState::NotFound) => unreachable!(),
            Some(ListState::Success(list)) => *list.clone(),
        };
        let mut tabs = ["text-black"; 3];
        let active = "text-black";
        let view = view();
        match view {
            ListPage::View => tabs[0] = active,
            ListPage::List => tabs[1] = active,
            ListPage::Edit => tabs[2] = active,
            _ => {}
        }
        let component = if crate::user_list(&list, &user.read()) {
            match view {
                ListPage::View => view! { <ListView list=list_signal /> }.into_any(),
                ListPage::List => {
                    view! { <ListItems user=user list=list_signal mode=mode /> }.into_any()
                }
                ListPage::Edit => {
                    view! { <Edit logged_in=move || user.read().is_some() list=list_signal /> }
                        .into_any()
                }
                ListPage::RandomMatches => {
                    view! { <RandomMatches id=list.id.clone() /> }.into_any()
                }
                ListPage::RandomRounds => view! { <RandomRounds id=list.id.clone() /> }.into_any(),
                ListPage::RandomTournament => {
                    view! { <RandomTournamentLoader list=list_signal /> }.into_any()
                }
                ListPage::Tournament => view! { <TournamentLoader list=list_signal /> }.into_any(),
            }
        } else {
            match view {
                ListPage::View => view! { <ListView list=list_signal /> }.into_any(),
                ListPage::List => {
                    view! { <ListItems user=user list=list_signal mode=mode /> }.into_any()
                }
                // TODO: move this up?
                _ => crate::not_found().into_any(),
            }
        };
        let user = crate::user_list(&list, &user.read());
        // TODO: fix dropdown centering
        Some(view! {
          <Content
            heading=list.name.clone()
            nav=view! {
              <div class="flex flex-col lg:flex-row gap-6 justify-between items-baseline text-sm font-medium">
                <div class="flex gap-8 flex-col lg:flex-row items-baseline">
                  <a class=tabs[0] href=format!("/lists/{}", list.id)>
                    "View"
                  </a>
                  <a class=tabs[1] href=format!("/lists/{}/items", list.id)>
                    "Items"
                  </a>
                  {move || {
                    if user {
                      Some(
                        view! {
                          {dropdown_html.clone()}
                          <a class=tabs[2] href=format!("/lists/{}/edit", list_signal().id)>
                            "Settings"
                          </a>
                        },
                      )
                    } else {
                      None
                    }
                  }}
                </div>
                {move || {
                  if matches!(view, ListPage::List) && !matches!(list.mode, ListMode::View(_)) {
                    Some(
                      view! {
                        <div class="flex gap-4 items-baseline">
                          <span class="text-black text-nowrap">"Item Mode:"</span>
                          <SelectWithCallback on_change=move |ev| {
                            set_mode
                              .set(
                                match ev.target().value().as_str() {
                                  "Update" => ItemMode::Update,
                                  "Delete" => ItemMode::Delete,
                                  _ => unreachable!(),
                                },
                              )
                          }>
                            <option selected=true>"Update"</option>
                            <option>"Delete"</option>
                          </SelectWithCallback>
                        </div>
                      },
                    )
                  } else {
                    None
                  }
                }}
              </div>
            }
            content=view! {
              <>
                {move || {
                  if user {
                    None
                  } else {
                    Some(view! { <h3>{format!("{}'s list", list_signal().user_id)}</h3> })
                  }
                }} {component}
              </>
            }
          />
        })
    }
}
