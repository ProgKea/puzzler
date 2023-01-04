use macroquad::prelude::*;
use std::collections::VecDeque;

const CELL_SIZE: f32 = 20.0;
const WALL_WIDTH: f32 = 2.0;
const HIGHLIGHT_COLOR1: Color = DARKPURPLE;
const HIGHLIGHT_COLOR2: Color = DARKBLUE;
const FOREGROUND_COLOR: Color = WHITE;
const BACKGROUND_COLOR: Color = BLACK;

fn index(row: i32, col: i32, rows: i32, cols: i32) -> Option<usize> {
    if row < 0 || col < 0 || row > rows - 1 || col > cols - 1 {
        return None;
    }

    return Some((row * rows + col) as usize);
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    row: usize,
    col: usize,
    visited: bool,
    top: bool,
    bot: bool,
    left: bool,
    right: bool,
}

impl Default for Cell {
    fn default() -> Self {
        return Self {
            row: 0,
            col: 0,
            visited: false,
            top: true,
            bot: true,
            left: true,
            right: true,
        };
    }
}

impl Cell {
    fn highlight(&self, color: Color) {
        let x = self.col as f32 * CELL_SIZE;
        let y = self.row as f32 * CELL_SIZE;

        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
    }

    fn draw(&self) {
        let x = self.col as f32 * CELL_SIZE;
        let y = self.row as f32 * CELL_SIZE;

        if self.visited {
            self.highlight(HIGHLIGHT_COLOR1);
        }

        // up
        if self.top {
            draw_line(x, y, x + CELL_SIZE, y, WALL_WIDTH, FOREGROUND_COLOR);
        }

        // down
        if self.bot {
            draw_line(
                x,
                y + CELL_SIZE,
                x + CELL_SIZE,
                y + CELL_SIZE,
                WALL_WIDTH,
                FOREGROUND_COLOR,
            );
        }

        // left
        if self.left {
            draw_line(x, y, x, y + CELL_SIZE, WALL_WIDTH, FOREGROUND_COLOR);
        }

        // right
        if self.right {
            draw_line(
                x + CELL_SIZE,
                y,
                x + CELL_SIZE,
                y + CELL_SIZE,
                WALL_WIDTH,
                FOREGROUND_COLOR,
            );
        }
    }
}

// TODO: make `current` a mutable reference of a cell
struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Cell>,
    stack: VecDeque<usize>,
    current: usize,
    next: Option<usize>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                cells.push(Cell {
                    row,
                    col,
                    ..Default::default()
                });
            }
        }

        let mut grid = Self {
            rows,
            cols,
            cells,
            stack: VecDeque::new(),
            current: fastrand::usize(..rows * cols),
            next: None,
        };
        grid.update_current();
        return grid;
    }

    fn get_random_neighbor(&self) -> Option<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let cell = &self.cells[self.current];

        let neighbor_index = vec![
            index(
                cell.row as i32 - 1,
                cell.col as i32,
                self.rows as i32,
                self.cols as i32,
            ), // Left
            index(
                cell.row as i32 + 1,
                cell.col as i32,
                self.rows as i32,
                self.cols as i32,
            ), // Right
            index(
                cell.row as i32,
                cell.col as i32 - 1,
                self.rows as i32,
                self.cols as i32,
            ), // Top
            index(
                cell.row as i32,
                cell.col as i32 + 1,
                self.rows as i32,
                self.cols as i32,
            ), // Bottom
        ];
        for maybe_index in neighbor_index {
            if let Some(index) = maybe_index {
                if let Some(neighbor) = self.cells.get(index) {
                    if !neighbor.visited {
                        neighbors.push(index);
                    }
                }
            }
        }

        if neighbors.is_empty() {
            return None;
        }

        return Some(neighbors[fastrand::usize(..neighbors.len())]);
    }

    // TODO: use if let
    fn remove_wall(&mut self) {
        let x = self.cells[self.current].col as i32 - self.cells[self.next.unwrap()].col as i32;

        match x {
            1 => {
                self.cells[self.current].left = false;
                self.cells[self.next.unwrap()].right = false;
            }
            -1 => {
                self.cells[self.current].right = false;
                self.cells[self.next.unwrap()].left = false;
            }
            _ => {}
        }

        let y = self.cells[self.current].row as i32 - self.cells[self.next.unwrap()].row as i32;

        match y {
            1 => {
                self.cells[self.current].top = false;
                self.cells[self.next.unwrap()].bot = false;
            }
            -1 => {
                self.cells[self.current].bot = false;
                self.cells[self.next.unwrap()].top = false;
            }
            _ => {}
        }
    }

    fn update_current(&mut self) {
        self.next = self.get_random_neighbor();

        self.cells[self.current].visited = true; // mark current cell as visited
        if let Some(next_index) = self.next {
            self.stack.push_back(self.current);
            self.remove_wall();
            self.current = next_index;
        } else {
            if let Some(popped) = self.stack.pop_back() {
                self.current = popped;
            }
        }
    }
}

fn window_conf() -> Conf {
    return Conf {
        window_resizable: false,
        window_width: 800,
        window_height: 800,
        window_title: "Puzzler".to_string(),
        ..Default::default()
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let rows = (screen_width() / CELL_SIZE).floor() as usize;
    let cols = (screen_height() / CELL_SIZE).floor() as usize;

    let mut grid = Grid::new(rows, cols);

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            grid = Grid::new(rows, cols);
        }

        clear_background(BACKGROUND_COLOR);

        for cell in grid.cells.iter() {
            cell.draw();
        }
        grid.cells[grid.current].highlight(HIGHLIGHT_COLOR2);

        grid.update_current();
        next_frame().await;
    }
}
