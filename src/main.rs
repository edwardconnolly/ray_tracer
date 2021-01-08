#[macro_use]
pub mod tuples;

fn main() {
    println!("The Ray Tracer Challenge");
    chapter_1::putting_it_all_together();
}

// Chapter 1: Putting it all together

mod chapter_1 {
    use crate::tuples::Tuple;

    struct Projectile {
        position: Tuple,
        velocity: Tuple,
    }

    struct Env {
        gravity: Tuple,
        wind: Tuple,
    }

    pub fn putting_it_all_together() {
        let mut p = Projectile {
            position: point!(0.0, 1.0, 0.0),
            velocity: vector!(1.0, 1.0, 0.0).normalize() * float!(100.0),
        };
        let e = Env {
            gravity: vector!(0.0, -1.0, 0.0),
            wind: vector!(-0.01, 0.0, 0.0),
        };

        let mut ticks = 0;
        while p.position.y > float!(0.0) {
            p = tick(&e, p);
            ticks += 1;
            println!("Position: {:?} Ticks: {}", p.position, ticks)
        }
    }

    fn tick(env: &Env, proj: Projectile) -> Projectile {
        let position = proj.position + proj.velocity;
        let velocity = proj.velocity + env.gravity + env.wind;
        Projectile { position, velocity }
    }
}
