// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use user::saver::SaverType;
use std::default::Default;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;
mod tests;
mod user;

// 0 to Death Age for time results
pub const DEATH_AGE: usize = 100;

#[derive(Serialize)]
struct ZeroDistribution {
    age: Vec<u8>,
    count: Vec<u16>,
    avg: f32,
    stdv: f32,
}

pub struct Savings(Arc<Mutex<Vec<f32>>>);
impl Default for Savings {
    fn default() -> Self {
        Savings(Arc::new(Mutex::new(vec![])))
    }
}

pub struct Age(Arc<Mutex<Vec<f32>>>);
impl Default for Age {
    fn default() -> Self {
        Age(Arc::new(Mutex::new(vec![])))
    }
}

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

#[tauri::command]
fn calculate(mut user_savings: user::saver::Saver, recalculate: bool) -> user::saver::Saver {

    user_savings.reset_rates(recalculate);
    let mut owner_savings = user::saver::Saver{
        monthly_rent: 0.0,
        ..user_savings.clone()
    };   
    let mut renter_savings = user::saver::Saver{
        home_value: 0.0,
        mortgage_debt: 0.0,
        mortgage_term: 0,
        mortgage_rate: 0.0,
        ..user_savings.clone()
    };

    user_savings.home_savings = owner_savings.calculate_savings(SaverType::HomeOwner);
    user_savings.home_owned_age = owner_savings.home_owned_age;

    user_savings.rental_savings = renter_savings.calculate_savings(SaverType::Renter);

    user_savings.ymax = user_savings.get_ymax();
    user_savings
}

// #[tauri::command]
// fn get_zero_distributions(
//         currentage: u8,
//         retirementage: u8,
//         monthlysavings: f32,
//         homevalue: f32,
//         monthlyrent: f32,
//         totalsavings: f32,
//         mortgageoutstanding: f32,
//         minbaselineretirementincome: f32,
//         maxbaselineretirementincome: f32,
//     ) -> ZeroDistribution {

//             let mut age_zero_distribution: HashMap<u8, u16> = HashMap::new();
//             let mut zd: ZeroDistribution = ZeroDistribution { age: vec![], count: vec![], avg: 0.0, stdv: 0.0 };
//             let mut avg: f32 = 0.0;
//             let mut stdv: f32 = 0.0;
//             let iter_count = 1000.0;
//             for _ in 0..iter_count as usize {
//                 let mut user = Saver::new(
//                     currentage,
//                     retirementage,
//                     monthlysavings,
//                     homevalue,
//                     monthlyrent,
//                     totalsavings,
//                     mortgageoutstanding,
//                     minbaselineretirementincome,
//                     maxbaselineretirementincome,
//                 );
//                 let inflation_rates = Inflation::get_random_rates();
//                 let interest_rates = Interest::get_random_rates();
//                 let res = user.apply_annual_changes(
//                     &inflation_rates,
//                     &interest_rates
//                 );

//                 for i in (0..=res.len() - 1).rev() {
//                     if res[i] != 0.0 {
//                         if age_zero_distribution.contains_key(&(i as u8)) {
//                             *age_zero_distribution.get_mut(&(i as u8)).unwrap() += 1;
//                         } else {
//                             age_zero_distribution.insert(i as u8, 1);
//                         };
//                         avg = avg + match mean(&interest_rates[currentage as usize..i]){
//                             Some(num) => num,
//                             None => continue
//                         };

//                         stdv = stdv + match std_deviation(&interest_rates[currentage as usize..i]){
//                             Some(num) => num,
//                             None => continue
//                         };
//                         break;
//                     }
//                 }

//             }
//             for (key, value) in age_zero_distribution.iter() {
//                 zd.age.push(*key);
//                 zd.count.push(*value);
//             }
//             zd.avg = avg / iter_count;
//             zd.stdv = stdv / iter_count;
//             zd
//         }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
