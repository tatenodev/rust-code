// https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html

struct Point<T, U> {
  x: T,
  y: U,
}

// ジェネリックなimpl
// implの後にTをジェネリックな型として宣言することで、コンパイラはPointの山カッコ内の型が具体的な型ではなくジェネリックな型であることを認識できる
impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
      &self.x
  }

  // 別な型のPointの要素を混ぜるメソッド
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      Point {
          x: self.x,
          y: other.y,
      }
  }
}

// 具体的な型f32にする場合。
// Pointの要素がf32の構造体にしか実装されないメソッド。
impl Point<f32, f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
  let p = Point { x: 5, y: 10 };
  println!("p.x = {}", p.x());
  // p.x = 5

  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point {x: "Hello", y: 'c'};
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.x = {}", p3.x, p3.y);
  // p3.x = 5, p3.x = c
}
