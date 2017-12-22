#[derive(Copy, Clone)]
enum State {
    CLEAN,
    WEAKENED,
    INFECTED,
    FLAGGED,
}

fn main() {
    let x = r".###.#.#####.##.#...#....
..####.##.##.#..#.....#..
.#####.........#####..###
#.#..##..#.###.###.#.####
.##.##..#.###.###...#...#
#.####..#.#.##.##...##.##
..#......#...#...#.#....#
###.#.#.##.#.##.######..#
###..##....#...##....#...
###......#..#..###.#...#.
#.##..####.##..####...##.
###.#.#....######.#.###..
.#.##.##...##.#.#..#...##
######....##..##.######..
##..##.#.####.##.###.#.##
#.###.#.##....#.##..####.
#.#......##..####.###.#..
#..###.###...#..#.#.##...
#######..#.....#######..#
##..##..#..#.####..###.#.
..#......##...#..##.###.#
....##..#.#.##....#..#.#.
..#...#.##....###...###.#
#.#.#.#..##..##..#..#.##.
#.####.#......#####.####.
";

    let m: Vec<Vec<State>> = x.lines()
        .rev()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == '#' {
                        State::INFECTED
                    } else {
                        State::CLEAN
                    }
                })
                .collect()
        })
        .collect();
    let szm = m.len();

    const sz: usize = 1000;
    let mut og = [[State::CLEAN; sz]; sz];

    m.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, &b)| {
            og[sz / 2 - szm / 2 + y][sz / 2 - szm / 2 + x] = b;
        })
    });

    let mov = |x: &mut isize, y: &mut isize, dir: usize| {
        let off = [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)];
        *y += off[dir].0;
        *x += off[dir].1;
    };

    {
        let mut g = og.clone();
        let (mut x, mut y) = ((sz / 2) as isize, (sz / 2) as isize);
        let mut dir = 0usize;
        let mut infected = 0;

        for _ in 0..10000 {
            let t = &mut g[y as usize][x as usize];
            match *t {
                State::CLEAN => {
                    *t = State::INFECTED;
                    dir = (dir + 3) % 4;
                    infected += 1;
                }
                State::INFECTED => {
                    *t = State::CLEAN;
                    dir = (dir + 1) % 4;
                }

                _ => {}
            }
            mov(&mut x, &mut y, dir);
        }

        println!("{}", infected);
    }

    {
        let mut g = og;
        let mut infected = 0;
        let mut dir = 0usize;
        let (mut x, mut y) = ((sz / 2) as isize, (sz / 2) as isize);

        for _ in 0..10000000 {
            let t = &mut g[y as usize][x as usize];
            match *t {
                State::CLEAN => {
                    *t = State::WEAKENED;
                    dir = (dir + 3) % 4;
                }
                State::WEAKENED => {
                    *t = State::INFECTED;
                    infected += 1;
                }
                State::INFECTED => {
                    *t = State::FLAGGED;
                    dir = (dir + 1) % 4;
                }
                State::FLAGGED => {
                    *t = State::CLEAN;
                    dir = (dir + 2) % 4;
                }
            }
            mov(&mut x, &mut y, dir);
        }

        println!("{}", infected);
    }
}

