pub fn quick_sort(arr: &mut [i32]) {
  if arr.len() <= 1 {
    return;
  }

  let pivot = partition(arr);
  quick_sort(&mut arr[..pivot]);
  quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
  let pivot = arr.len() - 1;
  let mut i = 0;

  for j in 0..pivot {
    if arr[j] < arr[pivot] {
      arr.swap(i, j);
      i += 1;
    }
  }

  arr.swap(i, pivot);

  return i
}