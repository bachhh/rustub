use criterion::Criterion;
use criterion::{criterion_group, criterion_main, BatchSize};
use std::collections::LinkedList;

fn increment_list(mut list: LinkedList<i64>) {
    for element in list.iter_mut() {
        *element += 1;
    }
}

// fn increment_vec(mut vec: Vec<i64>) {}

fn bench(c: &mut Criterion) {
    // test param = array & linkedlist of increasingly larger size, for array we will consider
    // 4 density factors: 25% 50% 75% 100%
    // test case involve exponentials of 2 up to 2^10
    for exp in 1..15 {
        let mut list = LinkedList::<i64>::new();
        for i in 1..2 ^ exp {
            list.push_back(i);
        }

        // // create 4 vectors of density 25, 50, 75, 100
        // let mut v25 = vec![0; (2 ^ exp * 4) as usize];
        // for i in 1..2 ^ exp * 4 {
        //     if i % 4 == 0 {
        //         v25[(i - 1) as usize] = i;
        //     }
        // }

        // let mut v50 = vec![0; (2 ^ exp * 2) as usize];
        // for i in 1..2 ^ exp * 4 {
        //     if i % 2 != 0 {
        //         v50[(i - 1) as usize] = i;
        //     }
        // }

        // let mut v75 = vec![0; (2 ^ exp * 2) as usize];
        // for i in 1..2 ^ exp * 4 {
        //     if i % 4 != 0 {
        //         v75[(i - 1) as usize] = i;
        //     }
        // }

        // let mut v100 = vec![0; (2 ^ exp * 2) as usize];
        // for i in 1..2 ^ exp * 4 {
        //     v100[(i - 1) as usize] = i;
        // }

        c.bench_function(&format!("llist {}", 2 ^ exp).to_string(), move |b| {
            b.iter_batched(|| list.clone(), increment_list, BatchSize::SmallInput);
        });

        // c.bench_function(&format!("array {}-25", 2 ^ exp).to_string(), |b| {
        //     b.iter(|| increment_vec(black_box(v25)))
        // });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
