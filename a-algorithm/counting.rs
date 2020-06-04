fn main() {
  let arr: [i32; 10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  let v = counting_sort(&arr);
  println!("the sorted array is:{:?}", v);
}
/**
* 计数排序
* 计数排序非比较排序，它的使用场景也比较苛刻，一般是对于非负整数的排序，它的原理是利用数组键的顺序来确定值的位置
* 如果数组间隔较大，通常要浪费很大的内存空间，对于密集紧凑且多重复的数据，就比较适合使用它
*/
fn counting_sort(arr: &[i32]) -> Vec<i32> {
  // 获取最大值和最小值
  let mut min = arr[0];
  let mut max = arr[0];
  for &num in arr{
    if num < min{
      min = num;
    }else if num > max{
      max = num;
    }
  }
  // 利用最大值和最小值差值来构建一个数组
  let mut counts = vec![0_u8;(max-min+1) as usize];
  for &num in arr{
    let index = (num - min) as usize;
    counts[index] += 1; 
  }
  // 把结果取出来
  let mut v:Vec<i32> = vec![];
  for (index, &count) in counts.iter().enumerate(){
    if count > 0{
        let numbers = vec![index as i32 + min;count as usize];
        v = [v, numbers].concat();
    }
  }
  v
}