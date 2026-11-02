export type Board = number[][];

export function createBoard(size: number, seedOn = true): Board {
    const board: Board = [];
    for (let r = 0; r < size; r++) {
        const row: number[] = [];
        for (let c = 0; c < size; c++) {
            row.push(seedOn ? 1 : 0);
        }
        board.push(row);
    }
    return board;
}

export function toggleCell(board: Board, row: number, col: number): Board {
    const size = board.length;
    const next = board.map((line) => [...line]);
    const deltas = [
        [0, 0],
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1],
    ];
    for (const [dr, dc] of deltas) {
        const r = row + dr;
        const c = col + dc;
        if (r >= 0 && r < size && c >= 0 && c < size) {
            next[r][c] ^= 1;
        }
    }
    return next;
}

function cloneMatrix(matrix: number[][]): number[][] {
    return matrix.map((row) => [...row]);
}

function gaussEliminationGF2(matrix: number[][]): number[] | null {
    const m = cloneMatrix(matrix);
    const n = m[0]?.length ?? 0;
    const pivots: number[] = [];
    let row = 0;

    for (let col = 0; col < n && row < m.length; col++) {
        let pivot = -1;
        for (let r = row; r < m.length; r++) {
            if (m[r][col] === 1) {
                pivot = r;
                break;
            }
        }
        if (pivot === -1) continue;

        if (pivot !== row) {
            [m[row], m[pivot]] = [m[pivot], m[row]];
        }

        for (let r = 0; r < m.length; r++) {
            if (r !== row && m[r][col] === 1) {
                for (let c = col; c < n; c++) {
                    m[r][c] ^= m[row][c];
                }
            }
        }
        pivots.push(col);
        row++;
    }

    for (let r = 0; r < m.length; r++) {
        const lhs = m[r].slice(0, n - 1);
        const rhs = m[r][n - 1];
        if (lhs.every((v) => v === 0) && rhs === 1) return null;
    }

    const solution = new Array<number>(n - 1).fill(0);
    for (let r = m.length - 1; r >= 0; r--) {
        let pivotCol = -1;
        for (let c = 0; c < n - 1; c++) {
            if (m[r][c] === 1) {
                pivotCol = c;
                break;
            }
        }
        if (pivotCol === -1) continue;
        let value = m[r][n - 1];
        for (let c = pivotCol + 1; c < n - 1; c++) {
            value ^= m[r][c] & solution[c];
        }
        solution[pivotCol] = value;
    }
    return solution;
}

export function buildToggleMatrix(size: number): number[][] {
    const n = size * size;
    const matrix: number[][] = [];

    for (let r = 0; r < size; r++) {
        for (let c = 0; c < size; c++) {
            const row: number[] = new Array(n + 1).fill(0);
            const idx = r * size + c;
            const deltas = [
                [0, 0],
                [-1, 0],
                [1, 0],
                [0, -1],
                [0, 1],
            ];
            for (const [dr, dc] of deltas) {
                const nr = r + dr;
                const nc = c + dc;
                if (nr >= 0 && nr < size && nc >= 0 && nc < size) {
                    row[nr * size + nc] = 1;
                }
            }
            matrix.push(row);
        }
    }
    return matrix;
}

export function solveLightsOut(board: Board): number[] | null {
    const size = board.length;
    const matrix = buildToggleMatrix(size);
    const flat: number[] = [];
    for (let r = 0; r < size; r++) {
        for (let c = 0; c < size; c++) {
            flat.push(board[r][c] & 1);
        }
    }
    for (let i = 0; i < matrix.length; i++) {
        matrix[i][size * size] = flat[i];
    }
    return gaussEliminationGF2(matrix);
}

export function pressesToBoard(size: number, presses: number[]): Board {
    let board = createBoard(size, false);
    for (let i = 0; i < presses.length; i++) {
        if (!presses[i]) continue;
        const r = Math.floor(i / size);
        const c = i % size;
        board = toggleCell(board, r, c);
    }
    return board;
}
