pub mod rates {
    use rand::{thread_rng, Rng};

    pub struct Interest {
        pub rates: Vec<f32>,
    }
    impl Default for Interest {
        fn default() -> Self {
            let mut rng = thread_rng();
            let mut values: Vec<f32> = vec![0.0; 100];
            for i in 0..100 {
                match i {
                    0..=29 => {
                        values[i] = rng.gen_range(-0.15..0.2) + 0.07;
                    }
                    30..=49 => {
                        values[i] = rng.gen_range(-0.10..0.15) + 0.05;
                    }
                    50..=64 => {
                        values[i] = rng.gen_range(-0.075..0.10) + 0.035;
                    }
                    _ => {
                        values[i] = rng.gen_range(-0.0375..0.075) + 0.017675;
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
                values[i] = rng.gen_range(-0.02..0.06) / rng.gen_range(1.0..2.0);
            }
            Self { rates: values }
        }
    }
}

pub mod saver {
    use serde::{Deserialize, Serialize};

    use super::rates::{Interest, Inflation};

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
        pub monthly_mortgage_payment: Option<f32>,
        pub min_baseline_retirement_income: f32,
        pub max_baseline_retirement_income: f32,
        pub interest_rates: Vec<f32>,
        pub inflation_rates: Vec<f32>,
        pub home_savings: Vec<f32>,
        pub rental_savings: Vec<f32>,
        pub active_retirement: bool,
        pub home_owned_age: Option<u8>,
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
                monthly_mortgage_payment: None,
                min_baseline_retirement_income: 0.0,
                max_baseline_retirement_income: 0.0,
                interest_rates: vec![0.0; 100],
                inflation_rates: vec![0.0; 100],
                home_savings: vec![0.0; 100],
                rental_savings: vec![0.0; 100],
                active_retirement: false,
                home_owned_age: None,
            }
        }
    }

    impl Saver {
        pub fn monthly_inflation(&self) -> f32 {
            self.inflation_rates[self.current_age as usize] / 12.0
        }
        pub fn monthly_interest(&self) -> f32 {
            self.interest_rates[self.current_age as usize] / 12.0
        }

        pub fn reset_inflation(&mut self, recalculate: bool) {
            if recalculate {
                self.inflation_rates = Inflation::default().rates;
            }
        }

        pub fn reset_interest(&mut self, recalculate: bool) {
            if recalculate {
                self.interest_rates = Interest::default().rates;
            }
        }
    }

    pub trait Owner<Saver>{
        fn monthly_mortgage_rate(&self) -> f32;
        fn mortgage_term_months(&self) -> f32;
        fn mortgage_installments(&self) -> f32;
        fn monthly_mortgage_interest_payment(&self) -> f32;
        fn make_mortgage_payment(&mut self);
        fn calculate_expenses(&mut self) -> f32;
        fn interest_earnings(&self) -> f32;
        fn monthly_withdrawal_rate(&self) -> f32;
        fn income(&mut self) -> f32;
        fn apply_monthly_changes(&mut self) -> f32;
        fn apply_annual_changes(&mut self);
        fn calculate_savings(&mut self) -> Vec<f32>;
    }

    pub trait Renter<Saver>{
        fn expenses(&mut self) -> f32;
        fn monthly_withdrawal_rate(&self) -> f32;
        fn calculate_earnings(&self) -> f32;
        fn income(&mut self) -> f32;
        fn apply_monthly_changes(&mut self) -> f32;
        fn apply_annual_changes(&mut self);
        fn calculate_savings(&mut self) -> Vec<f32>;
    }
}
pub mod renter {

    use super::saver::{Saver, Renter};
    pub const STD_WITHDRAWAL_RATE: f32 = 0.04;

    impl Renter<Saver> for super::saver::Saver {

        fn expenses(&mut self) -> f32 {
            let total_expenses = self.monthly_rent + self.monthly_expenses;
            self.monthly_expenses *= 1.0 + self.monthly_inflation();
            self.monthly_rent *= 1.0 + self.monthly_inflation();
            total_expenses
        }

        fn monthly_withdrawal_rate(&self) -> f32 {
            if self.min_baseline_retirement_income < self.total_savings * STD_WITHDRAWAL_RATE
                && self.max_baseline_retirement_income > self.total_savings * STD_WITHDRAWAL_RATE
            {
                STD_WITHDRAWAL_RATE / 12.0
            } else if self.min_baseline_retirement_income > self.total_savings * STD_WITHDRAWAL_RATE
            {
                self.min_baseline_retirement_income / self.total_savings / 12.0
            } else {
                self.max_baseline_retirement_income / self.total_savings / 12.0
            }
        }

        fn calculate_earnings(&self) -> f32 {
            self.total_savings * self.monthly_interest()
        }

        // end of month income for a renter (no mortgage)
        fn income(&mut self) -> f32 {
            let income;
            if self.active_retirement {
                income =
                    self.monthly_withdrawal_rate() * self.total_savings + self.calculate_earnings();
            } else {
                income = self.monthly_income + self.calculate_earnings();
                self.monthly_income *= 1.0 + self.monthly_inflation();
            }
            income
        }

