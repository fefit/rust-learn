fn main(){
  let mut arr:[i32;10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  insertion_sort(&mut arr);
  println!("the sorted array is:{:?}", arr);
}
/*
* 插入排序
* 1、将序列分成前后两个部分，前面部分是已排序部分，后面是未排序部分
* 2、从未排序部分取出第一个元素，按照冒泡的方式插入至已排序部分中
*/
fn insertion_sort<T:Ord>(arr: &mut [T]){
  let len = arr.len();
  // 外循环，执行n次
  for i in 1..len{   
    // i索引以前的数字是已排序好的序列
    let mut cur_index:usize = i;
    let mut j = i;
    // 因为前面的数列都是按照从小到大的方式已经排序好的
    // 所以需要挨个冒泡到最小位置，这里需要从已排序的最后一个元素往前对比，即先比较最大的元素
    while j > 0 && arr[cur_index] < arr[j-1]{
      j -= 1;
      // 往前移动一位
      arr.swap(cur_index, j);
      cur_index = j;
    }
  }
  // 复杂度：最差O(n^2)，最好O(n)
}