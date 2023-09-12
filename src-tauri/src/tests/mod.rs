#[cfg(test)]
mod test_owner {
    use crate::user::owner::Owner;

    #[test]
    fn home_test() {}

    #[test]
    fn rent_test() {}

    #[test]
    fn make_mortgage_payment_simple() {
        let mut o = Owner::default();
        o.mortgage_debt = 10.0;
        o.monthly_mortgage_payment = Some(1.0);
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
            monthly_mortgage_payment: Some(12.0),
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
        let mut o = Owner::default();
        o.total_savings = 100.0;
        o.monthly_income = 10.0;
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
            monthly_mortgage_payment: None,
            min_baseline_retirement_income: 0.0,
            max_baseline_retirement_income: 0.0,
            interest_rates: vec![0.0; 100],
            inflation_rates: vec![0.0; 100],
            home_savings: vec![0.0; 100],
            rental_savings: vec![0.0; 100],
            active_retirement: false,
            home_owned_age: None,
        };
        let home_expenses = (0.01 * o.home_value) / 12.0 + (0.01 * o.home_value) / 12.0;
        assert_eq!(o.apply_monthly_changes(), 110.0 - home_expenses);
    }

    #[test]
    fn mortgage_installments() {
        let mut o = Owner::default();
        o.mortgage_debt = 12.0;
        o.mortgage_term = 1;
        o.mortgage_rate = 0.0001;

        assert!(o.mortgage_installments() > 0.98);
    }

    #[test]
    fn one_year_simple() {
        let mut o = Owner::default();
        o.current_age = 1;
        o.retirement_age = 100;
        o.mortgage_debt = 12.0;
        o.mortgage_term = 1;
        o.mortgage_rate = 0.0;
        o.monthly_income = 1.0;
        o.monthly_mortgage_payment = Some(o.mortgage_installments());
        o.apply_annual_changes();

        let mut j = Owner::default();
        j.current_age = 2;
        j.retirement_age = 100;
        j.mortgage_debt = 0.0;
        j.mortgage_term = 1;
        j.mortgage_rate = 0.0;
        j.monthly_income = 1.0;
        j.total_savings = 12.0;
        j.monthly_mortgage_payment = o.monthly_mortgage_payment;
        assert_eq!(o.total_savings, j.total_savings);
    }
}
