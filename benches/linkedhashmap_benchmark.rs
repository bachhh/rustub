use criterion::Criterion;
use criterion::{criterion_group, criterion_main, BatchSize};
use std::collections::LinkedList;

fn increment_list(mut list: LinkedList<i64>) {
    for element in list.iter_mut() {
        *element += 1;
    }
}

fn increment_vec(mut vec: Vec<i64>) {
    for element in vec.iter_mut() {
        *element += 1;
    }
}

fn bench(c: &mut Criterion) {
    // test param = array & linkedlist of increasingly larger size, for array we will consider
    // 4 density factors: 25% 50% 75% 100%
    // test case involve exponentials of 2 up to 2^10
    let mut group = c.benchmark_group("My Group");
    for exp in 1..17 {
        let x = 1 << exp;

        group.bench_function(&format!("llist {}", x).to_string(), move |b| {
            b.iter_batched(
                || {
                    let mut list = LinkedList::<i64>::new();
                    for i in 1..x {
                        list.push_back(i);
                    }
                    list
                },
                increment_list,
                BatchSize::SmallInput,
            );
        });

        group.bench_function(&format!("array {}-25", x).to_string(), move |b| {
            b.iter_batched(
                || {
                    let mut v25: Vec<i64> = vec![0; (x * 2) as usize];
                    for (i, v) in v25.iter_mut().enumerate() {
                        if i % 4 == 0 {
                            *v = i as i64;
                        }
                    }
                    v25
                },
                increment_vec,
                BatchSize::SmallInput,
            );
        });

        //         group.bench_function(&format!("array {}-50", x).to_string(), move |b| {
        //             b.iter_batched(
        //                 || {
        //                     let mut v50: Vec<i64> = vec![0; (x * 2) as usize];
        //                     for (i, v) in v50.iter_mut().enumerate() {
        //                         if i % 2 != 0 {
        //                             *v = i as i64;
        //                         }
        //                     }
        //                     v50
        //                 },
        //                 increment_vec,
        //                 BatchSize::SmallInput,
        //             );
        //         });

        //         group.bench_function(&format!("array {}-75", x).to_string(), move |b| {
        //             b.iter_batched(
        //                 || {
        //                     let mut v75: Vec<i64> = vec![0; (x * 2) as usize];
        //                     for (i, v) in v75.iter_mut().enumerate() {
        //                         if i % 4 != 0 {
        //                             *v = i as i64;
        //                         }
        //                     }
        //                     v75
        //                 },
        //                 increment_vec,
        //                 BatchSize::SmallInput,
        //             );
        //         });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
