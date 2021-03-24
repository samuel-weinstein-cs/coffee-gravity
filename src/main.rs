#![feature(drain_filter)]
use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Transformation};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use coffee::input::mouse::*;
use nalgebra::{Point2, Vector2};

const TIMESTEP: f32 = 0.1;
const G: f32 = 1.0; // gravitational constant
const SIZE: f32 = 3.0; //size multiplier

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("G R A V I T Y!!!"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

#[derive(Clone, Debug)]
struct Planet {
    pos: Point2<f32>,
    vel: Vector2<f32>,
    acc: Vector2<f32>,
    mass: f32,
}

impl Planet {
    fn new(pos: Point2<f32>, vel: Vector2<f32>, mass: f32) -> Self {
        Self {
            pos,
            vel,
            acc: Vector2::new(0.0,0.0),
            mass
        }
    }

    fn add_force(&mut self, f: Vector2<f32>) {
        self.acc += f / self.mass;
    }

    fn timestep(&mut self) {
        // println!("vel: {:?} += acc: {:?} * timestep: {:?}", self.vel, self.acc, TIMESTEP);
        self.vel += self.acc * TIMESTEP;
        // println!("vel: {:?}", self.vel);
        self.pos += self.vel * TIMESTEP;
        self.acc = Vector2::new(0.0,0.0);
    }
    fn attract(&mut self, other: &Planet) {
        // println!("{:?}, {:?}", self as *const Planet, other as *const Planet);
        // if !std::ptr::eq(self, other) {// cannot apply gravity to self
        let m1 = self.mass;
        // println!("m1: {:?}",m1);
        let m2 = other.mass;
        // println!("m2: {:?}",m2);
        let r_squared = nalgebra::distance_squared(&self.pos, &other.pos);
        // println!("r^2: {:?}",r_squared);
        let f = G * (m1*m2)/r_squared;
        // println!("f: {:?}",f);
        let subtract_vec = other.pos.coords-self.pos.coords;
        // println!("subvec: {:?}",subtract_vec);
        // let r21 = subtract_vec.norm();
        // println!("r21: {:?}",r21);
        self.add_force(f*subtract_vec);
        // }
    }
}

struct MyGame {
    // Your game state and assets go here...
    planets: Vec<Planet>,
    mouse: Point2<f32>,
    click: Option<Point2<f32>>
}

impl Game for MyGame {
    type Input = Mouse; // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| MyGame {
            planets: Vec::new(),
            mouse: Point2::new(0.0,0.0),
            click: None
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        const MULT: f32 = 0.01; // velocity scale factor
        self.mouse = input.cursor_position();
        let mouse_pressed = input.is_button_pressed(Button::Left);
        self.click =
        if self.click.is_none() && mouse_pressed {//click down?
            Some(self.mouse)
        } else if self.click.is_some() && !mouse_pressed {//click up?
            let vel = self.mouse.coords - self.click.unwrap().coords;
            let new_planet = Planet::new(self.click.unwrap(), vel * MULT, 10.0);
            self.planets.push(new_planet);
            None
        } else {//no change
            self.click //thx rust optimizer :P
        }

    }

    fn update(&mut self, _window: &Window) {

        // println!("pos: {} click: {:?}", self.mouse, self.click);
        for planet in self.planets.iter_mut() {
            planet.timestep();
        }
        /* this runs in O(n^2) :/ n-body is notoriously difficult, even
        solutions like this but this is prob a bit naïve*/
        for i in 0..self.planets.len() {
            let planet1 = &self.planets[i].clone();
            for (j, planet2) in self.planets.iter_mut().enumerate() {
                if j!=i{
                    planet2.attract(planet1);
                }
            }
        }


    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // const VEC_LEN: f32 = 0.01; // velocity scale factor

        let mut target = frame.as_target();
        target.clear(Color::BLACK);

        let transformation = Transformation::scale(1.0);
        let mut camera = target.transform(transformation);


        // Clear the current frame

        let mut mesh = Mesh::new();

         if let Some(click) = self.click {
             let velocity_vec = Shape::Polyline{points:vec![self.mouse, click]};
             mesh.stroke(velocity_vec, Color::GREEN,2.0);
         }

        for planet in self.planets.iter() {// draw planets
            let circle = Shape::Circle{center: planet.pos, radius: planet.mass.sqrt() * SIZE};
            let acc = Shape::Polyline{points:vec![planet.pos, planet.pos + planet.acc*200.0]};
            let vel = Shape::Polyline{points:vec![planet.pos, planet.pos + planet.vel*6.0]};

            mesh.fill(circle, Color::WHITE);
            mesh.stroke(acc, Color::BLUE, 2.0);
            mesh.stroke(vel, Color::GREEN, 2.0);


        }
        // Draw your game here. Check out the `graphics` module!
        mesh.draw(&mut camera);
    }

    const TICKS_PER_SECOND: u16 = 60;

    const DEBUG_KEY: Option<coffee::input::keyboard::KeyCode> = Some(coffee::input::keyboard::KeyCode::F12);
}
