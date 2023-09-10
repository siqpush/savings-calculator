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
  monthlySavings: number;
  monthlyRent: number;
  homeValue: number;
  mortgageOutstanding: number;
  mortgageDebt: number; //these are rust specific fields
  monthlyMortgagePayment: number; //these are rust specific fields
  minBaselineRetirementIncome: number;
  maxBaselineRetirementIncome: number;
  mortgageRate: number;
  compareHomeOwnership: boolean;
  recalculateInterest: boolean;
  recalculateInflation: boolean;
  inflationRates: number[];
  interestRates: number[];
  rentalSavings: number[];
  homeSavings: number[];
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