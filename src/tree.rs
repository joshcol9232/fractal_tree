use ggez::nalgebra::{Vector2, Point2};
use ggez::GameResult;
use ggez::graphics::MeshBuilder;

use crate::tools;

pub struct Tree {
    pub sub_trees: Vec<Tree>,
    start: Vector2<f32>,
    end: Vector2<f32>,
    generation: usize,          // The generation the tree belongs to. Starts at 0
}

impl Tree {
    pub fn new(start: Vector2<f32>, end: Vector2<f32>, generation: usize) -> Self {
        Self {
            sub_trees: vec![],
            start,
            end,
            generation,
        }
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder, line_thickness: f32, max_generation: usize) -> GameResult {
        let col_ratio = (0.5 + self.generation as f32/max_generation as f32)/1.5;
        mesh_builder.line(
            &[Point2::new(self.start.x, self.start.y), Point2::new(self.end.x, self.end.y)],
            line_thickness,
            [col_ratio, col_ratio, col_ratio, 1.0].into()
        )?;

        if self.has_sub_trees() {
            for sub_tree in self.sub_trees.iter() {
                sub_tree.draw(mesh_builder, line_thickness, max_generation)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn has_sub_trees(&self) -> bool {
        !self.sub_trees.is_empty()
    }

    pub fn generate_new_sub_trees(&mut self, n: usize, angle: f32, length_multiplier: f32) {
        if self.has_sub_trees() {   // Traverse down tree
            for sub_tree in self.sub_trees.iter_mut() {
                sub_tree.generate_new_sub_trees(n, angle, length_multiplier);         // Recursively call function for each sub tree
            }
        } else {    // At end of branches now, so generate new branch coming off this one
            let (my_angle, my_magnitude) = tools::get_angle_and_magnitude(&self.start, &self.end);
            let new_magnitude = my_magnitude * length_multiplier;

            let mut branch_angle = -angle;
            let branch_angle_interval = (angle * 2.0)/(n - 1) as f32;

            for _ in 0..n {
                let branch_end = tools::vec_from_angle_and_mag(my_angle + branch_angle, new_magnitude) + self.end;
                self.sub_trees.push(Tree::new(self.end, branch_end, self.generation + 1));

                branch_angle += branch_angle_interval;
            }
        }
    }
}