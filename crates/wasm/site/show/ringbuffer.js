const HEADER_ELEMENTS_LENGTH = 3;
const HEADER_BYTE_LENGTH = HEADER_ELEMENTS_LENGTH * Int32Array.BYTES_PER_ELEMENT;

const HEADER_READER_WANT_MORE_OFFSET = 0;
const HEADER_READ_OFFSET = 1;
const HEADER_WRITE_OFFSET = 2;

export function ReaderWithBuffer(sharedArrayBuffer, sharedArrayBufferOffset, length) {
  const headerBuffer = new Int32Array(sharedArrayBuffer, sharedArrayBufferOffset, HEADER_ELEMENTS_LENGTH); // FIXME: offset in bytes?
  const dataBuffer = new Int32Array(sharedArrayBuffer,
      sharedArrayBufferOffset + HEADER_BYTE_LENGTH,
      (length - HEADER_BYTE_LENGTH) / Int32Array.BYTES_PER_ELEMENT,
  );
  const dataFloatBuffer = new Float32Array(sharedArrayBuffer,
      sharedArrayBufferOffset + HEADER_BYTE_LENGTH,
      (length - HEADER_BYTE_LENGTH) / Float32Array.BYTES_PER_ELEMENT,
  );
  let unflushedReads = 0;

  const utf8decoder = new TextDecoder();

  this.hasNext = () => {
      const writerOffset = Atomics.load(headerBuffer, HEADER_WRITE_OFFSET);
      const readerOffset = (headerBuffer[HEADER_READ_OFFSET] + unflushedReads) % dataBuffer.length;
      return writerOffset != readerOffset;
  };

  this._readerPosition = () => {
    return headerBuffer[HEADER_READ_OFFSET] + unflushedReads;
  };

  this.next = () => {
      if (!this.hasNext()) {
          throw new Error('next() called despite !hasNext()');
      }
      const readerOffset = this._readerPosition() % dataBuffer.length;
      unflushedReads += 1;
      return dataBuffer[readerOffset];
  };

  this.nextFloat = () => {
      if (!this.hasNext()) {
          throw new Error('nextFloat() called despite !hasNext()');
      }
      const readerOffset = this._readerPosition() % dataBuffer.length;
      unflushedReads += 1;
      return dataFloatBuffer[readerOffset];
  };

  this.nextString = () => {
      const stringLengthInBytes = this.next();
      const stringLengthInI32 = Math.floor(stringLengthInBytes / 4) + (stringLengthInBytes % 4 == 0 ? 0 : 1);
      const stringArray = dataBuffer.slice(this._readerPosition(), this._readerPosition() + stringLengthInI32);
      unflushedReads += stringLengthInI32;
      return utf8decoder.decode(stringArray);
  };

  this.wantMore = () => {
      let readerOffset = headerBuffer[HEADER_READ_OFFSET];
      const writerOffset = headerBuffer[HEADER_WRITE_OFFSET];
      if (unflushedReads > 0) {
        readerOffset = (readerOffset + unflushedReads) % dataBuffer.length;
        Atomics.store(headerBuffer, HEADER_READ_OFFSET, readerOffset);
        unflushedReads = 0;
      }
      const used = readerOffset > writerOffset ? (writerOffset - readerOffset + dataBuffer.length) : (writerOffset - readerOffset);

      if (used < dataBuffer.length/3) {
        Atomics.store(headerBuffer, HEADER_READER_WANT_MORE_OFFSET, 1);
        Atomics.notify(headerBuffer, HEADER_READER_WANT_MORE_OFFSET);
      } else {
        console.log('not requesting more as utilisation=' + (used*1.0/dataBuffer.length));
      }
  };
}
