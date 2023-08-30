const MAX_VIDEO_SIZE_BYTES = 100 * 1000 * 1000;

import loadMP4Module, {
  isWebCodecsSupported,
} from "https://unpkg.com/mp4-wasm@1.0.6";

export default function CanvasRecorder(
  canvas,
  audioStream,
  videoBitsPerSecond,
) {
  this.start = async () => {
    const MP4 = await loadMP4Module();
    const encoderConfig = {
      width: canvas.width,
      height: canvas.height,
      fps: 30,
    };
    console.log("encoderConfig", encoderConfig);
    this._encoder = MP4.createWebCodecsEncoder(encoderConfig);
  };

  this._videoFileBytes = new Uint8Array(MAX_VIDEO_SIZE_BYTES);
  this._videoFileOffset = 0;
  this._frameCount = 0;

  this.onFrame = async (canvas) => {
    this._frameCount++;
    const bitmap = await createImageBitmap(canvas);
    await this._encoder.addFrame(bitmap);
  };

  this.stopAndSave = async (fileName) => {
    const videoBytes = await this._encoder.end();
    const videoBlob = new Blob([videoBytes]);
    const url = window.URL.createObjectURL(videoBlob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${fileName}.mp4`;
    a.click();
    window.URL.revokeObjectURL(url);
  };
}
