use std::cmp::max;

use crate::utils::read_input_lines;

fn extract_input() -> Vec<Vec<i32>> {
    read_input_lines("input8.txt")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

enum IterationOrder {
    RowWise,
    ColumnWise,
}

enum VisibilityFrom {
    Left,
    Right,
    Up,
    Down,
}

impl VisibilityFrom {
    fn inner_iter(&self, m: usize, n: usize) -> Box<dyn Iterator<Item = usize>> {
        match *self {
            Self::Left => Box::new(0..n),
            Self::Right => Box::new((0..n).rev()),
            Self::Up => Box::new(0..m),
            Self::Down => Box::new((0..m).rev()),
        }
    }

    fn order(&self) -> IterationOrder {
        match self {
            &Self::Left | &Self::Right => IterationOrder::RowWise,
            &Self::Up | &Self::Down => IterationOrder::ColumnWise,
        }
    }
}

fn visibility_matrix(grid: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut visible = vec![vec![false; n]; m];

    for viz_from in [
        VisibilityFrom::Left,
        VisibilityFrom::Right,
        VisibilityFrom::Up,
        VisibilityFrom::Down,
    ] {
        update_visibility_matrix(grid, &mut visible, m, n, viz_from);
    }

    visible
}

fn update_visibility_matrix(
    grid: &[Vec<i32>],
    visible: &mut [Vec<bool>],
    m: usize,
    n: usize,
    viz_from: VisibilityFrom,
) {
    let order = viz_from.order();
    for i in 0..m {
        let mut max_h = -1;
        for j in viz_from.inner_iter(m, n) {
            match order {
                IterationOrder::RowWise => {
                    if grid[i][j] > max_h {
                        max_h = grid[i][j];
                        visible[i][j] = true;
                    }
                }
                IterationOrder::ColumnWise => {
                    if grid[j][i] > max_h {
                        max_h = grid[j][i];
                        visible[j][i] = true;
                    }
                }
            }
        }
    }
}

fn count_visible(visible: &[Vec<bool>]) -> u32 {
    visible.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |inner_acc, val| inner_acc + *val as u32)
    })
}

pub fn solve_a() {
    let grid = extract_input();
    let visible = visibility_matrix(&grid);
    let ans = count_visible(&visible);

    println!("{ans}");
}

fn compute_scenic_score(grid: &[Vec<i32>], i: usize, j: usize, m: usize, n: usize) -> i32 {
    if i == 0 || i == m || j == 0 || j == n {
        return 0;
    }

    let mut score = 1;
    let mut temp = 0;

    // Left
    for k in (0..j).rev() {
        temp += 1;
        if grid[i][k] >= grid[i][j] {
            break;
        }
    }
    score *= temp;

    temp = 0;
    // Right
    for k in j + 1..n {
        temp += 1;
        if grid[i][k] >= grid[i][j] {
            break;
        }
    }
    score *= temp;

    temp = 0;
    // Up
    for k in (0..i).rev() {
        temp += 1;
        if grid[k][j] >= grid[i][j] {
            break;
        }
    }
    score *= temp;

    temp = 0;
    // Up
    for k in i + 1..m {
        temp += 1;
        if grid[k][j] >= grid[i][j] {
            break;
        }
    }
    score *= temp;

    score
}

pub fn solve_b() {
    let grid = extract_input();
    let (m, n) = (grid.len(), grid[0].len());

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            ans = max(ans, compute_scenic_score(&grid, i, j, m, n));
        }
    }

    println!("{ans}");
}
