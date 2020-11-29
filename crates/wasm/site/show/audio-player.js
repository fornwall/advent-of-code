// https://www.html5rocks.com/en/tutorials/webaudio/intro/
export async function createAudioPlayer(...urlList) {
  const context = new AudioContext();

  const promises = urlList.map(async (url) => {
      const response = await fetch(url);
      const responseBuffer = await response.arrayBuffer();
      return await context.decodeAudioData(responseBuffer);
  });
  const buffers = await Promise.all(promises);

  return {
    play: (soundId) => {
        const source = context.createBufferSource();
        source.buffer = buffers[soundId];
        source.connect(context.destination);
        source.start(0);
    },
    createStream: () => {
        // https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination
        const mediaStreamDestination = context.createMediaStreamDestination();
        return mediaStreamDestination;
    }
  };
}
