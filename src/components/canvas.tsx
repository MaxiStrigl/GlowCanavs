import { useEffect, useRef, MouseEvent } from "react";
import { startLine, clearCanvas } from "../utils/drawing";
import { Point } from "../types";
import { finishStroke } from "../utils/gateway";

function Canvas() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  var points: [number, number][] = [];


  function scaleCanvas(canvas: HTMLCanvasElement) {
    if (!canvas) {
      return;
    }

    const context = canvas.getContext("2d");

    if (!context) {
      return;
    }

    const devicePixelRatio = window.devicePixelRatio;
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;

    const imageData = context.getImageData(0.0, 0.0, width, height);

    canvas.width = width * devicePixelRatio;
    canvas.height = height * devicePixelRatio;

    context.scale(devicePixelRatio, devicePixelRatio);

    context.putImageData(imageData, 0.0, 0.0)
  }

  useEffect(() => {
    const canvas = canvasRef.current;
    const context = canvas?.getContext("2d")

    if (canvas && context) {
      scaleCanvas(canvas);

      const handleResize = () => scaleCanvas(canvas);
      window.addEventListener("resize", handleResize);
      window.addEventListener("keydown", handleKeydown)

      return () => {
        window.removeEventListener("resize", handleResize);
        window.removeEventListener("keydown", handleKeydown)
      }
    }
  })

  const handleKeydown = (event: KeyboardEvent) => {
    const canvas = canvasRef.current;

    if (canvas) {
      if (event.ctrlKey && event.key == 'r') {
        clearCanvas(canvas);
      }
    }
  }

  const handleMouseDown = (e: MouseEvent<HTMLCanvasElement>) => {
    const x = e.pageX;
    const y = e.pageY;

    const point: Point = { x, y };

    const canvas = canvasRef.current;

    if (!canvas) {
      return;
    }

    startLine(point, canvas);
  }

  const handleMouseUp = (e: MouseEvent<HTMLCanvasElement>) => {
    const x = e.pageX;
    const y = e.pageY;

    const point: Point = { x, y };

    const canvas = canvasRef.current;

    if (!canvas) {
      return;
    }

    finishStroke(point, canvas)
  }


  return (
    <div>
      <canvas ref={canvasRef} onMouseDown={handleMouseDown} onMouseUp={handleMouseUp} className="bg-neutral-900 h-screen w-screen" />
    </div>
  )
}

export default Canvas;
