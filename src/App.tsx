import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { PlotData } from "./components/plots/plot";
import { Input } from "./components/input";
import { BarPlotData } from "./components/plots/barplot";
import {ZeroDistributionsType, UserSavingsType, updateUserSavings} from "./structs/userSavings";

function App() {

  // Initialize User Savings State Variables
  const [userSavings, setUserSavings] = useState({
    currentAge: 35,
    retirementAge: 67,
    totalSavings: 0,
    monthlySavings: 0,
    monthlyRent: 0,
    homeValue: 0,
    mortgageOutstanding: 0,
    mortgageDebt: 0,
    monthlyMortgagePayment: 0,
    minBaselineRetirementIncome: 0,
    maxBaselineRetirementIncome: 0,
    mortgageRate: 0.04,
    compareHomeOwnership: true,
    recalculateInflation: false,
    recalculateInterest: false,
    inflationRates: Array(100).fill(0.0),
    interestRates: Array(100).fill(0.0),
    rentalSavings: Array(100).fill(0.0),
    homeSavings: Array(100).fill(0.0),
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
    validateTotalSavings();
    validateMortgage();
    
  }

  const validateRetirementInput = () => {
    if (userSavings.minBaselineRetirementIncome > userSavings.maxBaselineRetirementIncome) {
      userSavings.maxBaselineRetirementIncome = userSavings.minBaselineRetirementIncome;
    } else if (userSavings.maxBaselineRetirementIncome < userSavings.minBaselineRetirementIncome) {
      userSavings.maxBaselineRetirementIncome = userSavings.minBaselineRetirementIncome;
    } else {
      // do nothing
    }
  }

  const validateMortgage = () => {

    if (userSavings.mortgageDebt > userSavings.homeValue) {
      userSavings.mortgageDebt = userSavings.homeValue
    }
    
    if (userSavings.mortgageDebt < 0) {
      userSavings.mortgageDebt = 0
    }
  }

  const validateTotalSavings = () => {
    if (userSavings.totalSavings < userSavings.homeValue - userSavings.mortgageDebt) {
      userSavings.mortgageDebt = userSavings.homeValue - userSavings.totalSavings
    }
  }

  useEffect(() => {    
    get_rental_savings()
    get_home_savings()
  },[
    userSavings.compareHomeOwnership,
    userSavings.currentAge,
    userSavings.retirementAge,
    userSavings.totalSavings,
    userSavings.monthlySavings,
    userSavings.monthlyRent,
    userSavings.homeValue,
    userSavings.mortgageDebt,
    userSavings.minBaselineRetirementIncome,
    userSavings.maxBaselineRetirementIncome,
    userSavings.recalculateInflation,
    userSavings.recalculateInterest,
    userSavings.mortgageRate,
  ]);
  
  async function get_home_savings() {
    validate();
    setUserSavings(await invoke("get_home_savings", {userSavings: userSavings}));
  }
  
  async function get_rental_savings() {
    validate();
    setUserSavings(await invoke("get_rental_savings", {userSavings: userSavings}));
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
                label="EOM Net Income" 
                value={userSavings.monthlySavings}
                multiplier={1000}
                onValueChange={(num) => handleUserSavingsChange('monthlySavings', Number(num))} 
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
                multiplier={userSavings.homeValue / 10}
                onValueChange={(num) => handleUserSavingsChange('mortgageDebt', Number(num))} 
            />

            <Input 
                label="Min Monthly Retirement Income" 
                value={userSavings.minBaselineRetirementIncome}
                multiplier={1000}
                onValueChange={(num) => handleUserSavingsChange('minBaselineRetirementIncome', Number(num))} 
            />

            <Input 
                label="Max Monthly Retirement Income"  
                value={userSavings.maxBaselineRetirementIncome}
                multiplier={1000} 
                onValueChange={(num) => handleUserSavingsChange('maxBaselineRetirementIncome', Number(num))} 
            />
            <Input 
                label="Mortgage Rate"  
                value={userSavings.mortgageRate}
                multiplier={0.1} 
                onValueChange={(num) => handleUserSavingsChange('mortgageRate', Number(num))} 
            />
          </form>
        </div>
        <div className="Plot">
          <div className="savingsPlot">
            <PlotData retirementAge={userSavings.retirementAge} homeSavings={userSavings.homeSavings} rentalSavings={userSavings.rentalSavings} yMax={10000000}></PlotData>
          </div>
            <div className="zeroDistributionsPlot">
            <BarPlotData age={zeroDistributions.age} count={zeroDistributions.count} avg={zeroDistributions.avg} std={zeroDistributions.stdv}></BarPlotData>
          </div>
        </div>
      </div>

        <div className="new-rates-refresh-button">
            Recalculate ROI
            <label className="button-arounder">
                <input 
                    type="checkbox" 
                    checked={userSavings.recalculateInterest}
                    onChange={() => handleUserSavingsChange("recalculateInterest", !userSavings.recalculateInterest)}
                />
            </label>
        </div>

        <div className="new-rates-refresh-button">
            Recalculate Inflation
            <label className="button-arounder">
                <input 
                    type="checkbox" 
                    checked={userSavings.recalculateInflation}
                    onChange={() => handleUserSavingsChange("recalculateInflation", !userSavings.recalculateInflation)}
                />
            </label>
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
                      <th>Networth</th>
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
