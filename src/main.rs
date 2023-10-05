use clap::Parser;
use maze_lib::maze_generator;
use maze_lib::maze_generator::cell::Cell;
use maze_lib::maze_generator::cell_edge::CellEdge;
use maze_lib::maze_generator::coordinates::Coordinates;
use maze_lib::maze_generator::direction::Direction;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    columns: u32,
    #[arg(short, long, default_value_t = 10)]
    rows: u32,
}

static WALL_CHAR: &'static str = "▏";
static FLOOR_CHAR: &'static str = "_";
static PASSAGE_CHAR: &'static str = " ";

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
            if let Some(cell) = maze.get_cell(&Coordinates::new(column, row)) {
                print_cell(cell);
            }
        }
    }
}

fn print_cell(cell: Cell) {
    if let Some(west_edge) = cell.get_edge(&Direction::East) {
        match west_edge {
            CellEdge::Wall | CellEdge::Border => print!("{}", WALL_CHAR),
            _ => print!("{}", PASSAGE_CHAR),
        }
    }
    if let Some(south_edge) = cell.get_edge(&Direction::South) {
        match south_edge {
            CellEdge::Wall | CellEdge::Border => print!("{}", FLOOR_CHAR),
            _ => print!("{}", PASSAGE_CHAR),
        }
    }
    if let Some(east_edge) = cell.get_edge(&Direction::West) {
        if east_edge == CellEdge::Border {
            println!("{}", WALL_CHAR);
        }
    }
}


