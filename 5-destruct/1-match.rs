#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

fn main() {
  // 模式解构
  // rust里的模式解构非常强大，在赋值语句、函数参数、match语句等地方都能使用解构，
  // 这里先来了解下match的基本功能
  let x = 1;
  match x {
    1 => println!("x is 1"),
    _ => println!("x is not 1"), // _下划线代表了所有其它情况
  };
  // 上面的语句对x的值进行了判定，rust里没有switch语句，这是因为match的功能能完全替代switch
  // 上面的语句如果表示为switch，类似如下：<javascript>
  /* switch x{
    case 1:
      println!("x is 1");
      break;
    default:
      println!("x is not 1");
  } */
  // match也可以对多个值进行匹配
  match x {
    1 | 2 | 3 => println!("x is 1 or 2 or 3"),
    _ => println!("x is not 1 or 2 or 3"),
  };
  // 上面的语句如果表示为switch，类似如下：<javascript>
  /* switch x{
    case 1:
    case 2:
    case 3:
      println!("x is 1");
      break;
    default:
      println!("x is not 1");
  } */
  match x {
    0.. => println!("x is greater than 0"), // 判断x是否在一个闭合区间内、如果非完全闭合区间，需要开启exclusive_range_pattern、half_open_range_patterns的feature
    _ => println!("x is negative"),
  };
  //
}
