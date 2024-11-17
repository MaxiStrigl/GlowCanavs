import { invoke } from "@tauri-apps/api/core"
import { Point, Stroke } from "../types";

export async function testing() {
  const x = invoke('hallo')
  console.log(x);
}

export async function clearStrokes() : Promise<Stroke[]> {
  const strokes: Stroke[] = await invoke("clear_strokes");
  console.log(strokes.length);
  
  return strokes;
}

export async function startStroke(point: Point) : Promise<Stroke> {
  const stroke: Stroke = await invoke("start_stroke", {point: point})
  console.log(stroke.points.length);
  
  return stroke
}


export async function finishStroke(point: Point) : Promise<Stroke> {
  const stroke: Stroke = await invoke("finish_stroke", {point: point }); 
  console.log(stroke.points.length);
  

  return stroke;
}

