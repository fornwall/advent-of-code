const HEADER_ELEMENTS_LENGTH = 3;
const HEADER_BYTE_LENGTH = HEADER_ELEMENTS_LENGTH * Int32Array.BYTES_PER_ELEMENT;

const HEADER_READER_WANT_MORE_OFFSET = 0;
const HEADER_READ_OFFSET = 1;
const HEADER_WRITE_OFFSET = 2;

export function ReaderWithBuffer(sharedArrayBuffer, offset, length) {
  const headerBuffer = new Int32Array(sharedArrayBuffer, offset, HEADER_ELEMENTS_LENGTH); // FIXME: offset in bytes?
  const dataBuffer = new Int32Array(sharedArrayBuffer, offset + HEADER_BYTE_LENGTH, (length - HEADER_BYTE_LENGTH) / Int32Array.BYTES_PER_ELEMENT);
  const dataFloatBuffer = new Float32Array(sharedArrayBuffer, offset + HEADER_BYTE_LENGTH, (length - HEADER_BYTE_LENGTH) / Float32Array.BYTES_PER_ELEMENT);
  let unflushedReads = 0;

  this.report = function() {
      console.info("readerOffset=" + headerBuffer[HEADER_READ_OFFSET] + ', unflushedReads=' + unflushedReads + ', writerOffset=' +headerBuffer[HEADER_WRITE_OFFSET] + ', length=' + dataBuffer.length);
  }

  this.hasNext = function() {
      const writerOffset = headerBuffer[HEADER_WRITE_OFFSET];
      const readerOffset = (headerBuffer[HEADER_READ_OFFSET] + unflushedReads) % dataBuffer.length;
      return writerOffset != readerOffset;
  };

  this.next = function() {
      if (!this.hasNext()) {
          throw new Error("next() called with !hasNext(). readerOffset=" + headerBuffer[HEADER_READ_OFFSET] + ', unflushedReads=' + unflushedReads + ', writerOffset=' +headerBuffer[HEADER_WRITE_OFFSET]);
      }
      const readerOffset = (headerBuffer[HEADER_READ_OFFSET] + unflushedReads) % dataBuffer.length;
      unflushedReads += 1;
      return dataBuffer[readerOffset];
  };

  this.nextFloat = function() {
      if (!this.hasNext()) {
          throw new Error("nextFloat() called with !hasNext(). readerOffset=" + headerBuffer[HEADER_READ_OFFSET] + ', unflushedReads=' + unflushedReads + ', writerOffset=' +headerBuffer[HEADER_WRITE_OFFSET]);
      }
      const readerOffset = (headerBuffer[HEADER_READ_OFFSET] + unflushedReads) % dataBuffer.length;
      unflushedReads += 1;
      return dataFloatBuffer[readerOffset];
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
