use std::cmp::{max, min};

use crate::component::Component;
use crate::context::Context;
use crate::utils::{
    event::{Event, Events},
    terminal::create_terminal,
};
use anyhow::{Context as _, Result};
use clap::Clap;
use kuon::{Tweet, TwitterAPI};
use maplit::hashmap;
use termion::event::Key;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph},
};

const ENDPOINT: &str = "https://api.twitter.com/1.1/statuses/home_timeline.json";

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
        let mut count = 0;
        let mut list_state = ListState::default();
        let mut tweet_list = Self::get_tweet(&client, None).await?;

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
                            client.favorite(&id).await?;
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
                            client.retweet(&id).await?;
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

    async fn get_tweet(client: &TwitterAPI, since_id: Option<String>) -> Result<Vec<Tweet>> {
        let mut params = since_id
            .as_deref()
            .map(|x| {
                hashmap! {
                    "since_id" => x
                }
            })
            .unwrap_or_default();
        params.insert("count", "200");

        let tweet_list: Vec<kuon::Tweet> = client.raw_get(ENDPOINT, &params, None).await?;
        Ok(tweet_list)
    }

    fn render<'a>(tweet_list: &Vec<Tweet>, area: &Rect) -> List<'a> {
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
}
