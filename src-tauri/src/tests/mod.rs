#[cfg(test)]
mod test_owner {
    use crate::user::owner::Owner;
    use crate::user::saver::Saver;
    use crate::user::saver::SaverType;

    #[test]
    fn monthly_mortgage_payment() {
        let mut o = Saver::default();
        o.mortgage_rate = 0.01;
        o.mortgage_debt = 120.0;
        o.mortgage_term = 1;
        o.cached_mortgage_installment = Some(o.mortgage_installments());
        dbg!(o.cached_mortgage_installment);
        // assert_eq!(o.monthly_mortgage_interest_payment(), 12.0);

        let mut j = Saver::default();
        j.mortgage_rate = 0.02;
        j.mortgage_debt = 120.0;
        j.mortgage_term = 1;
        j.cached_mortgage_installment = Some(j.mortgage_installments());
        dbg!(j.cached_mortgage_installment);
        assert_eq!(j.monthly_mortgage_interest_payment(), o.monthly_mortgage_interest_payment());
    }

    #[test]
    fn one_year_simple() {
        let mut o = Saver::default();
        o.current_age = 1;
        o.retirement_age = 100;
        o.mortgage_debt = 1000000.0;
        o.home_value = 1000000.0;
        o.mortgage_term = 30;
        o.mortgage_rate = 0.1;
        o.monthly_income = 10000.0;
        o.total_savings = 1200000.0;
        o.cached_mortgage_installment = Some(o.mortgage_installments());
        dbg!("{:?}",o.cached_mortgage_installment);
        o.apply_annual_changes(&SaverType::HomeOwner);

        let mut j = Saver::default();
        j.current_age = 1;
        j.retirement_age = 100;
        j.mortgage_debt = 1000000.0;
        j.home_value = 1000000.0;
        j.mortgage_term = 30;
        j.mortgage_rate = 0.1;
        j.monthly_income = 10000.0;
        j.total_savings = 1200000.0;
        j.cached_mortgage_installment = Some(j.mortgage_installments());
        dbg!("{:?}",j.cached_mortgage_installment);
        j.apply_annual_changes(&SaverType::HomeOwner);

        assert_eq!(o.total_savings, j.total_savings);
    }

    #[test]
    fn home_owned_age_simple(){
        let mut o = Saver::default();
        o.current_age = 30;
        o.mortgage_debt = 10000.0;
        o.total_savings = 100000.0;
        o.home_value = 100000.0;
        o.mortgage_term = 30;
        o.mortgage_rate = 0.05;
        o.cached_mortgage_installment = Some(o.mortgage_installments());
        dbg!(o.cached_mortgage_installment);
        //assert_eq!(o.cached_mortgage_installment.unwrap() * (o.mortgage_term_months() + 12.0), o.mortgage_debt);
        o.calculate_savings(SaverType::HomeOwner);

        assert_eq!(o.home_owned_age, Some(61));
    }

}
