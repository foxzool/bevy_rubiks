use cubesim::{parse_scramble, solve, Cube, FaceletCube};

fn main() {
    let moves = parse_scramble(String::from("Lw"));
    println!("moves {:?}", moves);
    let cube = &FaceletCube::new(3).apply_moves(&moves);
    println!("{:?}", cube.state());
    let solution = solve(cube);

    if let Some(s) = solution {
        println!("{:?}", cube.apply_moves(&s).is_solved());
        for i in s.iter() {
            println!("{i:?}");
        }
    }
}
