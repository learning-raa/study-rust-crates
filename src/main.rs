// mod study_log;

fn main() -> anyhow::Result<()> {
    println!("Hello, Multi-Verse!");


    //study_log::dive_in()
    Ok(())
}


#[cfg(test)]
mod test_rand {
    use anyhow::Result;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn seed_2() -> Result<()> {
        let mut rnd = ChaCha8Rng::seed_from_u64(1);
        print_10_values(&mut rnd);

        Ok(())
    }
    #[test]
    fn seed_1() -> Result<()> {
        let mut rnd = ChaCha8Rng::seed_from_u64(1);
        print_10_values(&mut rnd);

        Ok(())
    }

    #[test]
    fn unseed_2() -> Result<()> {
        let mut rnd = ChaCha8Rng::from_entropy();
        print_10_values(&mut rnd);

        Ok(())
    }
    #[test]
    fn unseed_1() -> Result<()> {
        let mut rnd = ChaCha8Rng::from_entropy();
        print_10_values(&mut rnd);

        Ok(())
    }

    fn print_10_values(rnd: &mut ChaCha8Rng) {
        for i in 0..10 {
            let v1 = rnd.gen::<u8>();
            println!("rnd1({}): {}", i, v1);
        }
    }
}
