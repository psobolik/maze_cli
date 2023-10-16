use clap::Parser;
use maze_lib::maze_generator;
use maze_lib::maze::cell::Cell;
use maze_lib::maze::cell_edge::CellEdge;
use maze_lib::maze::coordinates::Coordinates;
use maze_lib::maze::direction::Direction;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    columns: u32,
    #[arg(short, long, default_value_t = 10)]
    rows: u32,
}

static WALL_CHAR: &str = "▏";
static FLOOR_CHAR: &str = "_";
static PASSAGE_CHAR: &str = " ";

fn main() {
    let args = Args::parse();

    let maze = maze_generator::generate(args.columns, args.rows);
    println!("{:?}x{:?}", maze.columns(), maze.rows());

    println!(
        "{}",
        (" ".to_owned() + FLOOR_CHAR).repeat((maze.columns()) as usize)
    );

    for row in (0..maze.rows() as i32).rev() {
        for column in (0..maze.columns() as i32).rev() {
            if let Some(cell) = maze.cell(&Coordinates::new(column, row)) {
                print_cell(cell);
            }
        }
    }
}

fn print_cell(cell: Cell) {
    if let Some(west_edge) = cell.edge(&Direction::East) {
        match west_edge {
            CellEdge::Wall | CellEdge::Border => print!("{}", WALL_CHAR),
            _ => print!("{}", PASSAGE_CHAR),
        }
    }
    if let Some(south_edge) = cell.edge(&Direction::South) {
        match south_edge {
            CellEdge::Wall | CellEdge::Border => print!("{}", FLOOR_CHAR),
            _ => print!("{}", PASSAGE_CHAR),
        }
    }
    if let Some(east_edge) = cell.edge(&Direction::West) {
        if east_edge == CellEdge::Border {
            println!("{}", WALL_CHAR);
        }
    }
}


