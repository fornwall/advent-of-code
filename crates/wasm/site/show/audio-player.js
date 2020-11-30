// https://www.html5rocks.com/en/tutorials/webaudio/intro/
export function AudioPlayer(...urlList) {
  // Needs to be called after user interaction.
  // https://developers.google.com/web/updates/2017/09/autoplay-policy-changes#webaudio
  this.load = async () => {
    this.context = new AudioContext();
    const promises = urlList.map(async (url) => {
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
    source.start(0);
  },

    this.createStream = () => {
      // https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination
      const mediaStreamDestination = context.createMediaStreamDestination();
      return mediaStreamDestination;
    };
}
