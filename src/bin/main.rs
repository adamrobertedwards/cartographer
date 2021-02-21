use cartographer::{Map, Dijkstra, BreadthFirstSearch};


fn main() {
    let grid: Vec<Vec<Option<(&str, u32)>>> = vec!(
        vec!(Some(("(1,1)", 1)), Some(("(1,2)", 1)), Some(("(1,3)", 2)), None), 
        vec!(Some(("(2,1)", 5)), None, Some(("(2,3)", 2)), Some(("(2,4)", 1))), 
        vec!(Some(("(3,1)", 5)), Some(("(3,2)", 1)), None, Some(("(3,4)", 2))),
        vec!(None, Some(("(4,2)", 1)), Some(("(4,3)", 1)), Some(("(4,4)", 1))),
    );
    let mut map = Map::new();

    setup_grid(&mut map, &grid);
    draw_grid(&grid);
    
    let mut bfs = BreadthFirstSearch::new();
    let mut dijkstra = Dijkstra::new();

    let moves_bfs = bfs.calculate_moves(&map, "(1,1)", 2);
    println!("moves bfs: {:?}", moves_bfs);

    let solve_bfs = bfs.calculate_path(&map, "(1,1)", "(4,3)");
    println!("solve bfs {:?}", solve_bfs);

    let moves_dijkstra = dijkstra.calculate_moves(&map, "(1,1)", 3);
    println!("moves Dijkstra: {:?}", moves_dijkstra);

    let solve_dijkstra = dijkstra.calculate_path(&map, "(1,1)", "(4,3)");
    println!("solve Dijkstra: {:?}", solve_dijkstra);
    
    // println!("{:?}", solve_bfs);
}

fn setup_grid(map: &mut Map, grid: &Vec<Vec<Option<(&str, u32)>>>) {
    let directions: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut row_index: i32 = 0;

    for row in grid {
        let mut col_index: i32 = 0;

        for col in row {
            if let Some(node) = col {
                let pos = (&col_index, &row_index);
                map.add_node(node.0);

                for dir in &directions {
                    let neighbour_pos = (pos.1 + dir.1, pos.0 + dir.0);
                    let row_count = grid.len() as i32;
                    let col_count = row.len() as i32;

                    if neighbour_pos.1 >= 0 && neighbour_pos.1 < row_count && neighbour_pos.0 >= 0 && neighbour_pos.0 < col_count {
                        if let Some(neighbour) = grid[neighbour_pos.0 as usize][neighbour_pos.1 as usize] {
                            if let Err(_e) = map.connect_nodes(
                                &node.0,
                                neighbour.0,
                                neighbour.1
                            ) {
                                println!("Failed to connect node: {:?} to {:?}", node, neighbour);
                            };
                        }
                    }
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }
}

fn draw_grid(grid: &Vec<Vec<Option<(&str, u32)>>>) {
    for row in grid {
        let mut row_items = "".to_string();

        for col in row {
            let col_item: &str = match col {
                Some(_) => " · ",
                None => " # "
            };

            row_items.push_str(col_item);
        }
        println!("{:?}", row_items);
    } 
}
