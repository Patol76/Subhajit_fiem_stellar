#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, Address, Vec
};

#[derive(Clone)]
#[contracttype]
pub struct Bet {
    pub user: Address,
    pub amount: i128,
    pub team: Symbol,
}

#[derive(Clone)]
#[contracttype]
pub struct Match {
    pub team1: Symbol,
    pub team2: Symbol,
    pub result: Symbol,
    pub is_finished: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Match,
    Bets,
}

#[contract]
pub struct SportsBettingContract;

#[contractimpl]
impl SportsBettingContract {

    // Initialize admin
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Create match
    pub fn create_match(env: Env, team1: Symbol, team2: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let m = Match {
            team1,
            team2,
            result: symbol_short!("NONE"),
            is_finished: false,
        };

        env.storage().instance().set(&DataKey::Match, &m);

        let bets: Vec<Bet> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Bets, &bets);
    }

    // Place bet
    pub fn place_bet(env: Env, user: Address, amount: i128, team: Symbol) {
        user.require_auth();

        let mut bets: Vec<Bet> = env
            .storage()
            .instance()
            .get(&DataKey::Bets)
            .unwrap();

        let bet = Bet { user, amount, team };

        bets.push_back(bet);

        env.storage().instance().set(&DataKey::Bets, &bets);
    }

    // Declare result
    pub fn declare_result(env: Env, result: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut m: Match = env.storage().instance().get(&DataKey::Match).unwrap();

        m.result = result;
        m.is_finished = true;

        env.storage().instance().set(&DataKey::Match, &m);
    }

    // Get match
    pub fn get_match(env: Env) -> Match {
        env.storage().instance().get(&DataKey::Match).unwrap()
    }

    // Get bets
    pub fn get_bets(env: Env) -> Vec<Bet> {
        env.storage().instance().get(&DataKey::Bets).unwrap()
    }
}