use crate::component::Component;
use kuon::{TrimTweet, Tweet};
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{List, ListItem},
};

pub struct TweetView {
    pub id: String,
    pub user_name: String,
    pub screen_name: String,
    pub retweet_user_name: Option<String>,
    pub tweet: String,
    pub retweet_count: u64,
    pub favorite_count: u64,
}

impl From<&Tweet> for TweetView {
    fn from(x: &Tweet) -> Self {
        x.retweeted_status
            .as_ref()
            .map(|retweet| Self {
                id: retweet.id_str.clone().unwrap(),
                user_name: retweet.user.name.clone(),
                screen_name: retweet.user.screen_name.clone(),
                tweet: retweet.text.clone(),
                retweet_user_name: Some(x.user.screen_name.clone()),
                retweet_count: retweet.retweet_count,
                favorite_count: retweet.favorite_count,
            })
            .unwrap_or(Self {
                id: x.id_str.clone().unwrap(),
                user_name: x.user.name.clone(),
                screen_name: x.user.screen_name.clone(),
                tweet: x.text.clone(),
                retweet_user_name: None,
                retweet_count: x.retweet_count,
                favorite_count: x.favorite_count,
            })
    }
}

impl From<&TrimTweet> for TweetView {
    fn from(x: &TrimTweet) -> Self {
        x.retweeted_status
            .as_ref()
            .map(|retweet| TweetView {
                id: retweet.id_str.clone().unwrap(),
                user_name: retweet
                    .user
                    .name
                    .clone()
                    .unwrap_or(x.user.name.clone().unwrap()),
                screen_name: retweet
                    .user
                    .screen_name
                    .clone()
                    .unwrap_or(x.user.screen_name.clone().unwrap()),
                tweet: retweet.text.clone(),
                retweet_user_name: x.user.screen_name.clone(),
                retweet_count: retweet.retweet_count,
                favorite_count: retweet.favorite_count,
            })
            .unwrap_or(TweetView {
                id: x.id_str.clone().unwrap(),
                user_name: x.user.name.clone().unwrap(),
                screen_name: x.user.screen_name.clone().unwrap(),
                tweet: x.text.clone(),
                retweet_user_name: None,
                retweet_count: x.retweet_count,
                favorite_count: x.favorite_count,
            })
    }
}

impl<'a> TweetView {
    fn to_list_item(&self, width: usize) -> ListItem<'a> {
        let tweet_user = vec![Spans::from(vec![
            Span::styled(
                self.user_name.clone(),
                Style::default().fg(Color::Rgb(255, 128, 0)),
            ),
            Span::styled(
                format!("@{}", self.screen_name),
                Style::default().fg(Color::DarkGray),
            ),
            if let Some(retweet_user) = self.retweet_user_name.clone() {
                Span::styled(
                    format!(" 🔁 {} Retweeted", retweet_user),
                    Style::default().fg(Color::LightGreen),
                )
            } else {
                Span::raw("")
            },
        ])];
        let contents = textwrap::fill(&self.tweet, width)
            .lines()
            .map(|x| Spans::from(vec![Span::raw(x.to_string())]))
            .collect::<Vec<_>>();
        let tweet_info = vec![Spans::from(vec![
            Span::styled(
                format!("🔁 {}", self.retweet_count),
                Style::default().fg(Color::LightGreen),
            ),
            Span::raw(" "),
            Span::styled(
                format!("❤️ {}", self.favorite_count),
                Style::default().fg(Color::LightRed),
            ),
        ])];
        let margin = vec![Spans::default()];

        ListItem::new([tweet_user, contents, tweet_info, margin].concat())
    }
}

impl<'a> Component<List<'a>> for Vec<TrimTweet> {
    fn view(&self, area: &Rect) -> List<'a> {
        let tweets = self
            .iter()
            .map(|x| TweetView::from(x))
            .map(|x| x.to_list_item(area.width as usize))
            .collect::<Vec<_>>();
        List::new(tweets)
    }
}

impl<'a> Component<List<'a>> for Vec<Tweet> {
    fn view(&self, area: &Rect) -> List<'a> {
        let tweets = self
            .iter()
            .map(|x| TweetView::from(x))
            .map(|x| x.to_list_item(area.width as usize))
            .collect::<Vec<_>>();
        List::new(tweets)
    }
}
