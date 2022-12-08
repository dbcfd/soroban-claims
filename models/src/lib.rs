#![no_std]
use soroban_sdk::{contractimpl, contracttype, symbol, vec, Bytes, Env, Address, Symbol, Vec};

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct PatientVisit {
    pub id: Bytes,
    pub provider: Address,
    pub patient: Address,
    pub timestamp: u64,
    pub info: Bytes,
}

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct InsuranceClaim {
    pub insurer: Address,
    pub biller: Address,
    pub visit: PatientVisit,
    pub code: Bytes,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ClaimRejection {
    pub rejector: Address,
    pub claim: InsuranceClaim,
    pub appeal_id: Option<Bytes>,
    pub timestamp: u64,
    pub reason: Bytes,
}

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ClaimAppeal {
    pub appellant: Address,
    pub rejection: ClaimRejection,
    pub timestamp: u64,
    pub adjustment: InsuranceClaim,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PayableInsuranceClaim {
    Appealed(ClaimAppeal),
    Claim(InsuranceClaim),
}

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct PayableClaim {
    pub claim: PayableInsuranceClaim,
    pub amount: u64,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ClaimPayment {
    pub amount: u64,
    pub claim: PayableClaim,
    pub timestamp: u64,
}