use crate::context::Context;
use crate::utils::{
    event::{Event, Events},
    terminal::create_terminal,
};
use crate::widget::Widget;
use anyhow::{Context as _, Result};
use clap::Clap;
use kuon::{Tweet, TwitterAPI};
use maplit::hashmap;
use termion::event::Key;
use tui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
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
        let mut offset = 0;
        let mut tweet_list = Self::get_tweet(&client, None).await?;

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let chunk = Layout::default()
                    .margin(1)
                    .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
                    .split(size);

                let gauge = Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Refetch"))
                    .gauge_style(
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Black)
                            .add_modifier(Modifier::ITALIC),
                    )
                    .percent((count as f32 / 60f32 * 100f32) as u16);
                let tweet = Self::render(&tweet_list, &chunk[1], (0, count));

                f.render_widget(gauge, chunk[0]);
                f.render_widget(tweet, chunk[1]);
            })?;

            match events.next().await.with_context(|| "Events error")? {
                Event::Input(i) => match i {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('j') => offset += 1,
                    Key::Char('k') => offset = if offset == 0 { offset } else { offset - 1 },
                    _ => {}
                },
                Event::Tick => {
                    count += 1;
                    if count == 60 {
                        let since_id = tweet_list.get(0).and_then(|x| x.id_str.clone());
                        tweet_list =
                            [Self::get_tweet(&client, since_id).await?, tweet_list].concat();
                        count = 0;
                    }
                }
            }
        }

        Ok(())
    }

    async fn get_tweet(client: &TwitterAPI, since_id: Option<String>) -> Result<Vec<Tweet>> {
        let params = since_id
            .as_deref()
            .map(|x| {
                hashmap! {
                    "count" => "200",
                    "since_id" => x
                }
            })
            .unwrap_or_default();

        let tweet_list: Vec<kuon::Tweet> = client.raw_get(ENDPOINT, &params, None).await?;
        Ok(tweet_list)
    }

    fn render<'a>(tweet_list: &Vec<Tweet>, area: &Rect, offset: (u16, u16)) -> Paragraph<'a> {
        tweet_list
            .view(&area)
            .block(
                Block::default()
                    .title("TimeLine")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .scroll(offset)
    }
}
