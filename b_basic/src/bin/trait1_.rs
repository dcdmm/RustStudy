// trait(Default Implementations)

trait Summary {
    // Default Implementations
    fn summarize(&self) -> String {
        String::from("(Read more)")
    }
}

#[allow(warnings)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    // 覆盖默认实现
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[allow(warnings)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    // 调用覆盖后的默认实现
    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.
    // 调用默认实现
    println!("1 new tweet: {}", tweet.summarize());
}
