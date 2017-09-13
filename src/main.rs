extern crate rand;
use rand::Rng;
use std::fmt;

#[derive(PartialEq)]
enum Spin {
    Up,
    Down,
}

impl fmt::Debug for Spin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Spin::Up   => {write!(f, "Spin::Up")}
            &Spin::Down => {write!(f, "Spin::Down")}
        }
    }
}

impl fmt::Display for Spin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Spin::Up   => {write!(f, "+")}
            &Spin::Down => {write!(f, " ")}
        }
    }
}

//XXX to avoid double count, neighbor size is 2.
//      |o|
//       ^
// |o|<-| |   |x|
//
//      |x|
#[derive(Debug)]
struct Node {
    state    : Spin,
    neighbor : [usize;2],
}

impl Node {
    fn new(sp: Spin, n: usize, e: usize) -> Node {
        Node{state: sp, neighbor: [n, e]}
    }

    fn randomize<R: Rng>(&mut self, rng: &mut R) {
        if rng.gen::<bool>() {
            self.state = Spin::Up;
        } else {
            self.state = Spin::Down;
        }
    }
}

#[derive(Debug)]
struct Field {
    width  : usize,
    height : usize,
    beta   : f64,
    nodes  : Vec<Node>,
}

impl Field {
    fn create(width: usize, height: usize, beta: f64) -> Field {
        let mut ns = Vec::new();
        for h in 0..height {
            for w in 0..width {
                let h_prev = if h == 0 {height-1} else {h-1};
                let w_prev = if w == 0 {width-1}  else {w-1};
                ns.push(Node::new(Spin::Up,
                                  width * h_prev + w,
                                  width * h + w_prev));
            }
        }
        Field{width:width, height:height, beta:beta, nodes: ns}
    }

    fn randomize<R: Rng>(&mut self, rng: &mut R) {
        for i in 0..self.nodes.len() {
            self.nodes[i].randomize(rng);
        }
    }

    fn print_console(&self) {
        let lnwdth = self.width * 2+1;
        let line = String::from_utf8(vec![45; lnwdth]).unwrap();
        println!("{}", line);

        for h in 0..self.height {
            for w in 0..self.width {
                let idx = h * self.width + w;
                print!("|{}", self.nodes[idx].state);
            }
            println!("|");
            println!("{}", line);
        }
        print!("\n");
    }
}

struct Hamiltonian {
    j : f64,
    h : f64,
}

impl Hamiltonian {
    fn calc_energy_node(&self, idx: usize, field: &Field) -> f64 {
        let node = &field.nodes[idx];
        let mut energy = match node.state {
            Spin::Up   => {-self.h}
            Spin::Down => { self.h}
        };
        for i in 0..node.neighbor.len() {
            let nidx = node.neighbor[i];
            if node.state == field.nodes[nidx].state {
                energy -= self.j;
            }
        }
        energy
    }
    fn calc_energy(&self, field: &Field) -> f64 {
        let mut energy = 0.0;
        for i in 0..field.nodes.len() {
            energy += self.calc_energy_node(i, &field);
        }
        energy
    }

    fn calc_energy_diff_node(&self, idx: usize, field: &Field) -> f64 {
        let mut energy_prev = match field.nodes[idx].state {
            Spin::Up   => {-self.h}
            Spin::Down => { self.h}
        };
        let mut energy_next = -energy_prev;

        for i in 0..field.nodes[idx].neighbor.len() {
            if field.nodes[idx].state == field.nodes[field.nodes[idx].neighbor[i]].state {
                energy_prev -= self.j;
            } else {
                energy_next -= self.j;
            }
        }
        energy_prev - energy_next
    }
}

fn step<R: Rng>(field: &mut Field, hamiltonian: &Hamiltonian, rng: &mut R) {
    for i in 0..field.nodes.len() {
        let denergy = hamiltonian.calc_energy_diff_node(i, field);
        if denergy <= 0.0 || rand::random::<f64>() > (-1.0 * field.beta * denergy).exp() {
            field.nodes[i].state = match field.nodes[i].state {
                Spin::Up   => {Spin::Down}
                Spin::Down => {Spin::Up}
            }
        }
    }
}

fn main() {
    let hamiltonian = Hamiltonian{j: 1.0, h: 0.0};
    let mut field = Field::create(20, 20, 1.0);
    println!("{}", hamiltonian.calc_energy(&field));
    field.print_console();

    let mut rng = rand::thread_rng();
    field.randomize(&mut rng);
    println!("{}", hamiltonian.calc_energy(&field));
    field.print_console();

    for _ in 0..100 {
        step(&mut field, &hamiltonian, &mut rng);
        println!("{}", hamiltonian.calc_energy(&field));
        field.print_console();
    }
}
