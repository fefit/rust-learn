fn main() {
  let mut arr: [i32; 10] = [3, 0, 5, 2, 9, 16, 13, 11, 2, -2];
  quick_sort(&mut arr);
  println!("the sorted array is:{:?}", arr);
}
/**
* 快速排序
* 快速排序法也是使用了分治法，它的核心思想是先选取一个基准值，然后对序列进行扫描，大于基准值的往右排，小于基准值的往左排
*/
fn quick_sort<T:Ord>(arr: &mut [T]) -> Vec<T> {
  // 实现算法
}

fn partition<T:Ord>(arr: &mut[T], left: usize, right: usize, pivot: usize)->usize{
  let pivot_value = arr[pivot];
   
  while right >= left{

  }
}

fn _quick(){

}