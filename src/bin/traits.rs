trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({}) - {}", self.headline, self.author, self.location, self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} {} {}", self.username, self.content, self.reply, self.retweet)
    }
}

// fn notify(item: impl Summary) {
//     println!("{}", item.summarize());
// }

fn notify<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: String::from("Melbourne is the best"),
        location: String::from("Australia Victoria"),
        author: String::from("Glenn ji"),
        content: String::from("Melbourne is beautiful")
    };

    println!("{}", news_article.summarize());

    notify(tweet);
}