fn main(){
  let mut arr:[i32;10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  shell_sort(&mut arr);
  println!("the sorted array is:{:?}", arr);
}
/*
* 希尔排序
* 希尔排序也叫递减增量排序，是插入排序的优化版本
* 它的核心思想是，插入排序最好的情况是，所有排序部分越接近于有序状态，其时间复杂度越低
* 所以可以将数组按间隔逻辑上分为多个组，对这多个组依次进行插入排序，当间隔越来越小，分组插入排序后的结果就越接近于有序，
* 直到最终间隔值=1，也即最后进行一次插入排序，最终结果即为已排序结果
*/
fn shell_sort<T:Ord>(arr: &mut [T]){
  let len = arr.len();
  let min_count = 3; // 分组数增量，也代表着分组元素个数
  let mut gap = len / min_count; // 分组间隔、也即分组数
  // 按照gap递减分组
  while gap > 0{
    for index in 0..gap{
      // 实际排序元素的索引值为当前虚拟分组内的元素索引<第几个元素> * 分组数 + 分组偏移值 
      let mut now_index = gap + index;
      while now_index < len{
        let mut cur_index = now_index;
        // 因为前面的数列都是按照从小到大的方式已经排序好的
        // 所以需要挨个冒泡到最小位置，这里需要从已排序的最后一个元素往前对比，即先比较最大的元素
        loop{
            // 对比前一个元素
            let prev_index = cur_index - gap;
            if arr[cur_index] >= arr[prev_index]{
                break;
            }
            // 往前移动一位
            arr.swap(cur_index, prev_index);
            cur_index = prev_index;
            // 已经是分组的第一个元素，退出循环
            if cur_index < gap{
                break;
            }
        }
        now_index += gap;
      }
    }
    // 缩小增量，分组趋近于总长度，为1时即为正常的插入排序
    gap = gap / min_count;
  }
}