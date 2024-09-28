fn bubble_sort(a: &mut Vec<i32>) {
    let n = a.len();
    for i in (0..n).rev() {
        let mut swapped = false;
        for j in 0..i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut a = vec![2, 1, 3, 4];
    bubble_sort(&mut a);
    println!("{:?}", a);
}
