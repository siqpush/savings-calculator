import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { PlotData } from "./components/plots/plot";
import { Input } from "./components/input";
import { BarPlotData } from "./components/plots/barplot";
import {ZeroDistributionsType, UserSavingsType, updateUserSavings} from "./structs/userSavings";

function App() {
  const [userSavings, setUserSavings] = useState({
    currentAge: 30,
    retirementAge: 65,
    totalSavings: 100000,
    monthlyIncome: 10000,
    monthlyExpenses: 5000,
    monthlyRent: 2000,
    homeValue: 500000,
    mortgageDebt: 250000,
    mortgageOutstanding: 250000,
    activeRetirement: false,
    monthlyMortgagePayment: 0,
    minBaselineRetirementIncome: 5000,
    maxBaselineRetirementIncome: 10000,
    mortgageRate: 0.03,
    mortgageTerm: 30,
    inflationRates: Array(100).fill(0),
    interestRates: Array(100).fill(0),
    rentalSavings: [] as number[],
    rentalAnnualNet: [] as number[],
    homeSavings: [] as number[],
    homeAnnualNet: [] as number[],
  } as UserSavingsType);

  // User Savings State Variables
  const handleUserSavingsChange = <K extends keyof UserSavingsType>(property: K, value: UserSavingsType[K]) => {
    const updatedUserSavings = updateUserSavings(userSavings, property, value);
    setUserSavings(updatedUserSavings);
  };

  // Initialize User Savings Return State Variables (empty arrays of numbers) to be used in PlotData
  const [zeroDistributions, setZeroDistributions] = useState({
    age: [] as number[],
    count: [] as number[],
    avg: 0,
    stdv: 0,
  } as ZeroDistributionsType);

  const validate = () => {
    validateMortgage();
    validateTotalSavings();
    validateRetirementInput();
  }

  const validateRetirementInput = () => {
    if (userSavings.minBaselineRetirementIncome > userSavings.maxBaselineRetirementIncome) {
      userSavings.maxBaselineRetirementIncome = userSavings.minBaselineRetirementIncome+1;
    } else {
      // do nothing
    }
  }

  const validateMortgage = () => {

    if (userSavings.mortgageDebt >= userSavings.homeValue) {
      userSavings.mortgageDebt = userSavings.homeValue
    }
    
    if (userSavings.mortgageDebt <= 0) {
      userSavings.mortgageDebt = 0
    }
  }

  const validateTotalSavings = () => {
    if (userSavings.totalSavings < userSavings.homeValue - userSavings.mortgageDebt) {
      userSavings.mortgageDebt = userSavings.homeValue - userSavings.totalSavings
    }
  }

  useEffect(() => {
    calculate()
  },[
    userSavings.currentAge,
    userSavings.retirementAge,
    userSavings.totalSavings,
    userSavings.monthlyExpenses,
    userSavings.monthlyIncome,
    userSavings.monthlyRent,
    userSavings.homeValue,
    userSavings.mortgageDebt,
    userSavings.minBaselineRetirementIncome,
    userSavings.maxBaselineRetirementIncome,
    userSavings.mortgageRate,
    userSavings.mortgageTerm,
  ]);

  async function calculate(recalcInt: boolean = false, recalcInf: boolean = false) {
    validate();
    setUserSavings(await invoke("calculate", {userSavings: userSavings, recalculateInterest: recalcInt, recalculateInflation: recalcInf}));
  }


  async function getZeroDistributions() {

    // validateRetirementInput();
    // validateMortgage();

    // setZeroDistributions(await invoke("get_zero_distributions", {
    //   currentAge: currentAge, 
    //   retirementAge: retirementAge,
    //   totalSavings: totalSavings,
    //   monthlySavings: monthlySavings,
    //   homeValue: homeValue,
    //   mortgageOutstanding: mortgageOutstanding,
    //   minBaselineRetirementIncome: minBaselineRetirementIncome,
    //   maxBaselineRetirementIncome: maxBaselineRetirementIncome,
    // }));
    // setIsPlotVisible(!isPlotVisible);
  }


  return (
    <div className="container">
      <div className="Data">
      <div className="DataInput">
        <form
          id="main-form"
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
            <Input 
                label="Age" 
                value={userSavings.currentAge}
                multiplier={1}
                onValueChange={(num) => handleUserSavingsChange('currentAge', Number(num))} 
            />
            <Input 
                label="Retirement Age" 
                value={userSavings.retirementAge}
                multiplier={1}
                onValueChange={(num) => handleUserSavingsChange('retirementAge', Number(num))} 
            />

            <Input 
                label="Net Worth" 
                value={userSavings.totalSavings}
                multiplier={100000}
                onValueChange={(num) => handleUserSavingsChange('totalSavings', Number(num))} 
            />

            <Input 
                label="Monthly Income" 
                value={userSavings.monthlyIncome}
                multiplier={1000}
                onValueChange={(num) => handleUserSavingsChange('monthlyIncome', Number(num))} 
            />

            <Input 
                label="Monthly Expenses" 
                value={userSavings.monthlyExpenses}
                multiplier={1000}
                onValueChange={(num) => handleUserSavingsChange('monthlyExpenses', Number(num))} 
            />

            <Input 
                label="Rent"
                value={userSavings.monthlyRent}
                multiplier={500}
                onValueChange={(num) => handleUserSavingsChange('monthlyRent', Number(num))} 
            />

            <Input 
                label="Home Equity" 
                value={userSavings.homeValue}
                multiplier={250000}
                onValueChange={(num) => handleUserSavingsChange('homeValue', Number(num))} 
            />

            <Input 
                label="Mortgage" 
                value={userSavings.mortgageDebt}
                multiplier={userSavings.homeValue / 20}
                onValueChange={(num) => handleUserSavingsChange('mortgageDebt', Number(num))} 
            />

            <Input 
                label="Min Monthly Retirement Income" 
                value={userSavings.minBaselineRetirementIncome}
                multiplier={Math.max(userSavings.totalSavings * 0.001, 100)}
                onValueChange={(num) => handleUserSavingsChange('minBaselineRetirementIncome', Number(num))} 
            />

            <Input 
                label="Max Monthly Retirement Income"  
                value={userSavings.maxBaselineRetirementIncome}
                multiplier={Math.max(userSavings.totalSavings * 0.001, 100)} 
                onValueChange={(num) => handleUserSavingsChange('maxBaselineRetirementIncome', Number(num))} 
            />
            <Input 
                label="Mortgage Rate"  
                value={userSavings.mortgageRate}
                multiplier={0.005} 
                onValueChange={(num) => handleUserSavingsChange('mortgageRate', Number(num))} 
            />
            <Input 
                label="Mortgage Term"  
                value={userSavings.mortgageTerm}
                multiplier={1} 
                onValueChange={(num) => handleUserSavingsChange('mortgageTerm', Number(num))} 
            />
          </form>
        </div>
        <div className="Plot">
          <div className="savingsPlot">
            <PlotData retirementAge={userSavings.retirementAge} homeSavings={userSavings.homeSavings} rentalSavings={userSavings.rentalSavings} yMax={Math.max(userSavings.totalSavings * 10, 1000000)}></PlotData>
          </div>
            <div className="zeroDistributionsPlot">
            <BarPlotData age={zeroDistributions.age} count={zeroDistributions.count} avg={zeroDistributions.avg} std={zeroDistributions.stdv}></BarPlotData>
          </div>
        </div>
      </div>

        <div className="new-rates-refresh-button">
            Recalculate ROI
            <button type="button" onClick={() => calculate(true, false)}>
            X 
            </button>
        </div>

        <div className="new-rates-refresh-button">
            Recalculate Inflation
            <button type="button" onClick={() => calculate(false, true)}>
            X
            </button>   
        </div>
        
        {/* <div className="zero-distribution-button">
            Recalculate Distribution
            <label className="button-arounder">
                <input 
                    type="checkbox" 
                    // Not sure about the state value for this checkbox, using a placeholder state value.
                    checked={userSavings}
                    onChange={getZeroDistributions}
                />
            </label>
        </div> */}

      <div className="DataTable">
          <table>
              <thead>
                  <tr>
                      <th>Age</th>
                      <th>ROI</th>
                      <th>Inflation</th>
                      <th>Networth w/ Home</th>
                      <th>Networth wo/ Home</th>
                  </tr>
              </thead>
              <tbody>
                {userSavings.homeSavings.map((i, j) => (
                  i !== 0 ? (
                    <tr key={j}>
                      <td>{j}</td>
                      <td>{(userSavings.interestRates[j]*100).toFixed(1)}%</td>
                      <td>{(userSavings.inflationRates[j]*100).toFixed(1)}%</td>
                      <td>{i.toLocaleString("en-US", { style: "currency", currency: "USD", minimumFractionDigits: 0, maximumFractionDigits: 0 })}</td>
                      <td>{userSavings.rentalSavings[j].toLocaleString("en-US", { style: "currency", currency: "USD", minimumFractionDigits: 0, maximumFractionDigits: 0 })}</td>
                    </tr>
                  ) : null
                ))}
              </tbody>
          </table>
        </div>
      </div>
  );
}

export default App;
