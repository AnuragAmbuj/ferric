mod farm_hasher_ops {
    use std::sync::Mutex;
    use farmhash::FarmHasher;
    use lazy_static::lazy_static;

    fn get_default_farm_hasher() -> FarmHasher {
        FarmHasher::default()
    }

    lazy_static! {
        pub static ref HASHER: Mutex<FarmHasher> = Mutex::new(get_default_farm_hasher());
    }
}