        fn apply_monthly_changes(&mut self) -> f32 {
            self.total_savings + self.income() - self.expenses()
        }

        fn apply_annual_changes(&mut self) {
            for _ in 0..12 {
                self.total_savings = self.apply_monthly_changes();
            }
            self.rental_savings[self.current_age as usize] = self.total_savings;
        }

        fn calculate_savings(&mut self) -> Vec<f32> {
            //initualize empty
            self.rental_savings = vec![0.0; (self.current_age - 1) as usize];
            while self.current_age < 100 || self.total_savings <= 0.0 {
                self.active_retirement = self.current_age >= self.retirement_age;
                self.apply_annual_changes();
                self.current_age += 1;
            }
            self.rental_savings.clone()
        }
    }
}

pub mod owner {
    use super::saver::{Owner, Saver};
    pub const PROPERTY_TAX: f32 = 0.01 / 12.0;
    pub const HOME_EXPENSES: f32 = 0.01 / 12.0;
    pub const STD_WITHDRAWAL_RATE: f32 = 0.04;

    impl Owner<Saver> for super::saver::Saver {
        fn monthly_mortgage_rate(&self) -> f32 {
            self.mortgage_rate / 12.0
        }
        fn mortgage_term_months(&self) -> f32 {
            self.mortgage_term as f32 * 12.0
        }
        fn mortgage_installments(&self) -> f32 {
            if self.mortgage_rate == 0.0 {
                self.mortgage_debt / self.mortgage_term_months()
            } else {
                self.mortgage_debt
                    * (self.monthly_mortgage_rate()
                        * (1.0 + self.monthly_mortgage_rate()).powf(self.mortgage_term_months()))
                    / ((1.0 + self.monthly_mortgage_rate()).powf(self.mortgage_term_months()) - 1.0)
            }
        }
        // subtract monthly mortgage payment from mortgage debt and add monthly interest payment
        fn monthly_mortgage_interest_payment(&self) -> f32 {
            self.monthly_mortgage_payment.unwrap_or(0.0) * self.monthly_mortgage_rate()
        }
        fn make_mortgage_payment(&mut self) {
            if self.mortgage_debt < self.monthly_mortgage_payment.unwrap_or(0.0) {
                self.monthly_mortgage_payment = Some(self.mortgage_debt);
            }
            self.mortgage_debt -= self.monthly_mortgage_payment.unwrap();
            if self.mortgage_debt <= 0.0 {
                if self.home_owned_age.is_none() {
                    self.home_owned_age = Some(self.current_age);
                }
                self.mortgage_debt = 0.0;
            }
        }
        // calculate monthly expenses for a homeowner (mortgage, property tax, home expenses)
        fn calculate_expenses(&mut self) -> f32 {
            self.home_value * PROPERTY_TAX
                + self.home_value * HOME_EXPENSES
                + self.monthly_mortgage_interest_payment()
        }
        // calculate monthly interest earnings
        fn interest_earnings(&self) -> f32 {
            (self.total_savings - (self.home_value - self.mortgage_debt)) * self.monthly_interest()
        }
        // calculate monthly withdrawal rate (4% of total savings or min/max baseline retirement income)
        fn monthly_withdrawal_rate(&self) -> f32 {
            if self.min_baseline_retirement_income < self.total_savings * STD_WITHDRAWAL_RATE
                && self.max_baseline_retirement_income > self.total_savings * STD_WITHDRAWAL_RATE
            {
                STD_WITHDRAWAL_RATE / 12.0
            } else if self.min_baseline_retirement_income > self.total_savings * STD_WITHDRAWAL_RATE
            {
                self.min_baseline_retirement_income / self.total_savings / 12.0
            } else {
                self.max_baseline_retirement_income / self.total_savings / 12.0
            }
        }
        // your income is your monthly withdrawal rate (0 if not in retirement) times your total savings plus your earnings
        fn income(&mut self) -> f32 {
            if self.active_retirement {
                self.monthly_withdrawal_rate() * self.total_savings + self.interest_earnings()
            } else {
                self.monthly_income + self.interest_earnings()
            }
        }
        fn apply_monthly_changes(&mut self) -> f32 {
            let month_end = self.total_savings + self.income() - self.calculate_expenses();
            self.monthly_income = self.monthly_income * (1.0 + self.monthly_inflation());
            self.make_mortgage_payment();
            return month_end;
        }
        fn apply_annual_changes(&mut self) {
            for _ in 0..12 {
                self.total_savings = self.apply_monthly_changes();
            }
            self.home_savings[self.current_age as usize] = self.total_savings;
        }
        fn calculate_savings(&mut self) -> Vec<f32> {
            //initualize empty
            self.rental_savings = vec![0.0; (self.current_age - 1) as usize];
            if self.monthly_mortgage_payment.is_none() {
                self.monthly_mortgage_payment = Some(self.mortgage_installments());
            }
            while self.current_age < 100 || self.total_savings <= 0.0 {
                self.active_retirement = self.current_age >= self.retirement_age;
                self.apply_annual_changes();
                self.current_age += 1;
            }
            self.home_savings.clone()
        }
    }
}
