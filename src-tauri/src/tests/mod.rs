#[cfg(test)]
mod test_owner {
    use crate::user::owner::Owner;

    #[test]
    fn home_test() {}

    #[test]
    fn rent_test() {}

    #[test]
    fn make_mortgage_payment_simple() {
        let mut o = Owner {
            current_age: 0,
            retirement_age: 0,
            total_savings: 0.0,
            monthly_income: 0.0,
            monthly_expenses: 0.0,
            home_value: 0.0,
            monthly_rent: 0.0,
            mortgage_debt: 10.0,
            mortgage_rate: 0.0,
            mortgage_term: 1,
            monthly_mortgage_payment: 1.0,
            min_baseline_retirement_income: 0.0,
            max_baseline_retirement_income: 0.0,
            interest_rates: vec![0.0; 100],
            inflation_rates: vec![0.0; 100],
            home_savings: vec![],
            rental_savings: vec![],
            active_retirement: false,
            home_owned_age: None,
        };
        o.make_mortgage_payment();
        println!("mortgage debt: {}", o.mortgage_debt);
        assert_eq!(o.mortgage_debt, 9.0);
    }

    #[test]
    fn make_mortgage_payment_negative() {
        let mut o = Owner {
            current_age: 0,
            retirement_age: 0,
            total_savings: 0.0,
            monthly_income: 0.0,
            monthly_expenses: 0.0,
            home_value: 0.0,
            monthly_rent: 0.0,
            mortgage_debt: 10.0,
            mortgage_rate: 0.0,
            mortgage_term: 1,
            monthly_mortgage_payment: 12.0,
            min_baseline_retirement_income: 0.0,
            max_baseline_retirement_income: 0.0,
            interest_rates: vec![0.0; 100],
            inflation_rates: vec![0.0; 100],
            home_savings: vec![],
            rental_savings: vec![],
            active_retirement: false,
            home_owned_age: None,
        };
        o.make_mortgage_payment();
        println!("mortgage debt: {}", o.mortgage_debt);
        assert_eq!(o.mortgage_debt, 0.0);
    }

    #[test]
    fn apply_monthly_changes_simple() {
        let mut o = Owner {
            current_age: 0,
            retirement_age: 0,
            total_savings: 100.0,
            monthly_income: 10.0,
            monthly_expenses: 0.0,
            home_value: 0.0,
            monthly_rent: 0.0,
            mortgage_debt: 0.0,
            mortgage_rate: 0.0,
            mortgage_term: 0,
            monthly_mortgage_payment: 0.0,
            min_baseline_retirement_income: 0.0,
            max_baseline_retirement_income: 0.0,
            interest_rates: vec![0.0; 100],
            inflation_rates: vec![0.0; 100],
            home_savings: vec![],
            rental_savings: vec![],
            active_retirement: false,
            home_owned_age: None,
        };
        assert_eq!(o.apply_monthly_changes(), 110.0);
    }

    #[test]
    fn apply_monthly_changes_w_home() {
        let mut o = Owner {
            current_age: 0,
            retirement_age: 0,
            total_savings: 100.0,
            monthly_income: 10.0,
            monthly_expenses: 0.0,
            home_value: 100.0,
            monthly_rent: 0.0,
            mortgage_debt: 0.0,
            mortgage_rate: 0.0,
            mortgage_term: 0,
            monthly_mortgage_payment: 0.0,
            min_baseline_retirement_income: 0.0,
            max_baseline_retirement_income: 0.0,
            interest_rates: vec![0.0; 100],
            inflation_rates: vec![0.0; 100],
            home_savings: vec![],
            rental_savings: vec![],
            active_retirement: false,
            home_owned_age: None,
        };
        let home_expenses = (0.01 * o.home_value) / 12.0 + (0.01 * o.home_value) / 12.0;
        assert_eq!(o.apply_monthly_changes(), 110.0 - home_expenses);
    }
}
