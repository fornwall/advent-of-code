import {ReaderWithBuffer} from './ringbuffer.js';

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
const COMMAND_FILL_TEXT = 11;
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

export default function Renderer(message, layers, onNewAspectRatio) {
    const {buffer, offset, length} = message.data;
    const reader = new ReaderWithBuffer(buffer, offset, length);

    let ctx = layers[0];
    ctx.imageSmoothingEnabled = false;
    this.done = false;

    this.render = () => {
        if (this.paused) return;
        outer:
        while (reader.hasNext()) {
            const command = reader.next();
            switch (command) {
                case COMMAND_CLEAR: {
                    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
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
                    let [x, y, size] = [reader.nextFloat(), reader.nextFloat(), reader.nextFloat()];
                    ctx.fillRect(x, y, size, size);
                    break;
                }
                case COMMAND_FILL_RECT: {
                    let [x, y, width, height] = [reader.nextFloat(), reader.nextFloat(), reader.nextFloat(), reader.nextFloat()];
                    ctx.fillRect(x, y, width, height);
                    break;
                }
                case COMMAND_FILL_STYLE_RGB: {
                    let [r, g, b] = [reader.next(), reader.next(), reader.next()];
                    ctx.fillStyle = 'rgb(' + r + ', ' + g + ',' + b + ')';
                    break;
                }
                case COMMAND_FILL_STYLE_RGBA: {
                    let [r, g, b, a] = [reader.next(), reader.next(), reader.next(), reader.nextFloat()];
                    ctx.fillStyle = 'rgba(' + r + ', ' + g + ',' + b + ', ' + a + ')';
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
                    ctx.strokeStyle = 'rgb(' + r + ', ' + g + ',' + b + ')';
                    break;
                }
                case COMMAND_FILL_TEXT: {
                    let textLayer = layers[1];
                    textLayer.font = '40px Monospace';
                    const text = reader.nextString();
                    let x = reader.nextFloat();
                    let y = reader.nextFloat();
                    x = textLayer.canvas.width;
                    y = textLayer.canvas.height;
                    textLayer.clearRect(0, 0, textLayer.canvas.width, textLayer.canvas.height);
                    textLayer.fillStyle = 'white';
                    textLayer.strokeStyle = 'black';
                    textLayer.textAlign = 'right';
                    textLayer.textBaseline = 'bottom';
                    textLayer.fillText(text, x, y);
                    textLayer.strokeStyle = '2'
                    //textLayer.strokeText(text, x, y);
                    break;
                }
                case COMMAND_SHADOW_BLUR: {
                    ctx.shadowBlur = reader.next();
                    break;
                }
                case COMMAND_SHADOW_COLOR: {
                    let [r, g, b] = [reader.next(), reader.next(), reader.next()];
                    ctx.shadowColor = 'rgb(' + r + ', ' + g + ',' + b + ')';
                    break;
                }
                case COMMAND_ARC: {
                    ctx.arc(reader.nextFloat(), reader.nextFloat(), reader.nextFloat(), reader.nextFloat(), reader.nextFloat());
                    break;
                }
                case COMMAND_DONE: {
                    console.log('done rendering');
                    this.done = true;
                    break;
                }
                case COMMAND_DELAY: {
                    this.delay = reader.next();
                    return;
                }
                case COMMAND_SWITCH_LAYER: {
                    const activeLayer = reader.next();
                    ctx = layers[activeLayer];
                    return;
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
                default:
                    throw new Error('Unhandled command: ' + command + ', done=' + this.done);
            }
        }

        reader.wantMore();
    };
}