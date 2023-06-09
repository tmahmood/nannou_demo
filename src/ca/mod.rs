use std::ops::Range;
use nannou::prelude::*;

// A Type to manage the CA
pub struct Ca {
    generation: i32,    // How many generations?
    rule_set: Vec<i32>, // An array to store the ruleset, for example {0,1,1,0,1,1,0,1}
    w: usize,
    matrix: Vec<Vec<i32>>,
    columns: usize,
    rows: usize,
    col_range: Range<usize>,
}

impl Ca {
    pub fn new(r: Vec<i32>, rect: Rect) -> Self {
        let rule_set = r;
        let generation = 0;
        let w = 4;
        let columns = rect.w() as usize / w;
        let rows = rect.h() as usize / w;
        let col_range = 1..columns - 1;
        let matrix = vec![vec![0; rows]; columns];

        let mut ca = Ca {
            rule_set,
            generation,
            w,
            columns,
            rows,
            col_range,
            matrix,
        };
        ca.restart();
        ca
    }

    // Make a random rule set
    fn _randomize(&mut self) {
        self.rule_set = (0..self.rule_set.len())
            .map(|_| random_range(0i32, 2))
            .collect();
    }

    // Reset generation to 0
    pub fn restart(&mut self) {
        self.matrix = vec![vec![0; self.rows]; self.columns];
        self.matrix[self.columns / 2][0] = 1; // We arbitrarily start with just the middle cell having a state of "1"
        self.generation = 0;
    }

    // The process of creating the new generation
    pub fn generate(&mut self) {
        // For every spot, determine new state by examing current state, and neighbor states
        // Ignore edges that only have one neighor
        for i in self.col_range.clone() {
            let left = self.matrix[(i + self.columns - 1) % self.columns]
                [(self.generation % self.rows as i32) as usize]; // Left neighbor state
            let me = self.matrix[i][(self.generation % self.rows as i32) as usize]; // Current state
            let right =
                self.matrix[(i + 1) % self.columns][(self.generation % self.rows as i32) as usize]; // Right beighbor state
            self.matrix[i][((self.generation + 1) % self.rows as i32) as usize] =
                self.rules(left, me, right); // Compute next generation state based on ruleset
        }
        self.generation += 1;
    }

    // This is the easy part, just draw the cells fill white if 1, black if 0
    pub fn display(&self, draw: &Draw, rect: &Rect) {
        let offset = self.generation % self.rows as i32;
        for col in 0..self.columns {
            for row in 0..self.rows {
                let mut y = row as i32 - offset;
                if y <= rect.top() as i32 {
                    y = self.rows as i32 + y;
                }
                // Only draw if cell state is 1
                let mut fill = 1.0;
                if self.matrix[col][row] == 1 {
                    fill = 0.0;
                } else {
                    continue
                }
                let x =
                    ((self.w as i32 / 2) + col as i32 * self.w as i32) as f32 - rect.right() as f32;
                let y = rect.top() - (self.w / 2) as f32 - ((y - 1) * self.w as i32) as f32;
                draw.rect()
                    .x_y(x, y)
                    .w_h(self.w as f32, self.w as f32)
                    .gray(fill);
            }
        }
    }

    // Implementing the Wolfram rules
    // Could be improved and made more concise, but here we can explicitly see what is going on for each case
    pub fn rules(&self, a: i32, b: i32, c: i32) -> i32 {
        if a == 1 && b == 1 && c == 1 {
            return self.rule_set[0];
        }
        if a == 1 && b == 1 && c == 0 {
            return self.rule_set[1];
        }
        if a == 1 && b == 0 && c == 1 {
            return self.rule_set[2];
        }
        if a == 1 && b == 0 && c == 0 {
            return self.rule_set[3];
        }
        if a == 0 && b == 1 && c == 1 {
            return self.rule_set[4];
        }
        if a == 0 && b == 1 && c == 0 {
            return self.rule_set[5];
        }
        if a == 0 && b == 0 && c == 1 {
            return self.rule_set[6];
        }
        if a == 0 && b == 0 && c == 0 {
            return self.rule_set[7];
        }
        0
    }

    // The CA is done if it reaches the bottom of the screen
    fn _finished(&self, rect: &Rect) -> bool {
        if self.generation > rect.h() as i32 / self.w as i32 {
            true
        } else {
            false
        }
    }
}