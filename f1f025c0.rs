fn find_boundary(a: Vec<bool>) -> Option<i32> {
    let n = a.len();
    let (mut left, mut right) = (0, n as i32 - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if !a[mid as usize] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    if left >= n as i32 {
        None
    } else {
        Some(left)
    }
}

fn main() {
    println!("{:?}", find_boundary(Vec::<bool>::new()));
    println!("{:?}", find_boundary(vec![false, false, false]));
    println!("{:?}", find_boundary(vec![true, true, true]));
    println!("{:?}", find_boundary(vec![false, true, true]));
    println!("{:?}", find_boundary(vec![false, false, true]));
}
