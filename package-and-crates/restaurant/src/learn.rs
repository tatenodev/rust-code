mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {}
      fn seat_at_table() {}
  }

  mod serving {
      fn take_order() {}
      pub(in crate::front_of_house) fn serve_order() {}
      fn take_payment() {}
  }

  pub mod back_of_house {
      fn fix_incorrect_order() {
          cook_order();
          super::serving::serve_order();
      }
      fn cook_order() {}

      pub struct Breakfast {
          pub toast: String,
          seasonal_fruit: String,
      }

      impl Breakfast {
          pub fn summer(toast: &str) -> Breakfast {
              Breakfast {
                  toast: String::from(toast),
                  seasonal_fruit: String::from("peaches"),
              }
          }
      }

      pub enum Appetizer {
          Soup,
          Salad,
      }
  }
}

// useで絶対パスを使う
// use crate::front_of_house::hosting;
// 相対パス
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
  crate::front_of_house::hosting::add_to_waitlist();
  front_of_house::hosting::add_to_waitlist();
  // useでパスをスコープに持ち込んでいるため省略できる
  hosting::add_to_waitlist();

  // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
  let mut meal = front_of_house::back_of_house::Breakfast::summer("Rye");
  // やっぱり別のパンにする
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  let order1 = front_of_house::back_of_house::Appetizer::Soup;
  let order2 = front_of_house::back_of_house::Appetizer::Salad;
}
