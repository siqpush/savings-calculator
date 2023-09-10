pub mod rates {
    use std::sync::{Arc, Mutex};
    use rand::{thread_rng, Rng, distributions::WeightedIndex, prelude::Distribution};

    pub struct Interest {
        pub rates: Vec<f32>,
    }
    impl Default for Interest {
        fn default() -> Self {
            let mut rng = thread_rng();
            let mut values: Vec<f32> = vec![0.0; 100];
            for i in 0..100 {
                match i {
                    0..=29 => {values[i] = rng.gen_range(-0.2..0.2) + 0.07;},
                    30..=49 => {values[i] = rng.gen_range(-0.10..0.15) + 0.05;},
                    50..=64 => {values[i] = rng.gen_range(-0.075..0.10) + 0.035;},
                    _ => {values[i] = rng.gen_range(-0.0375..0.075) + 0.017675;},
                }
                
            }
            Self{rates: values}
        }
    }
    pub struct Inflation {
        pub rates: Vec<f32>,
    }

    impl Default for Inflation {
        fn default() -> Self {
            let mut rng = thread_rng();
            let mut values: Vec<f32> = vec![0.0; 100];
            for i in 0..100 {
                values[i] = rng.gen_range(-0.02..0.06) / rng.gen_range(1.0..2.0);
            }
            Self{rates: values}
        }
    }

}

pub mod savings {
    use serde::{Serialize, Deserialize};

    use crate::user::rates::{Inflation, Interest};

    pub const PROPERTY_TAX: f32 = 0.01;
    pub const HOME_EXPENSES: f32 = 0.01;
    pub const WITHDRAWAL_RATE: f32 = 0.04;
    pub const MORTGAGE_MONTHS: f32 = 30.0 * 12.0;
    pub const MORTGAGE_RATE_MONTHLY: f32 = 0.02 / 12.0;

    #[serde(rename_all = "camelCase")]
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Saver {
        pub current_age: u8,
        pub retirement_age: u8,
        pub total_savings: f32,
        pub monthly_savings: f32,
        pub home_value: f32,
        pub monthly_rent: f32,
        pub mortgage_debt: f32,
        pub mortgage_rate: f32,
        pub monthly_mortgage_payment: f32,
        pub min_baseline_retirement_income: f32,
        pub max_baseline_retirement_income: f32,
        pub interest_rates: Vec<f32>,
        pub inflation_rates: Vec<f32>,
        pub home_savings: Vec<f32>,
        pub rental_savings: Vec<f32>,
        pub compare_home_ownership: bool,
        pub recalculate_interest: bool,
        pub recalculate_inflation: bool,
    }

    impl Saver {

        pub fn get_rates(&self) -> (Vec<f32>, Vec<f32>){
            return (self.interest_rates.clone(), self.inflation_rates.clone())
        }

        pub fn monthly_mortgage_payment(&mut self) -> f32 {
            if self.mortgage_debt <= 0.0 {
                0.0
            } else {
                let mut interest_payment = 0.0;

                for _ in 0..12 {
                    self.mortgage_debt -= self.monthly_mortgage_payment;
                    interest_payment += self.mortgage_debt * (self.mortgage_rate / 12.0);
                }

                interest_payment
            }
        }

        pub fn calculate_annual_expenses(&mut self) -> f32 {
            let interest = self.monthly_mortgage_payment();
            let total_expenses =
                self.home_value * PROPERTY_TAX + self.home_value * HOME_EXPENSES + interest;
            total_expenses
        }

        pub fn calculate_annual_expenses_rent(&mut self) -> f32 {
            let total_expenses = self.monthly_rent * 12.0;
            self.monthly_rent = self.monthly_rent * (1.0 + self.inflation_rates[self.current_age as usize]);
            total_expenses
        }

        pub fn get_investible_assets(&self) -> f32 {
            if self.total_savings - (self.home_value + self.mortgage_debt) > 0.0 {
                self.total_savings - (self.home_value + self.mortgage_debt)
            } else {
                0.0
            }
        }

