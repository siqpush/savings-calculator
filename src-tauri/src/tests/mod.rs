#[cfg(test)]
mod test_owner {
    use crate::user::saver::{Saver, Owner, Renter};
    

    #[test]
    fn home_test() {}

    #[test]
    fn rent_test() {}

    #[test]
    fn make_mortgage_payment_simple() {
        let mut o = Saver::default();
        Owner::apply_annual_changes(&mut o);
        o.mortgage_debt = 10.0;
        o.monthly_mortgage_payment = Some(1.0);
        o.make_mortgage_payment();
        println!("mortgage debt: {}", o.mortgage_debt);
        assert_eq!(o.mortgage_debt, 9.0);
    }


    #[test]
    fn apply_monthly_changes_simple() {
        let mut o = Saver::default();
        o.total_savings = 100.0;
        o.monthly_income = 10.0;
        assert_eq!(Owner::apply_monthly_changes(&mut o), 110.0);
    }
    #[test]
    fn mortgage_installments() {
        let mut o = Saver::default();
        o.mortgage_debt = 12.0;
        o.mortgage_term = 1;
        o.mortgage_rate = 0.0001;

        assert!(o.mortgage_installments() > 0.98);
    }

    #[test]
    fn one_year_simple() {
        let mut o = Saver::default();
        o.current_age = 1;
        o.retirement_age = 100;
        o.mortgage_debt = 12.0;
        o.mortgage_term = 1;
        o.mortgage_rate = 0.0;
        o.monthly_income = 1.0;
        o.monthly_mortgage_payment = Some(o.mortgage_installments());
        Owner::apply_annual_changes(&mut o);

        let mut j = Saver::default();
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
