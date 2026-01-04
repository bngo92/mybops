use std::{
    array,
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
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
    const CONFERENCES: [(&'static str, [(&'static str, [&'static str; 4]); 4]); 2] = [
        (
            "AFC",
            [
                ("AFC East", ["BUF", "MIA", "NYJ", "NE"]),
                ("AFC North", ["BAL", "PIT", "CIN", "CLE"]),
                ("AFC South", ["HOU", "IND", "JAX", "TEN"]),
                ("AFC West", ["KC", "LAC", "DEN", "LV"]),
            ],
        ),
        (
            "NFC",
            [
                ("NFC East", ["PHI", "WSH", "DAL", "NYG"]),
                ("NFC North", ["DET", "MIN", "GB", "CHI"]),
                ("NFC South", ["TB", "ATL", "CAR", "NO"]),
                ("NFC West", ["LAR", "SEA", "ARI", "SF"]),
            ],
        ),
    ];
    const DIVISIONS: [(&'static str, [&'static str; 4]); 8] = {
        let mut divisions = [("", [""; 4]); 8];
        let mut i = 0;
        while i < 2 {
            let mut j = 0;
            while j < 4 {
                divisions[4 * i + j] = Nfl::CONFERENCES[i].1[j];
                j += 1;
            }
            i += 1;
        }
        divisions
    };
    const TEAMS: [&'static str; 32] = {
        let mut teams = [""; 32];
        let mut i = 0;
        while i < 8 {
            let mut j = 0;
            while j < 4 {
                teams[4 * i + j] = Nfl::DIVISIONS[i].1[j];
                j += 1;
            }
            i += 1;
        }
        teams
    };

    fn get_conference(team: &str) -> Option<&'static str> {
        Self::CONFERENCES
            .iter()
            .copied()
            .find_map(|(conference, divisions)| {
                divisions.iter().copied().find_map(|(division, _)| {
                    if Some(division) == Self::get_division(team) {
                        Some(conference)
                    } else {
                        None
                    }
                })
            })
    }

    fn get_conference_divisions(
        conference: &str,
    ) -> Option<&'static [(&'static str, [&'static str; 4]); 4]> {
        Self::CONFERENCES.iter().find_map(|(c, divisions)| {
            if *c == conference {
                Some(divisions)
            } else {
                None
            }
        })
    }

    fn get_division(team: &str) -> Option<&'static str> {
        Self::DIVISIONS
            .iter()
            .copied()
            .find_map(|(division, teams)| {
                teams
                    .iter()
                    .copied()
                    .find_map(|t| if t == team { Some(division) } else { None })
            })
    }

    fn get_division_teams(division: &str) -> Option<&'static [&'static str; 4]> {
        Self::DIVISIONS
            .iter()
            .find_map(|(d, teams)| if *d == division { Some(teams) } else { None })
    }
}

