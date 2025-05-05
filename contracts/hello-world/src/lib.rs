#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Vec, String};

#[contracttype]
#[derive(Clone)]
pub struct Expense {
    pub payer: Address,
    pub amount: i128,
    pub description: String,
}

#[contracttype]
pub enum TrackerKey {
    Members,
    Expenses,
    Balances,
}

#[contract]
pub struct SharedExpenseTracker;

#[contractimpl]
impl SharedExpenseTracker {
    pub fn init_group(env: Env, members: Vec<Address>) {
        env.storage().instance().set(&TrackerKey::Members, &members);
        env.storage().instance().set(&TrackerKey::Expenses, &Vec::<Expense>::new(&env));
        env.storage().instance().set(&TrackerKey::Balances, &Map::<Address, i128>::new(&env));
    }

    pub fn add_expense(env: Env, payer: Address, amount: i128, description: String) {
        let mut expenses = env
            .storage()
            .instance()
            .get::<TrackerKey, Vec<Expense>>(&TrackerKey::Expenses)
            .unwrap_or(Vec::new(&env));

        let mut balances = env
            .storage()
            .instance()
            .get::<TrackerKey, Map<Address, i128>>(&TrackerKey::Balances)
            .unwrap_or(Map::new(&env));

        let members = env
            .storage()
            .instance()
            .get::<TrackerKey, Vec<Address>>(&TrackerKey::Members)
            .expect("Group not initialized");

        let per_member = amount / (members.len() as i128);

        for member in members.iter() {
            let addr = member.clone();
            let mut balance = balances.get(addr.clone()).unwrap_or(0);

            if addr == payer {
                balance += amount - per_member;
            } else {
                balance -= per_member;
            }

            balances.set(addr, balance);
        }

        expenses.push_back(Expense {
            payer,
            amount,
            description,
        });

        env.storage().instance().set(&TrackerKey::Expenses, &expenses);
        env.storage().instance().set(&TrackerKey::Balances, &balances);
    }

    pub fn get_balances(env: Env) -> Map<Address, i128> {
        env.storage()
            .instance()
            .get::<TrackerKey, Map<Address, i128>>(&TrackerKey::Balances)
            .unwrap_or(Map::new(&env))
    }

    pub fn get_expenses(env: Env) -> Vec<Expense> {
        env.storage()
            .instance()
            .get::<TrackerKey, Vec<Expense>>(&TrackerKey::Expenses)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_members(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get::<TrackerKey, Vec<Address>>(&TrackerKey::Members)
            .unwrap_or(Vec::new(&env))
    }
}
