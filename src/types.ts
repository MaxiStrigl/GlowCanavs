
export interface Point {
  x: number;
  y: number;
}

export enum Mode {
  Draw,
  Erase
}

export interface Stroke {
  points: Array<Point>;
  color: number;
  mode: Mode
}
