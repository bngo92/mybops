use crate::{
    Content, ListsRoute,
    base::Input,
    bootstrap::Modal,
    list::item::{ItemMode, ListItems},
    nfl::Nfl,
    plot::{DataView, DataViewRender},
};
use leptos::{either::Either, html, prelude::*};
use leptos_router::{
    components::*,
    hooks::{use_params, use_query_map},
    params::Params,
    path,
};
use mybops::{List, ListMode, User};
use std::{borrow::Borrow, collections::HashMap, rc::Rc};
use web_sys::{HtmlSelectElement, MouseEvent};

type RouteQuery = &'static [(&'static str, &'static str)];

pub enum ListPage {
    View,
    List,
    Edit,
    RandomMatches,
    RandomRounds,
    Tournament,
    RandomTournament,
}

/* fn switch(
    routes: Route,
    user: Rc<Option<User>>,
    list_dropdown: bool,
    show_list_dropdown: Rc<Callback<MouseEvent>>,
) -> Html {
    let logged_in = user.is_some();
    match routes {
        Route::Home => html! { <Home {logged_in}/> },
        Route::Docs => docs::docs(),
        Route::ListsRoot => html! { <list::Lists {logged_in}/> },
        Route::Lists => {
            let render = move |view| {
                html! {
                  <ListComponent {view} user={Rc::clone(&user)} dropdown={list_dropdown} show_dropdown={Rc::clone(&show_list_dropdown)}/>
                }
            };
            html! { <Switch<ListsRoute> {render}/> }
        }
        Route::Search => html! { <Search {logged_in}/> },
        Route::Settings => html! {
            if let Some(user) = (*user).clone() {
                <Settings {user}/>
            } else {
                <Redirect<Route> to={Route::Home}/>
            }
        },
        Route::Spotify => html! { <SpotifyIntegration {logged_in}/> },
        Route::Nfl => html! { <Nfl/> },
    }
} */

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
    let user = LocalResource::new(|| async { crate::get_user().await.ok() });
    let (sidebar, set_sidebar) = signal(false);
    let (login, set_login) = signal(false);
    let (dropdown, set_dropdown) = signal(false);
    let (list_dropdown, set_list_dropdown) = signal(false);
    let (integrations_dropdown, set_integrations_dropdown) = signal(false);

    // We need to check which dropdown is clicked instead of relying on stop_propagation
    // TODO: fix multiple open dropdowns
    // let reset_dropdown = move |_| {
    //     set_dropdown.set(false);
    //     set_list_dropdown.set(false);
    //     set_integrations_dropdown.set(false);
    // };
    let show_list_dropdown = move |e: MouseEvent| {
        e.stop_propagation();
        set_list_dropdown.set(!list_dropdown.get());
    };
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
        "nav-link active"
    } else */{
        "nav-link text-white"
    };
    let toggle_dropdown = move |e: MouseEvent| {
        // Prevent reset_dropdown from triggering
        e.stop_propagation();
        set_dropdown.set(!dropdown.get());
    };
    let int_dropdown = move |e: MouseEvent| {
        e.stop_propagation();
        set_integrations_dropdown.set(!integrations_dropdown.get());
    };
    let sidebar_class = move || {
        if sidebar.get() {
            "p-3 bg-dark flex-shrink-0 h-100 offcanvas-sm offcanvas-start text-bg-dark show"
        } else {
            "p-3 bg-dark flex-shrink-0 h-100 offcanvas-sm offcanvas-start text-bg-dark"
        }
    };
    view! {
      // <div on:click=reset_dropdown>
      <div>
        <nav class="navbar navbar-expand navbar-dark bg-dark d-sm-none">
          <div class="container-lg d-flex justify-content-start gap-3">
            <button
              type="button"
              class="border-0"
              style="background-color: transparent; color: rgba(255,255,255,0.85)"
              on:click=move |_| set_sidebar.set(true)
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                fill="currentColor"
                class="bi bi-list"
                viewBox="0 0 16 16"
              >
                <path
                  fill-rule="evenodd"
                  d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5"
                />
              </svg>
            </button>
            <a class="navbar-brand" href="/">
              "mybops"
            </a>
          </div>
        </nav>
        <div class="d-flex vh-100 min-vh-100 align-items-stretch">
          <div class=sidebar_class style="width: 200px;">
            <div class="h-100 offcanvas-body d-flex flex-column">
              <div class="d-flex gap-2 align-items-baseline" data-bs-theme="dark">
                <a class="text-white text-decoration-none fs-5" href="/">
                  "mybops"
                </a>
                <button
                  type="button"
                  class="btn-close d-sm-none"
                  on:click=move |_| set_sidebar.set(false)
                ></button>
              </div>
              <hr />
              <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                  <a class=search href="/lists">
                    "Lists"
                  </a>
                </li>
                <li class="nav-item">
                  <a class=search href="/search">
                    "Query"
                  </a>
                </li>
                <li class="nav-item dropdown">
                  <a
                    class=dropdown_class(integrations_dropdown.get()).0
                    href="#"
                    on:click=int_dropdown
                  >
                    "Integrations"
                  </a>
                  <ul class=move || dropdown_class(integrations_dropdown.get()).1>
                    <li>
                      <a class="dropdown-item" href="/integrations/spotify">
                        "Spotify"
                      </a>
                    </li>
                  </ul>
                </li>
                <li class="nav-item">
                  <a class=search href="/docs">
                    "Docs"
                  </a>
                </li>
              </ul>
              <hr />
              <div>
                <ul class="nav nav-pills flex-column">
                  {move || {
                    if let Some(user) = user.get().flatten() {
                      Either::Left(
                        view! {
                          <li class="nav-item dropdown">
                            <a
                              class=move || dropdown_class(dropdown.get()).0
                              href="#"
                              on:click=toggle_dropdown
                            >
                              {user.user_id}
                            </a>
                            <ul
                              class=move || dropdown_class(dropdown.get()).1
                              style="inset: auto auto 0px 0px; transform: translate3d(0px, -34px, 0px)"
                            >
                              <li>
                                <a class="dropdown-item" href="/settings">
                                  "Settings"
                                </a>
                              </li>
                              <li>
                                <a class="dropdown-item" href="/api/logout" rel="external">
                                  "Log out"
                                </a>
                              </li>
                            </ul>
                          </li>
                        },
                      )
                    } else {
                      Either::Right(
                        view! {
                          <li class="nav-item">
                            <a class=search href="#" on:click=move |_| set_login.set(true)>
                              "Log in"
                            </a>
                          </li>
                        },
                      )
                    }
                  }}
                </ul>
              </div>
            </div>
          </div>
          <div class="flex-grow-1 h-100 w-100 d-flex flex-column">
            <Routes fallback=crate::not_found>
              <ParentRoute path=path!("/lists") view=Outlet>
                <Route
                  path=path!(":id")
                  view=move || {
                    view! {
                      <ListComponent
                        view=ListsRoute::View
                        user=move || user.get().flatten()
                        dropdown=list_dropdown
                        show_dropdown=show_list_dropdown
                      />
                    }
                  }
                />
              </ParentRoute>
              <Route path=path!("/nfl") view=Nfl />
            </Routes>
          </div>
          {move || {
            if login.get() {
              let origin = location().origin().unwrap();
              Some(
                view! {
                  <Modal header="Log in".to_owned() hide=move |_| set_login.set(false)>
                    <div class="modal-body d-grid gap-2">
                      <a
                        class="btn btn-success"
                        href=format!(
                          "https://accounts.spotify.com/authorize?client_id=ee3d1b4f8d80477ea48743a511ef3018&redirect_uri={}/api/login&response_type=code&scope=playlist-modify-public playlist-modify-private user-read-recently-played playlist-read-private",
                          origin.as_str(),
                        )
                      >
                        "Log in with Spotify"
                      </a>
                      <a
                        class="btn btn-success"
                        href=format!(
                          "https://accounts.google.com/o/oauth2/v2/auth?client_id=1038220726403-n55jha2cvprd8kdb4akdfvo0uiok4p5u.apps.googleusercontent.com&redirect_uri={}/api/login/google&response_type=code&scope=email",
                          origin.as_str(),
                        )
                      >
                        "Log in with Google"
                      </a>
                    </div>
                  </Modal>
                },
              )
            } else {
              None
            }
          }}
        </div>
      </div>
    }
}

