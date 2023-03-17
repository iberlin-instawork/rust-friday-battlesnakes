use core::panic;
use pathfinding::prelude::astar;

use crate::utils::{self};
use crate::{Board as BattlesnakeBoard, Coord as BattlesnakeCoord};
use rust_pathfinding::{Board as PathfindingBoard, PathfindingPos};

use super::{SnakeMode, SnakePersonality};

pub fn determine_goal(
    personality: &SnakePersonality,
    mode: &SnakeMode,
    pathfinding_board: &PathfindingBoard,
    board: &BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> Option<BattlesnakeCoord> {
    match personality {
        &SnakePersonality::Snacky => Some(find_delicious_food(board, head).clone()),
        &SnakePersonality::HeadHunter => {
            if *mode == SnakeMode::Eat {
                Some(find_delicious_food(board, head).clone())
            } else {
                Some(find_delicious_snake(board, head).clone())
            }
        }
        &SnakePersonality::QLearning => {
            Some(find_delicious_food(board, head).clone())
        }
        a => None,
    }
}

fn find_delicious_snake<'a>(
    board: &'a BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> &'a BattlesnakeCoord {
    let mut distances = Vec::new();
    let us = &board.snakes[0];
    println!("{:?}", us);
    let other_snakes = &board.snakes[1..];
    for s in other_snakes {
        let d = utils::coord_distance(head, &s.head);
        let mut smallest_snake_size = 999;
        for s in other_snakes {
            let osd = s.body.len();
            if osd < smallest_snake_size {
                smallest_snake_size = osd;
            }
        }
        distances.push((d, smallest_snake_size));
    }
    println!("{:?}", distances);
    let min = distances
        .iter()
        .filter(|f| f.0 != 0.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("{:?}", min);
    let min_clone = min.clone();
    let idx = distances
        .iter()
        .position(|d| d == &min_clone)
        .expect("cant find snake");
    println!("snake idx {}", idx);
    let snake = &other_snakes.get(idx).unwrap();
    println!("{:?}", snake);
    &snake.head
}

fn find_delicious_food<'a>(
    board: &'a BattlesnakeBoard,
    head: &BattlesnakeCoord,
) -> &'a BattlesnakeCoord {
    if board.snakes.len() == 1 {
        let mut distances = Vec::new();
        for f in &board.food {
            let d = utils::coord_distance(head, f);
            distances.push(d);
        }
        let min = distances
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min_clone = min.clone();
        let idx = distances
            .iter()
            .position(|d| d == &min_clone)
            .expect("cant find food");
        return board.food.get(idx).unwrap();
    } else {
        let mut distances = Vec::new();
        let other_snakes = &board.snakes[1..];
        for f in &board.food {
            let d = utils::coord_distance(head, f);
            let mut closest_other_snake = 999.0;
            for s in other_snakes {
                let osd = utils::coord_distance(&s.head, f);
                if osd < closest_other_snake {
                    closest_other_snake = osd;
                }
            }
            distances.push((d, closest_other_snake));
        }
        println!("{:?}", distances);
        let min = distances
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        println!("{:?}", min);
        let min_clone = min.clone();
        let idx = distances
            .iter()
            .position(|d| d == &min_clone)
            .expect("cant find food");
        return board.food.get(idx).unwrap();
    }
}
