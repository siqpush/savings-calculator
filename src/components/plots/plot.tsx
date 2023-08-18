import Plot from 'react-plotly.js';
import { ReactNode } from "react";

interface PlotDataProps {
    retirementage: number;
    savings: number[];
    yMax: number;
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
            width: 600,
            height: 200,
            margin: {
                t: 0,  // adjust as needed
                b: 80   // adjust as needed
              },
            xaxis: {
                range: [-5, props.savings.length],
                type: 'linear',
                linewidth: 1,
              },
                yaxis: {
                scaleratio: 4,
                position: 0,
                range: [0, props.yMax],
                rangeslider: {range: [0, props.yMax]},
                type: 'linear',
                },
            shapes: [{
                type: 'line',
                x0: props.retirementage,
                y0: 0,
                x1: props.retirementage,
                y1: props.yMax * .9,
                line: {
                    color: 'blue',
                    width: 1,
                },
                label: {
                    text: 'Begin Distributions...',
                    textposition: 'end',
                    textangle: 0,
                    font: {
                        family: 'Georgia, serif',
                        size: 12,
                        color: 'blue',
                    },
                },
            }]
        } }
        />
    );
}