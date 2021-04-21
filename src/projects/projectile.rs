use std::{fs,env};
use crate::point3::*;
use crate::vector3::*;
use crate::canvas::*;
use crate::color::*;

pub struct Projectile {
    pub position: Point3,
    pub velocity: Vector3
}

impl Projectile {
    pub fn projectile(position: Point3, velocity: Vector3) -> Projectile {
        Projectile {
            position,
            velocity
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Environment {
    gravity: Vector3,
    wind: Vector3
}

impl Environment {
    pub fn environment(gravity: Vector3, wind: Vector3) -> Environment {
        Environment {
            gravity,
            wind
        }
    }
}

pub fn tick(env: Environment, proj: Projectile) -> Projectile {
    println!("{} | {}", proj.position.0, proj.position.1);
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind
    }
}

pub fn log_projectile() {
    let mut proj = Projectile::projectile(
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(3.0, 3.0, 0.0)
    );

    let env = Environment::environment(
        Vector3::new(0.0, -0.1, 0.0),
        Vector3::new(-0.01, 0.0, 0.0)
    );

    let mut canvas = Canvas::new(200, 200);

    while proj.position.1 > 0.0 {
        proj = tick(env, proj);
        canvas.write_pixel(
            proj.position.0 as usize,
            199 - proj.position.1 as usize,
            Color::new(1.0, 1.0, 1.0)
        );
    }

    let ppm = canvas.canvas_to_ppm();

    let mut path = env::current_dir().unwrap();
    path.push("eye_candy/projectile.ppm");
    fs::write(path, ppm).expect("Unable to write file");

}
