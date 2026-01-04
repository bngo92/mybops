use std::{
    array,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, Request, RequestInit, RequestMode, Response};
use yew::{Component, Context, Html, NodeRef, html};

#[derive(Debug, Deserialize)]
pub struct Scoreboard {
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
    status: Status,
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

#[derive(Debug, Deserialize)]
struct Status {
    r#type: StatusType,
}

#[derive(Debug, Deserialize)]
struct StatusType {
    name: String,
}

pub enum NflMsg {
    Load(
        HashMap<String, HashMap<String, BTreeMap<i32, (i32, String)>>>,
        HashMap<String, (u32, u32, u32)>,
    ),
    Select,
    Clear,
}

pub struct Nfl {
    selected_teams: Vec<&'static str>,
    refs: [NodeRef; Self::TEAMS.len()],
    games: HashMap<String, HashMap<String, BTreeMap<i32, (i32, String)>>>,
    records: HashMap<String, (u32, u32, u32)>,
}

impl Nfl {
    const TEAMS: [&'static str; 32] = [
        "BUF", "MIA", "NYJ", "NE", // AFC East
        "BAL", "PIT", "CIN", "CLE", // AFC North
        "HOU", "IND", "JAX", "TEN", // AFC South
        "KC", "LAC", "DEN", "LV", // AFC West
        "PHI", "WSH", "DAL", "NYG", // NFC East
        "DET", "MIN", "GB", "CHI", // NFC North
        "TB", "ATL", "CAR", "NO", // NFC South
        "LAR", "SEA", "ARI", "SF", // NFC West
    ];
}

impl Component for Nfl {
    type Message = NflMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            let window = crate::window();
            let opts = RequestInit::new();
            opts.set_mode(RequestMode::Cors);
            let mut games: HashMap<String, HashMap<String, BTreeMap<i32, (i32, String)>>> = HashMap::new();
            let mut records: HashMap<String, (u32, u32, u32)> = HashMap::new();
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
                        let score1: i32 = team1.score.parse().unwrap();
                        let score2: i32 = team2.score.parse().unwrap();
                        if competition.status.r#type.name == "STATUS_FINAL" {
                            let cmp = score1.cmp(&score2);
                            let record1 = records.entry(team1.team.abbreviation.clone()).or_default();
                            match cmp {
                                Ordering::Less => record1.1 += 1,
                                Ordering::Equal => record1.2 += 1,
                                Ordering::Greater => record1.0 += 1,
                            }
                            let record2 = records.entry(team2.team.abbreviation.clone()).or_default();
                            match cmp {
                                Ordering::Less => record2.0 += 1,
                                Ordering::Equal => record2.2 += 1,
                                Ordering::Greater => record2.1 += 1,
                            }
                        }
                        games
                            .entry(team1.team.abbreviation.clone())
                            .or_default()
                            .entry(team2.team.abbreviation.clone())
                            .or_default()
                            .insert(
                                week,
                                (score1, competition.status.r#type.name.clone()),
                            );
                        games
                            .entry(team2.team.abbreviation.clone())
                            .or_default()
                            .entry(team1.team.abbreviation.clone())
                            .or_default()
                            .insert(
                                week,
                                (score2, competition.status.r#type.name),
                            );
                    }
                }
            }
            NflMsg::Load(games, records)
        });
        Nfl {
            selected_teams: Vec::new(),
            refs: array::from_fn(|_| NodeRef::default()),
            games: HashMap::new(),
            records: HashMap::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NflMsg::Load(games, records) => {
                self.games = games;
                self.records = records;
                true
            }
            NflMsg::Select => {
                self.selected_teams = self
                    .refs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, team_ref)| {
                        if team_ref.cast::<HtmlInputElement>().unwrap().checked() {
                            Some(Self::TEAMS[i])
                        } else {
                            None
                        }
                    })
                    .collect();
                true
            }
            NflMsg::Clear => {
                self.selected_teams.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected = !self.selected_teams.is_empty();
        let teams = if selected {
            &self.selected_teams
        } else {
            Self::TEAMS.as_slice()
        };
        let mut play_count = HashMap::new();
        let mut common_games: HashMap<&'static str, (u32, u32, u32)> = HashMap::new();
        if selected {
            for team in teams {
                if let Some(opponents) = self.games.get(*team) {
                    for opponent in opponents.keys() {
                        *play_count.entry(opponent.as_str()).or_default() += 1;
                    }
                }
            }
            for team2 in Self::TEAMS {
                for team1 in teams {
                    for (week, (score1, status)) in self
                        .games
                        .get(*team1)
                        .cloned()
                        .unwrap_or_default()
                        .get(team2)
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                    {
                        if status != "STATUS_FINAL" {
                            continue;
                        }
                        let (score2, _) = self.games[team2][*team1][week];
                        if play_count.get(team2) == Some(&self.selected_teams.len()) {
                            let record = common_games.entry(team1).or_default();
                            match score1.cmp(&score2) {
                                Ordering::Less => record.1 += 1,
                                Ordering::Equal => record.2 += 1,
                                Ordering::Greater => record.0 += 1,
                            }
                        }
                    }
                }
            }
        }
        let header = teams.iter().zip(&self.refs).map(|(team, team_ref)| {
            let (wins, losses, ties) = self.records.get(*team).unwrap_or(&(0, 0, 0));
            let team_record = if *ties != 0 {
                format!("{team} ({wins}-{losses}-{ties})")
            } else {
                format!("{team} ({wins}-{losses})")
            };
            if selected {
                let (wins, losses, ties) = common_games.get(*team).unwrap_or(&(0, 0, 0));
                let common_games = if *ties != 0 {
                    format!("{team} ({wins}-{losses}-{ties})")
                } else {
                    format!("{team} ({wins}-{losses})")
                };
                html! {
                  <div>
                    <div>{team_record}</div>
                    <div><strong>{common_games}</strong></div>
                  </div>
                }
            } else {
                html! {
                  <div class="form-check">
                    <label class="form-check-label">{team_record}</label>
                    <input ref={team_ref} class="form-check-input" type="checkbox"/>
                  </div>
                }
            }
        });
        let mut games = Vec::new();
        for team2 in Self::TEAMS {
            if selected && !play_count.contains_key(team2) {
                continue;
            }
            games.push(html! { <div>{team2}</div> });
            for team1 in teams {
                let mut scores = Vec::new();
                for (week, (score1, status)) in self
                    .games
                    .get(*team1)
                    .cloned()
                    .unwrap_or_default()
                    .get(team2)
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                {
                    let (score2, _) = self.games[team2][*team1][week];
                    if status != "STATUS_FINAL" {
                        let score = format!("Week {}", week);
                        let score = if play_count.get(team2) == Some(&self.selected_teams.len()) {
                            html! { <div><strong>{score}</strong></div> }
                        } else {
                            html! { <div>{score}</div> }
                        };
                        scores.push(score);
                        continue;
                    }
                    let score = format!("Week {}: {}-{}", week, score1, score2);
                    let class = match score1.cmp(&score2) {
                        Ordering::Less => "text-danger",
                        Ordering::Equal => "text-warning",
                        Ordering::Greater => "text-success",
                    };
                    let score = if play_count.get(team2) == Some(&self.selected_teams.len()) {
                        html! { <div {class}><strong>{score}</strong></div> }
                    } else {
                        html! { <div {class}>{score}</div> }
                    };
                    scores.push(score);
                }
                games.push(html! { <div>{for scores}</div> });
            }
        }
        let style = format!(
            "grid-template-columns: repeat({}, max-content)",
            teams.len() + 1
        );
        let html = html! {
          <div class="d-grid gap-3" {style}>
            <div></div>
            {for header}
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
                  <div>
                    <button type="button" class="btn btn-info" onclick={ctx.link().callback(move |_| if selected {NflMsg::Clear} else {NflMsg::Select})}>
                    {if selected {"Clear"} else {"Select"}}
                    </button>
                  </div>
                  {html}
                </div>
              </div>
            },
        )
    }
}
