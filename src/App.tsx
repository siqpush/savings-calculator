import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { PlotData } from "./components/plots/plot";
import { Input } from "./components/input";
import { BarPlotData } from "./components/plots/barplot";


interface zeroDistributionsType {
  age: number[];
  count: number[];
  avg: number;
  stdv: number;
}

function App() {


  const [currentAge, setCurrentAge] = useState(30);
  const [retirementAge, setRetirementAge] = useState(60);
  const [totalSavings, setTotalSavings] = useState(1000000);
  const [monthlySavings, setMonthlySavings] = useState(0); 
  const [homeValue, setHomeValue] = useState(250000);
  const [mortgageOutstanding, setMortgageOutstanding] = useState(100000);
  const [minBaselineRetirementIncome, setMinBaselineRetirementIncome] = useState(5000);
  const [maxBaselineRetirementIncome, setMaxBaselineRetirementIncome] = useState(8000);
  const [recalculate, setRecalculate] = useState(true);  
  const [inflationRates, setInflationRates] = useState([] as number[]);
  const [interestRates, setInterestRates] = useState([] as number[]);
  const [savings, setSavings] = useState([] as number[]);
  const [updateYAxis, setUpdateYAxis] = useState(10000000);
  const [zeroDistributions, setZeroDistributions] = useState({} as zeroDistributionsType);
  const [isPlotVisible, setIsPlotVisible] = useState(true);

  const handleValueChange = (newValue: number, setStateFunction: React.Dispatch<React.SetStateAction<number>>) => 
  {
    setStateFunction(newValue);
  };

  const validateRetirementInput = () => {
    if (minBaselineRetirementIncome > maxBaselineRetirementIncome) {
      setMaxBaselineRetirementIncome(minBaselineRetirementIncome);
    } else if (maxBaselineRetirementIncome < minBaselineRetirementIncome) {
      setMinBaselineRetirementIncome(maxBaselineRetirementIncome);
    }
  }

  const validateMortgage = () => {
    if (totalSavings < homeValue) {
      setMortgageOutstanding(homeValue - totalSavings);
    } else if (mortgageOutstanding > homeValue) {
      setMortgageOutstanding(homeValue);
    } else {

    }
  }

  async function getInterestRates() {
    setInterestRates(await invoke ("get_interest_rates", {recalc: recalculate}));
  }

  async function getInflationRates() {
    setInflationRates(await invoke ("get_inflation_rates", {recalc: recalculate}));
  }

  useEffect(() => {
    if (recalculate) {
      getInterestRates();
      getInflationRates();

      // only update the y-axis if user recalculates
      let max = Math.max(...savings);
      if (max >= 100000000) {
        setUpdateYAxis(1000000000);
      } else if (max >= 10000000) {
        setUpdateYAxis(100000000);
      } else if (max >= 7500000) {
        setUpdateYAxis(10000000);
      } else if (max >= 5000000) {
        setUpdateYAxis(7500000);
      } else {
        setUpdateYAxis(5000000);
      }
      // reset recalculate to false
      setRecalculate(false);
    }
    getSavings();
  }, [currentAge, retirementAge, totalSavings, monthlySavings, homeValue, mortgageOutstanding, minBaselineRetirementIncome, maxBaselineRetirementIncome, recalculate]);
  
  async function getSavings() {

    validateRetirementInput();
    validateMortgage();

    setSavings(await invoke("get_savings", {
      currentage: currentAge, 
      retirementage: retirementAge,
      totalsavings: totalSavings,
      monthlysavings: monthlySavings,
      homevalue: homeValue,
      mortgageoutstanding: mortgageOutstanding,
      minbaselineretirementincome: minBaselineRetirementIncome,
      maxbaselineretirementincome: maxBaselineRetirementIncome,
      inflationRates: inflationRates,
      interestRates: interestRates,
    }));
  }

  async function getZeroDistributions() {

    validateRetirementInput();
    validateMortgage();

    setZeroDistributions(await invoke("get_zero_distributions", {
      currentage: currentAge, 
      retirementage: retirementAge,
      totalsavings: totalSavings,
      monthlysavings: monthlySavings,
      homevalue: homeValue,
      mortgageoutstanding: mortgageOutstanding,
      minbaselineretirementincome: minBaselineRetirementIncome,
      maxbaselineretirementincome: maxBaselineRetirementIncome,
    }));
    console.log(zeroDistributions.avg);
    setIsPlotVisible(!isPlotVisible);
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
                value={currentAge}
                multiplier={1}
                onValueChange={(newValue: number) => handleValueChange(newValue, setCurrentAge)} 
              />
            <Input 
                label="Retirement Age" 
                value={retirementAge}
                multiplier={1} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setRetirementAge)} 
            />
            <Input 
                label="Net Worth" 
                value={totalSavings}
                multiplier={100000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setTotalSavings)} 
              />
            <Input 
                label="EOM Net Income" 
                value={monthlySavings}
                multiplier={1000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMonthlySavings)} 
              />
            <Input 
                label="Home Equity" 
                value={homeValue}
                multiplier={250000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setHomeValue)} 
              />
            <Input 
                label="Mortgage" 
                value={mortgageOutstanding}
                multiplier={100000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMortgageOutstanding)} 
              />
            <Input 
                label="Min Monthly Retirement Income" 
                value={minBaselineRetirementIncome}
                multiplier={1000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMinBaselineRetirementIncome)} 
              />
              <Input 
                label="Max Monthly Retirement Income"  
                value={maxBaselineRetirementIncome}
                multiplier={1000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMaxBaselineRetirementIncome)} 
              />
          </form>
        </div>
        <div className="Plot">
          <div className="savingsPlot">
            <PlotData retirementage={retirementAge} savings={savings} yMax={updateYAxis}></PlotData>
          </div>
            <div className="zeroDistributionsPlot">
            <BarPlotData age={zeroDistributions.age} count={zeroDistributions.count} avg={zeroDistributions.avg} std={zeroDistributions.stdv}></BarPlotData>
          </div>
        </div>
      </div>
        <div className="refresh-buttons">
          <div className="new-rates-refresh-button">
            <label>Generate New Rates</label>
              <button className="button-arounder-big" type="submit" onClick={() => (setRecalculate(true))}>X</button>
          </div>
          <div className="zero-distribution-button">
                    <label>Recalculate Distribution</label>
              <button className="button-arounder-big" type="submit" onClick={getZeroDistributions}>X</button>
          </div>
        </div>
      <div className="DataTable">
          <table>
              <thead>
                  <tr>
                    
                      <th>Age</th>
                      <th>Interest</th>
                      <th>Inflation</th>
                      <th>Interest Income</th>
                  </tr>
              </thead>
              <tbody>
                {savings.map((i, j) => (
                  i !== 0 ? (
                    <tr key={j}>
                      <td>{j}</td>
                      <td>{(interestRates[j]*100).toFixed(1)}%</td>
                      <td>{(inflationRates[j]*100).toFixed(1)}%</td>
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
