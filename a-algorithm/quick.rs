fn main() {
  let mut arr: [i32; 10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  quick_sort(&mut arr);
  println!("the sorted array is:{:?}", arr);
}
/**
* 快速排序
* 快速排序法也是使用了分治法，它的核心思想是先选取一个基准值，然后对序列进行扫描，大于基准值的往右排，小于基准值的往左排
* 完成上述一轮扫描后，基准值将基本处于一个较中心的位置<取决于基准值，所以好的基准值能带来排序的优化>，然后将基准值的左右两边分别继续进行排序
*/
fn quick_sort<T:Ord>(arr: &mut [T]) {
  let len = arr.len();
  if len > 1{
    // 实现算法
    _quick(arr, 0, (len - 1) as isize);
  }
}

// 获取基准值索引值
fn partition<T:Ord>(arr: &mut[T], start_index: usize, end_index: usize, pivot: usize)->usize{
  // 单边扫描法
  arr.swap(pivot, end_index);
  let mut store_index = start_index;
  for i in  start_index..end_index{
    if arr[i] < arr[end_index]{
      arr.swap(store_index, i);
      store_index += 1;
    }
  }
  arr.swap(store_index, end_index);
  store_index
}

fn _quick<T:Ord>(arr:&mut[T], start_index: isize, end_index: isize){
  if end_index > start_index{
    // 按照常用做法，基准值取最左边第一条
    // 也可以通过取中值等方法、优化排序
    let pivot_index = partition(arr, start_index as usize, end_index as usize, start_index as usize) as isize;
    _quick(arr, start_index, pivot_index - 1);
    _quick(arr, pivot_index + 1, end_index);
  }
}