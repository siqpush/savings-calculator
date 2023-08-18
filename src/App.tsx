import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { PlotData } from "./components/plot";
import { Input } from "./components/input";


function App() {

  const [currentAge, setCurrentAge] = useState(0);
  const [retirementAge, setRetirementAge] = useState(60);
  const [totalSavings, setTotalSavings] = useState(100000);
  const [monthlySavings, setMonthlySavings] = useState(0); 
  const [homeValue, setHomeValue] = useState(0);
  const [mortgageOutstanding, setMortgageOutstanding] = useState(0);
  const [minBaselineRetirementIncome, setMinBaselineRetirementIncome] = useState(4000);
  const [maxBaselineRetirementIncome, setMaxBaselineRetirementIncome] = useState(8000);
  const [recalculate, setRecalculate] = useState(false);
  const [inflationRates, setInflationRates] = useState([] as number[]);
  const [interestRates, setInterestRates] = useState([] as number[]);
  const [savings, setSavings] = useState([] as number[]);
  const handleValueChange = (newValue: number, setStateFunction: React.Dispatch<React.SetStateAction<number>>) => 
  {
    setStateFunction(newValue);
    getSavings();
  };
  const validateRetirementInput = (min: number, max: number) => {
    if (min > max) {
      setMaxBaselineRetirementIncome(min);
    } else if (max < min) {
      setMinBaselineRetirementIncome(max);
    }
  }

  useEffect(() => {
    getSavings();
  }, []);

  async function getSavings() {

    validateRetirementInput(minBaselineRetirementIncome, maxBaselineRetirementIncome);

    try {

      setInterestRates(await invoke ("get_interest_rates", {recalc: recalculate}));
      setInflationRates(await invoke ("get_inflation_rates", {recalc: recalculate}));
      if (inflationRates.length === 0 || interestRates.length === 0) {
        return;
      }

      setSavings(await invoke("get_savings", {
        currentage: currentAge, 
        retirementage: retirementAge,
        totalsavings: totalSavings,
        monthlysavings: monthlySavings,
        homevalue: homeValue,
        mortgageoutstanding: mortgageOutstanding,
        minbaselineretirementincome: minBaselineRetirementIncome,
        maxbaselineretirementincome: maxBaselineRetirementIncome,
        recalculate: recalculate,
        inflationRates: inflationRates,
        interestRates: interestRates,
      }));
      
    } catch (error) {
      console.log(error);
    };
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
                multiplier={250000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMortgageOutstanding)} 
              />
            <Input 
                label="Retirement Income Min." 
                value={minBaselineRetirementIncome}
                multiplier={1000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMinBaselineRetirementIncome)} 
              />
              <Input 
                label="Retirement Income Max." 
                value={maxBaselineRetirementIncome}
                multiplier={1000} 
                onValueChange={(newValue: number) => handleValueChange(newValue, setMaxBaselineRetirementIncome)} 
              />
              <div>
                <label>Generate Rates</label>
                <input type="checkbox" checked={recalculate} onChange={(e) => {console.log(e.target.checked), setRecalculate(e.target.checked)}} />
              </div>
          </form>
        </div>
      <div className="Plot">
          <PlotData retirementage={retirementAge} savings={savings}>
          </PlotData>
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
                {savings.map((_, index) => (
                  <tr key={index}>
                    <td>{index}</td>
                    <td>{(interestRates[index]*100).toFixed(1)}%</td>
                    <td>{(inflationRates[index]*100).toFixed(1)}%</td>
                    <td>{savings[index].toLocaleString("en-US", { style: "currency", currency: "USD" })}</td>
                  </tr>
                ))}
              </tbody>
          </table>
        </div>
      </div>
  );
}

export default App;
