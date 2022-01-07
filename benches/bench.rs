#![feature(test)]

extern crate game_of_life;
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn universe_ticks(bencher: &mut Bencher) {
        let mut universe = game_of_life::Universe::new();

        bencher.iter(|| {
            universe.tick();
        });
    }
}
