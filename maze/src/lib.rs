#![allow(dead_code)]
use colored::Colorize;
use rand::prelude::*;
use std::{collections::HashSet, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Entrance,
    Exit,
    Road,
    Wall,
    Trap,
    Treasure,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Entrance => {
                write!(f, "{}", "😐".green().on_black())
            }
            Tile::Exit => {
                write!(f, "{}", "⭐".yellow().on_black())
            }
            Tile::Road => {
                write!(f, "{}", "  ".on_black())
            }
            Tile::Wall => {
                write!(f, "{}", "██".white())
            }
            Tile::Trap => {
                write!(f, "{}", "🪤".on_black())
            }
            Tile::Treasure => {
                write!(f, "{}", "🎖".on_black())
            }
        }
    }
}

pub struct Maze {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    pub fn new(width: usize, height: usize, seed: Option<u64>) -> Self {
        let start = (0, 0);
        let mut rng = get_rng(seed);

        let base = gen_base(start, width, height, &mut rng);
        let path = get_path(width, height, &base);

        let mut tiles = get_walls(width, height, &base);

        let finish = *path.first().expect("Must exist");
        tiles[finish.0 * 2][finish.1 * 2] = Tile::Exit;
        tiles[start.0 * 2][start.1 * 2] = Tile::Entrance;

        // it makes sense to construct this and not just use Vec because we'll use it in traps_from_path anyway
        let path = path.into_iter().collect::<HashSet<_>>();

        let mut traps_on_path = 0;
        // set traps - 5 attempts, max 2 on path
        for _ in 0..5 {
            let pos = get_random_pos(&mut rng, width, height);
            if tiles[pos.0 * 2][pos.1 * 2] == Tile::Road {
                if path.contains(&pos) {
                    if traps_on_path < 2 {
                        tiles[pos.0 * 2][pos.1 * 2] = Tile::Trap;
                        traps_on_path += 1;
                    }
                } else {
                    tiles[pos.0 * 2][pos.1 * 2] = Tile::Trap;
                }
            }
        }

        // set treasure - 0 or 1, 3 attempts, not on path
        for _ in 0..3 {
            let pos = get_random_pos(&mut rng, width, height);
            if tiles[pos.0 * 2][pos.1 * 2] == Tile::Road
                && !path.contains(&pos)
                // is reachable
                && traps_from_path(&base, &tiles, &path, pos) <= 2 - traps_on_path
            {
                tiles[pos.0 * 2][pos.1 * 2] = Tile::Treasure;
                break;
            }
        }

        Self {
            width,
            height,
            tiles,
        }
    }
}

// pos is guaranteed not to be on path initially
#[allow(clippy::ptr_arg)]
fn traps_from_path(
    base: &Vec<Vec<usize>>,
    tiles: &Vec<Vec<Tile>>,
    path: &HashSet<(usize, usize)>,
    mut pos: (usize, usize),
) -> usize {
    let mut res = 0;

    while !path.contains(&pos) {
        if tiles[pos.0 * 2][pos.1 * 2] == Tile::Trap {
            res += 1;
        }
        pos = step_to_lower(base, pos).expect("Pos should be guaranteed to not be on path");
    }

    res
}

// steps to lower if it's possible, not possible if on start
#[allow(clippy::ptr_arg)]
fn step_to_lower(base: &Vec<Vec<usize>>, pos: (usize, usize)) -> Option<(usize, usize)> {
    let height = base.len();
    let width = base.first().unwrap().len();

    for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let (nrow, _) = pos.0.overflowing_add(dr);
        let (ncol, _) = pos.1.overflowing_add(dc);

        if nrow < height && ncol < width && base[pos.0][pos.1] == base[nrow][ncol] + 1 {
            return Some((nrow, ncol));
        }
    }

    None
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width * 2 + 1;

        for _ in 0..width {
            write!(f, "{}", Tile::Wall)?;
        }
        writeln!(f)?;

        for row in self.tiles.iter() {
            write!(f, "{}", Tile::Wall)?;
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f, "{}", Tile::Wall)?;
        }

        for _ in 0..width {
            write!(f, "{}", Tile::Wall)?;
        }
        writeln!(f)
    }
}

