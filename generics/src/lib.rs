use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub auhtor: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.auhtor, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Summary traitを実装している型なら受け付ける
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Summary traitを実装している型を2つ引数にとる
pub fn notiry2(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}

// 両方の引数が同じ型であることを強制する
pub fn notiry3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// 複数のトレイト境界の指定
pub fn notify4(item: &(impl Summary + Display)) {} // syntax sugar
pub fn notify5<T: Summary + Display>(item: &T) {}

// wehere句を使ったより明確なトレイト境界
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}
// whereを使わなかった場合
pub fn some_function2<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
  1
}

// トレイトを実装している型を返す
// この書き方だとTweetもしくはNewsArticleを返す、というような記述はできない
pub fn retruns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("foo"),
    reply: false,
    retweet: false,
  }
}

struct Pair<T> {
  x: T,
  y: T
}

// newは常に実装される
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

// 内部の型Tが比較を可能にするPartialOrdトレイトと出力を可能にするDisplayトレイトを実装している時のみ、cmp_displayメソッドを実装
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
