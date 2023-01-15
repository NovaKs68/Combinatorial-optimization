use rand::{thread_rng, Rng};

// Parameters
const POP_SIZE: usize = 100;
const POP_DIM: usize = 10; // Didn't work yet with odd numbers
const PROBABILITE_CROISEMENT: f32 = 0.7;
const PROBABILITE_MUTATION: f32 = 0.01;
const MAX_GEN: u32 = 2000;

const SUM_RES: i32 = 36;
const PROD_RES: i32 = 360;

fn main() {
    let mut best_solution: [bool; POP_DIM] = [false; POP_DIM]; // Generate default best solution by array of false
    let mut f_best: u32 =  u32::MAX; // Assign big number at starting

    let mut population = generate_population();
    evaluation_population(population, &mut best_solution, &mut f_best);

    let mut gen: u32 = 0;
    while gen <= MAX_GEN && f_best != 0 {
        let mut population_new: Vec::<[bool; POP_DIM]> = Vec::new();
        for _ in 0..POP_SIZE / 2 {
                let (mut i1, mut i2) = selection(population);
                croisement(&mut i1, &mut i2);
                mutation(&mut i1);
                mutation(&mut i2);

                population_new.push(i1);
                population_new.push(i2);
        }

        population = population_new.try_into().expect("Error during assigning new population to old one");
        evaluation_population(population, &mut best_solution, &mut f_best);
        gen += 1;
    }

    println!("Result : {}, {:?}", f_best, best_solution);
}

fn generate_population() -> [[bool; POP_DIM]; POP_SIZE] {
    let mut population: [[bool; POP_DIM]; POP_SIZE] = [[false; POP_DIM]; POP_SIZE];
    let mut rng = thread_rng();

    for i in 0..population.len() {
        let individu: [bool; POP_DIM] = [();POP_DIM].map(|_| rng.gen_bool(1.0 / 2.0));
        population[i] = individu;
    }

    return population;
}

fn selection(population: [[bool; POP_DIM]; POP_SIZE]) -> ([bool; POP_DIM], [bool; POP_DIM]) {
    let mut rng = thread_rng();
    
    let i1 = rng.gen_range(0..POP_SIZE);
    let range_without_i1: Vec<usize> = (0..POP_SIZE).into_iter().filter(|&item| item != i1).collect();
    let i2 = range_without_i1[rng.gen_range(0..range_without_i1.len())];

    return (population[i1], population[i2]);
}

fn evaluation(individu: &[bool; POP_DIM]) -> u32 {
    let mut sum: i32 = 0;
    let mut prod: i32 = 1;

    for i in 0..POP_DIM {
        if individu[i] == false {
            sum += (i as i32) + 1;
        } else {
            prod *= (i as i32) + 1;
        }
    }

    return ((sum - SUM_RES).abs() + (prod - PROD_RES).abs()) as u32;
}

fn evaluation_population(population: [[bool; POP_DIM]; POP_SIZE], best_solution: &mut [bool; POP_DIM], f_best: &mut u32) {
    for individu in population {
        let f_individu = evaluation(&individu);

        if f_individu < *f_best {
            println!("Find a better individu from {} to {}", f_best, f_individu);
            *best_solution = individu.clone();
            *f_best = f_individu;
        }
    }
}

fn croisement(i1: &mut [bool; POP_DIM], i2: &mut [bool; POP_DIM]) {
    let mut rng = thread_rng();
    let probability = rng.gen_range(0.0..1.0);

    if probability < PROBABILITE_CROISEMENT {
        const INDIVIDU_DIM: usize = POP_DIM / 2;
        const INDIVIDU_DIM_REST: usize = POP_DIM - INDIVIDU_DIM;
        
        let slice1_i1: [bool; INDIVIDU_DIM] = i1[0..INDIVIDU_DIM].try_into().expect("Error during slicing elements");
        let slice2_i1: [bool; INDIVIDU_DIM_REST] = i2[INDIVIDU_DIM_REST..POP_DIM].try_into().expect("Error during slicing elements");
        let slice1_i2: [bool; INDIVIDU_DIM] = i1[0..INDIVIDU_DIM].try_into().expect("Error during slicing elements");
        let slice2_i2: [bool; INDIVIDU_DIM_REST] = i2[INDIVIDU_DIM_REST..POP_DIM].try_into().expect("Error during slicing elements");

        *i1 = [slice1_i1, slice2_i2].concat().try_into().expect("Error during concatenate elements");
        *i2 = [slice2_i1, slice1_i2].concat().try_into().expect("Error during concatenate elements");
    }
}

fn mutation(i: &mut [bool; POP_DIM]) {
    let mut rng = thread_rng();
    let probability = rng.gen_range(0.0..1.0);

    if probability < PROBABILITE_MUTATION {
        let pos = rng.gen_range(0..POP_DIM);
        i[pos] = ((i[pos] as u8 + 1) % 2) != 0;
    }
}