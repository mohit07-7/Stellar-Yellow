#![cfg(test)]
use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_create_product() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Anyone can create a product (permissionless)
    client.create_product(
        &String::from_str(&env, "SKU-001"),
        &String::from_str(&env, "Organic Coffee Beans"),
        &String::from_str(&env, "Colombia"),
    );

    let product = client.get_product(&String::from_str(&env, "SKU-001"));
    assert_eq!(product.name, String::from_str(&env, "Organic Coffee Beans"));
    assert_eq!(product.origin, String::from_str(&env, "Colombia"));
    assert_eq!(product.status, String::from_str(&env, "Created"));
}

#[test]
fn test_update_status_permissionless() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Create product
    client.create_product(
        &String::from_str(&env, "SKU-002"),
        &String::from_str(&env, "Premium Rice"),
        &String::from_str(&env, "Thailand"),
    );

    // ANYONE can update status (permissionless - no auth required)
    client.update_status(
        &String::from_str(&env, "SKU-002"),
        &String::from_str(&env, "Shipped"),
    );

    let product = client.get_product(&String::from_str(&env, "SKU-002"));
    assert_eq!(product.status, String::from_str(&env, "Shipped"));
}

#[test]
fn test_get_history() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Create product
    client.create_product(
        &String::from_str(&env, "SKU-003"),
        &String::from_str(&env, "Handmade Textiles"),
        &String::from_str(&env, "India"),
    );

    // Update through multiple stages
    client.update_status(
        &String::from_str(&env, "SKU-003"),
        &String::from_str(&env, "Shipped from Factory"),
    );
    client.update_status(
        &String::from_str(&env, "SKU-003"),
        &String::from_str(&env, "Arrived at Port"),
    );
    client.update_status(
        &String::from_str(&env, "SKU-003"),
        &String::from_str(&env, "Customs Cleared"),
    );
    client.update_status(
        &String::from_str(&env, "SKU-003"),
        &String::from_str(&env, "Delivered"),
    );

    let history = client.get_history(&String::from_str(&env, "SKU-003"));
    assert_eq!(history.len(), 5); // Created + 4 updates

    // Check first entry is "Created"
    assert_eq!(
        history.get(0).unwrap().status,
        String::from_str(&env, "Created")
    );
    // Check last entry is "Delivered"
    assert_eq!(
        history.get(4).unwrap().status,
        String::from_str(&env, "Delivered")
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_product_not_found() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.get_product(&String::from_str(&env, "NON-EXISTENT"));
}

#[test]
#[should_panic(expected = "already exists")]
fn test_duplicate_product() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Create first product
    client.create_product(
        &String::from_str(&env, "SKU-004"),
        &String::from_str(&env, "Tea Leaves"),
        &String::from_str(&env, "Japan"),
    );

    // Try to create duplicate - should fail
    client.create_product(
        &String::from_str(&env, "SKU-004"),
        &String::from_str(&env, "Different Name"),
        &String::from_str(&env, "Different Origin"),
    );
}

#[test]
fn test_multiple_products() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Create multiple products
    client.create_product(
        &String::from_str(&env, "P1"),
        &String::from_str(&env, "Product 1"),
        &String::from_str(&env, "Origin 1"),
    );
    client.create_product(
        &String::from_str(&env, "P2"),
        &String::from_str(&env, "Product 2"),
        &String::from_str(&env, "Origin 2"),
    );
    client.create_product(
        &String::from_str(&env, "P3"),
        &String::from_str(&env, "Product 3"),
        &String::from_str(&env, "Origin 3"),
    );

    assert_eq!(
        client.get_product(&String::from_str(&env, "P1")).name,
        String::from_str(&env, "Product 1")
    );
    assert_eq!(
        client.get_product(&String::from_str(&env, "P2")).name,
        String::from_str(&env, "Product 2")
    );
    assert_eq!(
        client.get_product(&String::from_str(&env, "P3")).name,
        String::from_str(&env, "Product 3")
    );
}

#[test]
fn test_supply_chain_full_journey() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Step 1: Create a product
    client.create_product(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Single Origin Arabica"),
        &String::from_str(&env, "Ethiopia"),
    );

    let product = client.get_product(&String::from_str(&env, "COFFEE-BATCH-001"));
    assert_eq!(product.status, String::from_str(&env, "Created"));

    // Step 2: Ship from farm
    client.update_status(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Shipped from Farm"),
    );

    // Step 3: Arrived at processing facility
    client.update_status(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Processing Facility"),
    );

    // Step 4: Quality checked
    client.update_status(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Quality Checked"),
    );

    // Step 5: Exported
    client.update_status(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Exported to USA"),
    );

    // Step 6: Customs cleared
    client.update_status(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Customs Cleared"),
    );

    // Step 7: Delivered to roastery
    client.update_status(
        &String::from_str(&env, "COFFEE-BATCH-001"),
        &String::from_str(&env, "Delivered to Roastery"),
    );

    // Verify final state
    let final_product = client.get_product(&String::from_str(&env, "COFFEE-BATCH-001"));
    assert_eq!(
        final_product.status,
        String::from_str(&env, "Delivered to Roastery")
    );

    // Verify complete history
    let history = client.get_history(&String::from_str(&env, "COFFEE-BATCH-001"));
    assert_eq!(history.len(), 7); // Created + 6 updates
}
