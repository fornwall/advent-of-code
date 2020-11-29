// https://www.html5rocks.com/en/tutorials/webaudio/intro/
export function AudioPlayer() {
  const context = new AudioContext();

  const self = this;
  const bufferLoader = new BufferLoader(context, [
    './bounce.mp4'
  ],
    (loadedBufferList) => {
      self.buffers = loadedBufferList;
      console.log('buffer list loaded');
    }
  );
  bufferLoader.load();

  this.play = (soundId) => {
    const source = context.createBufferSource();
    source.buffer = this.buffers[soundId];
    source.connect(context.destination);
    source.start(0);
  }
}

function BufferLoader(context, urlList, callback) {
  this.context = context;
  this.urlList = urlList;
  this.onload = callback;
  this.bufferList = new Array();
  this.loadCount = 0;
}

BufferLoader.prototype.loadBuffer = function (url, index) {
  // Load buffer asynchronously
  var request = new XMLHttpRequest();
  request.open("GET", url, true);
  request.responseType = "arraybuffer";

  var loader = this;

  request.onload = function () {
    // Asynchronously decode the audio file data in request.response
    loader.context.decodeAudioData(
      request.response,
      function (buffer) {
        if (!buffer) {
          alert('error decoding file data: ' + url);
          return;
        }
        loader.bufferList[index] = buffer;
        if (++loader.loadCount == loader.urlList.length)
          loader.onload(loader.bufferList);
      },
      function (error) {
        console.error('decodeAudioData error', error);
      }
    );
  }

  request.onerror = function () {
    alert('BufferLoader: XHR error');
  }

  request.send();
}

BufferLoader.prototype.load = function () {
  for (var i = 0; i < this.urlList.length; ++i)
    this.loadBuffer(this.urlList[i], i);
}
