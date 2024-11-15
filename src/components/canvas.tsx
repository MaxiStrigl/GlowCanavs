import { useEffect, useRef, MouseEvent } from "react";
import { drawLine } from "../utils/drawing";

function Canvas() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  var points : [number,number][] = [];


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

      return () => {
        window.removeEventListener("resize", handleResize);
      }
    }
  })

  const handleMouseDown = (e: MouseEvent<HTMLCanvasElement>) => {
    const x = e.pageX;
    const y = e.pageY;

    points.push([x, y])
  }

  const handleMouseUp = (e: MouseEvent<HTMLCanvasElement>) => {
    const x = e.pageX;
    const y = e.pageY;

    const canvas = canvasRef.current;

    if (!canvas) {
      return;
    }

    points.push([x,y]);

    drawLine(points, canvas);

    points = [];
  }


  return (
    <div>
      <canvas ref={canvasRef} onMouseDown={handleMouseDown} onMouseUp={handleMouseUp} className="bg-neutral-900 h-screen w-screen" />
    </div>
  )
}

export default Canvas;
