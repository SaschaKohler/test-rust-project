use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

// ListNode for cache-unfriendly traversal
struct ListNode {
    value: i64,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(value: i64) -> Self {
        ListNode { value, next: None }
    }
}

// Compute-intensive task
fn compute_intensive_task() {
    const SIZE: usize = 1_000_000;
    let mut data: Vec<i32> = vec![0; SIZE];

    let mut rng = StdRng::seed_from_u64(42);
    for i in 0..SIZE {
        data[i] = rng.gen_range(1..=1000);
    }

    for _ in 0..10 {
        data.sort();
        data.reverse();
    }

    let sum: i64 = data.iter().map(|&x| x as i64).sum();

    println!("Sum: {}", sum);
    println!("Average: {:.2}", sum as f64 / SIZE as f64);
}

// Memory stress test
fn memory_stress_test() {
    const NUM_ALLOCATIONS: usize = 1000;
    const ALLOCATION_SIZE: usize = 1024 * 1024; // 1MB

    let mut allocations: Vec<Vec<u8>> = Vec::with_capacity(NUM_ALLOCATIONS);

    for i in 0..NUM_ALLOCATIONS {
        let mut buffer = vec![0u8; ALLOCATION_SIZE];
        for j in (0..ALLOCATION_SIZE).step_by(4096) {
            buffer[j] = (i % 256) as u8;
        }
        allocations.push(buffer);

        if i % 100 == 0 {
            thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    drop(allocations);

    println!("Allocated and freed {} MB of memory", NUM_ALLOCATIONS);
}

// Fibonacci calculation
fn fibonacci_calculation() {
    const N: i32 = 40;

    fn fibonacci(n: i32) -> i64 {
        if n <= 1 {
            return n as i64;
        }
        let mut a: i64 = 0;
        let mut b: i64 = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }

    println!("Calculating Fibonacci numbers...");
    for i in 1..=N {
        let result = fibonacci(i);
        if i % 10 == 0 {
            println!("F({}) = {}", i, result);
        }
    }
}

// Cache-unfriendly linked list traversal
fn cache_unfriendly_traversal() {
    println!("Running cache-unfriendly linked list traversal...");
    const NUM_NODES: usize = 500_000;

    let mut nodes: Vec<Box<ListNode>> = (0..NUM_NODES)
        .map(|i| Box::new(ListNode::new(i as i64)))
        .collect();

    let mut rng = StdRng::seed_from_u64(42);
    nodes.shuffle(&mut rng);

    // Link nodes
    for i in 0..nodes.len() - 1 {
        let next_ptr = nodes[i + 1].as_mut() as *mut ListNode;
        nodes[i].next = Some(unsafe { Box::from_raw(next_ptr) });
    }

    // Traverse
    let head = &nodes[0];
    let mut sum: i64 = 0;
    const ITERATIONS: usize = 50;

    for _ in 0..ITERATIONS {
        let mut current: Option<&ListNode> = Some(head);
        while let Some(node) = current {
            sum += node.value;
            current = node.next.as_ref().map(|b| b.as_ref());
        }
    }

    let expected_sum = (NUM_NODES as i64 * (NUM_NODES as i64 - 1) / 2) * ITERATIONS as i64;
    println!(
        "Cache-unfriendly sum (should be ~{}): {}",
        expected_sum, sum
    );

    // Prevent double-free by forgetting nodes
    std::mem::forget(nodes);
}

// Branch misprediction test
fn branch_misprediction_test() {
    println!("Running branch misprediction test...");
    const SIZE: usize = 10_000_000;
    let mut data: Vec<i32> = vec![0; SIZE];

    let mut rng = StdRng::seed_from_u64(42);
    for i in 0..SIZE {
        data[i] = rng.gen_range(0..100);
    }

    let mut sum: i64 = 0;
    for &val in &data {
        if val < 30 {
            sum += (val * 2) as i64;
        } else if val < 60 {
            sum += (val * 3) as i64;
        } else if val < 80 {
            sum += (val * 5) as i64;
        } else {
            sum += (val * 7) as i64;
        }
    }
    println!("Branch test sum: {}", sum);
}

// Deep recursive call
fn deep_recursive_call(depth: i32, iterations: i32) -> i32 {
    if depth <= 0 {
        let mut sum: i64 = 0;
        for i in 0..iterations {
            sum += i as i64;
        }
        return (sum % 1_000_000) as i32;
    }
    deep_recursive_call(depth - 1, iterations) + depth
}

// Deep recursion test
fn deep_recursion_test() {
    println!("Running deep recursion test...");
    let mut result: i32 = 0;
    for _ in 0..100 {
        result += deep_recursive_call(200, 1000);
    }
    println!("Deep recursion result: {}", result);
}

// Virtual function dispatch
trait Processor {
    fn process(&self, x: i32) -> i32;
}

struct ProcessorA;
struct ProcessorB;
struct ProcessorC;

impl Processor for ProcessorA {
    fn process(&self, x: i32) -> i32 {
        x * 2 + 1
    }
}

impl Processor for ProcessorB {
    fn process(&self, x: i32) -> i32 {
        x * 3 - 1
    }
}

impl Processor for ProcessorC {
    fn process(&self, x: i32) -> i32 {
        x * x + x
    }
}

// Virtual dispatch test
fn virtual_dispatch_test() {
    println!("Running virtual dispatch test...");
    const NUM_OBJECTS: usize = 1000;
    const ITERATIONS: usize = 10000;

    let mut processors: Vec<Box<dyn Processor>> = Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        match i % 3 {
            0 => processors.push(Box::new(ProcessorA)),
            1 => processors.push(Box::new(ProcessorB)),
            _ => processors.push(Box::new(ProcessorC)),
        }
    }

    let mut total: i64 = 0;
    for iter in 0..ITERATIONS {
        for (i, processor) in processors.iter().enumerate() {
            total += processor.process((i + iter) as i32) as i64;
        }
    }
    println!("Virtual dispatch total: {}", total);
}

// STL-heavy template test
fn stl_heavy_template_test() {
    println!("Running STL heavy template test...");
    const NUM_ELEMENTS: usize = 100_000;

    let mut complex_map: BTreeMap<String, Vec<(i32, f64)>> = BTreeMap::new();
    let mut reverse_lookup: HashMap<i32, HashSet<String>> = HashMap::new();

    let mut rng = StdRng::seed_from_u64(42);

    for i in 0..NUM_ELEMENTS {
        let key = format!("key_{}", i);
        let mut vec = Vec::with_capacity(10);
        for _ in 0..10 {
            vec.push((rng.gen_range(1..1000), rng.gen::<f64>()));
        }
        complex_map.insert(key.clone(), vec);

        reverse_lookup
            .entry((i % 100) as i32)
            .or_insert_with(HashSet::new)
            .insert(key);
    }

    let mut sum: f64 = 0.0;
    for _ in 0..100 {
        for vec in complex_map.values() {
            for (i, d) in vec {
                sum += (*i as f64) * *d;
            }
        }
    }

    let mut intersection_count = 0;
    for i in 0..50 {
        if let (Some(set1), Some(set2)) = (reverse_lookup.get(&i), reverse_lookup.get(&(i + 50))) {
            for key in set1 {
                if set2.contains(key) {
                    intersection_count += 1;
                }
            }
        }
    }

    println!(
        "STL heavy test sum: {:.2}, intersections: {}",
        sum, intersection_count
    );
}

// Lock contention test
fn lock_contention_test() {
    println!("Running lock contention test...");
    const NUM_THREADS: usize = 8;
    const ITERATIONS: usize = 100_000;

    let mtx = Arc::new(Mutex::new(0i64));
    let rw_mtx = Arc::new(RwLock::new(0i64));
    let atomic_counter = Arc::new(AtomicI64::new(0));
    let protected_counter = Arc::new(Mutex::new(0i64));

    let start = Instant::now();
    let mut handles = vec![];

    for thread_id in 0..NUM_THREADS {
        let mtx_clone = Arc::clone(&mtx);
        let rw_mtx_clone = Arc::clone(&rw_mtx);
        let atomic_clone = Arc::clone(&atomic_counter);
        let protected_clone = Arc::clone(&protected_counter);

        let handle = thread::spawn(move || {
            let mut rng = StdRng::seed_from_u64(thread_id as u64);

            for _ in 0..ITERATIONS {
                let op: i32 = rng.gen_range(0..10);
                if op < 4 {
                    let mut guard = protected_clone.lock().unwrap();
                    *guard += thread_id as i64;
                } else if op < 7 {
                    let _guard = rw_mtx_clone.read().unwrap();
                    let _ = *mtx_clone.lock().unwrap();
                } else {
                    atomic_clone.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!(
        "Lock contention test complete: {}ms, atomic: {}, protected: {}",
        duration.as_millis(),
        atomic_counter.load(Ordering::Relaxed),
        *protected_counter.lock().unwrap()
    );
}

// Memory fragmentation test
fn memory_fragmentation_test() {
    println!("Running memory fragmentation test...");
    let mut rng = StdRng::seed_from_u64(42);

    let mut allocations: Vec<Vec<u8>> = Vec::with_capacity(5000);

    for i in 0..5000 {
        let size: usize = rng.gen_range(64..=65536);
        let mut buf = vec![0u8; size];
        for j in (0..size).step_by(4096) {
            buf[j] = (i % 256) as u8;
        }
        allocations.push(buf);

        if i % 3 == 0 && !allocations.is_empty() {
            allocations.pop();
        }
    }

    for phase in 0..5 {
        for i in 0..allocations.len() {
            let size: usize = rng.gen_range(64..=65536);
            let mut buf = vec![0u8; size];
            for j in (0..size).step_by(4096) {
                buf[j] = phase as u8;
            }
            allocations[i] = buf;
        }
    }

    println!(
        "Memory fragmentation test complete, {} blocks",
        allocations.len()
    );
}

fn main() {
    println!("=== RealBench Rust Test Application ===");
    println!("Running performance-intensive tasks...");

    let start = Instant::now();

    println!("\n1. Running compute-intensive task...");
    compute_intensive_task();

    println!("\n2. Running memory stress test...");
    memory_stress_test();

    println!("\n3. Running Fibonacci calculation...");
    fibonacci_calculation();

    println!("\n4. Running cache-unfriendly traversal...");
    cache_unfriendly_traversal();

    println!("\n5. Running branch misprediction test...");
    branch_misprediction_test();

    println!("\n6. Running deep recursion test...");
    deep_recursion_test();

    println!("\n7. Running virtual dispatch test...");
    virtual_dispatch_test();

    println!("\n8. Running STL heavy template test...");
    stl_heavy_template_test();

    println!("\n9. Running lock contention test...");
    lock_contention_test();

    println!("\n10. Running memory fragmentation test...");
    memory_fragmentation_test();

    let duration = start.elapsed();

    println!("\n=== Performance Test Complete ===");
    println!("Total execution time: {} ms", duration.as_millis());
}

// create correct url to project-run in dashboard 
