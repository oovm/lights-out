#![feature(generators)]

use gen_iter::GenIter;
use ndarray::Array2;

// use ndarray_linalg::null_space;

#[test]
fn main() {
    let size = 3;
    let double = size * 2;
    let mut array = Array2::<bool>::from_shape_fn((double, double), |_| false);
    for (x, y) in make_true(size, double) {
        match array.get_mut((x, y)) {
            Some(s) => {
                *s = true;
            }
            None => {
                panic!("Out of bounds {x} {y}");
            }
        }
    }
    let array_u = Array2::<u8>::from_shape_fn((double, double), |(a, b)| {
        if array[[a, b]] {
            1
        } else {
            0
        }
    });
    println!("{}", array_u)
}

fn make_true(size: usize, double: usize) -> impl Iterator<Item=(usize, usize)> {
    let array = Array2::<usize>::from_shape_fn((size, 2), |(x, y)| x + y * size);
    println!("{}", array);
    GenIter(move || {
        // a_1, 0
        yield (0, 0);
        yield (0, 1);
        yield (0, size - 1);
        // a_n, 2
        yield (size - 1, size - 2);
        yield (size - 1, size - 1);
        yield (size - 1, double - 1);
        // b_1, 3
        yield (size, 0);
        yield (size, size);
        yield (size, size + 1);
        // b_n
        yield (double - 1, size - 1);
        yield (double - 1, double - 2);
        yield (double - 1, double - 1);
        for line in 1usize..(size - 1) {
            println!("line: {}", line);
            // a_k
            yield (line, line - 1);
            yield (line, line);
            yield (line, line + 1);
            yield (line, line + size);
            // b_k
            yield (line + size, line);
            yield (line + size, line + size - 1);
            yield (line + size, line + size);
            yield (line + size, line + size + 1);
        }
    })
}
