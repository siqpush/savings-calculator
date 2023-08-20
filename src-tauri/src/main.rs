// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::thread_rng;
use user::Saver;
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

trait Rates {
    fn get_random_rates() -> Vec<f32>;
}
pub struct Interest(Arc<Mutex<Vec<f32>>>);
impl Default for Interest {
    fn default() -> Self {
        Interest(Arc::new(Mutex::new(Self::get_random_rates())))
    }
}
impl Rates for Interest {
    fn get_random_rates() -> Vec<f32> {
        let mut rng = thread_rng();
        let mut values: Vec<f32> = vec![0.0; DEATH_AGE];
        for i in 0..DEATH_AGE {
            match i {
                0..=29 => {values[i] = rng.gen_range(-0.2..0.2) + 0.07;},
                30..=49 => {values[i] = rng.gen_range(-0.10..0.15) + 0.05;},
                50..=64 => {values[i] = rng.gen_range(-0.075..0.10) + 0.035;},
                _ => {values[i] = rng.gen_range(-0.0375..0.075) + 0.017675;},
            }
            
        }
        values
    }
}
pub struct Inflation(Arc<Mutex<Vec<f32>>>);
impl Default for Inflation {
    fn default() -> Self {
        Inflation(Arc::new(Mutex::new(vec![])))
    }
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
impl Rates for Inflation {
    fn get_random_rates() -> Vec<f32> {
        let weights = [1.0, 1.0, 1.0, 2.0, 1.0, 60.0, 90.0, 70.0, 70.0, 50.0, 20.0, 6.0, 1.0, 3.0, 2.0, 2.0, 1.0, 1.0];
        let choices = [-0.1, -0.09, -0.06, -0.02, 0.0, 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.13, 0.14, 0.19];
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        let mut values: Vec<f32> = vec![0.0; DEATH_AGE];
        for i in 0..DEATH_AGE {
            values[i] = choices[dist.sample(&mut rng)];
        }
        values
    }
}

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}


#[tauri::command]
fn get_inflation_rates(recalc: bool, state: tauri::State<'_, Inflation>) -> Vec<f32> {
    if recalc {
        state.0.lock().unwrap().clone_from(&Inflation::get_random_rates());
    }
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn get_interest_rates(recalc: bool, state: tauri::State<'_, Interest>) -> Vec<f32> {
    if recalc {
        state.0.lock().unwrap().clone_from(&Interest::get_random_rates());
    }
    state.0.lock().unwrap().clone()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_savings(
    currentage: u8,
    retirementage: u8,
    monthlysavings: f32,
    homevalue: f32,
    totalsavings: f32,
    mortgageoutstanding: f32,
    minbaselineretirementincome: f32,
    maxbaselineretirementincome: f32,
    inflationrates: tauri::State<'_, Inflation>,
    interestrates: tauri::State<'_, Interest>,
) -> Vec<f32> {
        let mut user = Saver::new(
            currentage,
            retirementage,
            monthlysavings,
            homevalue,
            totalsavings,
            mortgageoutstanding,
            minbaselineretirementincome,
            maxbaselineretirementincome,
        );
        user.apply_annual_changes(
            &inflationrates.0.lock().unwrap().clone(),
             &interestrates.0.lock().unwrap().clone()
        )
    }

#[tauri::command]
fn get_zero_distributions(
        currentage: u8,
        retirementage: u8,
        monthlysavings: f32,
        homevalue: f32,
        totalsavings: f32,
        mortgageoutstanding: f32,
        minbaselineretirementincome: f32,
        maxbaselineretirementincome: f32,
    ) -> ZeroDistribution {

            let mut age_zero_distribution: HashMap<u8, u16> = HashMap::new();
            let mut zd: ZeroDistribution = ZeroDistribution { age: vec![], count: vec![], avg: 0.0, stdv: 0.0 };
            let mut avg: f32 = 0.0;
            let mut stdv: f32 = 0.0;
            let iter_count = 1000.0;
            for _ in 0..iter_count as usize {
                let mut user = Saver::new(
                    currentage,
                    retirementage,
                    monthlysavings,
                    homevalue,
                    totalsavings,
                    mortgageoutstanding,
                    minbaselineretirementincome,
                    maxbaselineretirementincome,
                );
                let inflation_rates = Inflation::get_random_rates();
                let interest_rates = Interest::get_random_rates();
                let res = user.apply_annual_changes(
                    &inflation_rates,
                    &interest_rates
                );
                
                for i in (0..=res.len() - 1).rev() {
                    if res[i] != 0.0 {
                        if age_zero_distribution.contains_key(&(i as u8)) {
                            *age_zero_distribution.get_mut(&(i as u8)).unwrap() += 1;
                        } else {
                            age_zero_distribution.insert(i as u8, 1);
                        };
                        avg = avg + match mean(&interest_rates[currentage as usize..i]){
                            Some(num) => num,
                            None => continue
                        };
                        
                        stdv = stdv + match std_deviation(&interest_rates[currentage as usize..i]){
                            Some(num) => num,
                            None => continue
                        };
                        break;
                    }
                }

            }
            for (key, value) in age_zero_distribution.iter() {
                zd.age.push(*key);
                zd.count.push(*value);
            }
            zd.avg = avg / iter_count;
            zd.stdv = stdv / iter_count;
            zd
        }

fn main() {

    tauri::Builder::default()
        .manage(Interest::default())
        .manage(Inflation::default())
        .invoke_handler(tauri::generate_handler![
            get_savings,
            get_zero_distributions,
            get_inflation_rates,
            get_interest_rates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
