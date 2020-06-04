fn main() {
  let arr: [i32; 10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  let v = bucket_sort(&arr);
  println!("the sorted array is:{:?}", v);
}

/**
* 桶排序
*/
fn bucket_sort(arr: &[i32]) -> Vec<i32> {
}