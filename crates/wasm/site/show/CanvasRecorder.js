export default function CanvasRecorder(canvas, videoBitsPerSecond) {
    this.start = () => {
        const mimeType = [
            'video/webm;codecs=vp9',
            "video/webm",
            'video/vp8',
            "video/webm;codecs=vp8",
            "video/webm;codecs=daala",
            "video/webm;codecs=h264",
            "video/mpeg"
        ].find(MediaRecorder.isTypeSupported);

        if (!mimeType) {
            throw new Error("No supported mime type found for MediaRecorder");
        }

        const stream = canvas.captureStream();
        this.mediaRecorder = new MediaRecorder(stream, {
            mimeType,
            videoBitsPerSecond: videoBitsPerSecond || 5000000
        });

        this.mediaRecorder.ondataavailable = (event) => {
            const url = window.URL.createObjectURL(event.data);
            const a = document.createElement('a');
            a.href = url;
            a.download = this.fileName;
            a.click();
            window.URL.revokeObjectURL(url);
        }

        this.mediaRecorder.start();
    }

    this.stopAndSave = (fileName) => {
        this.fileName = fileName || 'recording.webm';
        this.mediaRecorder.stop();
    }
}
