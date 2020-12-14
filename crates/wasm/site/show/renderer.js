import { ReaderWithBuffer } from "./ringbuffer.js";

const COMMAND_BEGIN_PATH = 1;
const COMMAND_CLEAR = 2;
const COMMAND_CLOSE_PATH = 3;
const COMMAND_FILL_RECT = 4;
const COMMAND_FILL_SQUARE = 5;
const COMMAND_END_FRAME = 6;
const COMMAND_FILL_STYLE_RGB = 7;
const COMMAND_LINE_WIDTH = 8;
const COMMAND_STROKE_SQUARE = 9;
const COMMAND_STROKE_STYLE_RGB = 10;
const COMMAND_STATUS_TEXT = 11;
const COMMAND_SHADOW_BLUR = 12;
const COMMAND_SHADOW_COLOR = 13;
const COMMAND_DONE = 14;
const COMMAND_DELAY = 15;
const COMMAND_SWITCH_LAYER = 16;
const COMMAND_FILL_STYLE_RGBA = 17;
const COMMAND_SET_ASPECT_RATIO = 18;
const COMMAND_ARC = 19;
const COMMAND_FILL = 20;
const COMMAND_STROKE = 21;
const COMMAND_LINE_TO = 22;
const COMMAND_MOVE_TO = 23;
const COMMAND_PLAY_SOUND = 24;
const COMMAND_DRAW_TEXT = 25;
const COMMAND_TEXT_FILL = 26;

