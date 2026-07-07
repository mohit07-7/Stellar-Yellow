#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Env, Map, String, Vec};

#[contracttype]
pub enum DataKey {
    Products,
    ProductHistory(String),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub origin: String,
    pub status: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct HistoryEntry {
    pub status: String,
    pub timestamp: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProductNotFound = 1,
    ProductAlreadyExists = 2,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Create a new product in the supply chain. PERMISSIONLESS: Anyone can create products.
    pub fn create_product(env: Env, id: String, name: String, origin: String) -> Product {
        let mut products: Map<String, Product> = env
            .storage()
            .instance()
            .get(&DataKey::Products)
            .unwrap_or_else(|| Map::new(&env));

        assert!(!products.contains_key(id.clone()), "Product already exists");

        let timestamp = env.ledger().timestamp();
        let product = Product {
            id: id.clone(),
            name,
            origin,
            status: String::from_str(&env, "Created"),
            timestamp,
        };

        products.set(id.clone(), product.clone());
        env.storage().instance().set(&DataKey::Products, &products);

        // Initialize empty history
        let history: Vec<HistoryEntry> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::ProductHistory(id.clone()), &history);

        // Add initial "Created" entry to history
        let mut history: Vec<HistoryEntry> = env
            .storage()
            .instance()
            .get(&DataKey::ProductHistory(id.clone()))
            .unwrap();
        history.push_back(HistoryEntry {
            status: String::from_str(&env, "Created"),
            timestamp,
        });
        env.storage()
            .instance()
            .set(&DataKey::ProductHistory(id), &history);

        product
    }

    /// Update product status. PERMISSIONLESS: Anyone can update status.
    /// This creates an immutable audit trail.
    pub fn update_status(env: Env, id: String, status: String) -> Product {
        let mut products: Map<String, Product> = env
            .storage()
            .instance()
            .get(&DataKey::Products)
            .unwrap_or_else(|| Map::new(&env));

        assert!(products.contains_key(id.clone()), "Product not found");

        let timestamp = env.ledger().timestamp();
        let mut product = products.get(id.clone()).unwrap();
        product.status = status.clone();
        product.timestamp = timestamp;

        products.set(id.clone(), product.clone());
        env.storage().instance().set(&DataKey::Products, &products);

        // Append to history
        let mut history: Vec<HistoryEntry> = env
            .storage()
            .instance()
            .get(&DataKey::ProductHistory(id.clone()))
            .unwrap();
        history.push_back(HistoryEntry { status, timestamp });
        env.storage()
            .instance()
            .set(&DataKey::ProductHistory(id), &history);

        product
    }

    /// Get product details by ID
    pub fn get_product(env: Env, id: String) -> Result<Product, Error> {
        let products: Map<String, Product> = env
            .storage()
            .instance()
            .get(&DataKey::Products)
            .unwrap_or_else(|| Map::new(&env));

        products.get(id).ok_or(Error::ProductNotFound)
    }

    /// Get full status history for a product
    pub fn get_history(env: Env, id: String) -> Result<Vec<HistoryEntry>, Error> {
        let products: Map<String, Product> = env
            .storage()
            .instance()
            .get(&DataKey::Products)
            .unwrap_or_else(|| Map::new(&env));

        if !products.contains_key(id.clone()) {
            return Err(Error::ProductNotFound);
        }

        let history: Vec<HistoryEntry> = env
            .storage()
            .instance()
            .get(&DataKey::ProductHistory(id))
            .unwrap_or_else(|| Vec::new(&env));

        Ok(history)
    }
}

mod test;
