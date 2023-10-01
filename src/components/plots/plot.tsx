import Plot from 'react-plotly.js';
import { ReactNode } from "react";
import { UserSavingsType } from '../../structs/userSavings';

interface PlotDataProps {
    userSavings: UserSavingsType;
    children?: ReactNode;
}

export function PlotData(props: PlotDataProps) {

    return (
        <Plot
            data={[
                {
                    x: Array.from(Array(props.userSavings.homeSavings.length).keys()),
                    y: props.userSavings.homeSavings,
                    type: 'scatter',
                    mode: 'lines+markers',
                    marker: {color: 'red' , size: 1},
                    name: 'Home',
                },
                {
                    x: Array.from(Array(props.userSavings.rentalSavings.length).keys()),
                    y: props.userSavings.rentalSavings,
                    type: 'scatter',
                    mode: 'lines+markers',
                    marker: {color: 'blue', size: 1},
                    name: 'Rental',
                },
            ]}
            layout={ {
                width: 700,
                height: 500,
                margin: {
                    t: 10,  // adjust as needed
                    b: 60   // adjust as needed
                },
                xaxis: {
                    type: 'linear',
                    linewidth: 1,
                },
                yaxis: {
                    position: 0,
                    range: [0, props.userSavings.ymax],
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
                    x0: props.userSavings.retirementAge,
                    y0: 0,
                    x1: props.userSavings.retirementAge,
                    y1: props.userSavings.ymax * 0.75,
                    line: {
                        color: 'black',
                        width: 1,
                        dash: 'dot'
                    },
                    label: {
                        text: 'Retirement',
                        textposition: 'end',
                        textangle: 45,
                        font: {
                            family: 'Georgia, serif',
                            size: 12,
                            color: 'green',
                        },
                    },
                },
                {
                    type: 'line',
                    x0: (props.userSavings.homeOwnedAge) ? (props.userSavings.homeOwnedAge) : 0,
                    y0: 0,
                    x1: (props.userSavings.homeOwnedAge) ? (props.userSavings.homeOwnedAge) : 0,
                    y1: props.userSavings.ymax * 0.65,
                    line: {
                        color: 'black',
                        width: 1,
                        dash: 'dot'
                    },
                    label: {
                        text: 'Mortgage Paid',
                        textposition: 'end',
                        textangle: 45,
                        font: {
                            family: 'Georgia, serif',
                            size: (props.userSavings.homeOwnedAge) ?  12 : 0,
                            color: 'green',
                        },
                    },
                }
                ]
            } }
        />
    );
}
