use std::{
    array,
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
};

use leptos::{html, prelude::*};
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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

const CONFERENCES: [(&str, [(&str, [&str; 4]); 4]); 2] = [
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
const DIVISIONS: [(&str, [&str; 4]); 8] = {
    let mut divisions = [("", [""; 4]); 8];
    let mut i = 0;
    while i < 2 {
        let mut j = 0;
        while j < 4 {
            divisions[4 * i + j] = CONFERENCES[i].1[j];
            j += 1;
        }
        i += 1;
    }
    divisions
};
const TEAMS: [&str; 32] = {
    let mut teams = [""; 32];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 4 {
            teams[4 * i + j] = DIVISIONS[i].1[j];
            j += 1;
        }
        i += 1;
    }
    teams
};

fn get_conference(team: &str) -> Option<&'static str> {
    CONFERENCES
        .iter()
        .copied()
        .find_map(|(conference, divisions)| {
            divisions.iter().copied().find_map(|(division, _)| {
                if Some(division) == get_division(team) {
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
    CONFERENCES.iter().find_map(|(c, divisions)| {
        if *c == conference {
            Some(divisions)
        } else {
            None
        }
    })
}

fn get_division(team: &str) -> Option<&'static str> {
    DIVISIONS.iter().copied().find_map(|(division, teams)| {
        teams
            .iter()
            .copied()
            .find_map(|t| if t == team { Some(division) } else { None })
    })
}

fn get_division_teams(division: &str) -> Option<&'static [&'static str; 4]> {
    DIVISIONS
        .iter()
        .find_map(|(d, teams)| if *d == division { Some(teams) } else { None })
}

fn calculate_tiebreakers<'s>(
    games: &'s HashMap<String, HashMap<String, BTreeMap<i32, (i32, String)>>>,
    selected_teams: usize,
    teams: &[&'static str],
    play_count: &mut HashMap<&'s str, usize>,
    head_to_head: &mut HashMap<&'static str, (u32, u32, u32)>,
    division_record: &mut HashMap<&'static str, (u32, u32, u32)>,
    common_games: &mut HashMap<&'static str, (u32, u32, u32)>,
    conference_record: &mut HashMap<&'static str, (u32, u32, u32)>,
) {
    let mut divisions = HashSet::new();
    for team in teams {
        divisions.insert(get_division(team).unwrap());
        if let Some(opponents) = games.get(*team) {
            for opponent in opponents.keys() {
                *play_count.entry(opponent.as_str()).or_default() += 1;
            }
        }
    }
    let divisions: Vec<_> = divisions.into_iter().collect();
    for team1 in teams {
        for team2 in teams {
            for (week, (score1, status)) in games
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
                let (score2, _) = games[*team2][*team1][week];
                let record = head_to_head.entry(team1).or_default();
                match score1.cmp(&score2) {
                    Ordering::Less => record.1 += 1,
                    Ordering::Equal => record.2 += 1,
                    Ordering::Greater => record.0 += 1,
                }
            }
        }
        for (division, teams) in get_conference_divisions(get_conference(team1).unwrap()).unwrap() {
            if let [d] = divisions.as_slice()
                && d == division
            {
                for team2 in teams {
                    for (week, (score1, status)) in games
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
                        let (score2, _) = games[*team2][*team1][week];
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
                for (week, (score1, status)) in games
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
                    let (score2, _) = games[*team2][*team1][week];
                    let record = conference_record.entry(team1).or_default();
                    match score1.cmp(&score2) {
                        Ordering::Less => record.1 += 1,
                        Ordering::Equal => record.2 += 1,
                        Ordering::Greater => record.0 += 1,
                    }
                }
            }
        }
        for team2 in TEAMS {
            for (week, (score1, status)) in games
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
                let (score2, _) = games[team2][*team1][week];
                if play_count.get(team2) == Some(&selected_teams) {
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

#[component]
pub fn Nfl() -> impl IntoView {
    let (selected_teams, set_selected_teams) = signal(Vec::new());
    let refs = array::from_fn::<_, { TEAMS.len() }, _>(|_| NodeRef::<html::Input>::new());
    let games = LocalResource::new(|| async move {
        let window = crate::window();
        let opts = RequestInit::new();
        opts.set_mode(RequestMode::Cors);
        let mut games: HashMap<String, HashMap<String, BTreeMap<i32, (i32, String)>>> =
            HashMap::new();
        let mut records: HashMap<String, (u32, u32, u32)> = HashMap::new();
        for week in 1..19 {
            let request = Request::new_with_str_and_init(
                &format!(
                    "https://site.api.espn.com/apis/site/v2/sports/football/nfl/scoreboard?week={}",
                    week
                ),
                &opts,
            )
            .unwrap();
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
                        .insert(week, (score1, competition.status.r#type.name.clone()));
                    games
                        .entry(team2.team.abbreviation.clone())
                        .or_default()
                        .entry(team1.team.abbreviation.clone())
                        .or_default()
                        .insert(week, (score2, competition.status.r#type.name));
                }
            }
        }
        (games, records)
    });
    move || {
        let Some((games, records)) = games.get() else {
            return None;
        };
        let selected = !selected_teams.read().is_empty();
        let teams = if selected {
            &selected_teams.read()
        } else {
            TEAMS.as_slice()
        };
        let mut play_count = HashMap::new();
        let mut head_to_head = HashMap::new();
        let mut division_record = HashMap::new();
        let mut common_games = HashMap::new();
        let mut conference_record = HashMap::new();
        if selected {
            calculate_tiebreakers(
                &games,
                selected_teams.read().len(),
                teams,
                &mut play_count,
                &mut head_to_head,
                &mut division_record,
                &mut common_games,
                &mut conference_record,
            );
        }
        let team_records: HashMap<_, _> = TEAMS
            .iter()
            .copied()
            .map(|team| {
                let (wins, losses, ties) = records.get(team).unwrap_or(&(0, 0, 0));
                if *ties != 0 {
                    (team, format!("{team} ({wins}-{losses}-{ties})"))
                } else {
                    (team, format!("{team} ({wins}-{losses})"))
                }
            })
            .collect();
        let header = teams
            .iter()
            .zip(refs.iter())
            .map(|(team, team_ref)| {
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
                    view! {
                      <div>
                        <div>{team_record.clone()}</div>
                        <div>{head_to_head}</div>
                        {division_record}
                        <div>{common_games}</div>
                        <div>{conference_record}</div>
                      </div>
                    }
                    .into_any()
                } else {
                    view! {
                      <div class="form-check">
                        <label class="form-check-label">{team_record.clone()}</label>
                        <input node_ref=*team_ref class="form-check-input" type="checkbox" />
                      </div>
                    }
                    .into_any()
                }
            })
            .collect_view();
        let mut games_html = Vec::new();
        for team2 in TEAMS {
            if selected && !play_count.contains_key(team2) {
                continue;
            }
            let team_record = &team_records[team2];
            let common_game = play_count.get(team2) == Some(&selected_teams.read().len());
            games_html.push(if common_game {
                view! {
                  <div>
                    <strong>{team_record.clone()}</strong>
                  </div>
                }
                .into_any()
            } else {
                view! { <div>{team_record.clone()}</div> }.into_any()
            });
            for team1 in teams {
                let mut scores = Vec::new();
                for (week, (score1, status)) in games
                    .get(*team1)
                    .cloned()
                    .unwrap_or_default()
                    .get(team2)
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                {
                    let (score2, _) = games[team2][*team1][week];
                    if status == "STATUS_SCHEDULED" {
                        let score = format!("Week {}", week);
                        let score = if common_game {
                            view! {
                              <div>
                                <strong>{score}</strong>
                              </div>
                            }
                            .into_any()
                        } else {
                            view! { <div>{score}</div> }.into_any()
                        };
                        scores.push(score);
                        continue;
                    }
                    let score = format!("Week {}: {}-{}", week, score1, score2);
                    if status == "STATUS_IN_PROGRESS" {
                        let score = if common_game {
                            view! {
                              <div>
                                <strong>{score}</strong>
                              </div>
                            }
                            .into_any()
                        } else {
                            view! { <div>{score}</div> }.into_any()
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
                        view! {
                          <div class=class>
                            <strong>{score}</strong>
                          </div>
                        }
                        .into_any()
                    } else {
                        view! { <div class=class>{score}</div> }.into_any()
                    };
                    scores.push(score);
                }
                games_html.push(view! { <div>{scores}</div> }.into_any());
            }
        }
        let style = format!(
            "grid-template-columns: repeat({}, max-content)",
            teams.len() + 1
        );
        let html = view! {
          <div class="d-grid gap-3" style=style>
            <div></div>
            {header}
            {games_html}
          </div>
        };
        let onclick = move |_| {
            set_selected_teams.set(if selected {
                Vec::new()
            } else {
                refs.iter()
                    .enumerate()
                    .filter_map(|(i, team_ref)| {
                        if team_ref.get().unwrap().checked() {
                            Some(TEAMS[i])
                        } else {
                            None
                        }
                    })
                    .collect()
            })
        };
        Some(crate::nav_content(
            view! {
              <ul class="navbar-nav me-auto">
                <li class="navbar-brand">"NFL"</li>
              </ul>
            }
            .into_any(),
            view! {
              <div>
                <form class="d-flex">
                  <label class="col-form-label pe-2">"Sort by:"</label>
                  <select class="form-select" style="width: auto">
                    <option>"Previous Division Standings"</option>
                  </select>
                </form>
                <button type="button" class="btn btn-info" on:click=onclick>
                  {move || if selected { "Clear" } else { "Select" }}
                </button>
                {html}
              </div>
            }
            .into_any(),
        ))
    }
}

fn render_record(label: &str, (wins, losses, ties): &(u32, u32, u32)) -> String {
    if *ties != 0 {
        format!("{label}: {wins}-{losses}-{ties}")
    } else {
        format!("{label}: {wins}-{losses}")
    }
}
