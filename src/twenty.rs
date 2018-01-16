
struct Particle {
  id: usize,
  p: (i32, i32, i32),
  v: (i32, i32, i32),
  a: (i32, i32, i32)
}

fn parse_vector(part: &str) -> (i32, i32, i32) {
  let mut inner = part[3..part.len()-1].split(',');
  let a = inner.next().unwrap().trim().parse().unwrap();
  let b = inner.next().unwrap().trim().parse().unwrap();
  let c = inner.next().unwrap().trim().parse().unwrap();
  (a,b,c)
}

impl Particle {
  fn parse(id: usize, src: &str) -> Self {
    let mut parts = src.split(", ");
    let p = parse_vector(parts.next().unwrap());
    let v = parse_vector(parts.next().unwrap());
    let a = parse_vector(parts.next().unwrap());
    Particle { id, p, v, a }
  }

  fn distance(&self) -> usize {
    let (a,b,c) = self.p;
    a.abs() as usize + b.abs() as usize + c.abs() as usize
  }

  fn tick(&mut self) {
    let (p1,p2,p3) = self.p;
    let (v1,v2,v3) = self.v;
    let (a1,a2,a3) = self.a;
    self.v = (v1+a1,v2+a2,v3+a3);
    self.p = (p1+self.v.0, p2+self.v.1, p3+self.v.2)
  }

  fn collides_with(&self, other: &Particle) -> bool {
    let (a,b,c) = self.p;
    let (x,y,z) = other.p;
    a==x && b==y && c==z
  }
}

fn nearest_particle_simulation(particles: &mut Vec<Particle>) -> usize {
  let nearest_particle = 0;

  nearest_particle
}

fn remove_collided_particles(particles: &mut Vec<Particle>) -> bool {
  let mut collided = vec![];
  let p_len = particles.len();
  for m in 0..p_len {
    let this_particle = &particles[m];
    let mut this_particle_collided = false;

    for n in 0..p_len {
      if n != m {
        let other_particle = &particles[n];
        if this_particle.collides_with(other_particle) {
          collided.push(particles[n].id);
          this_particle_collided = true;
        }
      }
    }

    if this_particle_collided {
      collided.push(this_particle.id);
    }
  }

  particles.retain(|ref p| !collided.contains(&p.id));

  collided.len() > 0
}

fn particle_collision_simulation(particles: &mut Vec<Particle>) {
  let mut collision_streak = 0;
  let streak_req = 1000;

  loop {
    for particle in particles.iter_mut() {
      particle.tick();
    }

    if remove_collided_particles(particles) {
      collision_streak = 0;
    }
    else {
      collision_streak += 1;
    }

    if collision_streak == streak_req {
      break;
    }
  }
}

fn main_2() {
  let mut id = 0;
  let mut particles: Vec<Particle> = include_str!("../input/twenty").lines().map(|ln| {
    let p = Particle::parse(id, ln);
    id += 1;
    p
  })
  .collect();

  particle_collision_simulation(&mut particles);
  println!("Remaining particles = {}", particles.len());
}

pub fn main () {
  main_2();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_works() {
    let particle = Particle::parse(21, "p=<-1659,1444,-463>, v=<-14,-25,-77>, a=<10,-5,10>");

    assert_eq!((-1659,1444,-463), particle.p);
    assert_eq!((-14,-25,-77), particle.v);
    assert_eq!((10,-5,10), particle.a);
  }

  #[test]
  fn distance_works() {
    let p1 = &Particle {
      id: 0,
      p: (0, 12, -3),
      v: (0, 0, 0),
      a: (0, 0, 0)
    };
    assert_eq!(15, p1.distance());

    let p2 = &Particle {
      id: 1,
      p: (3, -12, 33),
      v: (0, 0, 0),
      a: (0, 0, 0)
    };
    assert_eq!(48, p2.distance());

    let p3 = &Particle {
      id: 2,
      p: (-100, 11, 50),
      v: (0, 0, 0),
      a: (0, 0, 0)
    };
    assert_eq!(161, p3.distance());
  }

  #[test]
  fn collides_with_works() {
    let a = Particle {
      id: 0,
      p: (3,2,12),
      v: (2,0,0),
      a: (-1,0,0)
    };
    let b = Particle {
      id: 1,
      p: (3,2,12),
      v: (2,-3,0),
      a: (0,0,0)
    };
    let c = Particle {
      id: 2,
      p: (-3,2,12),
      v: (2,0,0),
      a: (-1,0,0)
    };

    assert_eq!(true, a.collides_with(&b));
    assert_eq!(false, a.collides_with(&c));
  }

  #[test]
  fn tick_works() {
    let mut p1 = Particle {
      id: 12,
      p: (3,0,0),
      v: (2,0,0),
      a: (-1,0,0)
    };
    p1.tick();
    assert_eq!((4,0,0), p1.p);
    assert_eq!((1,0,0), p1.v);
    assert_eq!((-1,0,0), p1.a);

    p1.tick();
    assert_eq!((4,0,0), p1.p);
    assert_eq!((0,0,0), p1.v);
    assert_eq!((-1,0,0), p1.a);

    p1.tick();
    assert_eq!((3,0,0), p1.p);
    assert_eq!((-1,0,0), p1.v);
    assert_eq!((-1,0,0), p1.a);
  }

  #[test]
  #[ignore]
  fn nearest_particle_simulation_works() {
    let mut particles = vec![
      Particle::parse(1, "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>"),
      Particle::parse(2, "p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>")
    ];
    let nearest_particle = nearest_particle_simulation(&mut particles);

    assert_eq!(0, nearest_particle);
  }

  #[test]
  fn remove_collided_particles_works() {
    let mut particles = vec![
      Particle::parse(0, "p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>"),
      Particle::parse(1, "p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>"),
      Particle::parse(2, "p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>"),
      Particle::parse(3, "p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>")
    ];

    remove_collided_particles(&mut particles);

    assert_eq!(1, particles.len());
    assert_eq!(3, particles[0].id);
  }

  #[test]
  fn particle_collision_simulation_works() {
    let mut particles = vec![
      Particle::parse(0, "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>"),
      Particle::parse(1, "p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>"),
      Particle::parse(2, "p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>"),
      Particle::parse(3, "p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>")
    ];

    particle_collision_simulation(&mut particles);

    assert_eq!(1, particles.len());
    assert_eq!(3, particles[0].id);
    assert_eq!((-999,0,0), particles[0].p);
  }
}
