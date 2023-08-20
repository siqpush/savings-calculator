pub const PROPERTY_TAX: f32 = 0.01;
pub const HOME_EXPENSES: f32 = 0.01;
pub const WITHDRAWAL_RATE: f32 = 0.04;
pub const MORTGAGE_MONTHS: f32 = 30.0 * 12.0;
pub const MORTGAGE_RATE_MONTHLY: f32 = 0.02 / 12.0;


pub struct Saver {
    pub current_age: u8,
    pub retirement_age: u8,
    pub total_savings: f32,
    pub monthly_savings: f32,
    pub home_value: f32,
    pub mortgage_debt: f32,
    pub monthly_mortgage_payment: f32,
    pub min_baseline_retirement_income: f32,
    pub max_baseline_retirement_income: f32,
}

impl Saver {
    pub fn new(
        current_age: u8,
        retirement_age: u8,
        monthly_savings: f32,
        home_value: f32,
        total_savings: f32,
        mortgage_debt: f32,
        min_baseline_retirement_income: f32,
        max_baseline_retirement_income: f32,
    ) -> Saver {
        Saver {
            current_age,
            retirement_age,
            total_savings,
            monthly_savings,
            home_value,
            mortgage_debt: mortgage_debt,
            monthly_mortgage_payment: mortgage_debt
                * (MORTGAGE_RATE_MONTHLY * (1.0 + MORTGAGE_RATE_MONTHLY).powf(MORTGAGE_MONTHS))
                / ((1.0 + MORTGAGE_RATE_MONTHLY).powf(MORTGAGE_MONTHS) - 1.0),
            min_baseline_retirement_income,
            max_baseline_retirement_income,
        }
    }

    pub fn monthly_mortgage_payment(&mut self) -> f32 {
        if self.mortgage_debt <= 0.0 {
            0.0
        } else {
            let mut interest_payment = 0.0;

            for _ in 0..12 {
                self.mortgage_debt -= self.monthly_mortgage_payment;
                interest_payment += self.mortgage_debt * MORTGAGE_RATE_MONTHLY;
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

    pub fn get_investible_assets(&self) -> f32 {
        if self.total_savings - (self.home_value + self.mortgage_debt) > 0.0 {
            self.total_savings - (self.home_value + self.mortgage_debt)
        } else {
            0.0
        }
    }

    // Interest Rates - Inflation
    pub fn calculate_annual_earnings(&self, interest_rate: f32, inflation_rate: f32) -> f32 {   
        let investible_assets = self.get_investible_assets();
        let earnings = investible_assets * interest_rate - investible_assets * inflation_rate;
        earnings
    }

    // monthly savings are annualized, post retirement you only withdrawal
    pub fn calculate_annual_savings(&mut self, infl_rate: f32, age: u8) -> f32 {

        let annual_savings: f32;
        if age >= self.retirement_age {
            if self.min_baseline_retirement_income < self.total_savings * WITHDRAWAL_RATE / 12.0
                && self.max_baseline_retirement_income > self.total_savings * WITHDRAWAL_RATE / 12.0
            {
                self.monthly_savings = -1.0 * (self.total_savings * WITHDRAWAL_RATE / 12.0)
            } else if self.min_baseline_retirement_income
                > self.total_savings * WITHDRAWAL_RATE / 12.0
            {
                self.monthly_savings = -1.0 * self.min_baseline_retirement_income;
            } else {
                self.monthly_savings = -1.0 * self.max_baseline_retirement_income;
            }
            annual_savings = self.monthly_savings * 12.0;
        } else {
            annual_savings = self.monthly_savings * 12.0;
        }
        self.monthly_savings = self.monthly_savings * (1.0 + infl_rate);
        self.min_baseline_retirement_income = self.min_baseline_retirement_income * (1.0 + infl_rate);
        self.max_baseline_retirement_income = self.max_baseline_retirement_income * (1.0 + infl_rate);
        annual_savings
    }

    pub fn apply_annual_changes(&mut self, infl_rates: &Vec<f32>, interest_rates: &Vec<f32>) -> Vec<f32> {
        let mut display_savings: Vec<f32> = Vec::new();
        for age in 0..interest_rates.len() {
            if age < self.current_age as usize {
                display_savings.push(0.0);
            } else {
                match &self.total_savings 
                + self.calculate_annual_earnings(interest_rates[age as usize], infl_rates[age as usize])
                - self.calculate_annual_expenses()
                + self.calculate_annual_savings(infl_rates[age as usize], age as u8) 
                {
                     num if num >= 0.0 => {
                        display_savings.push(num);
                        self.total_savings = num;
                    },
                     _ => {
                        display_savings.push(0.0)
                    },
                };
            }
        }
        display_savings
    }
}