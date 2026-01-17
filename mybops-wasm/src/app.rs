use crate::{
    Content,
    ListsRoute,
    // Route,
    // base::Input,
    bootstrap::Modal,
    // dataframe::DataFrame,
    // docs,
    // edit::Edit,
    // home::Home,
    // integrations::spotify::SpotifyIntegration,
    // list,
    // list::item::{ItemMode, ListItems},
    nfl::Nfl,
    // plot::DataView,
    // random::{RandomMatches, RandomRounds},
    // search::Search,
    // settings::Settings,
    // tournament::{RandomTournamentLoader, TournamentLoader},
};
use leptos::{either::Either, prelude::*};
use leptos_router::{components::*, path};
use mybops::{List, ListMode, User};
use std::{borrow::Borrow, collections::HashMap, rc::Rc};
use web_sys::MouseEvent;

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
    let reset_dropdown = move |_| {
        set_dropdown.set(false);
        set_list_dropdown.set(false);
        set_integrations_dropdown.set(false);
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
      <div on:click=reset_dropdown>
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

/* enum ListViewMsg {
    Success(Option<DataFrame>),
    Failed(String),
    Select,
    Query,
}

#[derive(PartialEq, Properties)]
pub struct ListViewProps {
    list: List,
}

struct ListView {
    data: Option<DataFrame>,
    select_ref: NodeRef,
    view: DataView,
    query_ref: NodeRef,
    error: Option<String>,
}

impl Component for ListView {
    type Message = ListViewMsg;
    type Properties = ListViewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let list = ctx.props().list.clone();
        ctx.link().send_future(async move {
            match crate::query_list(&list, None).await {
                Ok(data) => ListViewMsg::Success(data),
                Err(e) => ListViewMsg::Failed(e.as_string().unwrap()),
            }
        });
        Self {
            data: None,
            select_ref: NodeRef::default(),
            view: DataView::Table,
            query_ref: NodeRef::default(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ListViewMsg::Success(data) => {
                let Some(data) = data else {
                    return false;
                };
                self.data = {
                    let mut data = data.clone();
                    data.drop_in_place("id");
                    Some(data)
                };
                self.error = None;
            }
            ListViewMsg::Failed(e) => {
                self.error = Some(e);
            }
            ListViewMsg::Select => {
                let view = self.select_ref.cast::<HtmlSelectElement>().unwrap().value();
                self.view = match &*view {
                    "Table" => DataView::Table,
                    "Column Graph" => DataView::ColumnGraph,
                    "Line Graph" => DataView::LineGraph,
                    "Scatter Plot" => DataView::ScatterPlot,
                    "Cumulative Line Graph" => DataView::CumLineGraph,
                    "CSV" => DataView::Csv,
                    _ => unreachable!(),
                };
            }
            ListViewMsg::Query => {
                let query = self.query_ref.cast::<HtmlSelectElement>().unwrap().value();
                let list = ctx.props().list.clone();
                ctx.link().send_future(async move {
                    match crate::query_list(&list, Some(query)).await {
                        Ok(data) => ListViewMsg::Success(data),
                        Err(e) => ListViewMsg::Failed(e.as_string().unwrap()),
                    }
                });
            }
        }
        if let Some(data) = &self.data
            && let Err(e) = self.view.draw(data)
        {
            self.error = Some(e.to_string());
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(|_| ListViewMsg::Select);
        let query = ctx.link().callback(|_| ListViewMsg::Query);
        html! {
            <div class="row">
                <div class="col-auto">
                    <select ref={self.select_ref.clone()} class="form-select mb-3" {onchange}>
                        <option selected=true>{"Table"}</option>
                        <option>{"Column Graph"}</option>
                        <option>{"Line Graph"}</option>
                        <option>{"Scatter Plot"}</option>
                        <option>{"Cumulative Line Graph"}</option>
                        <option>{"CSV"}</option>
                    </select>
                </div>
                <Input input_ref={self.query_ref.clone()} onclick={query.clone()} error={self.error.clone()} disabled={matches!(ctx.props().list.mode, ListMode::View(_))}/>
                if let Some(data) = &self.data {
                    {self.view.render(data)}
                }
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render || matches!(ctx.props().list.mode, ListMode::View(_)) {
            let query = self.query_ref.cast::<HtmlSelectElement>().unwrap();
            query.set_value(&ctx.props().list.query);
        }
    }
}

enum ListState {
    Success(Box<List>),
    NotFound,
}

#[derive(PartialEq, Properties)]
pub struct ListProps {
    pub view: ListsRoute,
    pub user: Rc<Option<User>>,
    pub dropdown: bool,
    pub show_dropdown: Rc<Callback<MouseEvent>>,
}

#[function_component]
pub fn ListComponent(
    ListProps {
        view,
        user,
        dropdown,
        show_dropdown,
    }: &ListProps,
) -> HtmlResult {
    let id = match view {
        ListsRoute::List { id }
        | ListsRoute::View { id }
        | ListsRoute::Edit { id }
        | ListsRoute::Match { id }
        | ListsRoute::Tournament { id } => id.clone(),
    };
    let select_ref = use_node_ref();
    let (ref state, ref mode) = *use_future(|| async move {
        if let Some(list) = crate::fetch_list(&id).await.unwrap() {
            let mode = if let ListMode::View(_) = list.mode {
                ItemMode::View
            } else {
                ItemMode::Update
            };
            (ListState::Success(Box::new(list)), mode)
        } else {
            (ListState::NotFound, ItemMode::View)
        }
    })?;
    let mode = use_state(|| mode.clone());
    let location = use_location();
    let select_view = {
        let select_ref = select_ref.clone();
        let mode = mode.clone();
        Callback::from(move |_| {
            mode.set(
                match select_ref
                    .cast::<HtmlSelectElement>()
                    .map(|s| s.value())
                    .as_deref()
                    .unwrap_or("Update")
                {
                    "Update" => ItemMode::Update,
                    "Delete" => ItemMode::Delete,
                    _ => unreachable!(),
                },
            )
        })
    };

    let list = match state {
        ListState::NotFound => return Ok(crate::not_found()),
        ListState::Success(list) => list,
    };
    let query = location
        .unwrap()
        .query::<HashMap<String, String>>()
        .unwrap_or_default();
    let view = match view.clone() {
        ListsRoute::View { .. } => ListPage::View,
        ListsRoute::List { .. } => ListPage::List,
        ListsRoute::Edit { .. } => ListPage::Edit,
        ListsRoute::Tournament { .. } => {
            if query.get("mode").map(String::as_str) == Some("random") {
                ListPage::RandomTournament
            } else {
                ListPage::Tournament
            }
        }
        ListsRoute::Match { .. } => {
            if query.get("mode").map(String::as_str) == Some("rounds") {
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
    let component = if crate::user_list(list, user) {
        match view {
            ListPage::View => html! { <ListView list={*list.clone()}/> },
            ListPage::List => {
                html! { <ListItems user={Rc::clone(user)} list={*list.clone()} mode={(*mode).clone()}/> }
            }
            ListPage::Edit => {
                html! { <Edit logged_in={user.is_some()} list={*list.clone()}/> }
            }
            ListPage::RandomMatches => html! { <RandomMatches id={list.id.clone()}/> },
            ListPage::RandomRounds => html! { <RandomRounds id={list.id.clone()}/> },
            ListPage::RandomTournament => {
                html! { <RandomTournamentLoader list={*list.clone()}/> }
            }
            ListPage::Tournament => html! { <TournamentLoader list={*list.clone()}/> },
        }
    } else {
        match view {
            ListPage::View => html! { <ListView list={*list.clone()}/> },
            ListPage::List => {
                html! { <ListItems user={Rc::clone(user)} list={*list.clone()} mode={(*mode).clone()}/> }
            }
            // TODO: move this up?
            _ => crate::not_found(),
        }
    };
    let toggle = match view {
        ListPage::RandomMatches => "Random Matches",
        ListPage::RandomRounds => "Random Rounds",
        ListPage::Tournament => "Tournament",
        ListPage::RandomTournament => "Random Tournament",
        _ => "Rank",
    };
    let toggle_class = match (toggle, dropdown) {
        ("Rank", false) => "nav-link dropdown-toggle",
        ("Rank", true) => "nav-link dropdown-toggle show",
        (_, false) => "nav-link active dropdown-toggle",
        (_, true) => "nav-link active dropdown-toggle show",
    };
    let menu_class = if *dropdown {
        "dropdown-menu show"
    } else {
        "dropdown-menu"
    };
    // TODO: handle GROUP BY queries
    let dropdown_html = if let ListMode::View(_) = list.mode {
        html! {}
    } else {
        html! {
            <li class="nav-item dropdown">
                <a class={toggle_class} href="#" onclick={Borrow::<Callback<_>>::borrow(show_dropdown).clone()}>{toggle}</a>
                <ul class={menu_class}>
                    <li><Link<ListsRoute> classes="dropdown-item" to={ListsRoute::Tournament{ id: list.id.clone() }}>{"Tournament"}</Link<ListsRoute>></li>
                    <li><Link<ListsRoute, RouteQuery> classes="dropdown-item" to={ListsRoute::Tournament{ id: list.id.clone() }} query={Some(&[("mode", "random")][..])}>{"Random Tournament"}</Link<ListsRoute, RouteQuery>></li>
                    <li><Link<ListsRoute> classes="dropdown-item" to={ListsRoute::Match{ id: list.id.clone() }}>{"Random Matches"}</Link<ListsRoute>></li>
                    <li><Link<ListsRoute, RouteQuery> classes="dropdown-item" to={ListsRoute::Match{ id: list.id.clone() }} query={Some(&[("mode", "rounds")][..])}>{"Random Rounds"}</Link<ListsRoute, RouteQuery>></li>
                </ul>
            </li>
        }
    };
    let user = crate::user_list(list, user);
    Ok(html! {
      <Content
        heading={list.name.clone()}
        nav={html! {
          <>
            <ul class="navbar-nav me-auto">
              <li class="nav-item">
                <Link<ListsRoute> classes={tabs[0]} to={ListsRoute::View{id: list.id.clone()}}>{"View"}</Link<ListsRoute>>
              </li>
              <li class="nav-item">
                <Link<ListsRoute> classes={tabs[1]} to={ListsRoute::List{id: list.id.clone()}}>{"Items"}</Link<ListsRoute>>
              </li>
              if user {
                {dropdown_html}
                <li class="nav-item">
                  <Link<ListsRoute> classes={tabs[2]} to={ListsRoute::Edit{id: list.id.clone()}}>{"Settings"}</Link<ListsRoute>>
                </li>
              }
            </ul>
            if matches!(view, ListPage::List) && !matches!(list.mode, ListMode::View(_)) {
              <div class="d-flex gap-3 align-items-baseline">
                <span class="navbar-text text-nowrap">{"Item Mode:"}</span>
                <select ref={select_ref.clone()} class="form-select" onchange={select_view}>
                  <option selected=true>{"Update"}</option>
                  <option>{"Delete"}</option>
                </select>
              </div>
            }
          </>
        }}
        content={html! {
          <>
            if !user {
              <h3>{&format!("{}'s list", list.user_id)}</h3>
            }
            {component}
          </>
        }}/>
    })
} */
