pub mod rates {
    use rand::{thread_rng, Rng};

    pub struct Interest {
        pub rates: Vec<f32>,
    }
    impl Interest {
        pub fn new(inflation: Inflation) -> Self {
            let mut rng = thread_rng();
            let mut values: Vec<f32> = vec![0.0; 100];
            for i in 0..100 {
                match i {
                    0..=29 => {
                        values[i] = rng
                            .gen_range((-0.075 + inflation.rates[i]/2.0)..(0.15 + inflation.rates[i]/2.0));
                    }
                    30..=49 => {
                        values[i] = rng
                            .gen_range((-0.05 + inflation.rates[i]/2.0)..(0.125 + inflation.rates[i]/2.0));
                    }
                    50..=64 => {
                        values[i] = rng
                            .gen_range((-0.025 + inflation.rates[i]/2.0)..(0.10 + inflation.rates[i]/2.0));
                    }
                    _ => {
                        values[i] = rng
                            .gen_range((-0.01 + inflation.rates[i]/2.0)..(0.075 + inflation.rates[i]/2.0));
                    }
                }
            }
            Self { rates: values }
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
                values[i] = rng.gen_range(-0.005..0.04) + rng.gen_range(0.0..0.01)
            }
            Self { rates: values }
        }
    }
}

pub mod saver {
    use serde::{Deserialize, Serialize};
    use super::{rates::{Inflation, Interest}, owner::{self, Owner}};
    pub const STD_MONTHLY_WITHDRAWAL_RATE: f32 = 0.04 / 12.0;

    pub enum SaverType {
        HomeOwner,
        Renter,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Saver {
        pub current_age: u8,
        pub retirement_age: u8,
        pub total_savings: f32,
        pub monthly_income: f32,
        pub monthly_expenses: f32,
        pub home_value: f32,
        pub monthly_rent: f32,
        pub mortgage_debt: f32,
        pub mortgage_rate: f32,
        pub mortgage_term: u8,
        pub min_baseline_retirement_income: f32,
        pub max_baseline_retirement_income: f32,
        pub interest_rates: Vec<f32>,
        pub inflation_rates: Vec<f32>,
        pub home_savings: Vec<f32>,
        pub rental_savings: Vec<f32>,
        pub active_retirement: bool,
        pub home_owned_age: Option<u8>,
        pub cached_mortgage_installment: Option<f32>,
        pub home_expenses: f32,
        pub ymax: f32,
    }

    impl Default for Saver {
        fn default() -> Self {
            Saver {
                current_age: 0,
                retirement_age: 0,
                total_savings: 0.0,
                monthly_income: 0.0,
                monthly_expenses: 0.0,
                home_value: 0.0,
                monthly_rent: 0.0,
                mortgage_debt: 0.0,
                mortgage_rate: 0.0,
                mortgage_term: 0,
                min_baseline_retirement_income: 0.0,
                max_baseline_retirement_income: 0.0,
                interest_rates: vec![0.0; 100],
                inflation_rates: vec![0.0; 100],
                home_savings: vec![0.0; 100],
                rental_savings: vec![0.0; 100],
                active_retirement: false,
                home_owned_age: None,
                cached_mortgage_installment: None,
                home_expenses: 0.01,
                ymax: 500000.0,
            }
        }
    }

