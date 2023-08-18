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
        if (value + increment < 0) {
            setValue(0.0);
            props.onValueChange(0.0);
        } else {
            setValue(value + increment);
            props.onValueChange(value + increment);
        }
    }

    return (
        <div className="buttons-container">
            {props.label}
            <button className="button-arounder" type="button" onClick={() => handleIncrementClick(1)}>+</button>
            {props.value.toLocaleString()}
            <button type="button" onClick={() => handleIncrementClick(-1)}>-</button>
        </div>
    )
}