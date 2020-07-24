use std::ops::Deref;
fn main(){
  // 定义一个整形int
  let int = 1i32;
  assert_eq!(int, 1);
  // 使用&取引用，p_int是一个指向int变量的指针，如果要取得int变量的值，需要对p_int使用*解引用
  let p_int = &int;
  assert_eq!(*p_int, 1);
  // 对p_int再次取引用、对应的也需要两次解引用才能得到int值
  let pp_int = &p_int;
  assert_eq!(**pp_int, 1);
  // 对int多次取引用、结果和pp_int一样
  let p2_int = &&int;
  assert_eq!(**p2_int, 1);
  // 定义一个struct
  #[derive(Debug)]
  struct MyStruct{
    value: i32
  };
  let st = MyStruct{
    value: 1
  };
  let mut st_remove = st;
  // println!("st is {:?}", st); // 这行会报错，因为MyStruct没有实现Copy trait，所以这里st变量的所有权会转移到st_remove，再次使用st会提示类似'value borrowed here after move'的错误
  // 如果使用借用、则不会转移所有权，&T是只读借用，&mut T是可变借用，同一时间、只读借用可以有多个，可变借用只能有一个，也不能同时有只读借用和可变借用，这就是rust里的共享不可变<还有可变不共享>原则
  let st_borrow = &st_remove;
  let st_borrow_2 = &st_remove;
  println!("原值:{:?},借用1:{:?},借用2:{:?}", st_remove, st_borrow, st_borrow_2);
  // 上面已经有两个只读借用了，这里再加一个可变借用，按照以前的词法作用域，这是不允许的
  let st_borrow_3 = &mut st_remove;
  (*st_borrow_3).value = 2; // 注意这里的小括号，解引用是针对st_borrow_3而不是针对value字段的
  println!("当前的st_remove的值为:{:?}", st_remove);
  // println!("当前的st_borrow的值为:{:?}", st_borrow); // 不过这里不能再使用只读借用了，否则就可变借用<st_borrow_3>和只读借用<st_borrow>同时存在了，st_borrow_2也同样。这样是合理的，在只读借用的时候，我们直观会认为st_borrow的value仍为原值1，而不是改变后的2，这显然会造成逻辑混乱。换个角度说，可变借用会屏蔽位于它声明之前的所有只读借用。
  // 再定义一个可变借用，只要上面的st_borrow_3不再该可变借用后使用，这是允许的，因为后面的作用域范围内还是只有st_borrow_4这一个st_remove的可变借用
  let mut st_borrow_4 = &mut st_remove;
  // 注意声明左右两侧两个mut的区别，左侧的mut表示st_borrow_4绑定的内存地址是可变更的，它可以指向其它MyStruct对象；后面的mut表示通过st_borrow_4这个可变引用、更改其指针所对应的变量st_remove内的数据。
  (*st_borrow_4).value = 3;
  println!("st_borrow_4的值为:{:?}", st_borrow_4); 
  // 更改st_borrow_4变量自身绑定的内存地址
  let mut another = MyStruct{value:4};
  st_borrow_4 = &mut another; // 不能直接写&mut MyStruct{value:4}，因为临时变量的生命周期会随后释放，无法直接当前可变引用直接使用
  println!("更改后st_borrow_4的值为:{:?}", st_borrow_4); 
  // 因为st_borrow_4不再指向原来的st_remove的可变引用，理论上这时st_borrow_3又成了唯一可变引用了，但rust编译器目前好像对这种情况还无法很好的处理，下面这段代码仍会报错
  // println!("当前st_borrow_3的值为:{:?}", st_borrow_3);
  // ==================================================
  // 讨论完上面，下面继续来看解引用
  impl Deref for MyStruct {
    type Target = i32;
    fn deref(&self)->&Self::Target{
      return &self.value;
    }
  }
  let st = MyStruct{
    value: 1
  };
  // 通过实现Deref trait的deref方法，就可以在自定义类型上使用*解引用操作了，解引用时会调用deref方法
  assert_eq!(*st, 1);
  // 对解引用后的数据执行对应类型上的方法
  assert_eq!((*st).checked_add(2), Some(3));
  // 实际上、在以下情况下，rust会强制做隐式类型转换，即会自动强制对对象进行deref解引用操作
  // 1、调用对象上的方法时
  // 所以下面的st调用checked_add方法时，发现该结构体上并没有该方法，然后发现st对应的MyStruct类型实现了Deref trait，执行七上的deref方法，发现返回值是&i32类型，i32上存在checked_add方法，从而进行解引用，完成强制类型转换，最终结果就相当于(*st.deref())
  assert_eq!(st.checked_add(2), Some(3));
  // 2、当函数参数为引用时，传递过去的参数会做自动解引用操作
  fn add_one(target: &i32)->i32{
    *target + 1
  }
  let result = add_one(&st);
  println!("after add one:{}", result);
}