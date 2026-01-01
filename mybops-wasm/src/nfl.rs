use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{Component, Context, Html, html};

#[derive(Debug, Deserialize)]
struct Scoreboard {
    week: Week,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Week {
    number: i32,
}

#[derive(Debug, Deserialize)]
struct Event {
    competitions: Vec<Competition>,
}

#[derive(Debug, Deserialize)]
struct Competition {
    competitors: Vec<Competitor>,
}

#[derive(Debug, Deserialize)]
struct Competitor {
    team: Team,
    score: String,
}

#[derive(Debug, Deserialize)]
struct Team {
    abbreviation: String,
}

pub enum NflMsg {
    Load(HashMap<String, HashMap<String, BTreeMap<i32, i32>>>),
}

pub struct Nfl {
    games: HashMap<String, HashMap<String, BTreeMap<i32, i32>>>,
}

impl Component for Nfl {
    type Message = NflMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let window = crate::window();
            let opts = RequestInit::new();
            opts.set_mode(RequestMode::Cors);
            let mut games: HashMap<String, HashMap<String, BTreeMap<i32, i32>>> = HashMap::new();
            for week in 1..19 {
                let request = Request::new_with_str_and_init(
                    &format!("https://site.api.espn.com/apis/site/v2/sports/football/nfl/scoreboard?week={}", week),
                    &opts,
                ).unwrap();
                let resp_value = JsFuture::from(window.fetch_with_request(&request))
                    .await
                    .unwrap();
                let resp: Response = resp_value.dyn_into().unwrap();
                let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
                let scoreboard: Scoreboard = serde_wasm_bindgen::from_value(json).unwrap();
                for event in scoreboard.events {
                    for competition in event.competitions {
                        let team1 = &competition.competitors[0];
                        let team2 = &competition.competitors[1];
                        games.entry(team1.team.abbreviation.clone()).or_default().entry(team2.team.abbreviation.clone()).or_default().insert(week, team1.score.parse().unwrap());
                        games.entry(team2.team.abbreviation.clone()).or_default().entry(team1.team.abbreviation.clone()).or_default().insert(week, team2.score.parse().unwrap());
                    }
                }
            }
            NflMsg::Load(games)
        });
        Nfl {
            games: HashMap::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NflMsg::Load(games) => {
                self.games = games;
                true
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let teams = self.games.keys().map(|team| html! { <div>{team}</div> });
        let games = self.games.keys().flat_map(|team1| {
            std::iter::once(html! { <div>{team1}</div> }).chain(self.games.keys().map(|team2| {
                let mut scores = Vec::new();
                for (week, score1) in self
                    .games
                    .get(team1)
                    .unwrap()
                    .get(team2)
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                {
                    let score2 = self
                        .games
                        .get(team2)
                        .unwrap()
                        .get(team1)
                        .unwrap()
                        .get(week)
                        .unwrap();
                    // Assume a game doesn't end 0-0
                    if *score1 == 0 && *score2 == 0 {
                        scores.push(html! { <div>{format!("Week {}", week)}</div> });
                        continue;
                    }
                    let score = format!("Week {}: {}-{}", week, score1, score2);
                    let class = match score1.cmp(score2) {
                        Ordering::Less => "text-danger",
                        Ordering::Equal => "text-warning",
                        Ordering::Greater => "text-success",
                    };
                    scores.push(html! { <div {class}>{score}</div> });
                }
                html! { <div>{for scores}</div> }
            }))
        });
        let html = html! {
          <div class="d-grid gap-3" style="grid-template-columns: repeat(33, max-content)">
            <div></div>
            {for teams}
            {for games}
          </div>
        };
        crate::nav_content(
            html! {
              <ul class="navbar-nav me-auto">
                <li class="navbar-brand">{"NFL"}</li>
              </ul>
            },
            html! {
              <div>
                <div class="row mt-3">
                  {html}
                </div>
              </div>
            },
        )
    }
}
