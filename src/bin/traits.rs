trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Acronym {
    fn acronym_name(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}) - {}",
            self.headline, self.author, self.location, self.content
        )
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Acronym for Tweet {
    fn acronym_name(&self) -> String {
        format!("{}", self.username)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {} {} {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

// fn notify(item: impl Summary) {
//     println!("{}", item.summarize());
// }

fn notify<T: Summary + Acronym>(item: &T) {
    println!("{}-{}", item.summarize(), item.acronym_name());
}

fn notify2<T, U>(t: &T, u: &U)
where
    T: Summary,
    U: Acronym,
{
    println!("{}={}", t.summarize(), u.acronym_name());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("jiqiang"),
        content: String::from("Glenn is a good man"),
        reply: true,
        retweet: true,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: String::from("Melbourne is the best"),
        location: String::from("Australia Victoria"),
        author: String::from("Glenn ji"),
        content: String::from("Melbourne is beautiful"),
    };

    println!("{}", news_article.summarize());

    notify(&tweet);
    notify2(&tweet, &tweet2);
}