fn get_rng(seed: Option<u64>) -> StdRng {
    match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    }
}

fn get_random_pos(rng: &mut StdRng, width: usize, height: usize) -> (usize, usize) {
    (rng.gen_range(0..height), rng.gen_range(0..width))
}

fn gen_base(
    start: (usize, usize),
    width: usize,
    height: usize,
    rng: &mut StdRng,
) -> Vec<Vec<usize>> {
    let mut field = vec![vec![None::<usize>; width]; height];

    // row, col, dist
    let mut stack: Vec<(usize, usize, usize)> = vec![(start.0, start.1, 0)];

    while let Some((row, col, dist)) = stack.last() {
        field[*row][*col] = Some(*dist);

        let mut possible_ways = vec![];

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let (nrow, _) = row.overflowing_add(dr);
            let (ncol, _) = col.overflowing_add(dc);
            if nrow < height && ncol < width && field[nrow][ncol].is_none() {
                possible_ways.push((nrow, ncol));
            }
        }

        match possible_ways.len() {
            0 => {
                // no possible ways
                let _ = stack.pop();
            }
            n_ways => {
                // do random walk
                let n = rng.gen_range(0..n_ways);
                let (x, y) = possible_ways[n];
                stack.push((x, y, dist + 1));
            }
        }
    }

    field
        .into_iter()
        .map(|row| {
            row.into_iter()
                .collect::<Option<_>>()
                .expect("Distances to all tiles should be already known by this point")
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
fn get_path(width: usize, height: usize, base: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut max_dist = 0;
    let mut tail = (0, 0);

    let mut set_if_max = |pos: (usize, usize)| {
        if base[pos.0][pos.1] > max_dist {
            max_dist = base[pos.0][pos.1];
            tail = pos;
        }
    };

    for i in 0..width {
        set_if_max((0, i));
        set_if_max((height - 1, i));
    }

    for j in 0..height {
        set_if_max((j, 0));
        set_if_max((j, width - 1));
    }

    let mut path = vec![];

    while base[tail.0][tail.1] != 0 {
        let mut possible_ways = vec![];

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let (nrow, _) = tail.0.overflowing_add(dr);
            let (ncol, _) = tail.1.overflowing_add(dc);
            if nrow < height && ncol < width && base[tail.0][tail.1].abs_diff(base[nrow][ncol]) == 1
            {
                possible_ways.push((nrow, ncol));
            }
        }

        path.push(tail);

        tail = possible_ways
            .into_iter()
            .min_by_key(|k| base[k.0][k.1])
            .expect("This is a descent and there should be no drains except of start");
    }
    path.push(tail);

    path
}

#[allow(clippy::ptr_arg)]
fn get_walls(width: usize, height: usize, base: &Vec<Vec<usize>>) -> Vec<Vec<Tile>> {
    let mut tiles = vec![vec![Tile::Road; width * 2 - 1]; height * 2 - 1];
    // set "corner" walls
    (0..height - 1).for_each(|mut row| {
        row = row * 2 + 1;
        (0..width - 1).for_each(|mut col| {
            col = col * 2 + 1;
            tiles[row][col] = Tile::Wall;
        });
    });

    // add walls
    (0..height).for_each(|row| {
        (0..width - 1).for_each(|col| {
            if base[row][col].abs_diff(base[row][col + 1]) != 1 {
                tiles[row * 2][col * 2 + 1] = Tile::Wall;
            }
        });
    });
    (0..height - 1).for_each(|row| {
        (0..width).for_each(|col| {
            if base[row][col].abs_diff(base[row + 1][col]) != 1 {
                tiles[row * 2 + 1][col * 2] = Tile::Wall;
            }
        });
    });

    tiles
}
