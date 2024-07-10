use anchor_lang::prelude::*;

// calculate the % represent by the <amount> over the <total>
// example:
// total = 53 sponsorships (total event tokens)
// amount = 5 sponsorships (event tokes from a sponsor)
// share = (5*100)/53 = 9.43
pub fn calculate_share(total: u64, amount: u64) -> f64 {
    let temp = amount*100; 
    let share = (temp as f64)/(total as f64);
    share
}

// calculate the <share>% of the <total>
// example: 
// total = 150 USDC
// share = 9.43 (9.43%)
// earnings = (150.0)*(9.43)/(100.0).floor() = 33
pub fn calculate_earnings(total: u64, share: f64) -> u64 {
    let temp = (total as f64)*share; 
    let earnings = (temp as f64)/(100.);
    earnings.floor() as u64 
}