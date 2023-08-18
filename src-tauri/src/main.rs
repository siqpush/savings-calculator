// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::collections::HashMap;
use std::default::Default;
use std::env;
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
        let weights = [20.0, 10.0, 2.0, 1.0, 3.0, 2.0, 12.0, 3.0, 8.0, 3.0, 1.0];
        let choices = [7.0, 10.0, 15.0, -5.0, -1.0, 3.0, 6.0, 2.0, -0.4, -21.0, 30.0];
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        let mut values: Vec<f32> = vec![0.0; DEATH_AGE];
        for i in 0..DEATH_AGE {
            values[i] = choices[dist.sample(&mut rng)] / 100.0;
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
        let weights = [10.0, 5.0, 2.0, 1.0, 1.0];
        let choices = [3.0, 2.0, 4.0, -1.0, 8.0];
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        let mut values: Vec<f32> = vec![0.0; DEATH_AGE];
        for i in 0..DEATH_AGE {
            values[i] = choices[dist.sample(&mut rng)] / 100.0;
        }
        values
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
            inflationrates.0.lock().unwrap().clone(),
             interestrates.0.lock().unwrap().clone()
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
            let mut zd: ZeroDistribution = ZeroDistribution { age: vec![], count: vec![] };

            for _ in 0..1000 {
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
                let res = user.apply_annual_changes(
                    Inflation::get_random_rates(),
                    Interest::get_random_rates()
                );

                for i in (0..=res.len() - 1).rev() {
                    if res[i] != 0.0 {
                        if age_zero_distribution.contains_key(&(i as u8)) {
                            *age_zero_distribution.get_mut(&(i as u8)).unwrap() += 1;
                        } else {
                            age_zero_distribution.insert(i as u8, 1);
                        };
                        break;
                    }
                }

            }
            for (key, value) in age_zero_distribution.iter() {
                zd.age.push(*key);
                zd.count.push(*value);
            }

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
