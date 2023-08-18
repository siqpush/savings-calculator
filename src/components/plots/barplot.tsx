import Plot from 'react-plotly.js';
import { ReactNode } from "react";

interface BarPlotDataProps {
    age: number[];
    count: number[];
    children?: ReactNode;
}

export function BarPlotData(props: BarPlotDataProps) {
    return (
        <Plot
        data={[
            {
            x: props.age,
            y: props.count,
            type: 'bar',
            marker: {color: 'green'},
            },
        ]}
        layout={ {
            width: 600,
            height: 200,
            margin: {
                t: 10,  // adjust as needed
                b: 30   // adjust as needed
              },
            xaxis: {
                range: [0, 100],
                type: 'linear',
                linewidth: 1,
              },
                yaxis: {
                position: 0,
                range: [0, 3],
                type: 'log',
                },
        } }
        />
    );
}