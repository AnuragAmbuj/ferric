mod map_ops {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use bytes::Bytes;

    fn get_new_memory_map() -> HashMap<Bytes,Bytes> {
        HashMap::new()
    }

    //Singleton patten
    lazy_static! {
        pub static ref MAP_CACHE: Mutex<HashMap<Bytes,Bytes>> = Mutex::new(get_new_memory_map());
    }
}