fn power_level_fn(x: i32, y: i32, grid_serial_number: i32) -> i32 {

    let mut power_level: i32;
    let rack_id = x + 10;
    power_level = rack_id * y;
    power_level = power_level + grid_serial_number;
    power_level = power_level * rack_id;
    if power_level >= 100 {
        power_level = (power_level / 100) % 10 - 5;
    }
    else {
        power_level = 0;
    }

    power_level
}

fn main() -> (){

    let grid_serial_number = 7857;
    let mut max_power_level = 0;
    let mut cell: String = String::new();
    let mut largest_grid: String = String::new();
    let grid_size = 300;

    let mut grid = vec![vec![0; grid_size]; grid_size];

    for x in 1..grid_size + 1 {
        for y in 1..grid_size + 1 {
            let power_level = power_level_fn(x as i32, y as i32, grid_serial_number);
            grid[(x-1) as usize][(y-1) as usize] = power_level;
        }
    }

    for x in 0..grid_size - 3 {
        for y in 0..grid_size - 3 {
            let power_level = grid[x][y]   + grid[x][y+1]   + grid[x][y+2] +
                              grid[x+1][y] + grid[x+1][y+1] + grid[x+1][y+2] +
                              grid[x+2][y] + grid[x+2][y+1] + grid[x+2][y+2];

            if power_level > max_power_level {
                max_power_level = power_level;
                cell = format!("x: {}, y: {}", x+1, y+1);
            }
        }
    }

    println!("Part 1: {}", cell);

    max_power_level = 0;

    for dimension in 2..grid_size + 1 {
        for x in 0..grid_size - dimension {
            for y in 0..grid_size - dimension {
                let mut power_level = 0;
                for i in 0..dimension {
                    for j in 0..dimension {
                        power_level += grid[x+i][y+j];
                    }
                }

                if power_level > max_power_level {
                    max_power_level = power_level;
                    largest_grid = format!("x: {}, y: {}, dimension: {}", x+1, y+1, dimension);
                    // super duper hacky solution..
                    // this loop won't terminate for a very long time, so to work out the answer
                    // I print out the current largest grid data (x,y,dimension) and observe stdout
                    // until the value stays consistent => we've reached the maximum...
                    // This should be implemented properly to most likely use a Summed Table Area
                    //println!("Part 2: new max.. largest {}", largest_grid);
                }
            }
        }
    }

    println!("Part 2: {}", largest_grid);
}