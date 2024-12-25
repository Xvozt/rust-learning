use blog::Post;
use blog_type::{DraftPost, PendingReviewPost, PostToPublish};
use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 50,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();

    post.add_text("I ate salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.approve();
    assert_eq!("I ate salad for lunch today", post.content());

    let mut post_to_publish = PostToPublish::new();

    post_to_publish.add_test("I didn't eat a salad for lunch today");

    let post = post_to_publish.request_review();
    let post = post.approve();
    assert_eq!("I didn't eat a salad for lunch today", post.content());
}