impl Nfl {
    fn calculate_tiebreakers<'s>(
        &'s self,
        teams: &[&'static str],
        play_count: &mut HashMap<&'s str, usize>,
        head_to_head: &mut HashMap<&'static str, (u32, u32, u32)>,
        division_record: &mut HashMap<&'static str, (u32, u32, u32)>,
        common_games: &mut HashMap<&'static str, (u32, u32, u32)>,
        conference_record: &mut HashMap<&'static str, (u32, u32, u32)>,
    ) {
        let mut divisions = HashSet::new();
        for team in teams {
            divisions.insert(Self::get_division(team).unwrap());
            if let Some(opponents) = self.games.get(*team) {
                for opponent in opponents.keys() {
                    *play_count.entry(opponent.as_str()).or_default() += 1;
                }
            }
        }
        let divisions: Vec<_> = divisions.into_iter().collect();
        for team1 in teams {
            for team2 in teams {
                for (week, (score1, status)) in self
                    .games
                    .get(*team1)
                    .cloned()
                    .unwrap_or_default()
                    .get(*team2)
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                {
                    if status != "STATUS_FINAL" {
                        continue;
                    }
                    let (score2, _) = self.games[*team2][*team1][week];
                    let record = head_to_head.entry(team1).or_default();
                    match score1.cmp(&score2) {
                        Ordering::Less => record.1 += 1,
                        Ordering::Equal => record.2 += 1,
                        Ordering::Greater => record.0 += 1,
                    }
                }
            }
            for (division, teams) in
                Self::get_conference_divisions(Self::get_conference(team1).unwrap()).unwrap()
            {
                if let [d] = divisions.as_slice()
                    && d == division
                {
                    for team2 in teams {
                        for (week, (score1, status)) in self
                            .games
                            .get(*team1)
                            .cloned()
                            .unwrap_or_default()
                            .get(*team2)
                            .cloned()
                            .unwrap_or_default()
                            .iter()
                        {
                            if status != "STATUS_FINAL" {
                                continue;
                            }
                            let (score2, _) = self.games[*team2][*team1][week];
                            let record = division_record.entry(team1).or_default();
                            match score1.cmp(&score2) {
                                Ordering::Less => record.1 += 1,
                                Ordering::Equal => record.2 += 1,
                                Ordering::Greater => record.0 += 1,
                            }
                        }
                    }
                }
                for team2 in teams {
                    for (week, (score1, status)) in self
                        .games
                        .get(*team1)
                        .cloned()
                        .unwrap_or_default()
                        .get(*team2)
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                    {
                        if status != "STATUS_FINAL" {
                            continue;
                        }
                        let (score2, _) = self.games[*team2][*team1][week];
                        let record = conference_record.entry(team1).or_default();
                        match score1.cmp(&score2) {
                            Ordering::Less => record.1 += 1,
                            Ordering::Equal => record.2 += 1,
                            Ordering::Greater => record.0 += 1,
                        }
                    }
                }
            }
            for team2 in Self::TEAMS {
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
        let mut head_to_head = HashMap::new();
        let mut division_record = HashMap::new();
        let mut common_games = HashMap::new();
        let mut conference_record = HashMap::new();
        if selected {
            self.calculate_tiebreakers(
                teams,
                &mut play_count,
                &mut head_to_head,
                &mut division_record,
                &mut common_games,
                &mut conference_record,
            );
        }
        let team_records: HashMap<_, _> = Self::TEAMS
            .iter()
            .copied()
            .map(|team| {
                let (wins, losses, ties) = self.records.get(team).unwrap_or(&(0, 0, 0));
                if *ties != 0 {
                    (team, format!("{team} ({wins}-{losses}-{ties})"))
                } else {
                    (team, format!("{team} ({wins}-{losses})"))
                }
            })
            .collect();
        let header = teams.iter().zip(&self.refs).map(|(team, team_ref)| {
            let team_record = &team_records[team];
            if selected {
                let head_to_head = render_record(
                    "Head-to-Head",
                    head_to_head.get(*team).unwrap_or(&(0, 0, 0)),
                );
                let division_record = if let Some(record) = division_record.get(team) {
                    Some(render_record("Divison Record", record))
                } else {
                    None
                };
                let common_games = render_record(
                    "Common Games",
                    common_games.get(*team).unwrap_or(&(0, 0, 0)),
                );
                let conference_record = render_record(
                    "Conference Record",
                    conference_record.get(*team).unwrap_or(&(0, 0, 0)),
                );
                html! {
                  <div>
                    <div>{team_record}</div>
                    <div>{head_to_head}</div>
                    {division_record}
                    <div>{common_games}</div>
                    <div>{conference_record}</div>
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
            let team_record = &team_records[team2];
            let common_game = play_count.get(team2) == Some(&self.selected_teams.len());
            games.push(if common_game {
                html! { <div><strong>{team_record}</strong></div> }
            } else {
                html! { <div>{team_record}</div> }
            });
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
                    if status == "STATUS_SCHEDULED" {
                        let score = format!("Week {}", week);
                        let score = if common_game {
                            html! { <div><strong>{score}</strong></div> }
                        } else {
                            html! { <div>{score}</div> }
                        };
                        scores.push(score);
                        continue;
                    }
                    let score = format!("Week {}: {}-{}", week, score1, score2);
                    if status == "STATUS_IN_PROGRESS" {
                        let score = if common_game {
                            html! { <div><strong>{score}</strong></div> }
                        } else {
                            html! { <div>{score}</div> }
                        };
                        scores.push(score);
                        continue;
                    }
                    let class = match score1.cmp(&score2) {
                        Ordering::Less => "text-danger",
                        Ordering::Equal => "text-warning",
                        Ordering::Greater => "text-success",
                    };
                    let score = if common_game {
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

fn render_record(label: &str, (wins, losses, ties): &(u32, u32, u32)) -> String {
    if *ties != 0 {
        format!("{label}: {wins}-{losses}-{ties}")
    } else {
        format!("{label}: {wins}-{losses}")
    }
}
