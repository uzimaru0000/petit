use crate::component::Component;
use kuon::Tweet;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{List, ListItem},
};

fn tweet_view<'a>(tweet: &Tweet, width: usize) -> ListItem<'a> {
    let retweet = tweet.retweeted_status.clone().map_or(false, |_| true);
    let original_tweet = tweet
        .retweeted_status
        .clone()
        .unwrap_or(Box::from(tweet.clone()));

    let tweet_user = vec![Spans::from(vec![
        Span::styled(
            original_tweet.user.name.clone(),
            Style::default().fg(Color::Rgb(255, 128, 0)),
        ),
        Span::styled(
            format!("@{}", original_tweet.user.screen_name.clone()),
            Style::default().fg(Color::DarkGray),
        ),
        if retweet {
            Span::styled(
                format!(" 🔁 {} Retweeted", tweet.user.name.clone()),
                Style::default().fg(Color::LightGreen),
            )
        } else {
            Span::raw("")
        },
    ])];
    let contents = textwrap::fill(&original_tweet.text.clone(), width)
        .lines()
        .map(|x| Spans::from(vec![Span::raw(x.to_string())]))
        .collect::<Vec<_>>();
    let tweet_info = vec![Spans::from(vec![
        Span::styled(
            format!("🔁 {}", original_tweet.retweet_count),
            Style::default().fg(Color::LightGreen),
        ),
        Span::raw(" "),
        Span::styled(
            format!("❤️ {}", original_tweet.favorite_count),
            Style::default().fg(Color::LightRed),
        ),
    ])];
    let margin = vec![Spans::default()];

    ListItem::new([tweet_user, contents, tweet_info, margin].concat())
}

impl<'a> Component<List<'a>> for Vec<Tweet> {
    fn view(&self, area: &Rect) -> List<'a> {
        let tweets = self
            .iter()
            .map(|x| tweet_view(x, area.width as usize))
            .collect::<Vec<_>>();
        List::new(tweets)
    }
}
