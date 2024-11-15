import { invoke } from "@tauri-apps/api/core"


export async function testing() {
  const x = invoke('hallo')
  console.log(x);
}

export async function finishStroke(x : [number,number][]) : Promise<[number, number][]> {
  const points: [number,number][] = await invoke("finish_stroke", {points: x }); 

  return points;
}

