use std::collections::BTreeMap;
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Pallet { balances: BTreeMap::new() }
	}
}
