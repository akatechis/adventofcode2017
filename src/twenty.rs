
struct Particle {
  p: (i32, i32, i32),
  v: (i32, i32, i32),
  a: (i32, i32, i32)
}

fn parse_vector(part: &str) -> (i32, i32, i32) {
  let mut inner = part[3..part.len()-1].split(',');
  let a = inner.next().unwrap().parse().unwrap();
  let b = inner.next().unwrap().parse().unwrap();
  let c = inner.next().unwrap().parse().unwrap();
  (a,b,c)
}

impl Particle {
  fn parse(src: &str) -> Self {
    let mut parts = src.split(", ");
    let p = parse_vector(parts.next().unwrap());
    let v = parse_vector(parts.next().unwrap());
    let a = parse_vector(parts.next().unwrap());
    Particle {
      p, v, a
    }
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
}

fn nearest_particle_simulation(particles: &mut Vec<Particle>) -> usize {
  let nearest_particle = 0;

  loop {

  }

  nearest_particle
}

pub fn main () {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_works() {
    let particle = Particle::parse("p=<-1659,1444,-463>, v=<-14,-25,-77>, a=<10,-5,10>");

    assert_eq!((-1659,1444,-463), particle.p);
    assert_eq!((-14,-25,-77), particle.v);
    assert_eq!((10,-5,10), particle.a);
  }

  #[test]
  fn distance_works() {
    let p1 = &Particle {
      p: (0, 12, -3),
      v: (0, 0, 0),
      a: (0, 0, 0)
    };
    assert_eq!(15, p1.distance());

    let p2 = &Particle {
      p: (3, -12, 33),
      v: (0, 0, 0),
      a: (0, 0, 0)
    };
    assert_eq!(48, p2.distance());

    let p3 = &Particle {
      p: (-100, 11, 50),
      v: (0, 0, 0),
      a: (0, 0, 0)
    };
    assert_eq!(161, p3.distance());
  }

  #[test]
  fn tick_works() {
    let mut p1 = Particle {
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
      Particle::parse("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>"),
      Particle::parse("p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>")
    ];
    let nearest_particle = nearest_particle_simulation(&mut particles);

    assert_eq!(0, nearest_particle);
  }
}
