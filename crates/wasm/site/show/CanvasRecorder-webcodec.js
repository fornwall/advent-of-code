const MAX_VIDEO_SIZE_BYTES = 100 * 1000 * 1000;

export default function CanvasRecorder(
  canvas,
  audioStream,
  videoBitsPerSecond
) {
  this.start = async () => {
    const recorder = this;
    const init = {
      output: (chunk, metadata) => {
        if (metadata.decoderConfig) {
          // Decoder needs to be configured (or reconfigured) with new parameters
          // when metadata has a new decoderConfig.
          // Usually it happens in the beginning or when the encoder has a new
          // codec specific binary configuration. (VideoDecoderConfig.description).
          if (false && metadata.decoderConfig.description) {
            this._videoFileBytes.set(
              new Uint8Array(metadata.decoderConfig.description),
              this._videoFileOffset
            );
            this._videoFileOffset +=
              metadata.decoderConfig.description.byteLength;
            console.log(
              "_videoFileBytes=",
              this._videoFileBytes.subarray(0, 100),
              "offset=",
              this._videoFileOffset
            );
          }
        }

        // actual bytes of encoded data
        const sliceToWriteTo = recorder._videoFileBytes.subarray(
          this._videoFileOffset,
          this._videoFileOffset + chunk.byteLength
        );
        chunk.copyTo(sliceToWriteTo);
        this._videoFileOffset += chunk.byteLength;
      },
      error: (e) => {
        console.error(e.message);
        window.alert(e.message);
      },
    };

    const config = {
      codec: "avc1.42001E",
      // codec: "vp8",
      width: 640,
      height: 480,
      bitrate: 2_000_000, // 2 Mbps
      framerate: 30,
    };

    const { supported } = await VideoEncoder.isConfigSupported(config);
    if (supported) {
      this._encoder = new VideoEncoder(init);
      this._encoder.configure(config);
    } else {
      alert("not suppported config!");
      return;
    }
  };

  this._videoFileBytes = new Uint8Array(MAX_VIDEO_SIZE_BYTES);
  this._videoFileOffset = 0;
  this._frameCount = 0;

  this.onFrame = (canvas) => {
    const frame = new VideoFrame(canvas, {
      timestamp: this._frameCount * 1000 * 1000,
    });
    this._frameCount++;

    if (this._encoder.encodeQueueSize > 2) {
      // Too many frames in flight, encoder is overwhelmed
      // let's drop this frame.
      console.warn("Dropping frame");
    } else {
      this._frameCount++;
      const keyFrame = this._frameCount % 150 == 0;
      this._encoder.encode(frame, { keyFrame });
    }
    frame.close();
  };

  this.stopAndSave = (fileName) => {
    this.fileName = (fileName || "recording") + ".h264";
    const videoBlob = new Blob([
      this._videoFileBytes.slice(0, this._videoFileOffset),
    ]);
    const url = window.URL.createObjectURL(videoBlob);
    const a = document.createElement("a");
    a.href = url;
    a.download = this.fileName;
    a.click();
    window.URL.revokeObjectURL(url);
  };
}
