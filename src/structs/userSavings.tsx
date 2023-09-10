export interface ZeroDistributionsType {
  age: number[];
  count: number[];
  avg: number;
  stdv: number;
}

export interface UserSavingsType {
  currentAge: number;
  retirementAge: number;
  totalSavings: number;
  monthlyIncome: number;
  monthlyExpenses: number;
  monthlyRent: number;
  homeValue: number;
  mortgageOutstanding: number; //these are rust specific fields
  mortgageDebt: number; 
  monthlyMortgagePayment: number; //these are rust specific fields
  minBaselineRetirementIncome: number;
  maxBaselineRetirementIncome: number;
  mortgageRate: number;
  mortgageTerm: number;
  activeRetirement: boolean; //these are rust specific fields
  inflationRates: number[];
  interestRates: number[];
  rentalSavings: number[];
  rentalAnnualNet: number[];
  homeSavings: number[];
  homeAnnualNet: number[];
}

export const updateUserSavings = <K extends keyof UserSavingsType>(
  prevState: UserSavingsType,
  property: K,
  value: UserSavingsType[K]
): UserSavingsType => {
  return {
    ...prevState,
    [property]: value
  };
};