        // Interest Rates - Inflation
        pub fn calculate_annual_earnings(&self, interest_rate: f32) -> f32 {   
            let investible_assets = self.get_investible_assets();
            let earnings = investible_assets * interest_rate;
            earnings
        }

            // Interest Rates - Inflation
        pub fn calculate_annual_earnings_rent(&self, interest_rate: f32) -> f32 {   
            self.total_savings * interest_rate
        }

        // monthly savings are annualized, post retirement you only withdrawal
        pub fn calculate_annual_savings(&mut self, infl_rate: f32, age: u8) -> f32 {

            let annual_savings: f32;
            if age >= self.retirement_age {
                
                if self.min_baseline_retirement_income < self.total_savings * WITHDRAWAL_RATE / 12.0
                    && self.max_baseline_retirement_income > self.total_savings * WITHDRAWAL_RATE / 12.0
                {
                    self.monthly_savings = -1.0 * (self.total_savings * WITHDRAWAL_RATE / 12.0)

                } 

                else if self.min_baseline_retirement_income > self.total_savings * WITHDRAWAL_RATE / 12.0
                {
                    self.monthly_savings = -1.0 * self.min_baseline_retirement_income;
                } 

                else 
                {
                    self.monthly_savings = -1.0 * self.max_baseline_retirement_income;
                }

                annual_savings = self.monthly_savings * 12.0;
                self.monthly_savings = self.monthly_savings * (1.0 + infl_rate);

            } else {
                annual_savings = self.monthly_savings * 12.0;
                self.monthly_savings = self.monthly_savings * (1.0 - infl_rate);
            }
            
            annual_savings
        }

        pub fn apply_annual_changes(&mut self) {

            if self.recalculate_inflation {
                self.inflation_rates = Inflation::default().rates;
                self.recalculate_inflation = false;
            }

            if self.recalculate_interest {
                self.interest_rates = Interest::default().rates;
                self.recalculate_interest = false;
            }

            if self.mortgage_debt != 0.0 {
                self.monthly_mortgage_payment = self.mortgage_debt
                * ((self.mortgage_rate / 12.0) * (1.0 + (self.mortgage_rate / 12.0)).powf(MORTGAGE_MONTHS))
                / ((1.0 + (self.mortgage_rate / 12.0)).powf(MORTGAGE_MONTHS) - 1.0);
            }

            for age in 0..self.inflation_rates.len() {
                if age < self.current_age as usize {
                    self.home_savings[age] = 0.0;
                } else {
                    match self.total_savings 
                    - self.calculate_annual_expenses()
                    + self.calculate_annual_savings(self.inflation_rates[age as usize], age as u8)
                    + self.calculate_annual_earnings(self.interest_rates[age as usize])
                    
                    {
                        num if num >= 0.0 => {
                            self.home_savings[age] = num;
                            self.total_savings = num;
                        },
                        _ => {
                            self.home_savings[age] = 0.0;
                        },
                    };
                }
            }
        }

        pub fn apply_annual_changes_rent(&mut self) {

            if self.recalculate_inflation {
                self.inflation_rates = Inflation::default().rates;
                self.recalculate_inflation = false;
            }

            if self.recalculate_interest {
                self.interest_rates = Interest::default().rates;
                self.recalculate_interest = false;
            }

            for age in 0..self.inflation_rates.len() {
                if age < self.current_age as usize {
                    self.rental_savings[age] = 0.0;
                } else {
                    match self.total_savings 
                    - self.calculate_annual_expenses_rent()
                    + self.calculate_annual_savings(self.inflation_rates[age as usize], age as u8)
                    + self.calculate_annual_earnings_rent(self.interest_rates[age as usize])
                    
                    {
                        num if num >= 0.0 => {
                            self.rental_savings[age] = num;
                            self.total_savings = num;
                        },
                        _ => {
                            self.rental_savings[age] = 0.0;
                        },
                    };
                }
            }
        }

    }
}