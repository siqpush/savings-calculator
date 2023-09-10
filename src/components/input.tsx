import { useState } from "react";

interface InputProps {
    value: number;
    label: string;
    multiplier: number;
    onValueChange: (newValue: number) => void;
}

export function Input(props: InputProps)  {
    const [increment, setIncrement] = useState(0);
    const [value, setValue] = useState(props.value);

    const handleIncrementClick = (dir: number) => {

        setIncrement(dir * props.multiplier);
        if (value === undefined) {
            console.log("value is undefined");
            setValue(0.0);
        }
        if (increment === undefined) {
            console.log("increment is undefined");
            setIncrement(0);
            return;
        }
        if (props.label === "Age" || props.label === "Retirement Age") {
            if (value + increment > 100) {
                setValue(100);
                props.onValueChange(100);
            } else if (value + increment < 0) {
                setValue(0);
                props.onValueChange(0);
            } else {
                setValue(value + increment);
                props.onValueChange(value + increment);
            }
        } else if (props.label === "Net Worth" 
                    || props.label === "Home Equity" 
                    || props.label === "Mortgage" 
                    || props.label === "Min Monthly Retirement Income" 
                    || props.label === "Max Monthly Retirement Income" 
                    || props.label === "Rent"
                    || props.label === "Mortgage Rate"
        ) {
            if (value + increment < 0) {
                setValue(0);
                props.onValueChange(0);
            } else {
                setValue(value + increment);
                props.onValueChange(value + increment);
            }
        } else {
            setValue(value + increment);
            props.onValueChange(value + increment);
        }
    }

    return (
        <div className="buttons-container">
            {props.label}
            <button className="button-arounder" type="button" onClick={() => handleIncrementClick(1)}>+</button>
            {Math.abs(props.value) >= 100 ? Math.round(props.value).toLocaleString() : props.value.toPrecision(2)}
            <button className="button-arounder" type="button" onClick={() => handleIncrementClick(-1)}>-</button>
        </div>
    )
}