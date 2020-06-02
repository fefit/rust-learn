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
fn shell_sort(arr: &mut [i32;10]){
  let len = arr.len();
  let mut gap = 1; // 分组间隔、也即分组数
  let min_count = 3; // 分组数增量，也代表着分组元素个数
  // 分组间隔，增量计算
  while gap < len / min_count{
    gap = min_count * gap + 1;
    // 最终gap >= len / min_count 且 (prevGap = (gap - 1)/min_count) < len / min_count即 gap < len + 1也即 gap <= len;
  }
  // 按照gap递减分组
  while gap >= 1{
    // 分组后元素最少个数
    let ele_num = len / gap; // 整数相除，相当于取floor
    let last_num = len % gap; // 余数
    for index in 0..gap{   
      // 根据余数，确定当前分组元素个数
      let cur_len = if last_num > index{
        ele_num + 1
      } else{
        ele_num
      };
      // 对该分组内的元素进行插入排序
      for i in 1..cur_len{   
        // i索引以前的数字是已排序好的序列
        // 实际排序元素的索引值为当前分组内的元素索引<第几个元素> * 分组数 + 分组偏移值  
        let mut cur_index = i * gap + index;
        // 因为前面的数列都是按照从小到大的方式已经排序好的
        // 所以需要挨个冒泡到最小位置，这里需要从已排序的最后一个元素往前对比，即先比较最大的元素
        for j in (0..i).rev(){
          let prev_index = j * gap + index;
          if arr[cur_index] >= arr[prev_index]{
            break;
          }
          // 往前移动一位
          arr.swap(cur_index, prev_index);
          cur_index = prev_index;
        }
      }
    }
    // 上述取增量间隔操作的逆过程
    gap = gap / min_count;
  }
}