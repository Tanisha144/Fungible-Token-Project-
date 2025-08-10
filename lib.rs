use candid::{CandidType, Nat, Principal};
use ic_cdk_macros::{query, update, init};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TOKEN_NAME: Mutex<String> = Mutex::new("EduCoin".to_string());
    static ref TOKEN_SYMBOL: Mutex<String> = Mutex::new("EDU".to_string());
    static ref TOTAL_SUPPLY: Mutex<Nat> = Mutex::new(Nat::from(0u8));
    static ref BALANCES: Mutex<HashMap<Principal, Nat>> = Mutex::new(HashMap::new());
    static ref ADMIN: Mutex<Option<Principal>> = Mutex::new(None);
}

#[derive(CandidType, serde::Deserialize)]
struct InitArgs {
    admin: Option<Principal>,
    name: Option<String>,
    symbol: Option<String>,
    initial_supply: Option<Nat>,
}

#[init]
fn init(args: InitArgs) {
    let caller = ic_cdk::api::caller();
    let admin = args.admin.unwrap_or(caller);
    *ADMIN.lock().unwrap() = Some(admin);

    *TOKEN_NAME.lock().unwrap() = args.name.unwrap_or("EduCoin".to_string());
    *TOKEN_SYMBOL.lock().unwrap() = args.symbol.unwrap_or("EDU".to_string());

    let initial = args.initial_supply.unwrap_or(Nat::from(0u8));
    *TOTAL_SUPPLY.lock().unwrap() = initial.clone();
    if initial > Nat::from(0u8) {
        BALANCES.lock().unwrap().insert(admin, initial);
    }
}

#[query]
fn token_name() -> String {
    TOKEN_NAME.lock().unwrap().clone()
}

#[query]
fn token_symbol() -> String {
    TOKEN_SYMBOL.lock().unwrap().clone()
}

#[query]
fn total_supply() -> Nat {
    TOTAL_SUPPLY.lock().unwrap().clone()
}

#[query]
fn balance_of(p: Principal) -> Nat {
    BALANCES.lock().unwrap().get(&p).cloned().unwrap_or_else(|| Nat::from(0u8))
}

#[update]
fn transfer(to: Principal, amount: Nat) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    let mut balances = BALANCES.lock().unwrap();

    let sender_balance = balances.get(&caller).cloned().unwrap_or_else(|| Nat::from(0u8));
    if sender_balance < amount {
        return Err("❌ Not enough tokens".to_string());
    }

    let new_sender = sender_balance - amount.clone();
    if new_sender == Nat::from(0u8) {
        balances.remove(&caller);
    } else {
        balances.insert(caller, new_sender);
    }

    let receiver_balance = balances.get(&to).cloned().unwrap_or_else(|| Nat::from(0u8));
    balances.insert(to, receiver_balance + amount.clone());

    Ok(format!("✅ Sent {} {}", amount, TOKEN_SYMBOL.lock().unwrap()))
}

#[update]
fn mint(to: Principal, amount: Nat) -> Result<String, String> {
    let caller = ic_cdk::api::caller();
    if Some(caller) != *ADMIN.lock().unwrap() {
        return Err("❌ Only admin can mint".to_string());
    }

    let mut balances = BALANCES.lock().unwrap();
    let receiver_balance = balances.get(&to).cloned().unwrap_or_else(|| Nat::from(0u8));
    balances.insert(to, receiver_balance + amount.clone());

    let mut supply = TOTAL_SUPPLY.lock().unwrap();
    *supply = supply.clone() + amount.clone();

    Ok(format!("✅ Minted {} {}", amount, TOKEN_SYMBOL.lock().unwrap()))
}

candid::export_service!();
#[no_mangle]
pub extern "C" fn __get_candid_interface_tmp_hack() -> *const u8 {
    candid::export_service()
}
