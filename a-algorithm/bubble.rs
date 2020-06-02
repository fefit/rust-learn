fn main(){
  let mut arr:[i32;10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  bubble_sort(&mut arr);
  println!("the sorted array is:{:?}", arr);
}
/*
* 冒泡排序
* 1、扫描所有未排序序列，方式是挨个对比前后两个元素的大小，小的往前排，大的往后排，最终最大的元素一定会被排到最后
* 2、排除掉后面的已排序好的序列，继续执行上述步骤
*/
fn bubble_sort(arr: &mut [i32;10]){
  let len = arr.len();
  // 外循环，执行n次
  for i in 0..len{
    // 剩余的未排序项数位len-i，需要两两对比的次数为项数-1
    // 内循环，执行n-i-1次
    let unsorted_len = len - i - 1;
    for j in 0..unsorted_len{
      // 如果后面的数比前面小，交换两个数字的位置
      if arr[j] > arr[j+1]{
        arr.swap(j, j+1);
      }
    }
  }
  // 最终执行的总次数为 0 + 1 + 2 + ... + (n-1) = (n-1) * n/2，所以时间复杂度是O(n^2)
}