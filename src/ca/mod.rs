/*
class CA {
int[] cells;
int[] ruleset;
int w = 10;
The CA should keep track of how many generations.
int generation = 0;
*/

pub struct CA {
    cells: Vec<i32>,
    rule_set: Vec<i32>,
    w: i32,
    generation: i32,
}

impl CA {
    pub fn cells(&self) -> &Vec<i32> {
        &self.cells
    }
    pub fn rule_set(&self) -> &Vec<i32> {
        &self.rule_set
    }
    pub fn w(&self) -> i32 {
        self.w
    }
    pub fn generation(&self) -> i32 {
        self.generation
    }


    pub fn set_cells(&mut self, cells: Vec<i32>) {
        self.cells = cells;
    }
    pub fn set_rule_set(&mut self, rule_set: Vec<i32>) {
        self.rule_set = rule_set;
    }
    pub fn set_w(&mut self, w: i32) {
        self.w = w;
    }
    pub fn set_generation(&mut self, generation: i32) {
        self.generation = generation;
    }


    pub fn new(cells: Vec<i32>, rule_set: Vec<i32>, w: i32, generation: i32) -> Self {
        Self { cells, rule_set, w, generation }
    }
}

/*


CA() {
cells = new int[width/w];
ruleset = {0,1,0,1,1,0,1,0};
cells[cells.length/2] = 1;
}


Function to compute the next generation

void generate() {
int[] nextgen = new int[cells.length];
for (int i = 1; i < cells.length-1; i++) {
int left   = cells[i-1];
int me     = cells[i];
int right  = cells[i+1];
nextgen[i] = rules(left, me, right);
}
cells = nextgen;

Increment the generation counter.

generation++;
}

int rules(int a, int b, int c) {
    String s = "" + a + b + c;
    int index = Integer.parseInt(s,2);
    return ruleset[index];
}

for (int i = 0; i < cells.length; i++) {
if (cells[i] == 1) fill(0);
else               fill(255);

Set the y-location according to the generation.

rect(i*w, generation*w, w, w);
}
}
 */

