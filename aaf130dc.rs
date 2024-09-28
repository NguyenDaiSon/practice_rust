fn select_sort(a: &mut Vec<i32>) {
    let n = a.len();
    for i in 0..n {
        let mut m = i;
        for j in i..n {
            if a[j] < a[m] {
                m = j
            }
        }
        if m != i {
            a.swap(m, i)
        }
    }
}

fn main() {
    let mut a = vec![2, 1, 3, 4];
    select_sort(&mut a);
    println!("{:?}", a);
}
