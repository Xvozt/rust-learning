use traits::{notify, notify2, notify3, NewsArticle, Summary, Tweet};

// we can return type that implements trait without returning concrete types
#[allow(dead_code)]
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("some content"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("Xvozt"),
        content: String::from("It's not twitter anymore, it's X"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Dos caras de la tortilla"),
        location: String::from("Mexico"),
        author: String::from("Luis"),
        content: String::from("Algo perturbador ocurrió"),
    };
    // we created a method, that can accept types that implements summary as parameters
    notify(&tweet);
    println!("{}", article.summarize());
    //we cannot use tweet and article in the case of notify2
    // notify2(&tweet, &article);
    notify2(&tweet, &tweet2);
    // and with this signature we can use different types as long as both of them implement Summary
    notify3(&tweet, &article);
}
