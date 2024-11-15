import { finishStroke } from "./gateway";

export async function drawLine(line_points: [number,number][], canvas: HTMLCanvasElement) {

  if (!canvas) {
    return;
  }

  const context = canvas.getContext('2d');

  if (!context) {
    return;
  }
  
  const segment_points: [number,number][] = await finishStroke(line_points);

  console.log(segment_points[0]);
  console.log(segment_points[1]);
  

  context.lineWidth = 4;
  context.strokeStyle = "#FFFFFF";
  context.moveTo(segment_points[0][0], segment_points[0][1]);
  context.lineTo(segment_points[1][0], segment_points[1][1]);
  context.stroke();
}
