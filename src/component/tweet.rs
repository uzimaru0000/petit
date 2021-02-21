use crate::component::Component;
use kuon::Tweet;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Wrap},
};

fn tweet_view<'a>(tweet: &Tweet) -> Vec<Spans<'a>> {
    let retweet = tweet.retweeted_status.clone().map_or(false, |_| true);
    let original_tweet = tweet
        .retweeted_status
        .clone()
        .unwrap_or(Box::from(tweet.clone()));

    vec![
        Spans::from(vec![
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
        ]),
        Spans::from(vec![Span::raw(original_tweet.text.clone())]),
        Spans::from(vec![]),
    ]
}

impl<'a> Component<Paragraph<'a>> for Vec<Tweet> {
    fn view(&self, _area: &Rect) -> Paragraph<'a> {
        let tweets = self.iter().flat_map(|x| tweet_view(x)).collect::<Vec<_>>();

        Paragraph::new(tweets).wrap(Wrap { trim: true })
    }
}
