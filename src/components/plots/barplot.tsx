import Plot from 'react-plotly.js';
import { ReactNode, useEffect, useState } from "react";

interface BarPlotDataProps {
    age: number[];
    count: number[];
    avg: number;
    std: number;
    children?: ReactNode;
}

export function BarPlotData(props: BarPlotDataProps) {

    const [annotationPos, setAnnotationsPos] = useState(100);
    
    useEffect(() => {
        if (props.count !== undefined) {
            setAnnotationsPos(Math.max(...props.count));
        }
        console.log('annotationPos: ' + annotationPos);
    }, [props.count]);

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
                range: [0, annotationPos * 1.1],
                type: 'linear',
                },
            annotations: [{
                x: 25,
                y: annotationPos * .9,
                text: 'Average Interest: ' + (props.avg !== undefined
                ? props.avg.toLocaleString('en-US', { style: 'percent', minimumFractionDigits: 1, maximumFractionDigits: 1 }) 
                : 'N/A'),
                showarrow: false,
                bgcolor: 'white',
                font: {
                    family: 'Arial',
                    size: 12,
                    color: 'black'
                }
            },
            {
                x: 25,
                y: annotationPos * .8,
                text: 'Standard Deviation: ' + (props.std !== undefined 
                ? props.std.toLocaleString('en-US', { style: 'percent', minimumFractionDigits: 1, maximumFractionDigits: 1 }) 
                : 'N/A'),
                showarrow: false,
                bgcolor: 'white',
                font: {
                    family: 'Arial',
                    size: 12,
                    color: 'black'
                }
            }],
        } }
        />
    );
}