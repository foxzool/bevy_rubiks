use cubesim::{Cube, FaceletCube, GeoCube, parse_scramble, solve};

fn main() {

    let cube = &GeoCube::new(3);

    println!("{}", cube);

    let cube = &FaceletCube::new(3).apply_moves(&parse_scramble(String::from("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2")));

    let solution = solve(cube);

    if let Some(s) = solution {

        println!("{:?}", cube.apply_moves(&s).is_solved());
        for i in s.iter() {
            println!("{i:?}");
        }

    }
}
