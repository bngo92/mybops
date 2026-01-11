use crate::base::IframeCompare;
use mybops::{ItemMetadata, Items};
use rand::prelude::SliceRandom;
use std::borrow::Cow;
use yew::{
    Callback, Html, HtmlResult, Properties, function_component, html, suspense::use_future,
    use_state,
};

#[derive(Clone, PartialEq, Properties)]
pub struct MatchProps {
    pub id: String,
}

#[function_component]
pub fn RandomMatches(MatchProps { id }: &MatchProps) -> Html {
    html! {
        <Match id={id.clone()} mode={Mode::Match}/>
    }
}

#[function_component]
pub fn RandomRounds(MatchProps { id }: &MatchProps) -> Html {
    html! {
        <Match id={id.clone()} mode={Mode::Round}/>
    }
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

#[derive(Clone, PartialEq, Properties)]
pub struct MatchComponentProps {
    pub id: String,
    pub mode: Mode,
}

#[function_component]
pub fn Match(MatchComponentProps { id, mode }: &MatchComponentProps) -> HtmlResult {
    let mode = *mode;
    let random_queue = use_state(Vec::new);
    let data = use_state(|| None);
    let update = {
        let random_queue = random_queue.clone();
        let data = data.clone();
        Callback::from(move |query: Items| {
            let (left, right) = match mode {
                Mode::Round => {
                    let mut queue = (*random_queue).clone();
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
                    random_queue.set(queue);
                    (left, right)
                }
                Mode::Match => {
                    let mut queued_scores: Vec<_> = query.items.iter().collect();
                    queued_scores.shuffle(&mut rand::thread_rng());
                    (
                        queued_scores.pop().unwrap().clone().unwrap(),
                        queued_scores.pop().unwrap().clone().unwrap(),
                    )
                }
            };
            data.set(Some(MatchData { left, right, query }))
        })
    };
    {
        let id = id.clone();
        let update = update.clone();
        use_future(|| async move {
            let query = crate::get_items(&id).await.unwrap();
            update.emit(query);
        })?;
    }
    let update_stats = {
        let list = id.clone();
        let update = update.clone();
        Callback::from(move |(win, lose): (String, String)| {
            let list = list.clone();
            let win = win.clone();
            let lose = lose.clone();
            let update = update.clone();
            wasm_bindgen_futures::spawn_local(async move {
                crate::update_stats(&list, &win, &lose).await.unwrap();
                update.emit(crate::get_items(&list).await.unwrap());
            });
        })
    };

    let Some(MatchData { left, right, query }) = (*data).clone() else {
        return Ok(html! {});
    };
    let left_param = (left.id.clone(), right.id.clone());
    let on_left_select = {
        let update_stats = update_stats.clone();
        Callback::from(move |_| update_stats.emit(left_param.clone()))
    };
    let right_param = (right.id.clone(), left.id.clone());
    let on_right_select = Callback::from(move |_| update_stats.emit(right_param.clone()));
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
    Ok(html! {
        <div>
            <IframeCompare left={left} {on_left_select} right={right} {on_right_select}/>
            {crate::base::responsive_table_view(&["Track", "Record", "Score"], items)}
        </div>
    })
}