    impl Saver {
        // monthly inflation rate
        pub fn monthly_inflation(&self) -> f32 {
            self.inflation_rates[self.current_age as usize] / 12.0
        }
        // monthly interest rate
        pub fn monthly_interest(&self) -> f32 {
            self.interest_rates[self.current_age as usize] / 12.0
        }
        // reset rates to default or recalculate
        pub fn reset_rates(&mut self, recalculate: bool) {
            if recalculate {
                self.inflation_rates = Inflation::default().rates;
                self.interest_rates = Interest::new(Inflation::default()).rates;
            }
        }
        // calculate liquid assets (total savings - (home value - mortgage debt))
        pub fn liquid_assets(&self) -> f32 {
            if self.total_savings - (self.home_value - self.mortgage_debt) < 0.0 {
                0.0
            } else {
                self.total_savings - (self.home_value - self.mortgage_debt)
            }
        }
        // result plots y axis
        pub fn get_ymax(&self) -> f32{
            let mut ymax = 0.0;
            self.home_savings.iter().for_each(|x| {
                if *x > ymax {
                    ymax = *x
                }
            });
            self.rental_savings.iter().for_each(|x| {
                if *x > ymax {
                    ymax = *x
                }
            });
            match ymax {
                num if (num >= 100000000.0) => {
                    500000000.0
                },
                num if num >= 50000000.0 => {
                    100000000.0
                },
                num if num >= 25000000.0 => {
                    50000000.0
                },
                num if num >= 10000000.0 => {
                    25000000.0
                },
                num if num >= 10000000.0 => {
                    10000000.0
                },
                num if num >= 7500000.0 => {
                    10000000.0
                },
                num if num >= 1500000.0 => {
                    7500000.0
                },
                num if num >= 500000.0 => {
                    1500000.0
                },
                _ => 500000.0
            }
        }
        // calculate monthly withdrawal rate (4% of total savings or min/max baseline retirement income)
        pub fn monthly_withdrawal(&self) -> f32 {
            if self.liquid_assets() <= 0.0 {
                0.0
            } else {
                if self.min_baseline_retirement_income < self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
                    && self.max_baseline_retirement_income > self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
                {
                    -1.0 * STD_MONTHLY_WITHDRAWAL_RATE * self.liquid_assets()
                // if the min baseline retirement income is greater than the standard monthly withdrawal rate use that (we always need more than the min)
                } else if self.min_baseline_retirement_income > self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
                {
                    -1.0 * (self.min_baseline_retirement_income / (self.liquid_assets() / 12.0))
                // if the max baseline retirement income is less than the standard monthly withdrawal rate use that (we never need more than the max)
                } else if self.max_baseline_retirement_income < self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE {
                    -1.0 * (self.min_baseline_retirement_income / (self.liquid_assets() / 12.0))
                } else {
                    unreachable!("shouldn't be here")
                }
            }
        }
        // calculate monthly interest earnings
        pub fn interest_earnings(&self) -> f32 {
            self.liquid_assets() * self.monthly_interest()
        }        
        // income is monthly income + interest earnings
        pub fn income(&mut self) -> f32 {
            if self.active_retirement {
                self.monthly_expenses = 0.0;
                self.monthly_withdrawal()
            } else {
                self.monthly_income
            }
        }
        // end of month expenses for a renter and owner (if renter -> owner is zeroed out, if owner -> renter is zeroed out)
        pub fn expenses(&mut self) -> f32 {
            self.monthly_expenses + owner::Owner::expenses(self) + self.monthly_rent
        }
        // c
        pub fn apply_monthly_changes(&mut self) -> f32 {
            let mut month_end = self.total_savings + self.income() - self.expenses();
            self.monthly_income *=  1.0 + self.monthly_inflation();
            self.monthly_expenses *= 1.0 + self.monthly_inflation();
            self.monthly_rent *= 1.0 + self.monthly_inflation();
            self.home_expenses *= 1.0 + self.monthly_inflation();
            self.min_baseline_retirement_income *= 1.0 + self.monthly_inflation();
            self.max_baseline_retirement_income *= 1.0 + self.monthly_inflation();
            return month_end;
        }
        // run through months then apply the total savings to show only the end of year savings
        pub fn apply_annual_changes(&mut self, st: &SaverType) {
            for _ in 0..12 {
                // apply interest on the savings from the month prior
                let interest = self.interest_earnings();
                match self.apply_monthly_changes() {
                    // you can not spend continue if you have no more than your home 
                    num if num > self.home_value - self.mortgage_debt => {
                        self.total_savings = num + interest;
                    }
                    _ => {
                        self.total_savings = 0.0;
                        break;
                    }
                } 
            }
            match st {
                SaverType::HomeOwner => {
                    self.home_savings[self.current_age as usize] = self.total_savings;
                },
                SaverType::Renter => {
                    self.rental_savings[self.current_age as usize] = self.total_savings;
                }
            }            
        }
        
