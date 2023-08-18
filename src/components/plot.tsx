import Plot from 'react-plotly.js';
import { ReactNode } from "react";

interface PlotDataProps {
    retirementage: number;
    savings: number[];
    children?: ReactNode;
}

export function PlotData(props: PlotDataProps) {
    return (
        <Plot
        data={[
            {
            x: Array.from(Array(props.savings.length).keys()),
            y: props.savings,
            type: 'scatter',
            mode: 'lines+markers',
            marker: {color: 'red'},
            },
        ]}
        layout={ {
            width: 500,
            height: 500,
            title: 'savings',
            xaxis: {
                range: [0, props.savings.length],
                type: 'linear',
              },
                yaxis: {
                range: [3, 8],
                type: 'log',
                },
            shapes: [{
                type: 'line',
                x0: props.retirementage,
                y0: 0,
                x1: props.retirementage,
                y1: Math.max(...props.savings)*1.1,
                line: {
                    color: 'black',
                    width: 1
                }
            }]
        } }
        />
    );
}