
use std::Collections::HashMap;

struct DoubleMap<K, V> {
	keytoval: HashMap<K, V>,
	valtokey: HashMap<V, K>
}

impl DoubleMap<K, V> {
	
	pub fn new() -> DoubleMap<K, V> {
		DoubleMap {
			keytoval: HashMap::new(),
			valtokey: HashMap::new()


}
