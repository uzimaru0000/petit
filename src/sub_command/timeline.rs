use std::cmp::min;

use crate::component::Component;
use crate::context::{Cache, Context};
use crate::utils::{
    event::{Event, Events},
    terminal::create_terminal,
};
use anyhow::{Context as _, Result};
use chrono::Utc;
use clap::Clap;
use kuon::{TrimTweet, TwitterAPI};
use termion::event::Key;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListState},
};

#[derive(Debug, Clap)]
#[clap(name = "tl")]
pub struct TimeLine {}

impl TimeLine {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        let client = ctx
            .client
            .with_context(|| "Please login. run \"petit login\"")?;
        let mut terminal = create_terminal()?;
        let mut events = Events::new();
        let mut count = ctx.cache.as_ref().map(|x| x.count).unwrap_or(0);
        let mut list_state = ListState::default();
        let mut tweet_list = if ctx
            .cache
            .as_ref()
            .and_then(|x| x.latest_call)
            .map(|x| Self::is_latest_request(x))
            .unwrap_or(false)
        {
            ctx.cache
                .as_ref()
                .map(|x| x.timeline.clone())
                .unwrap_or_default()
        } else {
            Self::get_tweet(&client, None).await?
        };

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let tweet_view = Self::render(&tweet_list, &size);
                f.render_stateful_widget(tweet_view, size, &mut list_state);
            })?;

            match events.next().await.with_context(|| "Events error")? {
                Event::Input(i) => match i {
                    Key::Esc => {
                        list_state.select(None);
                    }
                    Key::Char('q') => {
                        let maybe_cache = ctx.cache.clone();
                        if let Some(mut cache) = maybe_cache {
                            cache.count = count;
                            Context::save_cache(&cache).await?;
                        }
                        break;
                    }
                    Key::Char('j') | Key::Up => {
                        list_state.select(Some(
                            list_state
                                .selected()
                                .map(|x| min(x + 1, tweet_list.len() - 1))
                                .unwrap_or_default(),
                        ));
                    }
                    Key::Char('k') | Key::Down => {
                        list_state.select(Some(
                            list_state
                                .selected()
                                .map(|x| x.checked_sub(1).unwrap_or(0))
                                .unwrap_or_default(),
                        ));
                    }
                    Key::Char('f') => {
                        let selected_tweet = list_state
                            .selected()
                            .and_then(|x| tweet_list.get(x))
                            .and_then(|x| x.id_str.clone());

                        if let Some(id) = selected_tweet {
                            client.favorite().id(&id).send().await?;
                            tweet_list
                                .iter_mut()
                                .filter(|x| {
                                    if let Some(id_str) = x.id_str.clone() {
                                        id_str == id
                                    } else {
                                        false
                                    }
                                })
                                .for_each(|x| x.favorite_count += 1);
                        }
                    }
                    Key::Char('r') => {
                        let selected_tweet = list_state
                            .selected()
                            .and_then(|x| tweet_list.get(x))
                            .and_then(|x| x.id_str.clone());

                        if let Some(id) = selected_tweet {
                            client.retweet().id(&id).send().await?;
                            tweet_list
                                .iter_mut()
                                .filter(|x| {
                                    if let Some(id_str) = x.id_str.clone() {
                                        id_str == id
                                    } else {
                                        false
                                    }
                                })
                                .for_each(|x| x.retweet_count += 1);
                        }
                    }
                    _ => {}
                },
                Event::Tick => {
                    count += 1;
                    if count == 60 {
                        let since_id = tweet_list.get(0).and_then(|x| x.id_str.clone());
                        let new_tweet_list = Self::get_tweet(&client, since_id).await?;
                        let new_tweet_list_num = new_tweet_list.len();
                        tweet_list = [new_tweet_list, tweet_list].concat();
                        list_state.select(list_state.selected().map(|x| x + new_tweet_list_num));
                        count = 0;
                    }
                }
            }
        }

        Ok(())
    }

    async fn get_tweet(client: &TwitterAPI, since_id: Option<String>) -> Result<Vec<TrimTweet>> {
        let mut timeline = client.home_timeline();
        if let Some(id) = since_id.and_then(|x| x.parse::<u64>().ok()) {
            timeline.since_id(id);
        }
        let tweet_list = timeline.count(30).send().await?;

        let now = chrono::Utc::now();
        let cache = Cache {
            latest_call: Some(now),
            timeline: tweet_list.clone(),
            count: 0,
        };
        Context::save_cache(&cache).await?;

        Ok(tweet_list)
    }

    fn render<'a>(tweet_list: &Vec<TrimTweet>, area: &Rect) -> List<'a> {
        tweet_list
            .view(&area)
            .block(
                Block::default()
                    .title("TimeLine")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .highlight_symbol(">>")
            .highlight_style(Style::default().bg(Color::Rgb(0, 64, 128)))
    }

    fn is_latest_request(latest_call: chrono::DateTime<Utc>) -> bool {
        let now = Utc::now();
        let delta = now - latest_call;

        delta.num_minutes() < 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_latest_request() {}
}
