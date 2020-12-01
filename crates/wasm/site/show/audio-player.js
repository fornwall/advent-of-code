// https://www.html5rocks.com/en/tutorials/webaudio/intro/
export function AudioPlayer(...urlList) {
  // Needs to be called after user interaction.
  // https://developers.google.com/web/updates/2017/09/autoplay-policy-changes#webaudio
  this.load = async(recording) => {
    this.context = new AudioContext();
    if (recording) {
      this.mediaStreamDestination = this.context.createMediaStreamDestination();
    }
    const promises = urlList.map(async(url) => {
      const response = await fetch(url);
      const responseBuffer = await response.arrayBuffer();
      return await this.context.decodeAudioData(responseBuffer);
    });
    this.buffers = await Promise.all(promises);
  };

  this.play = (soundId) => {
    if (!this.buffers) return;
    const source = this.context.createBufferSource();
    source.buffer = this.buffers[soundId];
    source.connect(this.context.destination);
    if (this.mediaStreamDestination) {
      source.connect(this.mediaStreamDestination);
    }
    source.start(0);
  },

  this.createStream = () => {
    const silence = this.context.createGain();
    silence.gain.value = 0;
    silence.connect(this.mediaStreamDestination);

    const osc = this.context.createOscillator();
    osc.connect(silence);
    osc.start(0);

    // https://stackoverflow.com/questions/52134781/webrtc-video-audio-streams-out-of-sync-mediastream-mediarecorder-mediasou
    // https://stackoverflow.com/questions/40687010/canvascapturemediastream-mediarecorder-frame-synchronization
    // const osc = this.context.createOscillator();
    // osc.connect(this.mediaStreamDestination);
    // https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination
    return this.mediaStreamDestination.stream;
  };
}
