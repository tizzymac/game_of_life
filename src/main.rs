// Compute the next generation
fn next_generation(grid: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    
    let rows = grid.len();
    let cols = grid[0].len();

    // create an empty grid to compute the next generation
    let mut next_gen: Vec<Vec<i8>> = vec![vec![0; rows]; cols];

    // iterate through each and every cell
    for i in 0..rows {
        for j in 0..cols {

            let cell_state = grid[i][j]; // the current cell (alive | dead)
            let mut live_neighbors = 0;

             // iterate through every neighbor incl. the current cell
             for x in -1i8..=1 
             {
                for y in -1i8..=1 
                {
                    // position of one of the neighbors
                    let new_x = (i as i8) + x;
                    let new_y = (j as i8) + y;

                    // make sure the position is within the bounds of the grid
                    if new_x > 0 && new_y > 0 && new_x < rows as i8 && new_y < cols as i8 
                    {
                        live_neighbors += grid[new_x as usize][new_y as usize];
                    }
                }
            }

            // subtract the state of the current cell to get the number of alive neighbors
            live_neighbors -= cell_state;

            // apply the rules of game of life to get the next generation:
            if cell_state == 1 && live_neighbors < 2 
            {
                // #1 Underpopulation:
                // A live cell with less than two live neighbors dies
                next_gen[i][j] = 0;
            } 
            else if cell_state == 1 && live_neighbors > 3 
            {
                // #2 Overpopulation
                // A live cell with more than three live neighbors dies
                next_gen[i][j] = 0;
            } 
            else if cell_state == 0 && live_neighbors == 3 
            {
                // #3 Reproduction
                // A dead cell with exactly three neighbors becomes alive
                next_gen[i][j] = 1;
            } 
            else 
            {
                next_gen[i][j] = cell_state;
            }
        }
    }

    // return the next generation
    next_gen
}

fn main() 
{
    println!("Welcome to the Game of Life!\n");

    const GENS: u8 = 8; // Number of generations
    let (rows, cols) = (6, 6); // Grid is 6x6
    let mut grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    // set the initial state of the grid to "Toad"
    grid[2][2] = 1;
    grid[2][3] = 1;
    grid[2][4] = 1;
    grid[3][1] = 1;
    grid[3][2] = 1;
    grid[3][3] = 1;

    // print the initial state of the grid;
    println!("Initial grid:");
    grid.iter().for_each(|i| {
        println!("{:?}", i);
    });
    println!("");

    // compute and print the next generation
    for i in 0..GENS 
    {
        grid = next_generation(&grid);

        println!("Generation {}:", i+1);
        grid.iter().for_each(|i| {
            println!("{:?}", i);
        });
        println!("");
    }
}
