fn main(){
  let mut arr:[i32;10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  selection_sort(&mut arr);
  println!("the sorted array is:{:?}", arr);
}
/*
* 选择排序
* 1、假设当前未排序序列第一个数为最小数，标记其索引，然后循环剩余部分序列，对比当前元素和最小元素，如果该值比最小元素还小，更新最小元素的索引值为当前索引值
* 2、完成上述步骤后，将最小索引值元素和当前未排序第一个元素做交换，则该元素及以前的部分都是按从小到大已排序序列，如此继续排序剩余部分
*/
fn selection_sort(arr: &mut [i32;10]){
  let len = arr.len();
  // 外循环，执行n次
  for i in 0..len{   
    let mut min_index = i;
    // i索引以前的数字是已排序好的数字
    for j in i+1..len{
      // 如果当前的数字小于当前设定的最小值，设置当前数的索引为最小值索引
      if arr[j] < arr[min_index]{
        min_index = j;
      }
    }
    if min_index != i{
      arr.swap(i, min_index);
    }
  }
  // 最终执行的总次数为 (n-1) + (n-2) + ... + 0 = (n-1) * n/2，所以时间复杂度是O(n^2)
}