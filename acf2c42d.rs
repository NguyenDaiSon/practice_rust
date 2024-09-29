fn binary_search(a: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, a.len() as isize - 1);
    while left <= right {
        let middle = left + (right - left) / 2;
        if a[middle as usize] == target {
            return Some(middle as usize);
        }

        if a[middle as usize] < target {
            left = middle + 1
        } else {
            right = middle - 1
        }
    }

    None
}

fn main() {
    println!("{:?}", binary_search(&[], 0));
    println!("{:?}", binary_search(&[1], 2));
    for i in 0..5 {
        println!("{:?}", binary_search(&[1, 2, 3, 4], i));
    }
}
