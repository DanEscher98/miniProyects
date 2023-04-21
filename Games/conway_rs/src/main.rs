use conway::ui::*;
use bevy::prelude::*;

// struct Board<T> {
//     width: usize,
//     height: usize,
//     cells: Vec<T>
// }
// 
// #[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
// enum State {
//     #[default]
//     Death,
//     Alive
// }
// 
// impl<T: Default + Copy> Default for Board<T> {
//     fn default() -> Board<T> {
//         let cells = vec![T::default(); 64usize];
//         Board {
//             width: 8usize,
//             height: 8usize,
//             cells
//         }
//     }
// }
// 
// impl<T: Default + Copy> Board<T> {
//     fn new(width: usize, height: usize, cells: Vec<T>) -> Option<Self> {
//         if width * height != cells.len() {
//             return None;
//         }
//         Some(Board {
//             width,
//             height,
//             cells
//         })
//     }
//     fn point2idx(&self, point: (usize, usize)) -> usize {
//         (point.0 - 1usize) * self.width + (point.1 - 1usize)
//     }
//     fn idx2point(&self, index: usize) -> (usize, usize) {
//         (1usize + index / self.width, 1usize + index % self.width)
//     }
//     fn getby_point(&self, point: (usize, usize)) -> T {
//         let idx = self.point2idx(point);
//         self.cells[idx]
//     }
// }

const GRID_SIZE: i32 = 180;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1024f32,
                height: 720f32,
                title: String::from("Conway's Game of Life"),
                ..default()
            },
            ..default()
        }))
        .run();
}

// #[cfg(test)]
// mod test {
//     use super::*;
// 
//     #[test]
//     fn point2idx() {
//         let board = Board::<bool>::default();
//         assert_eq!(board.point2idx((2, 2)), 4);
//     }
// 
//     #[test]
//     fn getby_point() {
//         let board = Board::<State>::default();
//         assert_eq!(board.getby_point((2, 2)), State::Death);
//     }
// }
