fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    if rows == 0 {
        return false;
    }

    let cols = matrix[0].len();
    if cols == 0 {
        return false;
    }

    let (mut left, mut right) = (0, rows as i32 * cols as i32 - 1);

    while left <= right {
        let mid = left + (right - left) / 2;
        let val = matrix[mid as usize / cols][mid as usize % cols];
        if val == target {
            return true;
        }

        if val < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }

    false
}

fn main() {
    println!("{}", search_matrix(vec![vec![]], 0));
    println!("{}", search_matrix(vec![vec![1]], 0));
    println!("{}", search_matrix(vec![vec![1]], 1));
    println!("{}", search_matrix(vec![vec![1,3,5,7], vec![10,11,16,20],vec![23,30,34,60]], 1));
}
