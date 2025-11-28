#![cfg(feature = "std")]

use criterion::{Criterion, criterion_group, criterion_main};
use embed_db::Database;
use std::hint::black_box;

mod helpers;

fn bench_insert(c: &mut Criterion) {
    let mock_data = helpers::create_mock_data();

    let mut group = c.benchmark_group("Insert Operations");

    group.bench_function("HashMap insert 1k", |b| {
        b.iter(|| {
            let mut map = Database::default().json_map;

            for user in &mock_data {
                map.insert(black_box(user.clone())).unwrap();
            }

            black_box(map)
        });
    });

    group.bench_function("Btree insert 1k", |b| {
        b.iter(|| {
            let mut btree = Database::default().json_btree;

            for user in &mock_data {
                btree.insert(black_box(user.clone())).unwrap();
            }

            black_box(btree);
        });
    });

    group.finish();
}

fn bench_get(c: &mut Criterion) {
    let mock_data = helpers::create_mock_data();

    let mut db = Database::default();

    for user in &mock_data {
        db.json_map.insert(user.clone()).unwrap();
        db.json_btree.insert(user.clone()).unwrap();
    }

    let mut group = c.benchmark_group("Get Operation");

    group.bench_function("HashMap get 1k", |b| {
        b.iter(|| {
            for user in &mock_data {
                black_box(db.json_map.get(&user.id).expect("Failed to get user"));
            }
        });
    });

    group.bench_function("Btree get 1k", |b| {
        b.iter(|| {
            for user in &mock_data {
                black_box(
                    db.json_btree
                        .get(black_box(&user.id))
                        .expect("Failed to get user"),
                );
            }
        });
    });

    group.finish();
}

fn bench_delete(c: &mut Criterion) {
    let mock_data = helpers::create_mock_data();

    let mut group = c.benchmark_group("Delete Operation");

    group.bench_function("HashMap delete 1k", |b| {
        b.iter_batched(
            || {
                let mut map = Database::default().json_map;
                for user in &mock_data {
                    map.insert(user.clone()).unwrap();
                }

                map
            },
            |mut map| {
                for user in &mock_data {
                    black_box(
                        map.delete(&user.id)
                            .expect(&format!("Failed to delete {:?}", user)),
                    );
                }
            },
            // This tells Criterion how often to re-run the setup:
            // SmallInput - Run setup for every iteration
            // LargeInput - Run setup less frequently (better for expensive setup like inserting 1k records)
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("Btree delete 1k", |b| {
        b.iter_batched(
            || {
                let mut btree = Database::default().json_btree;
                for user in &mock_data {
                    btree.insert(user.clone()).unwrap();
                }
                btree
            },
            |mut btree| {
                for user in &mock_data {
                    black_box(
                        btree
                            .delete(&user.id)
                            .expect(&format!("Failed to delete {:?}", user)),
                    );
                }
            },
            criterion::BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_insert, bench_get, bench_delete);
criterion_main!(benches);
