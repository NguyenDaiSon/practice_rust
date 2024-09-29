fn find_min(a: Vec<i32>) -> Option<i32> {
    let n = a.len();
    if n == 0 {
        return None;
    }

    let (mut left, mut right) = (0, n as i32 - 1);
    let mut index = 0;

    while left <= right {
        let mid = left + (right - left) / 2;
        if a[mid as usize] <= a[n - 1] {
            index = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    Some(a[index as usize])
}

fn main() {
    println!("{:?}", find_min(vec![3, 4, 5, 1, 2]));
    println!("{:?}", find_min(vec![4,5,6,7,0,1,2]));
    println!("{:?}", find_min(vec![11,13,15,17]));
    println!("{:?}", find_min(Vec::<i32>::new()));
}
