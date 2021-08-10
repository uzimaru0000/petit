use crate::component::{input::Input, Component, ComponentWithContext};
use crate::context::Context;
use crate::utils::event::{Event, Events};
use crate::utils::terminal::create_terminal;
use anyhow::{Context as _, Result};
use clap::Clap;
use termion::event::Key;
use tui::layout::{Constraint, Layout};
use tui::widgets::{Block, BorderType, Borders, List};

#[derive(Debug, Clap)]
pub struct Search {
    query: Option<String>,
}

impl Search {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        let client = ctx
            .client
            .with_context(|| "Please login. run \"petit login\"")?;
        let mut terminal = create_terminal()?;
        let mut events = Events::new();
        let mut input = Input {
            value: self.query.clone().unwrap_or_default(),
        };
        let mut tweet_list = if input.value.is_empty() {
            Vec::new()
        } else {
            client
                .search_tweets()
                .q(input.value.as_str())
                .send()
                .await?
                .statuses
        };

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let chunk = Layout::default()
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                    .split(size);

                let input_widget = input.view(&chunk[0], f).block(
                    Block::default()
                        .title("Search")
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL),
                );
                f.render_widget(input_widget, chunk[0]);

                let result_block = Block::default()
                    .title("Result")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL);
                let tweet_widget = if tweet_list.is_empty() {
                    List::new(vec![])
                } else {
                    tweet_list.view(&chunk[1]).block(result_block)
                };
                f.render_widget(tweet_widget, chunk[1]);
            })?;

            match events.next().await {
                Some(Event::Input(Key::Esc)) => {
                    break;
                }
                Some(Event::Input(Key::Char('\n'))) => {
                    tweet_list = client
                        .search_tweets()
                        .q(input.value.as_str())
                        .send()
                        .await?
                        .statuses;
                }
                Some(Event::Input(Key::Char(c))) => {
                    input.value.push(c);
                }
                Some(Event::Input(Key::Backspace)) => {
                    input.value.pop();
                }
                // Some(Event::Input(Key::Down)) => offset += 1,
                // Some(Event::Input(Key::Up)) => {
                //     offset = if offset == 0 { offset } else { offset - 1 }
                // }
                Some(_) => {}
                None => {
                    break;
                }
            }
        }

        Ok(())
    }
}
