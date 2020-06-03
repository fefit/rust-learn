fn main() {
  let arr: [i32; 10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  let sorted_arr = merge_sort(&arr[..]);
  println!("the sorted array is:{:?}", sorted_arr);
}
/**
* 归并排序
* 归并排序是利用分治法来排序的算法，它的核心思想是将待排序序列分成两组，两组分别排序后然后合并
* 分组后的两个序列如果可以再分，则递归上述过程，这样由内到外层层递归后就能得到最终已排序结果
* 归并排序不是在原序列上直接进行排序操作，所以它需要消耗额外的内存空间用来保存结果
*/
fn merge_sort(arr: &[i32]) -> Vec<i32> {
  let len = arr.len();
  if len > 1 {
      let avg = len / 2;
      let first = &arr[0..avg];
      let second = &arr[avg..];
      let left = merge_sort(first);
      let right = merge_sort(second);
      merge(&left, &right)
  } else {
      arr.to_vec()
  }
}

// 合并两个已排序序列
fn merge(first: &[i32], second: &[i32]) -> Vec<i32> {
  let f_len = first.len();
  let s_len = second.len();
  let mut v = Vec::with_capacity(f_len + s_len);
  let mut f_index = 0;
  let mut s_index = 0;
  // 对比两个序列开头的元素，分别插入新序列
  while f_index < f_len && s_index < s_len {
      let f_ele = first[f_index];
      let s_ele = second[s_index];
      if f_ele < s_ele {
          f_index = f_index + 1;
          v.push(f_ele);
      } else {
          s_index = s_index + 1;
          v.push(s_ele);
      }
  }
  // 检查第一个排序序列里是否有剩余元素
  if f_index < f_len {
      v.extend_from_slice(&first[f_index..]);
  }
  // 检查第二个排序序列里是否有剩余元素
  if s_index < s_len {
      v.extend_from_slice(&second[s_index..]);
  }
  v
}
