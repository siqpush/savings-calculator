import Plot from 'react-plotly.js';
import { ReactNode } from "react";

interface PlotDataProps {
    retirementAge: number;
    homeSavings: number[];
    rentalSavings: number[];
    yMax: number;
    children?: ReactNode;
}

export function PlotData(props: PlotDataProps) {
    const plotlyData: Partial<typeof PlotData>[] = [
        
    ];

    return (
        <Plot
            data={[
                {
                    x: Array.from(Array(props.homeSavings.length).keys()),
                    y: props.homeSavings,
                    type: 'scatter',
                    mode: 'lines+markers',
                    marker: {color: 'red' , size: 1},
                    name: 'Home',
                },
                {
                    x: Array.from(Array(props.rentalSavings.length).keys()),
                    y: props.rentalSavings,
                    type: 'scatter',
                    mode: 'lines+markers',
                    marker: {color: 'blue', size: 1},
                    name: 'Rental',
                },
            ]}
            layout={ {
                width: 600,
                height: 200,
                margin: {
                    t: 10,  // adjust as needed
                    b: 60   // adjust as needed
                },
                xaxis: {
                    range: [-5, props.homeSavings.length],
                    type: 'linear',
                    linewidth: 1,
                },
                yaxis: {
                    position: 0,
                    range: [0, props.yMax],
                    rangeslider: {range: [0, props.yMax]},
                    type: 'linear',
                },
                legend: {
                    x: 0,
                    y: .9,
                    traceorder: 'normal',
                    font: {
                        family: 'sans-serif',
                        size: 12,
                        color: '#000'
                    },
                },
                shapes: [{
                    type: 'line',
                    x0: props.retirementAge,
                    y0: 0,
                    x1: props.retirementAge,
                    y1: props.yMax * .8,
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
