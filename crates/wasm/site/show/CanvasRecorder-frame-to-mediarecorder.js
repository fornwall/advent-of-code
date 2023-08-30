// See https://github.com/legokichi/ts-ebml for making videos seekable:
// $  npm install --global ts-ebml
// $ ts-ebml -s advent-of-code-2019-13-part2.webm > s.webm
//
// Also:
// $ ffmpeg -i in.webm -c copy out.webm
//
// Then https://www.matroska.org/downloads/mkclean.html:
// $ mkclean --optimize
//
// Or use the webm-cleaner scirpt:
// $ webm-cleaner in.webm
export default function CanvasRecorder(
  canvas,
  audioStream,
  videoBitsPerSecond,
) {
  this.start = () => {
    const mimeType = [
      "video/webm;codecs=vp9",
      "video/webm",
      "video/vp8",
      "video/webm;codecs=vp8",
      "video/webm;codecs=daala",
      "video/webm;codecs=h264",
      "video/mpeg",
    ].find(MediaRecorder.isTypeSupported);

    if (!mimeType) {
      throw new Error("No supported mime type found for MediaRecorder");
    }

    const generator = new MediaStreamTrackGenerator({ kind: "video" });
    this._writer = generator.writable.getWriter();
    const streamToRecord = new MediaStream();
    streamToRecord.addTrack(generator);

    if (false) {
      // if (audioStream) {
      // FIXME: Do stuff
      // const videoTrack = videoStream.getVideoTracks()[0];
      const audioTrack = audioStream.getAudioTracks()[0];
      // const audioTrackProcessor = new MediaStreamTrackProcessor({ track: audioTrack });

      console.log("audio stream", audioStream, "audio track", audioTrack);
      // streamToRecord = new MediaStream([audioTrack, videoTrack]);
      videoStream.addTrack(audioTrack);
      streamToRecord = videoStream;
    } else {
      //streamToRecord = videoStream;
    }
    this.mediaRecorder = new MediaRecorder(streamToRecord, {
      mimeType,
      videoBitsPerSecond: videoBitsPerSecond || 5000000,
    });

    this.mediaRecorder.ondataavailable = (event) => {
      const url = window.URL.createObjectURL(event.data);
      const a = document.createElement("a");
      a.href = url;
      a.download = this.fileName;
      a.click();
      window.URL.revokeObjectURL(url);
    };

    this._frameCount = 0;
    this.mediaRecorder.start();
  };

  this.onFrame = async (canvas) => {
    const fps = 30;
    const timestamp = Math.floor((this._frameCount * 1000 * 1000) / fps);
    console.log("timestamp", timestamp);
    const frame = new VideoFrame(canvas, {
      timestamp: timestamp,
      duration: Math.floor(1000_000 / fps),
    });
    await this._writer.write(frame);
    this._frameCount++;
  };

  this.stopAndSave = (fileName) => {
    this.fileName = `${fileName}.webm`;
    this.mediaRecorder.stop();
  };
}
