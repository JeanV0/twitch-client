use rand::{distributions::Alphanumeric, Rng};


pub fn get_rng() -> impl Rng {
    rand::thread_rng()
}

pub fn generate_id() -> String {
  let mut rng = get_rng();
  std::iter::repeat(()).map(|_| rng.sample(Alphanumeric)).map(char::from).take(32).collect()
}