mod tools;
mod tree;
mod config;

use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, MeshBuilder, DrawParam, Text};
use ggez::nalgebra::Vector2;
use ggez::timer;

use yaml_rust::Yaml;

use std::f32::consts::PI;
use std::io::{self, Read};

use crate::tree::Tree;
use crate::config::Config;

const CONFIG_FILE_LOCATION: &str = "./config.yaml";
const SCREEN_DIMS: (f32, f32) = (1000.0, 800.0);
const RAD_TO_DEG: f32 = 180.0/PI;
const TWO_PI: f32 = PI * 2.0;

struct MainState {
    tree: Tree,
    angle: f32,
    angular_velocity: f32,
    iters: usize,
    branches_per_iter: usize,
    base_tree_pos: (Vector2<f32>, Vector2<f32>),
    length_multiplier: f32,
    line_thickness: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let config = Config::from(&Self::load_config_file().unwrap()[0]);

        let base_tree_pos = (
            Vector2::new(SCREEN_DIMS.0/2.0, SCREEN_DIMS.1 - SCREEN_DIMS.1/10.0),
            Vector2::new(SCREEN_DIMS.0/2.0, SCREEN_DIMS.1 - (SCREEN_DIMS.1/10.0 + SCREEN_DIMS.1/3.0)),
        );

        let mut s = MainState {
            tree: Tree::new(base_tree_pos.0, base_tree_pos.1, 0),
            angle: config.start_angle,
            angular_velocity: config.angular_velocity,
            iters: config.iterations,
            branches_per_iter: config.branches_per_iteration,
            base_tree_pos,
            length_multiplier: config.length_multiplier,
            line_thickness: config.line_thickness,
        };

        s.gen_tree();

        Ok(s)
    }

    fn load_config_file() -> io::Result<Vec<Yaml>> {
        use yaml_rust::YamlLoader;
        use std::fs;

        let mut conf_file_string = String::new();
        fs::File::open(CONFIG_FILE_LOCATION)?
            .read_to_string(&mut conf_file_string)?;

        let confs = YamlLoader::load_from_str(&conf_file_string).unwrap();
        if confs.is_empty() { panic!("Error: Config file is empty.") }
        Ok(confs)
    }

    fn reload_config(&mut self) {
        let config = Config::from(&Self::load_config_file().unwrap()[0]);
        self.angle = config.start_angle;
        self.angular_velocity = config.angular_velocity;
        self.iters = config.iterations;
        self.branches_per_iter = config.branches_per_iteration;
        self.length_multiplier = config.length_multiplier;
        self.line_thickness = config.line_thickness;
    }

    fn gen_tree(&mut self) {
        self.tree = Tree::new(self.base_tree_pos.0, self.base_tree_pos.1, 0);
        for _ in 0..self.iters {
            self.tree.generate_new_sub_trees(self.branches_per_iter, self.angle, self.length_multiplier);
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::duration_to_f64(timer::average_delta(ctx)) as f32;

        self.angle += self.angular_velocity * dt;

        self.gen_tree();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let mut mesh_builder = MeshBuilder::new();
        self.tree.draw(&mut mesh_builder, self.line_thickness, self.iters)?;         // Generate mesh in mesh_builder

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;

        // Draw info
        let abs_angle = self.angle.abs() % TWO_PI;
        let info_text = Text::new(format!("Angle: {:.3}\nRads: {:.3}", abs_angle * RAD_TO_DEG, abs_angle));
        graphics::draw(ctx, &info_text, DrawParam::default().dest([10.0, 10.0]))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::R => self.reload_config(),
            _ => ()
        }
    }
}

pub fn main() -> GameResult {
    use ggez::conf::{WindowMode, WindowSetup, NumSamples};

    let cb = ContextBuilder::new("Tree", "ggez")
        .window_mode(
            WindowMode::default()
                .dimensions(SCREEN_DIMS.0, SCREEN_DIMS.1)
        )
        .window_setup(
            WindowSetup::default()
                .samples(NumSamples::Four)
        );

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}