export default function Renderer(
  message,
  layers,
  onNewAspectRatio,
  audioPlayer
) {
  const { buffer, offset, length } = message.data;
  const reader = new ReaderWithBuffer(buffer, offset, length);

  let ctx = layers[0];

  const overlayCtx = layers[1];
  overlayCtx.fillStyle = "white";

  this.done = false;

  this.render = () => {
    if (this.paused) return;
    outer: while (reader.hasNext()) {
      const command = reader.next();
      switch (command) {
        case COMMAND_CLEAR: {
          ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
          overlayCtx.clearRect(
            0,
            0,
            overlayCtx.canvas.width,
            overlayCtx.canvas.height
          );
          this.renderStatusText();
          break;
        }
        case COMMAND_END_FRAME: {
          break outer;
        }
        case COMMAND_BEGIN_PATH: {
          ctx.beginPath();
          break;
        }
        case COMMAND_CLOSE_PATH: {
          ctx.closePath();
          break;
        }
        case COMMAND_FILL_SQUARE: {
          let [x, y, size] = [
            reader.nextFloat(),
            reader.nextFloat(),
            reader.nextFloat(),
          ];
          ctx.fillRect(x, y, size, size);
          break;
        }
        case COMMAND_FILL_RECT: {
          let [x, y, width, height] = [
            reader.nextFloat(),
            reader.nextFloat(),
            reader.nextFloat(),
            reader.nextFloat(),
          ];
          ctx.fillRect(x, y, width, height);
          break;
        }
        case COMMAND_FILL_STYLE_RGB: {
          let [r, g, b] = [reader.next(), reader.next(), reader.next()];
          ctx.fillStyle = "rgb(" + r + ", " + g + "," + b + ")";
          break;
        }
        case COMMAND_FILL_STYLE_RGBA: {
          let [r, g, b, a] = [
            reader.next(),
            reader.next(),
            reader.next(),
            reader.nextFloat(),
          ];
          ctx.fillStyle = "rgba(" + r + ", " + g + "," + b + ", " + a + ")";
          break;
        }
        case COMMAND_LINE_WIDTH: {
          ctx.lineWidth = reader.nextFloat();
          break;
        }
        case COMMAND_STROKE_SQUARE: {
          let [x, y, size] = [reader.next(), reader.next(), reader.next()];
          ctx.strokeRect(x, y, size, size);
          break;
        }
        case COMMAND_STROKE_STYLE_RGB: {
          let [r, g, b] = [reader.next(), reader.next(), reader.next()];
          ctx.strokeStyle = "rgb(" + r + ", " + g + "," + b + ")";
          break;
        }
        case COMMAND_STATUS_TEXT: {
          this.statusText = reader.nextString();
          this.renderStatusText();
          break;
        }
        case COMMAND_SHADOW_BLUR: {
          ctx.shadowBlur = reader.next();
          break;
        }
        case COMMAND_SHADOW_COLOR: {
          let [r, g, b] = [reader.next(), reader.next(), reader.next()];
          ctx.shadowColor = "rgb(" + r + ", " + g + "," + b + ")";
          break;
        }
        case COMMAND_ARC: {
          ctx.arc(
            reader.nextFloat(),
            reader.nextFloat(),
            reader.nextFloat(),
            reader.nextFloat(),
            reader.nextFloat()
          );
          break;
        }
        case COMMAND_DONE: {
          console.log("[main] Please exit");
          reader.pleaseExit();
          this.done = true;
          return;
        }
        case COMMAND_DELAY: {
          this.delay = reader.next();
          break outer;
        }
        case COMMAND_SWITCH_LAYER: {
          const activeLayer = reader.next();
          ctx = layers[activeLayer];
          break;
        }
        case COMMAND_SET_ASPECT_RATIO: {
          const newAspectRatio = reader.nextFloat();
          onNewAspectRatio(newAspectRatio);
          return;
        }
        case COMMAND_FILL: {
          ctx.fill();
          break;
        }
        case COMMAND_STROKE: {
          ctx.stroke();
          break;
        }
        case COMMAND_LINE_TO: {
          ctx.lineTo(reader.nextFloat(), reader.nextFloat());
          break;
        }
        case COMMAND_MOVE_TO: {
          ctx.moveTo(reader.nextFloat(), reader.nextFloat());
          break;
        }
        case COMMAND_PLAY_SOUND: {
          const soundId = reader.next();
          if (audioPlayer) audioPlayer.play(soundId);
          break;
        }
        case COMMAND_DRAW_TEXT: {
          overlayCtx.save();
          overlayCtx.resetTransform();

          const alignment = reader.next();
          const x = reader.nextFloat();
          const y = reader.nextFloat();
          const fontSize = reader.nextFloat();
          const text = reader.nextString();

          const actualFontSize = fontSize * overlayCtx.canvas.height;

          overlayCtx.font = "normal " + actualFontSize + "px Monospace";
          if (alignment == 0) {
            overlayCtx.textAlign = "left";
            overlayCtx.textBaseline = "top";
          } else {
            overlayCtx.textAlign = "center";
            overlayCtx.textBaseline = "middle";
          }

          if (alignment != 0) {
            // FIXME: Hack - use separate flag?
            const boxMargin = 0;
            const textWidth = overlayCtx.measureText(text).width;
            const rectX =
              overlayCtx.canvas.width * x - textWidth / 2 - boxMargin;
            const rectY = overlayCtx.canvas.height * y - actualFontSize / 2;
            const rectWidth = textWidth + boxMargin * 2;
            const rectHeight = actualFontSize;
            const savedFillStyle = overlayCtx.fillStyle;
            overlayCtx.fillStyle = "rgba(13, 12, 26, 0.3)";
            overlayCtx.fillRect(rectX, rectY, rectWidth, rectHeight);
            overlayCtx.fillStyle = savedFillStyle;
          }

          overlayCtx.fillText(
            text,
            overlayCtx.canvas.width * x,
            overlayCtx.canvas.height * y
          );
          overlayCtx.restore();
          break;
        }
        case COMMAND_TEXT_FILL: {
          overlayCtx.fillStyle = reader.nextString();
          break;
        }
        default:
          throw new Error(
            "Unhandled command: " +
              command +
              ", done=" +
              this.done +
              ", buffer=" +
              reader.toDebug()
          );
      }
    }

    reader.wantMore();
  };

  this.renderStatusText = () => {
    if (!this.statusText) return;

    // Save state and restore at end, so that other text rendering
    // is unaffected by status text rendering:
    overlayCtx.save();
    overlayCtx.resetTransform();

    const yOffset = 0;
    const boxMargin = 10;
    const textHeight = 80;
    overlayCtx.font = textHeight + "px Monospace";

    overlayCtx.clearRect(
      0,
      0,
      overlayCtx.canvas.width,
      overlayCtx.canvas.height
    );

    const textWidth = overlayCtx.measureText(this.statusText).width;

    overlayCtx.fillStyle = "rgba(13, 12, 26, 0.3)";
    const rectX = overlayCtx.canvas.width / 2 - textWidth / 2 - boxMargin;
    const rectY = yOffset;
    const rectWidth = textWidth + boxMargin * 2;
    const rectHeight = textHeight;
    overlayCtx.fillRect(rectX, rectY, rectWidth, rectHeight);

    overlayCtx.fillStyle = "white";
    overlayCtx.strokeStyle = "black";
    overlayCtx.textBaseline = "top";
    overlayCtx.textAlign = "center";
    const maxWidth = overlayCtx.canvas.width;
    overlayCtx.fillText(
      this.statusText,
      overlayCtx.canvas.width / 2,
      yOffset,
      maxWidth
    );

    overlayCtx.restore();
  };
}
