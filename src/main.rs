fn main() {
    let my_vec: Vec<u8> = vec![6, 2, 1, 8, 7, 3, 5, 4];
    let new_vec = quick_sort(my_vec);
    for value in new_vec {
        print!("{} ", value);
    }
}

fn quick_sort(mut pool: Vec<u8>) -> Vec<u8>{
    if pool.len() == 0 {
        return pool;
    }

    let mut pivot = pool.len();
    let mut i = 0;
    let mut j = 1;

    while j != pivot {
        if pool[j - 1] < pool[pivot - 1] {
            i += 1;
            let swapping_number = pool[j - 1];
            pool[j - 1] = pool[i - 1];
            pool[i - 1] = swapping_number;
        }
        j += 1;
    }
    let saved_pivot = pool[pivot - 1];

    pool.remove(pivot - 1);

    pivot = i;

    pool.insert(pivot, saved_pivot);

    if pool.len() < 3 {
        return pool;
    }

    let split = pool.split_at(pivot);

    let left = quick_sort(split.0.to_vec());
    let mut right = quick_sort(split.1.to_vec());

    let mut new_pool = left;
    new_pool.append(&mut right);

    new_pool
}