fn quick_sort(a: &mut Vec<i32>) {
    let (start, end) = (0, a.len());
    quick_sort_impl(a, start, end);
}

fn quick_sort_impl(a: &mut Vec<i32>, start: usize, end: usize) {
    if end - start <= 1 {
        return;
    }

    let pivot = a[end - 1];
    let (mut i, mut j) = (start, end - 1);

    while i < j {
        while a[i] < pivot && i < j {
            i += 1
        }

        while a[j] >= pivot && i < j {
            j -= 1
        }

        a.swap(i, j);
    }
    a.swap(i, end - 1);

    quick_sort_impl(a, start, i);
    quick_sort_impl(a, i + 1, end);
}

fn main() {
    let mut a = vec![2, 1, 3, 4];
    quick_sort(&mut a);
    println!("{:?}", a);
}
