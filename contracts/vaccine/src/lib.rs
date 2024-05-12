#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    VaccineSupplyChain,
}

#[derive(Clone)]
#[contracttype]
pub struct VaccineBatch {
    id: u64,
    manufacturer: String,
    tester: String,
    customer: Option<String>, // Changed customer to an optional field to represent when it's not yet purchased
    production_date: String,
    expiration_date: String,
    current_location: String,
    status: VaccineStatus,
}

#[derive(Clone)]
#[contracttype]
pub enum VaccineStatus {
    InTransit,
    InStorage,
    QualityTesting,
    Administered,
}

#[contract]
pub struct VaccineSupplyChainContract;

#[contractimpl]
impl VaccineSupplyChainContract {
    pub fn create_batch(
        env: Env,
        from: Address,
        id: u64,
        manufacturer: String,
        tester: String,
        production_date: String,
        expiration_date: String,
        current_location: String,
    ) {
        from.require_auth();

        let batch = VaccineBatch {
            id,
            manufacturer,
            tester,
            customer: None, // Set customer to None initially
            production_date,
            expiration_date,
            current_location,
            status: VaccineStatus::QualityTesting, // Change initial status to QualityTesting
        };

        env.storage().instance().set(&StorageKey::VaccineSupplyChain, &batch);
    }

    pub fn update_batch_status(
        env: Env,
        from: Address,
        id: u64,
        new_status: VaccineStatus,
        new_location: String,
    ) {
        from.require_auth();

        let mut batch: VaccineBatch = env
            .storage()
            .instance()
            .get(&StorageKey::VaccineSupplyChain)
            .unwrap_or_else(|| panic!("Vaccine batch with ID {} does not exist", id));

        batch.status = new_status;
        batch.current_location = new_location;

        env.storage().instance().set(&StorageKey::VaccineSupplyChain, &batch);
    }

    pub fn purchase_vaccine(
        env: Env,
        from: Address,
        id: u64,
        customer_name: String,
    ) {
        from.require_auth();

        let mut batch: VaccineBatch = env
            .storage()
            .instance()
            .get(&StorageKey::VaccineSupplyChain)
            .unwrap_or_else(|| panic!("Vaccine batch with ID {} does not exist", id));

        if let Some(_) = batch.customer {
            panic!("Vaccine batch with ID {} has already been purchased", id);
        }

        batch.customer = Some(customer_name);
        batch.status = VaccineStatus::InStorage;

        env.storage().instance().set(&StorageKey::VaccineSupplyChain, &batch);
    }

    pub fn get_batch(env: Env) -> VaccineBatch {
        env.storage()
            .instance()
            .get(&StorageKey::VaccineSupplyChain)
            .unwrap_or_else(|| panic!("No vaccine batch found"))
    }
}
