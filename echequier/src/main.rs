use rand::{thread_rng, Rng, seq::SliceRandom};

// Parameters
const POP_SIZE: usize = 100;
const POP_DIM: usize = 4; // Didn't work yet with odd numbers
const PROBABILITE_CROISEMENT: f32 = 0.7;
const PROBABILITE_MUTATION: f32 = 0.01;
const MAX_GEN: u32 = 2000;

fn main() {
    let mut best_solution: [usize; POP_DIM] = [0; POP_DIM]; // Generate default best solution by array of false
    let mut f_best: u32 =  u32::MAX; // Assign big number at starting

    let mut population = generate_population();
    evaluation_population(population, &mut best_solution, &mut f_best);

    let mut gen: u32 = 0;
    while gen <= MAX_GEN && f_best != 0 {
        let mut population_new: Vec::<[usize; POP_DIM]> = Vec::new();
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

fn generate_population() -> [[usize; POP_DIM]; POP_SIZE] {
    let mut population: [[usize; POP_DIM]; POP_SIZE] = [[0; POP_DIM]; POP_SIZE];
    let mut rng = thread_rng();

    for i in 0..population.len() {
        let mut individu: [usize; POP_DIM] = core::array::from_fn(|i| (i + 1));
        individu.shuffle(&mut rng);
        population[i] = individu;
    }

    return population;
}

fn selection(population: [[usize; POP_DIM]; POP_SIZE]) -> ([usize; POP_DIM], [usize; POP_DIM]) {
    let mut rng = thread_rng();
    
    let i1 = rng.gen_range(0..POP_SIZE);
    let range_without_i1: Vec<usize> = (0..POP_SIZE).into_iter().filter(|&item| item != i1).collect();
    let i2 = range_without_i1[rng.gen_range(0..range_without_i1.len())];

    return (population[i1], population[i2]);
}

fn evaluation(individu: &[usize; POP_DIM]) -> u32 {
    let mut conflit = 0;
    for i in 0..(POP_DIM - 2) {
        for j in 1..(POP_DIM - 1) {
            
            if i != j && i.abs_diff(j) == individu[i].abs_diff(individu[j]) {
                conflit += 1;
            }
        }
    }

    return conflit;
}

fn evaluation_population(population: [[usize; POP_DIM]; POP_SIZE], best_solution: &mut [usize; POP_DIM], f_best: &mut u32) {
    for individu in population {
        let f_individu = evaluation(&individu);

        if f_individu < *f_best {
            println!("Find a better individu from {} to {}", f_best, f_individu);
            *best_solution = individu.clone();
            *f_best = f_individu;
        }
    }
}

fn croisement(i1: &mut [usize; POP_DIM], i2: &mut [usize; POP_DIM]) {
    let mut rng = thread_rng();
    let probability = rng.gen_range(0.0..1.0);

    if probability < PROBABILITE_CROISEMENT {
        for index in 0..POP_DIM / 2 {
            // 1
            let pos1 = i1.iter().position(|&item| item == i2[index]).expect("Could not find item from i1 to i2");
            i1[pos1] = i1[index];

            // 2
            let pos2 = i2.iter().position(|&item| item == i1[index]).expect("Could not find item from i2 to i1");
            i2[pos2] = i2[index];

            // 3
            let temp_i1 = i1[index];
            i1[index] = i2[index];
            i2[index] = temp_i1;
        }
    }
}

fn mutation(i: &mut [usize; POP_DIM]) {
    let mut rng = thread_rng();
    let probability = rng.gen_range(0.0..1.0);

    // Switch position of two elements
    if probability < PROBABILITE_MUTATION {
        let pos_1 = rng.gen_range(0..POP_DIM);
        let range_without_i1: Vec<usize> = (0..POP_DIM).into_iter().filter(|&item| item != pos_1).collect();
        let pos_2 = range_without_i1[rng.gen_range(0..range_without_i1.len())];
        let save_pos_1 = i[pos_1];
        i[pos_1] = i[pos_2];
        i[pos_2] = save_pos_1;
    }
}