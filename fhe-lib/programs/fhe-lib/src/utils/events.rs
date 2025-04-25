use anchor_lang::prelude::*;

#[event]
pub struct Add8 {
    pub lhs: [u8; 32],
    pub rhs: [u8; 32],
    pub sum: [u8; 32],
}