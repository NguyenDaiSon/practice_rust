fn insert_sort(a: &mut Vec<i32>) {
    let n = a.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && a[j] < a[j - 1] {
            a.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut a = vec![2, 1, 3, 4];
    insert_sort(&mut a);
    println!("{:?}", a);
}
