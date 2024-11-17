import {clearStrokes, startStroke, finishStroke } from "./gateway";
import { Point, Stroke } from "../types";

export async function startLine(point: Point, canvas: HTMLCanvasElement) {
  if (!canvas) {
    return
  }
  const context = canvas.getContext("2d");

  if (!context) {
    return;
  }

  const stroke: Stroke = await startStroke(point);
  console.log(stroke);
  

  if (stroke.points.length == 1) {
    drawPoint(stroke.points[0], canvas); 
  }
}

export async function finishLine(point: Point, canvas: HTMLCanvasElement) {
  if (!canvas) {
    return;
  }

  const context = canvas.getContext("2d");

  if (!context) {
    return;
  }

  await finishStroke(point);
}

export async function clearCanvas(canvas: HTMLCanvasElement) {
  const strokes = await clearStrokes();

  rerenderCanvas(strokes, canvas);
}

function rerenderCanvas(strokes: Stroke[], canvas: HTMLCanvasElement) {
  console.log("Clear");
  
  const ctx = canvas.getContext("2d");

  if (!ctx) {
    return;
  }

  ctx.reset();
  ctx.clearRect(0, 0, 1000, 1000);

  strokes.forEach((stroke) => {
    if (stroke.points.length > 1) {
      //TODO: Implememnt rerender lines
    }
    else if (stroke.points.length == 1) {
      drawPoint(stroke.points[0], canvas);    
    }
  });
}

function drawPoint(point: Point, canvas: HTMLCanvasElement) {
  const ctx = canvas.getContext("2d"); 
  if (!ctx) {
    return;
  }

  ctx.moveTo(point.x, point.y);
  ctx.arc(point.x, point.y, 2.5, 0, 2 * Math.PI);
  ctx.fillStyle = "#FFFFFF";
  ctx.fill();
}
