fn gen_djb_2_hash(value: &str) -> u64 {
    let mut hash: u64 = 5381;
    let value_bytes = value.bytes();

    for byte in value_bytes {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64)
    }
    hash
}

struct CustomHashMap<V> {
    buckets: Vec<Vec<(String, V)>>,
    capacity: usize
}

impl<V: Clone> CustomHashMap<V> {
    fn new(capacity: Option<usize>) -> Self {
        let cap = match capacity {
            Some(x) => x,
            None => 16
        };

        Self {
            capacity: cap,
            buckets: vec![Vec::new(); cap]
        }
    }

    fn get_bucket_index(&self, key: &str) -> usize {
        (gen_djb_2_hash(key) as usize) % self.capacity
    }

    fn insert(&mut self, key: String, value: V) {
        let bucket_index = self.get_bucket_index(&key);
        let bucket = &mut self.buckets[bucket_index];

        for pair in bucket.iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return
            }
        }

        bucket.push((key, value))
    }

    fn get(&self, key: &str) -> Option<&V> {
        let bucket_index = self.get_bucket_index(&key);
        let bucket = &self.buckets[bucket_index];

        for (k,v) in bucket {
            if key == k {
                return Some(v)
            }
        }
        return None
    }
}


fn main() {
    let mut hmap: CustomHashMap<i32> = CustomHashMap::new(None);

    for i in 1..=100 {
        let val = i*i;
        hmap.insert(i.to_string(), val);
        println!("Key '{}': {:?}", i, hmap.get(&i.to_string()));
    }
}
