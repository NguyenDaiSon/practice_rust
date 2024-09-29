fn approximate_square_root(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }

    let (mut index, mut left, mut right) = (-1, 1, x / 2);
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid == x / mid {
            index = mid;
            break;
        }

        if mid < x / mid {
            index = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
            index = right;
        }
    }

    index
}

fn main() {
    for i in 0..129 {
        println!("{} -> {}", i, approximate_square_root(i))
    }
}
