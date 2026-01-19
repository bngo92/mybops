use crate::base::IframeCompare;
use leptos::prelude::*;
use mybops::{ItemMetadata, Items};
use rand::prelude::SliceRandom;
use std::borrow::Cow;

#[component]
pub fn RandomMatches(id: String) -> impl IntoView {
    view! { <Match id=id mode=Mode::Match /> }
}

#[component]
pub fn RandomRounds(id: String) -> impl IntoView {
    view! { <Match id=id mode=Mode::Round /> }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Match,
    Round,
}

#[derive(Clone)]
struct MatchData {
    left: ItemMetadata,
    right: ItemMetadata,
    query: Items,
}

#[component]
pub fn Match(id: String, mode: Mode) -> impl IntoView {
    let (_, set_random_queue) = signal(Vec::new());
    let (data, set_data) = signal(None);
    let update = move |query: Items| {
        match mode {
            Mode::Round => {
                set_random_queue.update(|queue| {
                    match queue.len() {
                        // Reload the queue if it's empty
                        0 => {
                            let mut items = query.items.clone();
                            items.shuffle(&mut rand::thread_rng());
                            queue.extend(items);
                        }
                        // Always queue the last song next before reloading
                        1 => {
                            let last = queue.pop().unwrap();
                            let mut items = query.items.clone();
                            items.shuffle(&mut rand::thread_rng());
                            queue.extend(items);
                            queue.push(last);
                        }
                        _ => {}
                    }
                    let (left, right) =
                        (queue.pop().unwrap().unwrap(), queue.pop().unwrap().unwrap());
                    set_data.set(Some(MatchData { left, right, query }))
                })
            }
            Mode::Match => {
                let mut queued_scores: Vec<_> = query.items.iter().collect();
                queued_scores.shuffle(&mut rand::thread_rng());
                let (left, right) = (
                    queued_scores.pop().unwrap().clone().unwrap(),
                    queued_scores.pop().unwrap().clone().unwrap(),
                );
                set_data.set(Some(MatchData { left, right, query }));
            }
        };
    };

    {
        let id = id.clone();
        LocalResource::new(move || {
            let id = id.clone();
            async move {
                let query = crate::get_items(&id).await.unwrap();
                update(query);
            }
        })
    };

    let update_stats = Action::new_unsync(move |(win, lose): &(String, String)| {
        let list = id.clone();
        let win = win.clone();
        let lose = lose.clone();
        async move {
            crate::update_stats(&list, &win, &lose).await.unwrap();
            update(crate::get_items(&list).await.unwrap());
        }
    });

    move || {
        let Some(MatchData { left, right, query }) = &*data.read() else {
            return None;
        };
        let left_param = (left.id.clone(), right.id.clone());
        let on_left_select = move |_| {
            update_stats.dispatch(left_param.clone());
        };
        let right_param = (right.id.clone(), left.id.clone());
        let on_right_select = move |_| {
            update_stats.dispatch(right_param.clone());
        };
        let items = query
            .items
            .iter()
            .zip(1..)
            .map(|(item, i)| {
                item.as_ref().map(|m| {
                    (
                        i,
                        Cow::from(vec![
                            m.name.to_owned(),
                            format!("{}-{}", m.wins, m.losses),
                            m.score.to_string(),
                        ]),
                    )
                })
            })
            .collect();
        Some(view! {
          <div>
            <IframeCompare
              left=left.clone()
              on_left_select=on_left_select
              right=right.clone()
              on_right_select=on_right_select
            />
            {crate::base::responsive_table_view(&["Track", "Record", "Score"], items)}
          </div>
        })
    }
}
