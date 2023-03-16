use ndarray::{Array1, Array2};

#[test]
fn test() {
    let room = Array2::from_shape_fn((3, 3), |_| 1);
    let solved = get_buttons_states(&room);
    println!("{:?}", solved)
}

// 计算房间中所有灯的状态
pub fn get_lights_states(room: &Array2<i8>) -> Array2<i8> {
    let (rows, cols) = room.dim();

    // 灯的状态数组
    let mut lights = Array2::<i8>::zeros((rows, cols));

    for i in 0..rows {
        for j in 0..cols {
            // 根据算法2计算灯的状态
            let mut sum = 0;
            for k in 0..cols {
                sum += room[[i, k]] * if k == j { 1 } else { 0 };
            }
            for k in 0..rows {
                sum += room[[k, j]] * if k == i { 1 } else { 0 };
            }
            sum -= room[[i, j]] * 2;
            lights[[i, j]] = if sum == 0 { 0 } else { 1 };
        }
    }

    lights
}

// 使用高斯消元法计算按钮的状态
pub fn get_buttons_states(room: &Array2<i8>) -> Array2<i8> {
    let (rows, cols) = room.dim();

    // 构造方程组
    let mut mat = Array2::<i8>::zeros((rows * cols, rows * cols));
    let mut b = Array1::<i8>::zeros(rows * cols);

    for i in 0..rows {
        for j in 0..cols {
            let index = i * cols + j;

            // 对于每个按钮，根据算法2构造方程
            let mut row = vec![0; rows * cols];
            let mut rhs = 0;
            for k in 0..cols {
                row[i * cols + k] += 1;
                row[k * cols + j] += 1;
                rhs += room[[i, k]] + room[[k, j]];
            }
            row[index] -= 2;
            b[index] = -rhs;

            // 将方程加入矩阵
            for k in 0..rows * cols {
                mat[[index, k]] = row[k];
            }
        }
    }

    // 解方程组
    gauss_elimination(&mut mat, &mut b);

    // 将解转换为按钮状态数组
    let mut buttons = Array2::<i8>::zeros((rows, cols));
    for i in 0..rows {
        for j in 0..cols {
            let index = i * cols + j;
            buttons[[i, j]] = if b[index] == 1 { 0 } else { 1 };
        }
    }

    buttons
}

fn gauss_elimination(mat: &mut Array2<i8>, b: &mut Array1<i8>) {
    let (rows, cols) = mat.dim();

    // 逐行进行高斯消元
    for i in 0..rows {
        // 选取主元
        let mut max = i;
        for j in i + 1..rows {
            if mat[[j, i]].abs() > mat[[max, i]].abs() {
                max = j;
            }
        }

        // 交换主元所在行
        if max != i {
            for j in i..cols {
                mat.swap((i, j), (max, j));
            }
            b.swap(i, max);
        }

        // 消元
        for j in i + 1..rows {
            let mut factor = mat[[j, i]] / mat[[i, i]];
            for k in i..cols {
                mat[[j, k]] -= factor * mat[[i, k]];
            }
            b[j] -= factor * b[i];
        }
    }

    // 回代
    for i in (0..rows).rev() {
        for j in i + 1..cols {
            b[i] -= mat[[i, j]] * b[j];
        }
        b[i] /= mat[[i, i]];
    }




}