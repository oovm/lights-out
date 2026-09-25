#![feature(generators)]

use gen_iter::GenIter;
use ndarray::Array2;

// use ndarray_linalg::null_space;
mod gauss_method;

#[test]
fn main() {
    let size = 3;
    let double = size * 2;
    let mut array = Array2::<bool>::from_shape_fn((double, double), |_| false);
    for x in 0..double {
        for y in adjacent_indices(x, size) {
            match array.get_mut((x, y)) {
                Some(s) => {
                    *s = true;
                }
                None => {
                    panic!("Out of bounds {x} {y}");
                }
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

fn adjacent_indices(index: usize, size: usize) -> impl Iterator<Item=usize> {
    let array_size = size * 2;
    // Calculate row and column indices from index
    let row = if index < size { 0 } else { 1 };
    let col = if index < size { index } else { index - size };
    // Check adjacent elements
    GenIter(move || {
        yield index;
        if col > 0 {
            yield row * size + col - 1
        }
        if col < size - 1 {
            yield row * size + col + 1;
        }
        if row == 0 {
            yield (size + col) % array_size;
        } else {
            yield col;
        }
    })
}

fn solve_lights_out_game(room: &Array2<u8>) -> Array2<u8> {
    let n = room.shape()[0];
    let mut button = room.clone();
    let mut light = Array2::<u8>::zeros([n, n]);

    // 计算每个灯的状态
    for i in 0..n {
        for j in 0..n {
            light[[i, j]] = button[[i, j]];
            if i > 0 {
                light[[i, j]] += light[[i - 1, j]];
            }
            if j > 0 {
                light[[i, j]] += light[[i, j - 1]];
            }
            if i < n - 1 {
                light[[i, j]] += light[[i + 1, j]];
            }
            if j < n - 1 {
                light[[i, j]] += light[[i, j + 1]];
            }
            light[[i, j]] %= 2;
        }
    }

    // 根据按钮的状态更新灯的状态
    for i in 0..n {
        for j in 0..n {
            if button[[i, j]] == 1 {
                light[[i, j]] = (light[[i, j]] + 1) % 2;
                if i > 0 {
                    light[[i - 1, j]] = (light[[i - 1, j]] + 1) % 2;
                }
                if j > 0 {
                    light[[i, j - 1]] = (light[[i, j - 1]] + 1) % 2;
                }
                if i < n - 1 {
                    light[[i + 1, j]] = (light[[i + 1, j]] + 1) % 2;
                }
                if j < n - 1 {
                    light[[i, j + 1]] = (light[[i, j + 1]] + 1) % 2;
                }
            }
        }
    }

    light
}

#[test]
fn main3() {
    let mut room = Array2::<u8>::from_elem((3, 3), 1);

    // turn on some lights
    // room[[0, 0]] = 1;
    // room[[1, 1]] = 1;
    // room[[2, 2]] = 1;

    let button_states = solve_lights_out_game(&room);

    println!("{}", button_states)
}

fn make_true(size: usize, double: usize) -> impl Iterator<Item=(usize, usize)> {
    let array = Array2::<usize>::from_shape_fn((2, size), |(x, y)| x + y * size);
    println!("{}", array);
    for i in array {}
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