fn dropdown_class(dropdown: bool) -> (&'static str, &'static str) {
    match dropdown {
        true => (
            "nav-link dropdown-toggle show text-white",
            "dropdown-menu dropdown-menu-dark show",
        ),
        false => (
            "nav-link dropdown-toggle text-white",
            "dropdown-menu dropdown-menu-dark",
        ),
    }
}

#[component]
fn ListView(#[prop(into)] list: Signal<List>) -> impl IntoView {
    let (view, set_view) = signal(DataView::Table);
    let (query, set_query) = signal(None);
    let (error, set_error) = signal(None);
    let query_ref = NodeRef::<html::Input>::new();

    let data = LocalResource::new(move || {
        let list = list.clone();
        async move {
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
        }
    });

    view! {
      <div class="row">
        <div class="col-auto">
          <select
            class="form-select mb-3"
            on:change:target=move |ev| {
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
            }
          >
            <option selected=true>"Table"</option>
            <option>"Column Graph"</option>
            <option>"Line Graph"</option>
            <option>"Scatter Plot"</option>
            <option>"Cumulative Line Graph"</option>
            <option>"CSV"</option>
          </select>
        </div>
        <Input
          input_ref=query_ref
          value=if matches!(list.read().mode, ListMode::View(_)) {
            Some(list.get().query)
          } else {
            None
          }
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
pub fn ListComponent(
    view: ListsRoute,
    #[prop(into)] user: Signal<Option<User>>,
    dropdown: ReadSignal<bool>,
    show_dropdown: impl FnMut(MouseEvent) + 'static + Send + Clone,
) -> impl IntoView {
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

    let show_dropdown = show_dropdown.clone();
    move || {
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
        let query = use_query_map();
        let view = match view {
            ListsRoute::View { .. } => ListPage::View,
            ListsRoute::List { .. } => ListPage::List,
            ListsRoute::Edit { .. } => ListPage::Edit,
            ListsRoute::Tournament { .. } => {
                if query.read().get("mode").as_deref() == Some("random") {
                    ListPage::RandomTournament
                } else {
                    ListPage::Tournament
                }
            }
            ListsRoute::Match { .. } => {
                if query.read().get("mode").as_deref() == Some("rounds") {
                    ListPage::RandomRounds
                } else {
                    ListPage::RandomMatches
                }
            }
        };
        let mut tabs = ["nav-link"; 3];
        let active = "nav-link active";
        match view {
            ListPage::View => tabs[0] = active,
            ListPage::List => tabs[1] = active,
            ListPage::Edit => tabs[2] = active,
            _ => {}
        }
        let component = if crate::user_list(&list, &*user.read()) {
            match view {
                ListPage::View => view! { <ListView list=list_signal /> }.into_any(),
                // ListPage::List => {
                //     view! { <ListItems user={Rc::clone(user)} list={*list.clone()} mode={(*mode).clone()}/> }
                // }
                // ListPage::Edit => {
                //     view! { <Edit logged_in={user.is_some()} list={*list.clone()}/> }
                // }
                // ListPage::RandomMatches => view! { <RandomMatches id={list.id.clone()}/> },
                // ListPage::RandomRounds => view! { <RandomRounds id={list.id.clone()}/> },
                // ListPage::RandomTournament => {
                //     view! { <RandomTournamentLoader list={*list.clone()}/> }
                // }
                // ListPage::Tournament => view! { <TournamentLoader list={*list.clone()}/> },
                _ => todo!(),
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
        let toggle = match view {
            ListPage::RandomMatches => "Random Matches",
            ListPage::RandomRounds => "Random Rounds",
            ListPage::Tournament => "Tournament",
            ListPage::RandomTournament => "Random Tournament",
            _ => "Rank",
        };
        let toggle_class = match (toggle, dropdown.get()) {
            ("Rank", false) => "nav-link dropdown-toggle",
            ("Rank", true) => "nav-link dropdown-toggle show",
            (_, false) => "nav-link active dropdown-toggle",
            (_, true) => "nav-link active dropdown-toggle show",
        };
        let menu_class = if dropdown.get() {
            "dropdown-menu show"
        } else {
            "dropdown-menu"
        };
        // TODO: handle GROUP BY queries
        let show_dropdown = show_dropdown.clone();
        let dropdown_html = move || {
            if let ListMode::View(_) = list_signal().mode {
                None
            } else {
                Some(view! {
                  <li class="nav-item dropdown">
                    <a class=toggle_class href="#" on:click=show_dropdown.clone()>
                      {toggle}
                    </a>
                    <ul class=menu_class>
                      <li>
                        <a>
                          classes="dropdown-item"href=format!("/lists/{}/tournament", list.id)>
                          "Tournament"
                        </a>
                        >
                      </li>
                      <li>
                        <a>
                          classes="dropdown-item"
                          href=format!("/lists/{}/tournament?mode=random", list.id)>
                          "Random Tournament"
                        </a>
                      </li>
                      <li>
                        <a>
                          classes="dropdown-item"href=format!("/lists/{}/tournament", list.id)>
                          "Random Matches"
                        </a>
                        >
                      </li>
                      <li>
                        <a>
                          classes="dropdown-item"
                          href=format!("/lists/{}/tournament?mode=rounds", list.id)>"Random Rounds"
                        </a>
                      </li>
                    </ul>
                  </li>
                })
            }
        };
        let user = crate::user_list(&list, &*user.read());
        Some(view! {
          <Content
            heading=list.name.clone()
            nav=view! {
              <>
                <ul class="navbar-nav me-auto">
                  <li class="nav-item">
                    <a class=tabs[0] href=format!("/lists/{}", list.id)>
                      "View"
                    </a>
                  </li>
                  <li class="nav-item">
                    <a class=tabs[1] href=format!("/lists/{}/items", list.id)>
                      "Items"
                    </a>
                  </li>
                  {move || {
                    if user {
                      Some(
                        view! {
                          {dropdown_html.clone()}
                          <li class="nav-item">
                            <a class=tabs[2] href=format!("/lists/{}/edit", list_signal().id)>
                              "Settings"
                            </a>
                          </li>
                        },
                      )
                    } else {
                      None
                    }
                  }}
                </ul>
                {move || {
                  if matches!(view, ListPage::List) && !matches!(list.mode, ListMode::View(_)) {
                    Some(
                      view! {
                        <div class="d-flex gap-3 align-items-baseline">
                          <span class="navbar-text text-nowrap">"Item Mode:"</span>
                          <select
                            class="form-select"
                            on:input:target=move |ev| {
                              set_mode
                                .set(
                                  match ev.target().value().as_str() {
                                    "Update" => ItemMode::Update,
                                    "Delete" => ItemMode::Delete,
                                    _ => unreachable!(),
                                  },
                                )
                            }
                          >
                            <option selected=true>"Update"</option>
                            <option>"Delete"</option>
                          </select>
                        </div>
                      },
                    )
                  } else {
                    None
                  }
                }}
              </>
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
