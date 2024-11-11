import { useEffect, useRef } from "react";

function Canvas() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);


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

      context.fillStyle = "blue";
      context.fillRect(0, 0, 100, 100);

      return () => {
        window.removeEventListener("resize", handleResize);
      }
    }
  })


  return (
    <div>
      <canvas ref={canvasRef} className="bg-neutral-900 h-screen w-screen" />
    </div>
  )
}

export default Canvas;