        // end of month income adjusted for inflation
        pub fn calculate_savings(&mut self, st: SaverType) -> Vec<f32> {

            match st {
                SaverType::HomeOwner => {
                    self.cached_mortgage_installment = Some(Owner::mortgage_installments(self));
                    self.home_savings.fill(0.0);
                    self.home_owned_age = None;
                    self.home_savings[self.current_age as usize] = self.total_savings;
                },
                SaverType::Renter => {
                    self.rental_savings.fill(0.0);
                    self.rental_savings[self.current_age as usize] = self.total_savings;
                }
            }
            self.current_age += 1;
            while self.current_age < 100 && self.total_savings > 0.0 {
                self.active_retirement = self.current_age >= self.retirement_age;
                self.apply_annual_changes(&st);
                self.current_age += 1;
            }

            match st {
                SaverType::HomeOwner => {
                    self.cached_mortgage_installment = None;
                    self.home_savings.clone()
                },
                SaverType::Renter => {
                    self.rental_savings.clone()
                }
            }
        }
    }
}
pub mod owner {

    use super::saver::Saver;

    pub const PROPERTY_TAX: f32 = 0.01 / 12.0;

    pub trait Owner<Saver> {
        fn monthly_mortgage_rate(&self) -> f32;
        fn mortgage_term_months(&self) -> f32;
        fn mortgage_installments(&self) -> f32;
        fn monthly_mortgage_interest_payment(&self) -> f32;
        fn monthly_home_expenses(&self) -> f32;
        fn expenses(&mut self) -> f32;
    }

    impl Owner<Saver> for super::saver::Saver {
        // calculate monthly mortgage rate
        fn monthly_mortgage_rate(&self) -> f32 {
            self.mortgage_rate / 12.0
        }
        // calculate mortgage term in months
        fn mortgage_term_months(&self) -> f32 {
            self.mortgage_term as f32 * 12.0
        }
        // calculate monthly mortgage payment
        fn mortgage_installments(&self) -> f32 {
            if self.monthly_mortgage_rate() == 0.0 {
                self.mortgage_debt / self.mortgage_term_months()
            } else {
                self.mortgage_debt * ((
                    self.monthly_mortgage_rate() * (1.0 + self.monthly_mortgage_rate()).powi(self.mortgage_term_months() as i32)
                ) / (
                    (1.0 + self.monthly_mortgage_rate()).powi(self.mortgage_term_months() as i32) - 1.0
                ))
            }
        }
        // subtract monthly mortgage payment from mortgage debt and add monthly interest payment
        fn monthly_mortgage_interest_payment(&self) -> f32 {
            self.mortgage_debt * self.monthly_mortgage_rate()
        }

        fn monthly_home_expenses(&self) -> f32 {
            self.home_value * (self.home_expenses / 12.0)
        }

        // calculate monthly expenses for a homeowner (mortgage, property tax, home expenses) + other
        fn expenses(&mut self) -> f32 {
            let mortgage_interest = self.monthly_mortgage_interest_payment();
            let monthly_principle = self.cached_mortgage_installment.unwrap_or(0.0) - mortgage_interest;
            let monthly_expenses = self.home_value * PROPERTY_TAX
                + self.monthly_home_expenses()
                + mortgage_interest;
            // make a mortgage payment if you have a mortgage
            if self.mortgage_debt > 0.0 {
                self.mortgage_debt -= monthly_principle;
            } else {
                if self.home_owned_age.is_none() {
                    self.home_owned_age = Some(self.current_age);
                }
            }
            monthly_expenses
        }
    }
}
