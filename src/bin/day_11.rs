//! `cargo run --bin day_11`

fn power_level_fn(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = (rack_id * y + grid_serial_number) * rack_id;
    (power_level / 100) % 10 - 5
}

fn main(){

    let grid_serial_number = 7857;
    let mut max_power_level = i32::MIN;
    let mut cell: String = String::new();
    let mut largest_grid: String = String::new();
    let grid_size = 300;

    let mut grid = vec![vec![0; grid_size + 1]; grid_size + 1];
    let mut sat = vec![vec![0; grid_size + 1]; grid_size + 1];

    for (x, row) in grid.iter_mut().enumerate().skip(1) {
        for (y, cell) in row.iter_mut().enumerate().skip(1) {
            *cell = power_level_fn(x as i32, y as i32, grid_serial_number);
        }
    }

    for x in 1..grid_size + 1 {
        let mut row_sum: i32 = 0;
        for y in 1..grid_size + 1 {
            row_sum += grid[x][y];
            sat[x][y] = sat[x-1][y] + row_sum;
        }
    }

    for x in 1..grid_size - 1 {
        for y in 1..grid_size - 1 {
            let power_level = grid[x][y]   + grid[x][y+1]   + grid[x][y+2] +
                              grid[x+1][y] + grid[x+1][y+1] + grid[x+1][y+2] +
                              grid[x+2][y] + grid[x+2][y+1] + grid[x+2][y+2];

            if power_level > max_power_level {
                max_power_level = power_level;
                cell = format!("x: {}, y: {}", x, y);
            }
        }
    }

    println!("Part 1: {}", cell);

    max_power_level = i32::MIN;

    for d in 2..grid_size {
        for x in 1..grid_size - d + 1 {
            for y in 1..grid_size - d + 1 {
                // https://en.wikipedia.org/wiki/Summed-area_table#The_algorithm
                let total = sat[x-1][y-1] + sat[x+d-1][y+d-1] - sat[x-1][y+d-1] - sat[x+d-1][y-1];
                if total > max_power_level {
                    max_power_level = total;
                    largest_grid = format!("x: {}, y: {}, dimension: {}", x, y, d);
                }
            }
        }
    }

    println!("Part 2: {}", largest_grid);
}