fn merge_sort(a: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    if n <= 1 {
        return a;
    }

    let (mut l, mut r) = partition(a);
    l = merge_sort(l);
    r = merge_sort(r);

    merge(l, r)
}

fn partition(a: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let n = a.len();
    let l = a[..n / 2].to_vec();
    let r = a[n / 2..].to_vec();

    (l, r)
}

fn merge(l: Vec<i32>, r: Vec<i32>) -> Vec<i32> {
    let mut a = Vec::new();
    let (mut i, mut j) = (0, 0);
    let (nl, nr) = (l.len(), r.len());

    while i < nl && j < nr {
        if l[i] < r[j] {
            a.push(l[i]);
            i += 1;
        } else {
            a.push(r[j]);
            j += 1;
        }
    }

    while i < nl {
        a.push(l[i]);
        i += 1;
    }

    while j < nr {
        a.push(r[j]);
        j += 1;
    }

    a
}

fn main() {
    println!("{:?}", merge_sort(vec![2, 1, 3, 4]));
}
