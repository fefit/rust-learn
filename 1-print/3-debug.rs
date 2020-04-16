use std::fmt;

struct RGB{
  red: u8,
  green: u8,
  blue: u8
}
// 实现Display trait，以便可以通过{}格式化
impl fmt::Display for RGB{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "RGB{{red:{},green:{},blue:{}}}", self.red, self.green, self.blue)
  }
}
// 实现LowerHex，以便可以通过{:x}格式化
impl fmt::LowerHex for RGB{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f,"{:x}{:x}{:x}", self.red, self.green, self.blue)
  }
}

#[derive(Debug)]
struct Shadow<T>{
  bgcolor: T,
}

fn main(){
  /**
   * 前面已经了解了常见的数据打印输出方式，现在来看看rust是如何确定一个类型的打印形式的
   * rust里有个重要的概念就是trait(特征)
   * 传统的面向对象语言有几个重要的特性：
   * 1、封装
   * 封装主要体现在类的概念上，可以将一些具有相同特性和行为的事物抽象成一个类。最终所有对这个事物的处理都在类上进行。
   * 一般都通过class关键字来声明一个类，但rust里是没有类的概念的，class也不是rust的关键字。
   * 针对相同特性（属性、字段）的事物，rust里可以通过结构体struct来表示。那么，如何来定义这些结构体能拥有哪些行为呢？
   * 这时候trait就派上用场了，trait有点像接口(interface)，它描述了一些方法，这些方法可能没有函数体内容、也可能包含着这个方法的默认实现，总之，可以为不同的数据类型实现这个trait里的方法，然后我们就可以通过以trait限定的方式声明某个变量，在变量上直接调用trait的方法了。
   */
  let c = RGB{
    red: 255,
    green: 255,
    blue: 255
  };
  // println!("{}", c);
  // 如果没有生面的impl部分，上面注释的一行会报这样的错误：the trait `std::fmt::Display` is not implemented for `RGB` 
  println!("{:x}", c);
  // 大部分类型我们可以通过实现fmt::Display trait来获得好的打印结果
  // 但针对一些带泛型的类型，我们就不好去实现了，其数据类型可能是多样的，我们无法区分的打印出来所有不同类型的打印结果。
  // 这时我们就需要用到fmt::Debug了，rust里通过过程宏#[derive(Debug)]就可以为类型自动添加实现。 
}