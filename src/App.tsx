import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { PlotData } from "./components/plots/plot";
import { Input } from "./components/input";

import {UserSavingsType, updateUserSavings} from "./structs/userSavings";

function App() {
  const [userSavings, setUserSavings] = useState({
    currentAge: 30,
    retirementAge: 65,
    totalSavings: 1000000,
    monthlyIncome: 8000,
    monthlyExpenses: 4000,
    monthlyRent: 3000,
    homeValue: 750000,
    mortgageDebt: 500000,
    activeRetirement: false,
    minBaselineRetirementIncome: 2000,
    maxBaselineRetirementIncome: 3000,
    mortgageRate: 0.035,
    mortgageTerm: 30,
    inflationRates: Array(100).fill(0.0),
    interestRates: Array(100).fill(0.0),
    rentalSavings: Array(100).fill(0.0),
    homeSavings: Array(100).fill(0.0),
    ymax: 1000000,
    homeExpenses: 0.01,
  } as UserSavingsType);

  // User Savings State Variables
  const handleUserSavingsChange = <K extends keyof UserSavingsType>(property: K, value: UserSavingsType[K]) => {
    const updatedUserSavings = updateUserSavings(userSavings, property, value);
    setUserSavings(updatedUserSavings);
  };

  // validations
  function validate() {
      // validations
    if (userSavings.mortgageDebt > userSavings.homeValue) {
        document.getElementById('form-field-Mortgage')?.setAttribute("style", "font-style: italic; color: red;");
        let element = document.getElementById('data-warning');
        if (element) {
          element.innerText = "Mortgage cannot be greater than home value.";
          element.setAttribute("style", "font-style: italic; color: red;");
          userSavings.homeSavings = Array(100).fill(0.0);
          userSavings.rentalSavings = Array(100).fill(0.0);
        }
        return false
    } else {
        document.getElementById('form-field-Mortgage')?.setAttribute("style", "font-style: regular; color: black;");
        let element = document.getElementById('data-warning');
        if (element) {
          element.innerText = "";
          element.setAttribute("style", "font-style: regular; color: black;");
        }
      }
    if (userSavings.homeValue > userSavings.totalSavings + userSavings.mortgageDebt) {
        document.getElementById('form-field-Net Worth')?.setAttribute("style", "font-style: italic; color: red;");
        let element = document.getElementById('data-warning');
        if (element) {
          element.innerText = "Net worth cannot be less than home value + mortgage.";
          element.setAttribute("style", "font-style: italic; color: red;");
          userSavings.homeSavings = Array(100).fill(0.0);
          userSavings.rentalSavings = Array(100).fill(0.0);
        }
        return false
    } else {
        document.getElementById('form-field-Net Worth')?.setAttribute("style", "font-style: regular; color: black;");
        let element = document.getElementById('data-warning');
        if (element) {
          element.innerText = "";
          element.setAttribute("style", "font-style: regular; color: black;");
        }
      }
    if (userSavings.minBaselineRetirementIncome > userSavings.maxBaselineRetirementIncome) {
        document.getElementById('form-field-Min Monthly Retirement Income')?.setAttribute("style", "font-style: italic; color: red;");
        document.getElementById('form-field-Max Monthly Retirement Income')?.setAttribute("style", "font-style: italic; color: red;");
        let element = document.getElementById('data-warning');
        if (element) {
          element.innerText = "Min retirement income cannot be greater than max retirement income.";
          element.setAttribute("style", "font-style: italic; color: red;");
          userSavings.homeSavings = Array(100).fill(0.0);
          userSavings.rentalSavings = Array(100).fill(0.0);
        }
        return false
    } else {
        document.getElementById('form-field-Min Monthly Retirement Income')?.setAttribute("style", "font-style: regular; color: black;");
        document.getElementById('form-field-Max Monthly Retirement Income')?.setAttribute("style", "font-style: regular; color: black;");
        let element = document.getElementById('data-warning');
        if (element) {
          element.innerText = "";
          element.setAttribute("style", "font-style: regular; color: black;");
      }
    }
    return true
  }
  useEffect(() => {
    if (validate()) {
      calculate(false)
    }
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

  async function calculate(recalc: boolean = false) {
    setUserSavings(await invoke("calculate", {userSavings: userSavings, recalculate: recalc}));
  }


  return (
    <div className="container">
      <div className="Data">
      
      <div className="DataInput">
      <div id="data-warning"></div>
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
                multiplier={50000}
                onValueChange={(num) => handleUserSavingsChange('totalSavings', Number(num))} 
            />

            <Input 
                label="Monthly Income" 
                value={userSavings.monthlyIncome}
                multiplier={500}
                onValueChange={(num) => handleUserSavingsChange('monthlyIncome', Number(num))} 
            />

            <Input 
                label="Monthly Expenses" 
                value={userSavings.monthlyExpenses}
                multiplier={500}
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
                multiplier={100000}
                onValueChange={(num) => handleUserSavingsChange('homeValue', Number(num))} 
            />

            <Input 
                label="Mortgage" 
                value={userSavings.mortgageDebt}
                multiplier={50000}
                onValueChange={(num) => handleUserSavingsChange('mortgageDebt', Number(num))} 
            />
            <Input 
                label="Mortgage Rate"  
                value={userSavings.mortgageRate}
                multiplier={.005} 
                onValueChange={(num) => handleUserSavingsChange('mortgageRate', num)} 
            />
            <Input 
                label="Mortgage Term"  
                value={userSavings.mortgageTerm}
                multiplier={1} 
                onValueChange={(num) => handleUserSavingsChange('mortgageTerm', Number(num))} 
            />
            <Input 
                label="Min Monthly Retirement Income" 
                value={userSavings.minBaselineRetirementIncome}
                multiplier={500}
                onValueChange={(num) => handleUserSavingsChange('minBaselineRetirementIncome', Number(num))} 
            />

            <Input 
                label="Max Monthly Retirement Income"  
                value={userSavings.maxBaselineRetirementIncome}
                multiplier={500} 
                onValueChange={(num) => handleUserSavingsChange('maxBaselineRetirementIncome', Number(num))} 
            />
          </form>
        </div>
        <div className="Plot">
          <div className="savingsPlot">
            <PlotData userSavings={userSavings}></PlotData>
          </div>
          {/* 
            <div className="zeroDistributionsPlot">
            <BarPlotData age={zeroDistributions.age} count={zeroDistributions.count} avg={zeroDistributions.avg} std={zeroDistributions.stdv}></BarPlotData>
          </div>*/}
        </div> 
      </div>

        <div className="new-rates-refresh-button">
            New Rates
            <button type="button" onClick={() => calculate(true)}>
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
                      <th>Home Owner</th>
                      <th>Renter</th>
                  </tr>
              </thead>
                <tbody>
                  {userSavings.homeSavings.map((_, j) => (
                    j > userSavings.currentAge ? (
                      <tr key={j-2}>
                        <td>{j-1}</td>
                        <td>{(userSavings.interestRates[j-1]*100).toFixed(1)}%</td>
                        <td>{(userSavings.inflationRates[j-1]*100).toFixed(1)}%</td>
                        <td>{userSavings.homeSavings[j-1].toLocaleString("en-US", { style: "currency", currency: "USD", minimumFractionDigits: 0, maximumFractionDigits: 0 })}</td>
                        <td>{userSavings.rentalSavings[j-1].toLocaleString("en-US", { style: "currency", currency: "USD", minimumFractionDigits: 0, maximumFractionDigits: 0 })}</td>
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
