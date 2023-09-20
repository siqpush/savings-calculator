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
  mortgageDebt: number; 
  minBaselineRetirementIncome: number;
  maxBaselineRetirementIncome: number;
  mortgageRate: number;
  mortgageTerm: number;
  activeRetirement: boolean; //these are rust specific fields
  inflationRates: number[];
  interestRates: number[];
  rentalSavings: number[];
  homeSavings: number[];
  homeOwnedAge?: number | null;
  cachedMortgageInstallment?: number | null;
  ymax: number;
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