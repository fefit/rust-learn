const PI:f64 = 3.1415;
#[derive(Debug)]
struct Circle{
  radius: f64
}

impl Circle{
  fn area(&self) -> f64{
    println!("1:常量pi的地址,{:p}", &PI);
    PI * self.radius * self.radius 
  }
  fn perimeter(&self) -> f64{
    println!("2:常量pi的地址,{:p}", &PI);
    2.0 * PI * self.radius
  }
}

const fn get_coords(lng: f64, lat:f64)->[f64;2]{
  [lng, lat]
}

fn main(){
  let c = Circle{
    radius: 2.0
  };
  println!("周长：{},面积{}", c.perimeter(), c.area());
  // 通过const可以定义一个常量
  // rust中定义常量需要注意以下几点：
  // 1、常量的名称默认情况下必须大写，否则会warning
  // 2、常量必须标明数据类型
  // 3、常量的值必须是固定的值，在整个程序生命周期内，其值是不可变的；所以常量会被内联优化，没有固定的内存地址；根据前面的说明，常量的值在运行时必须是确定，因此不能使用函数返回值来充当值。但新的rust版本里支持了const_fn常量函数，但该函数的返回值仍然必须是一个常量，编译器会在编译时会执行相应的函数，将运行结果代替实际值。这对于提高代码的整体封装性提供了便利。
  const BJ_COORDS:[f64;2] = get_coords(116.41667, 39.91667);
  println!("经纬度为：{},{}", BJ_COORDS[0], BJ_COORDS[1]);
  // 下面代码不会生效，但也没有报错
  // 对应的值不会发生变更
  BJ_COORDS[0] = 123.456;
  println!("修改后的经纬度为：{},{}", BJ_COORDS[0], BJ_COORDS[1]);
  // 常量是不允许重新赋值的，和上面更改数组常量内的元素不同，下面的代码会报错；
  // PI = 3.1415926; 
  /*
  // 下面的代码也是不允许的，不能将变量的值赋值给常量，即使变量当前是immutable的
  let pi:f64 = 3.1415926;
  const EX_PI:f64 = pi;
  println!("更精确的PI的值：{}", EX_PI);
  */
}