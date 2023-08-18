// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::default::Default;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::thread_rng;
use user::Saver;
mod user;

// 0 to Death Age for time results
pub const DEATH_AGE: usize = 100;

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
    state.0.lock().unwrap().to_vec()
}

#[tauri::command]
fn get_interest_rates(recalc: bool, state: tauri::State<'_, Interest>) -> Vec<f32> {
    if recalc {
        state.0.lock().unwrap().clone_from(&Interest::get_random_rates());
    }
    state.0.lock().unwrap().to_vec()
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
    recalculate: bool,
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
            recalculate,
        );
        user.apply_annual_changes(
            inflationrates.0.lock().unwrap().to_vec(),
             interestrates.0.lock().unwrap().to_vec()
        )
}

fn main() {
    let interest_rates = Interest::default();
    let inflation_rates = Inflation::default();

    tauri::Builder::default()
        .manage(interest_rates)
        .manage(inflation_rates)
        .invoke_handler(tauri::generate_handler![
            get_savings,
            get_inflation_rates,
            get_interest_rates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
