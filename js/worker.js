var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));

// ../node_modules/base64-js/index.js
var require_base64_js = __commonJS({
  "../node_modules/base64-js/index.js"(exports) {
    "use strict";
    exports.byteLength = byteLength;
    exports.toByteArray = toByteArray;
    exports.fromByteArray = fromByteArray;
    var lookup = [];
    var revLookup = [];
    var Arr = typeof Uint8Array !== "undefined" ? Uint8Array : Array;
    var code2 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for (i = 0, len = code2.length; i < len; ++i) {
      lookup[i] = code2[i];
      revLookup[code2.charCodeAt(i)] = i;
    }
    var i;
    var len;
    revLookup["-".charCodeAt(0)] = 62;
    revLookup["_".charCodeAt(0)] = 63;
    function getLens(b64) {
      var len2 = b64.length;
      if (len2 % 4 > 0) {
        throw new Error("Invalid string. Length must be a multiple of 4");
      }
      var validLen = b64.indexOf("=");
      if (validLen === -1) validLen = len2;
      var placeHoldersLen = validLen === len2 ? 0 : 4 - validLen % 4;
      return [validLen, placeHoldersLen];
    }
    function byteLength(b64) {
      var lens = getLens(b64);
      var validLen = lens[0];
      var placeHoldersLen = lens[1];
      return (validLen + placeHoldersLen) * 3 / 4 - placeHoldersLen;
    }
    function _byteLength(b64, validLen, placeHoldersLen) {
      return (validLen + placeHoldersLen) * 3 / 4 - placeHoldersLen;
    }
    function toByteArray(b64) {
      var tmp;
      var lens = getLens(b64);
      var validLen = lens[0];
      var placeHoldersLen = lens[1];
      var arr = new Arr(_byteLength(b64, validLen, placeHoldersLen));
      var curByte = 0;
      var len2 = placeHoldersLen > 0 ? validLen - 4 : validLen;
      var i2;
      for (i2 = 0; i2 < len2; i2 += 4) {
        tmp = revLookup[b64.charCodeAt(i2)] << 18 | revLookup[b64.charCodeAt(i2 + 1)] << 12 | revLookup[b64.charCodeAt(i2 + 2)] << 6 | revLookup[b64.charCodeAt(i2 + 3)];
        arr[curByte++] = tmp >> 16 & 255;
        arr[curByte++] = tmp >> 8 & 255;
        arr[curByte++] = tmp & 255;
      }
      if (placeHoldersLen === 2) {
        tmp = revLookup[b64.charCodeAt(i2)] << 2 | revLookup[b64.charCodeAt(i2 + 1)] >> 4;
        arr[curByte++] = tmp & 255;
      }
      if (placeHoldersLen === 1) {
        tmp = revLookup[b64.charCodeAt(i2)] << 10 | revLookup[b64.charCodeAt(i2 + 1)] << 4 | revLookup[b64.charCodeAt(i2 + 2)] >> 2;
        arr[curByte++] = tmp >> 8 & 255;
        arr[curByte++] = tmp & 255;
      }
      return arr;
    }
    function tripletToBase64(num) {
      return lookup[num >> 18 & 63] + lookup[num >> 12 & 63] + lookup[num >> 6 & 63] + lookup[num & 63];
    }
    function encodeChunk(uint8, start, end) {
      var tmp;
      var output = [];
      for (var i2 = start; i2 < end; i2 += 3) {
        tmp = (uint8[i2] << 16 & 16711680) + (uint8[i2 + 1] << 8 & 65280) + (uint8[i2 + 2] & 255);
        output.push(tripletToBase64(tmp));
      }
      return output.join("");
    }
    function fromByteArray(uint8) {
      var tmp;
      var len2 = uint8.length;
      var extraBytes = len2 % 3;
      var parts = [];
      var maxChunkLength = 16383;
      for (var i2 = 0, len22 = len2 - extraBytes; i2 < len22; i2 += maxChunkLength) {
        parts.push(encodeChunk(uint8, i2, i2 + maxChunkLength > len22 ? len22 : i2 + maxChunkLength));
      }
      if (extraBytes === 1) {
        tmp = uint8[len2 - 1];
        parts.push(
          lookup[tmp >> 2] + lookup[tmp << 4 & 63] + "=="
        );
      } else if (extraBytes === 2) {
        tmp = (uint8[len2 - 2] << 8) + uint8[len2 - 1];
        parts.push(
          lookup[tmp >> 10] + lookup[tmp >> 4 & 63] + lookup[tmp << 2 & 63] + "="
        );
      }
      return parts.join("");
    }
  }
});

// ../node_modules/ieee754/index.js
var require_ieee754 = __commonJS({
  "../node_modules/ieee754/index.js"(exports) {
    exports.read = function(buffer, offset, isLE, mLen, nBytes) {
      var e, m;
      var eLen = nBytes * 8 - mLen - 1;
      var eMax = (1 << eLen) - 1;
      var eBias = eMax >> 1;
      var nBits = -7;
      var i = isLE ? nBytes - 1 : 0;
      var d = isLE ? -1 : 1;
      var s = buffer[offset + i];
      i += d;
      e = s & (1 << -nBits) - 1;
      s >>= -nBits;
      nBits += eLen;
      for (; nBits > 0; e = e * 256 + buffer[offset + i], i += d, nBits -= 8) {
      }
      m = e & (1 << -nBits) - 1;
      e >>= -nBits;
      nBits += mLen;
      for (; nBits > 0; m = m * 256 + buffer[offset + i], i += d, nBits -= 8) {
      }
      if (e === 0) {
        e = 1 - eBias;
      } else if (e === eMax) {
        return m ? NaN : (s ? -1 : 1) * Infinity;
      } else {
        m = m + Math.pow(2, mLen);
        e = e - eBias;
      }
      return (s ? -1 : 1) * m * Math.pow(2, e - mLen);
    };
    exports.write = function(buffer, value, offset, isLE, mLen, nBytes) {
      var e, m, c;
      var eLen = nBytes * 8 - mLen - 1;
      var eMax = (1 << eLen) - 1;
      var eBias = eMax >> 1;
      var rt = mLen === 23 ? Math.pow(2, -24) - Math.pow(2, -77) : 0;
      var i = isLE ? 0 : nBytes - 1;
      var d = isLE ? 1 : -1;
      var s = value < 0 || value === 0 && 1 / value < 0 ? 1 : 0;
      value = Math.abs(value);
      if (isNaN(value) || value === Infinity) {
        m = isNaN(value) ? 1 : 0;
        e = eMax;
      } else {
        e = Math.floor(Math.log(value) / Math.LN2);
        if (value * (c = Math.pow(2, -e)) < 1) {
          e--;
          c *= 2;
        }
        if (e + eBias >= 1) {
          value += rt / c;
        } else {
          value += rt * Math.pow(2, 1 - eBias);
        }
        if (value * c >= 2) {
          e++;
          c /= 2;
        }
        if (e + eBias >= eMax) {
          m = 0;
          e = eMax;
        } else if (e + eBias >= 1) {
          m = (value * c - 1) * Math.pow(2, mLen);
          e = e + eBias;
        } else {
          m = value * Math.pow(2, eBias - 1) * Math.pow(2, mLen);
          e = 0;
        }
      }
      for (; mLen >= 8; buffer[offset + i] = m & 255, i += d, m /= 256, mLen -= 8) {
      }
      e = e << mLen | m;
      eLen += mLen;
      for (; eLen > 0; buffer[offset + i] = e & 255, i += d, e /= 256, eLen -= 8) {
      }
      buffer[offset + i - d] |= s * 128;
    };
  }
});

// ../node_modules/buffer/index.js
var require_buffer = __commonJS({
  "../node_modules/buffer/index.js"(exports) {
    "use strict";
    var base64 = require_base64_js();
    var ieee754 = require_ieee754();
    var customInspectSymbol = typeof Symbol === "function" && typeof Symbol["for"] === "function" ? Symbol["for"]("nodejs.util.inspect.custom") : null;
    exports.Buffer = Buffer4;
    exports.SlowBuffer = SlowBuffer;
    exports.INSPECT_MAX_BYTES = 50;
    var K_MAX_LENGTH = 2147483647;
    exports.kMaxLength = K_MAX_LENGTH;
    Buffer4.TYPED_ARRAY_SUPPORT = typedArraySupport();
    if (!Buffer4.TYPED_ARRAY_SUPPORT && typeof console !== "undefined" && typeof console.error === "function") {
      console.error(
        "This browser lacks typed array (Uint8Array) support which is required by `buffer` v5.x. Use `buffer` v4.x if you require old browser support."
      );
    }
    function typedArraySupport() {
      try {
        const arr = new Uint8Array(1);
        const proto = { foo: function() {
          return 42;
        } };
        Object.setPrototypeOf(proto, Uint8Array.prototype);
        Object.setPrototypeOf(arr, proto);
        return arr.foo() === 42;
      } catch (e) {
        return false;
      }
    }
    Object.defineProperty(Buffer4.prototype, "parent", {
      enumerable: true,
      get: function() {
        if (!Buffer4.isBuffer(this)) return void 0;
        return this.buffer;
      }
    });
    Object.defineProperty(Buffer4.prototype, "offset", {
      enumerable: true,
      get: function() {
        if (!Buffer4.isBuffer(this)) return void 0;
        return this.byteOffset;
      }
    });
    function createBuffer(length) {
      if (length > K_MAX_LENGTH) {
        throw new RangeError('The value "' + length + '" is invalid for option "size"');
      }
      const buf = new Uint8Array(length);
      Object.setPrototypeOf(buf, Buffer4.prototype);
      return buf;
    }
    function Buffer4(arg, encodingOrOffset, length) {
      if (typeof arg === "number") {
        if (typeof encodingOrOffset === "string") {
          throw new TypeError(
            'The "string" argument must be of type string. Received type number'
          );
        }
        return allocUnsafe(arg);
      }
      return from(arg, encodingOrOffset, length);
    }
    Buffer4.poolSize = 8192;
    function from(value, encodingOrOffset, length) {
      if (typeof value === "string") {
        return fromString(value, encodingOrOffset);
      }
      if (ArrayBuffer.isView(value)) {
        return fromArrayView(value);
      }
      if (value == null) {
        throw new TypeError(
          "The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type " + typeof value
        );
      }
      if (isInstance(value, ArrayBuffer) || value && isInstance(value.buffer, ArrayBuffer)) {
        return fromArrayBuffer(value, encodingOrOffset, length);
      }
      if (typeof SharedArrayBuffer !== "undefined" && (isInstance(value, SharedArrayBuffer) || value && isInstance(value.buffer, SharedArrayBuffer))) {
        return fromArrayBuffer(value, encodingOrOffset, length);
      }
      if (typeof value === "number") {
        throw new TypeError(
          'The "value" argument must not be of type number. Received type number'
        );
      }
      const valueOf = value.valueOf && value.valueOf();
      if (valueOf != null && valueOf !== value) {
        return Buffer4.from(valueOf, encodingOrOffset, length);
      }
      const b = fromObject(value);
      if (b) return b;
      if (typeof Symbol !== "undefined" && Symbol.toPrimitive != null && typeof value[Symbol.toPrimitive] === "function") {
        return Buffer4.from(value[Symbol.toPrimitive]("string"), encodingOrOffset, length);
      }
      throw new TypeError(
        "The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type " + typeof value
      );
    }
    Buffer4.from = function(value, encodingOrOffset, length) {
      return from(value, encodingOrOffset, length);
    };
    Object.setPrototypeOf(Buffer4.prototype, Uint8Array.prototype);
    Object.setPrototypeOf(Buffer4, Uint8Array);
    function assertSize(size) {
      if (typeof size !== "number") {
        throw new TypeError('"size" argument must be of type number');
      } else if (size < 0) {
        throw new RangeError('The value "' + size + '" is invalid for option "size"');
      }
    }
    function alloc(size, fill, encoding) {
      assertSize(size);
      if (size <= 0) {
        return createBuffer(size);
      }
      if (fill !== void 0) {
        return typeof encoding === "string" ? createBuffer(size).fill(fill, encoding) : createBuffer(size).fill(fill);
      }
      return createBuffer(size);
    }
    Buffer4.alloc = function(size, fill, encoding) {
      return alloc(size, fill, encoding);
    };
    function allocUnsafe(size) {
      assertSize(size);
      return createBuffer(size < 0 ? 0 : checked(size) | 0);
    }
    Buffer4.allocUnsafe = function(size) {
      return allocUnsafe(size);
    };
    Buffer4.allocUnsafeSlow = function(size) {
      return allocUnsafe(size);
    };
    function fromString(string, encoding) {
      if (typeof encoding !== "string" || encoding === "") {
        encoding = "utf8";
      }
      if (!Buffer4.isEncoding(encoding)) {
        throw new TypeError("Unknown encoding: " + encoding);
      }
      const length = byteLength(string, encoding) | 0;
      let buf = createBuffer(length);
      const actual = buf.write(string, encoding);
      if (actual !== length) {
        buf = buf.slice(0, actual);
      }
      return buf;
    }
    function fromArrayLike(array) {
      const length = array.length < 0 ? 0 : checked(array.length) | 0;
      const buf = createBuffer(length);
      for (let i = 0; i < length; i += 1) {
        buf[i] = array[i] & 255;
      }
      return buf;
    }
    function fromArrayView(arrayView) {
      if (isInstance(arrayView, Uint8Array)) {
        const copy = new Uint8Array(arrayView);
        return fromArrayBuffer(copy.buffer, copy.byteOffset, copy.byteLength);
      }
      return fromArrayLike(arrayView);
    }
    function fromArrayBuffer(array, byteOffset, length) {
      if (byteOffset < 0 || array.byteLength < byteOffset) {
        throw new RangeError('"offset" is outside of buffer bounds');
      }
      if (array.byteLength < byteOffset + (length || 0)) {
        throw new RangeError('"length" is outside of buffer bounds');
      }
      let buf;
      if (byteOffset === void 0 && length === void 0) {
        buf = new Uint8Array(array);
      } else if (length === void 0) {
        buf = new Uint8Array(array, byteOffset);
      } else {
        buf = new Uint8Array(array, byteOffset, length);
      }
      Object.setPrototypeOf(buf, Buffer4.prototype);
      return buf;
    }
    function fromObject(obj) {
      if (Buffer4.isBuffer(obj)) {
        const len = checked(obj.length) | 0;
        const buf = createBuffer(len);
        if (buf.length === 0) {
          return buf;
        }
        obj.copy(buf, 0, 0, len);
        return buf;
      }
      if (obj.length !== void 0) {
        if (typeof obj.length !== "number" || numberIsNaN(obj.length)) {
          return createBuffer(0);
        }
        return fromArrayLike(obj);
      }
      if (obj.type === "Buffer" && Array.isArray(obj.data)) {
        return fromArrayLike(obj.data);
      }
    }
    function checked(length) {
      if (length >= K_MAX_LENGTH) {
        throw new RangeError("Attempt to allocate Buffer larger than maximum size: 0x" + K_MAX_LENGTH.toString(16) + " bytes");
      }
      return length | 0;
    }
    function SlowBuffer(length) {
      if (+length != length) {
        length = 0;
      }
      return Buffer4.alloc(+length);
    }
    Buffer4.isBuffer = function isBuffer(b) {
      return b != null && b._isBuffer === true && b !== Buffer4.prototype;
    };
    Buffer4.compare = function compare(a, b) {
      if (isInstance(a, Uint8Array)) a = Buffer4.from(a, a.offset, a.byteLength);
      if (isInstance(b, Uint8Array)) b = Buffer4.from(b, b.offset, b.byteLength);
      if (!Buffer4.isBuffer(a) || !Buffer4.isBuffer(b)) {
        throw new TypeError(
          'The "buf1", "buf2" arguments must be one of type Buffer or Uint8Array'
        );
      }
      if (a === b) return 0;
      let x = a.length;
      let y = b.length;
      for (let i = 0, len = Math.min(x, y); i < len; ++i) {
        if (a[i] !== b[i]) {
          x = a[i];
          y = b[i];
          break;
        }
      }
      if (x < y) return -1;
      if (y < x) return 1;
      return 0;
    };
    Buffer4.isEncoding = function isEncoding(encoding) {
      switch (String(encoding).toLowerCase()) {
        case "hex":
        case "utf8":
        case "utf-8":
        case "ascii":
        case "latin1":
        case "binary":
        case "base64":
        case "ucs2":
        case "ucs-2":
        case "utf16le":
        case "utf-16le":
          return true;
        default:
          return false;
      }
    };
    Buffer4.concat = function concat(list, length) {
      if (!Array.isArray(list)) {
        throw new TypeError('"list" argument must be an Array of Buffers');
      }
      if (list.length === 0) {
        return Buffer4.alloc(0);
      }
      let i;
      if (length === void 0) {
        length = 0;
        for (i = 0; i < list.length; ++i) {
          length += list[i].length;
        }
      }
      const buffer = Buffer4.allocUnsafe(length);
      let pos = 0;
      for (i = 0; i < list.length; ++i) {
        let buf = list[i];
        if (isInstance(buf, Uint8Array)) {
          if (pos + buf.length > buffer.length) {
            if (!Buffer4.isBuffer(buf)) buf = Buffer4.from(buf);
            buf.copy(buffer, pos);
          } else {
            Uint8Array.prototype.set.call(
              buffer,
              buf,
              pos
            );
          }
        } else if (!Buffer4.isBuffer(buf)) {
          throw new TypeError('"list" argument must be an Array of Buffers');
        } else {
          buf.copy(buffer, pos);
        }
        pos += buf.length;
      }
      return buffer;
    };
    function byteLength(string, encoding) {
      if (Buffer4.isBuffer(string)) {
        return string.length;
      }
      if (ArrayBuffer.isView(string) || isInstance(string, ArrayBuffer)) {
        return string.byteLength;
      }
      if (typeof string !== "string") {
        throw new TypeError(
          'The "string" argument must be one of type string, Buffer, or ArrayBuffer. Received type ' + typeof string
        );
      }
      const len = string.length;
      const mustMatch = arguments.length > 2 && arguments[2] === true;
      if (!mustMatch && len === 0) return 0;
      let loweredCase = false;
      for (; ; ) {
        switch (encoding) {
          case "ascii":
          case "latin1":
          case "binary":
            return len;
          case "utf8":
          case "utf-8":
            return utf8ToBytes(string).length;
          case "ucs2":
          case "ucs-2":
          case "utf16le":
          case "utf-16le":
            return len * 2;
          case "hex":
            return len >>> 1;
          case "base64":
            return base64ToBytes(string).length;
          default:
            if (loweredCase) {
              return mustMatch ? -1 : utf8ToBytes(string).length;
            }
            encoding = ("" + encoding).toLowerCase();
            loweredCase = true;
        }
      }
    }
    Buffer4.byteLength = byteLength;
    function slowToString(encoding, start, end) {
      let loweredCase = false;
      if (start === void 0 || start < 0) {
        start = 0;
      }
      if (start > this.length) {
        return "";
      }
      if (end === void 0 || end > this.length) {
        end = this.length;
      }
      if (end <= 0) {
        return "";
      }
      end >>>= 0;
      start >>>= 0;
      if (end <= start) {
        return "";
      }
      if (!encoding) encoding = "utf8";
      while (true) {
        switch (encoding) {
          case "hex":
            return hexSlice(this, start, end);
          case "utf8":
          case "utf-8":
            return utf8Slice(this, start, end);
          case "ascii":
            return asciiSlice(this, start, end);
          case "latin1":
          case "binary":
            return latin1Slice(this, start, end);
          case "base64":
            return base64Slice(this, start, end);
          case "ucs2":
          case "ucs-2":
          case "utf16le":
          case "utf-16le":
            return utf16leSlice(this, start, end);
          default:
            if (loweredCase) throw new TypeError("Unknown encoding: " + encoding);
            encoding = (encoding + "").toLowerCase();
            loweredCase = true;
        }
      }
    }
    Buffer4.prototype._isBuffer = true;
    function swap(b, n, m) {
      const i = b[n];
      b[n] = b[m];
      b[m] = i;
    }
    Buffer4.prototype.swap16 = function swap16() {
      const len = this.length;
      if (len % 2 !== 0) {
        throw new RangeError("Buffer size must be a multiple of 16-bits");
      }
      for (let i = 0; i < len; i += 2) {
        swap(this, i, i + 1);
      }
      return this;
    };
    Buffer4.prototype.swap32 = function swap32() {
      const len = this.length;
      if (len % 4 !== 0) {
        throw new RangeError("Buffer size must be a multiple of 32-bits");
      }
      for (let i = 0; i < len; i += 4) {
        swap(this, i, i + 3);
        swap(this, i + 1, i + 2);
      }
      return this;
    };
    Buffer4.prototype.swap64 = function swap64() {
      const len = this.length;
      if (len % 8 !== 0) {
        throw new RangeError("Buffer size must be a multiple of 64-bits");
      }
      for (let i = 0; i < len; i += 8) {
        swap(this, i, i + 7);
        swap(this, i + 1, i + 6);
        swap(this, i + 2, i + 5);
        swap(this, i + 3, i + 4);
      }
      return this;
    };
    Buffer4.prototype.toString = function toString() {
      const length = this.length;
      if (length === 0) return "";
      if (arguments.length === 0) return utf8Slice(this, 0, length);
      return slowToString.apply(this, arguments);
    };
    Buffer4.prototype.toLocaleString = Buffer4.prototype.toString;
    Buffer4.prototype.equals = function equals(b) {
      if (!Buffer4.isBuffer(b)) throw new TypeError("Argument must be a Buffer");
      if (this === b) return true;
      return Buffer4.compare(this, b) === 0;
    };
    Buffer4.prototype.inspect = function inspect() {
      let str = "";
      const max = exports.INSPECT_MAX_BYTES;
      str = this.toString("hex", 0, max).replace(/(.{2})/g, "$1 ").trim();
      if (this.length > max) str += " ... ";
      return "<Buffer " + str + ">";
    };
    if (customInspectSymbol) {
      Buffer4.prototype[customInspectSymbol] = Buffer4.prototype.inspect;
    }
    Buffer4.prototype.compare = function compare(target, start, end, thisStart, thisEnd) {
      if (isInstance(target, Uint8Array)) {
        target = Buffer4.from(target, target.offset, target.byteLength);
      }
      if (!Buffer4.isBuffer(target)) {
        throw new TypeError(
          'The "target" argument must be one of type Buffer or Uint8Array. Received type ' + typeof target
        );
      }
      if (start === void 0) {
        start = 0;
      }
      if (end === void 0) {
        end = target ? target.length : 0;
      }
      if (thisStart === void 0) {
        thisStart = 0;
      }
      if (thisEnd === void 0) {
        thisEnd = this.length;
      }
      if (start < 0 || end > target.length || thisStart < 0 || thisEnd > this.length) {
        throw new RangeError("out of range index");
      }
      if (thisStart >= thisEnd && start >= end) {
        return 0;
      }
      if (thisStart >= thisEnd) {
        return -1;
      }
      if (start >= end) {
        return 1;
      }
      start >>>= 0;
      end >>>= 0;
      thisStart >>>= 0;
      thisEnd >>>= 0;
      if (this === target) return 0;
      let x = thisEnd - thisStart;
      let y = end - start;
      const len = Math.min(x, y);
      const thisCopy = this.slice(thisStart, thisEnd);
      const targetCopy = target.slice(start, end);
      for (let i = 0; i < len; ++i) {
        if (thisCopy[i] !== targetCopy[i]) {
          x = thisCopy[i];
          y = targetCopy[i];
          break;
        }
      }
      if (x < y) return -1;
      if (y < x) return 1;
      return 0;
    };
    function bidirectionalIndexOf(buffer, val, byteOffset, encoding, dir) {
      if (buffer.length === 0) return -1;
      if (typeof byteOffset === "string") {
        encoding = byteOffset;
        byteOffset = 0;
      } else if (byteOffset > 2147483647) {
        byteOffset = 2147483647;
      } else if (byteOffset < -2147483648) {
        byteOffset = -2147483648;
      }
      byteOffset = +byteOffset;
      if (numberIsNaN(byteOffset)) {
        byteOffset = dir ? 0 : buffer.length - 1;
      }
      if (byteOffset < 0) byteOffset = buffer.length + byteOffset;
      if (byteOffset >= buffer.length) {
        if (dir) return -1;
        else byteOffset = buffer.length - 1;
      } else if (byteOffset < 0) {
        if (dir) byteOffset = 0;
        else return -1;
      }
      if (typeof val === "string") {
        val = Buffer4.from(val, encoding);
      }
      if (Buffer4.isBuffer(val)) {
        if (val.length === 0) {
          return -1;
        }
        return arrayIndexOf(buffer, val, byteOffset, encoding, dir);
      } else if (typeof val === "number") {
        val = val & 255;
        if (typeof Uint8Array.prototype.indexOf === "function") {
          if (dir) {
            return Uint8Array.prototype.indexOf.call(buffer, val, byteOffset);
          } else {
            return Uint8Array.prototype.lastIndexOf.call(buffer, val, byteOffset);
          }
        }
        return arrayIndexOf(buffer, [val], byteOffset, encoding, dir);
      }
      throw new TypeError("val must be string, number or Buffer");
    }
    function arrayIndexOf(arr, val, byteOffset, encoding, dir) {
      let indexSize = 1;
      let arrLength = arr.length;
      let valLength = val.length;
      if (encoding !== void 0) {
        encoding = String(encoding).toLowerCase();
        if (encoding === "ucs2" || encoding === "ucs-2" || encoding === "utf16le" || encoding === "utf-16le") {
          if (arr.length < 2 || val.length < 2) {
            return -1;
          }
          indexSize = 2;
          arrLength /= 2;
          valLength /= 2;
          byteOffset /= 2;
        }
      }
      function read(buf, i2) {
        if (indexSize === 1) {
          return buf[i2];
        } else {
          return buf.readUInt16BE(i2 * indexSize);
        }
      }
      let i;
      if (dir) {
        let foundIndex = -1;
        for (i = byteOffset; i < arrLength; i++) {
          if (read(arr, i) === read(val, foundIndex === -1 ? 0 : i - foundIndex)) {
            if (foundIndex === -1) foundIndex = i;
            if (i - foundIndex + 1 === valLength) return foundIndex * indexSize;
          } else {
            if (foundIndex !== -1) i -= i - foundIndex;
            foundIndex = -1;
          }
        }
      } else {
        if (byteOffset + valLength > arrLength) byteOffset = arrLength - valLength;
        for (i = byteOffset; i >= 0; i--) {
          let found = true;
          for (let j = 0; j < valLength; j++) {
            if (read(arr, i + j) !== read(val, j)) {
              found = false;
              break;
            }
          }
          if (found) return i;
        }
      }
      return -1;
    }
    Buffer4.prototype.includes = function includes(val, byteOffset, encoding) {
      return this.indexOf(val, byteOffset, encoding) !== -1;
    };
    Buffer4.prototype.indexOf = function indexOf(val, byteOffset, encoding) {
      return bidirectionalIndexOf(this, val, byteOffset, encoding, true);
    };
    Buffer4.prototype.lastIndexOf = function lastIndexOf(val, byteOffset, encoding) {
      return bidirectionalIndexOf(this, val, byteOffset, encoding, false);
    };
    function hexWrite(buf, string, offset, length) {
      offset = Number(offset) || 0;
      const remaining = buf.length - offset;
      if (!length) {
        length = remaining;
      } else {
        length = Number(length);
        if (length > remaining) {
          length = remaining;
        }
      }
      const strLen = string.length;
      if (length > strLen / 2) {
        length = strLen / 2;
      }
      let i;
      for (i = 0; i < length; ++i) {
        const parsed = parseInt(string.substr(i * 2, 2), 16);
        if (numberIsNaN(parsed)) return i;
        buf[offset + i] = parsed;
      }
      return i;
    }
    function utf8Write(buf, string, offset, length) {
      return blitBuffer(utf8ToBytes(string, buf.length - offset), buf, offset, length);
    }
    function asciiWrite(buf, string, offset, length) {
      return blitBuffer(asciiToBytes(string), buf, offset, length);
    }
    function base64Write(buf, string, offset, length) {
      return blitBuffer(base64ToBytes(string), buf, offset, length);
    }
    function ucs2Write(buf, string, offset, length) {
      return blitBuffer(utf16leToBytes(string, buf.length - offset), buf, offset, length);
    }
    Buffer4.prototype.write = function write(string, offset, length, encoding) {
      if (offset === void 0) {
        encoding = "utf8";
        length = this.length;
        offset = 0;
      } else if (length === void 0 && typeof offset === "string") {
        encoding = offset;
        length = this.length;
        offset = 0;
      } else if (isFinite(offset)) {
        offset = offset >>> 0;
        if (isFinite(length)) {
          length = length >>> 0;
          if (encoding === void 0) encoding = "utf8";
        } else {
          encoding = length;
          length = void 0;
        }
      } else {
        throw new Error(
          "Buffer.write(string, encoding, offset[, length]) is no longer supported"
        );
      }
      const remaining = this.length - offset;
      if (length === void 0 || length > remaining) length = remaining;
      if (string.length > 0 && (length < 0 || offset < 0) || offset > this.length) {
        throw new RangeError("Attempt to write outside buffer bounds");
      }
      if (!encoding) encoding = "utf8";
      let loweredCase = false;
      for (; ; ) {
        switch (encoding) {
          case "hex":
            return hexWrite(this, string, offset, length);
          case "utf8":
          case "utf-8":
            return utf8Write(this, string, offset, length);
          case "ascii":
          case "latin1":
          case "binary":
            return asciiWrite(this, string, offset, length);
          case "base64":
            return base64Write(this, string, offset, length);
          case "ucs2":
          case "ucs-2":
          case "utf16le":
          case "utf-16le":
            return ucs2Write(this, string, offset, length);
          default:
            if (loweredCase) throw new TypeError("Unknown encoding: " + encoding);
            encoding = ("" + encoding).toLowerCase();
            loweredCase = true;
        }
      }
    };
    Buffer4.prototype.toJSON = function toJSON() {
      return {
        type: "Buffer",
        data: Array.prototype.slice.call(this._arr || this, 0)
      };
    };
    function base64Slice(buf, start, end) {
      if (start === 0 && end === buf.length) {
        return base64.fromByteArray(buf);
      } else {
        return base64.fromByteArray(buf.slice(start, end));
      }
    }
    function utf8Slice(buf, start, end) {
      end = Math.min(buf.length, end);
      const res = [];
      let i = start;
      while (i < end) {
        const firstByte = buf[i];
        let codePoint = null;
        let bytesPerSequence = firstByte > 239 ? 4 : firstByte > 223 ? 3 : firstByte > 191 ? 2 : 1;
        if (i + bytesPerSequence <= end) {
          let secondByte, thirdByte, fourthByte, tempCodePoint;
          switch (bytesPerSequence) {
            case 1:
              if (firstByte < 128) {
                codePoint = firstByte;
              }
              break;
            case 2:
              secondByte = buf[i + 1];
              if ((secondByte & 192) === 128) {
                tempCodePoint = (firstByte & 31) << 6 | secondByte & 63;
                if (tempCodePoint > 127) {
                  codePoint = tempCodePoint;
                }
              }
              break;
            case 3:
              secondByte = buf[i + 1];
              thirdByte = buf[i + 2];
              if ((secondByte & 192) === 128 && (thirdByte & 192) === 128) {
                tempCodePoint = (firstByte & 15) << 12 | (secondByte & 63) << 6 | thirdByte & 63;
                if (tempCodePoint > 2047 && (tempCodePoint < 55296 || tempCodePoint > 57343)) {
                  codePoint = tempCodePoint;
                }
              }
              break;
            case 4:
              secondByte = buf[i + 1];
              thirdByte = buf[i + 2];
              fourthByte = buf[i + 3];
              if ((secondByte & 192) === 128 && (thirdByte & 192) === 128 && (fourthByte & 192) === 128) {
                tempCodePoint = (firstByte & 15) << 18 | (secondByte & 63) << 12 | (thirdByte & 63) << 6 | fourthByte & 63;
                if (tempCodePoint > 65535 && tempCodePoint < 1114112) {
                  codePoint = tempCodePoint;
                }
              }
          }
        }
        if (codePoint === null) {
          codePoint = 65533;
          bytesPerSequence = 1;
        } else if (codePoint > 65535) {
          codePoint -= 65536;
          res.push(codePoint >>> 10 & 1023 | 55296);
          codePoint = 56320 | codePoint & 1023;
        }
        res.push(codePoint);
        i += bytesPerSequence;
      }
      return decodeCodePointsArray(res);
    }
    var MAX_ARGUMENTS_LENGTH = 4096;
    function decodeCodePointsArray(codePoints) {
      const len = codePoints.length;
      if (len <= MAX_ARGUMENTS_LENGTH) {
        return String.fromCharCode.apply(String, codePoints);
      }
      let res = "";
      let i = 0;
      while (i < len) {
        res += String.fromCharCode.apply(
          String,
          codePoints.slice(i, i += MAX_ARGUMENTS_LENGTH)
        );
      }
      return res;
    }
    function asciiSlice(buf, start, end) {
      let ret = "";
      end = Math.min(buf.length, end);
      for (let i = start; i < end; ++i) {
        ret += String.fromCharCode(buf[i] & 127);
      }
      return ret;
    }
    function latin1Slice(buf, start, end) {
      let ret = "";
      end = Math.min(buf.length, end);
      for (let i = start; i < end; ++i) {
        ret += String.fromCharCode(buf[i]);
      }
      return ret;
    }
    function hexSlice(buf, start, end) {
      const len = buf.length;
      if (!start || start < 0) start = 0;
      if (!end || end < 0 || end > len) end = len;
      let out = "";
      for (let i = start; i < end; ++i) {
        out += hexSliceLookupTable[buf[i]];
      }
      return out;
    }
    function utf16leSlice(buf, start, end) {
      const bytes = buf.slice(start, end);
      let res = "";
      for (let i = 0; i < bytes.length - 1; i += 2) {
        res += String.fromCharCode(bytes[i] + bytes[i + 1] * 256);
      }
      return res;
    }
    Buffer4.prototype.slice = function slice(start, end) {
      const len = this.length;
      start = ~~start;
      end = end === void 0 ? len : ~~end;
      if (start < 0) {
        start += len;
        if (start < 0) start = 0;
      } else if (start > len) {
        start = len;
      }
      if (end < 0) {
        end += len;
        if (end < 0) end = 0;
      } else if (end > len) {
        end = len;
      }
      if (end < start) end = start;
      const newBuf = this.subarray(start, end);
      Object.setPrototypeOf(newBuf, Buffer4.prototype);
      return newBuf;
    };
    function checkOffset(offset, ext, length) {
      if (offset % 1 !== 0 || offset < 0) throw new RangeError("offset is not uint");
      if (offset + ext > length) throw new RangeError("Trying to access beyond buffer length");
    }
    Buffer4.prototype.readUintLE = Buffer4.prototype.readUIntLE = function readUIntLE(offset, byteLength2, noAssert) {
      offset = offset >>> 0;
      byteLength2 = byteLength2 >>> 0;
      if (!noAssert) checkOffset(offset, byteLength2, this.length);
      let val = this[offset];
      let mul = 1;
      let i = 0;
      while (++i < byteLength2 && (mul *= 256)) {
        val += this[offset + i] * mul;
      }
      return val;
    };
    Buffer4.prototype.readUintBE = Buffer4.prototype.readUIntBE = function readUIntBE(offset, byteLength2, noAssert) {
      offset = offset >>> 0;
      byteLength2 = byteLength2 >>> 0;
      if (!noAssert) {
        checkOffset(offset, byteLength2, this.length);
      }
      let val = this[offset + --byteLength2];
      let mul = 1;
      while (byteLength2 > 0 && (mul *= 256)) {
        val += this[offset + --byteLength2] * mul;
      }
      return val;
    };
    Buffer4.prototype.readUint8 = Buffer4.prototype.readUInt8 = function readUInt8(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 1, this.length);
      return this[offset];
    };
    Buffer4.prototype.readUint16LE = Buffer4.prototype.readUInt16LE = function readUInt16LE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 2, this.length);
      return this[offset] | this[offset + 1] << 8;
    };
    Buffer4.prototype.readUint16BE = Buffer4.prototype.readUInt16BE = function readUInt16BE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 2, this.length);
      return this[offset] << 8 | this[offset + 1];
    };
    Buffer4.prototype.readUint32LE = Buffer4.prototype.readUInt32LE = function readUInt32LE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 4, this.length);
      return (this[offset] | this[offset + 1] << 8 | this[offset + 2] << 16) + this[offset + 3] * 16777216;
    };
    Buffer4.prototype.readUint32BE = Buffer4.prototype.readUInt32BE = function readUInt32BE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 4, this.length);
      return this[offset] * 16777216 + (this[offset + 1] << 16 | this[offset + 2] << 8 | this[offset + 3]);
    };
    Buffer4.prototype.readBigUInt64LE = defineBigIntMethod(function readBigUInt64LE(offset) {
      offset = offset >>> 0;
      validateNumber(offset, "offset");
      const first = this[offset];
      const last = this[offset + 7];
      if (first === void 0 || last === void 0) {
        boundsError(offset, this.length - 8);
      }
      const lo = first + this[++offset] * 2 ** 8 + this[++offset] * 2 ** 16 + this[++offset] * 2 ** 24;
      const hi = this[++offset] + this[++offset] * 2 ** 8 + this[++offset] * 2 ** 16 + last * 2 ** 24;
      return BigInt(lo) + (BigInt(hi) << BigInt(32));
    });
    Buffer4.prototype.readBigUInt64BE = defineBigIntMethod(function readBigUInt64BE(offset) {
      offset = offset >>> 0;
      validateNumber(offset, "offset");
      const first = this[offset];
      const last = this[offset + 7];
      if (first === void 0 || last === void 0) {
        boundsError(offset, this.length - 8);
      }
      const hi = first * 2 ** 24 + this[++offset] * 2 ** 16 + this[++offset] * 2 ** 8 + this[++offset];
      const lo = this[++offset] * 2 ** 24 + this[++offset] * 2 ** 16 + this[++offset] * 2 ** 8 + last;
      return (BigInt(hi) << BigInt(32)) + BigInt(lo);
    });
    Buffer4.prototype.readIntLE = function readIntLE(offset, byteLength2, noAssert) {
      offset = offset >>> 0;
      byteLength2 = byteLength2 >>> 0;
      if (!noAssert) checkOffset(offset, byteLength2, this.length);
      let val = this[offset];
      let mul = 1;
      let i = 0;
      while (++i < byteLength2 && (mul *= 256)) {
        val += this[offset + i] * mul;
      }
      mul *= 128;
      if (val >= mul) val -= Math.pow(2, 8 * byteLength2);
      return val;
    };
    Buffer4.prototype.readIntBE = function readIntBE(offset, byteLength2, noAssert) {
      offset = offset >>> 0;
      byteLength2 = byteLength2 >>> 0;
      if (!noAssert) checkOffset(offset, byteLength2, this.length);
      let i = byteLength2;
      let mul = 1;
      let val = this[offset + --i];
      while (i > 0 && (mul *= 256)) {
        val += this[offset + --i] * mul;
      }
      mul *= 128;
      if (val >= mul) val -= Math.pow(2, 8 * byteLength2);
      return val;
    };
    Buffer4.prototype.readInt8 = function readInt8(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 1, this.length);
      if (!(this[offset] & 128)) return this[offset];
      return (255 - this[offset] + 1) * -1;
    };
    Buffer4.prototype.readInt16LE = function readInt16LE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 2, this.length);
      const val = this[offset] | this[offset + 1] << 8;
      return val & 32768 ? val | 4294901760 : val;
    };
    Buffer4.prototype.readInt16BE = function readInt16BE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 2, this.length);
      const val = this[offset + 1] | this[offset] << 8;
      return val & 32768 ? val | 4294901760 : val;
    };
    Buffer4.prototype.readInt32LE = function readInt32LE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 4, this.length);
      return this[offset] | this[offset + 1] << 8 | this[offset + 2] << 16 | this[offset + 3] << 24;
    };
    Buffer4.prototype.readInt32BE = function readInt32BE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 4, this.length);
      return this[offset] << 24 | this[offset + 1] << 16 | this[offset + 2] << 8 | this[offset + 3];
    };
    Buffer4.prototype.readBigInt64LE = defineBigIntMethod(function readBigInt64LE(offset) {
      offset = offset >>> 0;
      validateNumber(offset, "offset");
      const first = this[offset];
      const last = this[offset + 7];
      if (first === void 0 || last === void 0) {
        boundsError(offset, this.length - 8);
      }
      const val = this[offset + 4] + this[offset + 5] * 2 ** 8 + this[offset + 6] * 2 ** 16 + (last << 24);
      return (BigInt(val) << BigInt(32)) + BigInt(first + this[++offset] * 2 ** 8 + this[++offset] * 2 ** 16 + this[++offset] * 2 ** 24);
    });
    Buffer4.prototype.readBigInt64BE = defineBigIntMethod(function readBigInt64BE(offset) {
      offset = offset >>> 0;
      validateNumber(offset, "offset");
      const first = this[offset];
      const last = this[offset + 7];
      if (first === void 0 || last === void 0) {
        boundsError(offset, this.length - 8);
      }
      const val = (first << 24) + // Overflow
      this[++offset] * 2 ** 16 + this[++offset] * 2 ** 8 + this[++offset];
      return (BigInt(val) << BigInt(32)) + BigInt(this[++offset] * 2 ** 24 + this[++offset] * 2 ** 16 + this[++offset] * 2 ** 8 + last);
    });
    Buffer4.prototype.readFloatLE = function readFloatLE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 4, this.length);
      return ieee754.read(this, offset, true, 23, 4);
    };
    Buffer4.prototype.readFloatBE = function readFloatBE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 4, this.length);
      return ieee754.read(this, offset, false, 23, 4);
    };
    Buffer4.prototype.readDoubleLE = function readDoubleLE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 8, this.length);
      return ieee754.read(this, offset, true, 52, 8);
    };
    Buffer4.prototype.readDoubleBE = function readDoubleBE(offset, noAssert) {
      offset = offset >>> 0;
      if (!noAssert) checkOffset(offset, 8, this.length);
      return ieee754.read(this, offset, false, 52, 8);
    };
    function checkInt(buf, value, offset, ext, max, min) {
      if (!Buffer4.isBuffer(buf)) throw new TypeError('"buffer" argument must be a Buffer instance');
      if (value > max || value < min) throw new RangeError('"value" argument is out of bounds');
      if (offset + ext > buf.length) throw new RangeError("Index out of range");
    }
    Buffer4.prototype.writeUintLE = Buffer4.prototype.writeUIntLE = function writeUIntLE(value, offset, byteLength2, noAssert) {
      value = +value;
      offset = offset >>> 0;
      byteLength2 = byteLength2 >>> 0;
      if (!noAssert) {
        const maxBytes = Math.pow(2, 8 * byteLength2) - 1;
        checkInt(this, value, offset, byteLength2, maxBytes, 0);
      }
      let mul = 1;
      let i = 0;
      this[offset] = value & 255;
      while (++i < byteLength2 && (mul *= 256)) {
        this[offset + i] = value / mul & 255;
      }
      return offset + byteLength2;
    };
    Buffer4.prototype.writeUintBE = Buffer4.prototype.writeUIntBE = function writeUIntBE(value, offset, byteLength2, noAssert) {
      value = +value;
      offset = offset >>> 0;
      byteLength2 = byteLength2 >>> 0;
      if (!noAssert) {
        const maxBytes = Math.pow(2, 8 * byteLength2) - 1;
        checkInt(this, value, offset, byteLength2, maxBytes, 0);
      }
      let i = byteLength2 - 1;
      let mul = 1;
      this[offset + i] = value & 255;
      while (--i >= 0 && (mul *= 256)) {
        this[offset + i] = value / mul & 255;
      }
      return offset + byteLength2;
    };
    Buffer4.prototype.writeUint8 = Buffer4.prototype.writeUInt8 = function writeUInt8(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 1, 255, 0);
      this[offset] = value & 255;
      return offset + 1;
    };
    Buffer4.prototype.writeUint16LE = Buffer4.prototype.writeUInt16LE = function writeUInt16LE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 2, 65535, 0);
      this[offset] = value & 255;
      this[offset + 1] = value >>> 8;
      return offset + 2;
    };
    Buffer4.prototype.writeUint16BE = Buffer4.prototype.writeUInt16BE = function writeUInt16BE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 2, 65535, 0);
      this[offset] = value >>> 8;
      this[offset + 1] = value & 255;
      return offset + 2;
    };
    Buffer4.prototype.writeUint32LE = Buffer4.prototype.writeUInt32LE = function writeUInt32LE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 4, 4294967295, 0);
      this[offset + 3] = value >>> 24;
      this[offset + 2] = value >>> 16;
      this[offset + 1] = value >>> 8;
      this[offset] = value & 255;
      return offset + 4;
    };
    Buffer4.prototype.writeUint32BE = Buffer4.prototype.writeUInt32BE = function writeUInt32BE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 4, 4294967295, 0);
      this[offset] = value >>> 24;
      this[offset + 1] = value >>> 16;
      this[offset + 2] = value >>> 8;
      this[offset + 3] = value & 255;
      return offset + 4;
    };
    function wrtBigUInt64LE(buf, value, offset, min, max) {
      checkIntBI(value, min, max, buf, offset, 7);
      let lo = Number(value & BigInt(4294967295));
      buf[offset++] = lo;
      lo = lo >> 8;
      buf[offset++] = lo;
      lo = lo >> 8;
      buf[offset++] = lo;
      lo = lo >> 8;
      buf[offset++] = lo;
      let hi = Number(value >> BigInt(32) & BigInt(4294967295));
      buf[offset++] = hi;
      hi = hi >> 8;
      buf[offset++] = hi;
      hi = hi >> 8;
      buf[offset++] = hi;
      hi = hi >> 8;
      buf[offset++] = hi;
      return offset;
    }
    function wrtBigUInt64BE(buf, value, offset, min, max) {
      checkIntBI(value, min, max, buf, offset, 7);
      let lo = Number(value & BigInt(4294967295));
      buf[offset + 7] = lo;
      lo = lo >> 8;
      buf[offset + 6] = lo;
      lo = lo >> 8;
      buf[offset + 5] = lo;
      lo = lo >> 8;
      buf[offset + 4] = lo;
      let hi = Number(value >> BigInt(32) & BigInt(4294967295));
      buf[offset + 3] = hi;
      hi = hi >> 8;
      buf[offset + 2] = hi;
      hi = hi >> 8;
      buf[offset + 1] = hi;
      hi = hi >> 8;
      buf[offset] = hi;
      return offset + 8;
    }
    Buffer4.prototype.writeBigUInt64LE = defineBigIntMethod(function writeBigUInt64LE(value, offset = 0) {
      return wrtBigUInt64LE(this, value, offset, BigInt(0), BigInt("0xffffffffffffffff"));
    });
    Buffer4.prototype.writeBigUInt64BE = defineBigIntMethod(function writeBigUInt64BE(value, offset = 0) {
      return wrtBigUInt64BE(this, value, offset, BigInt(0), BigInt("0xffffffffffffffff"));
    });
    Buffer4.prototype.writeIntLE = function writeIntLE(value, offset, byteLength2, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) {
        const limit = Math.pow(2, 8 * byteLength2 - 1);
        checkInt(this, value, offset, byteLength2, limit - 1, -limit);
      }
      let i = 0;
      let mul = 1;
      let sub = 0;
      this[offset] = value & 255;
      while (++i < byteLength2 && (mul *= 256)) {
        if (value < 0 && sub === 0 && this[offset + i - 1] !== 0) {
          sub = 1;
        }
        this[offset + i] = (value / mul >> 0) - sub & 255;
      }
      return offset + byteLength2;
    };
    Buffer4.prototype.writeIntBE = function writeIntBE(value, offset, byteLength2, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) {
        const limit = Math.pow(2, 8 * byteLength2 - 1);
        checkInt(this, value, offset, byteLength2, limit - 1, -limit);
      }
      let i = byteLength2 - 1;
      let mul = 1;
      let sub = 0;
      this[offset + i] = value & 255;
      while (--i >= 0 && (mul *= 256)) {
        if (value < 0 && sub === 0 && this[offset + i + 1] !== 0) {
          sub = 1;
        }
        this[offset + i] = (value / mul >> 0) - sub & 255;
      }
      return offset + byteLength2;
    };
    Buffer4.prototype.writeInt8 = function writeInt8(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 1, 127, -128);
      if (value < 0) value = 255 + value + 1;
      this[offset] = value & 255;
      return offset + 1;
    };
    Buffer4.prototype.writeInt16LE = function writeInt16LE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 2, 32767, -32768);
      this[offset] = value & 255;
      this[offset + 1] = value >>> 8;
      return offset + 2;
    };
    Buffer4.prototype.writeInt16BE = function writeInt16BE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 2, 32767, -32768);
      this[offset] = value >>> 8;
      this[offset + 1] = value & 255;
      return offset + 2;
    };
    Buffer4.prototype.writeInt32LE = function writeInt32LE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 4, 2147483647, -2147483648);
      this[offset] = value & 255;
      this[offset + 1] = value >>> 8;
      this[offset + 2] = value >>> 16;
      this[offset + 3] = value >>> 24;
      return offset + 4;
    };
    Buffer4.prototype.writeInt32BE = function writeInt32BE(value, offset, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) checkInt(this, value, offset, 4, 2147483647, -2147483648);
      if (value < 0) value = 4294967295 + value + 1;
      this[offset] = value >>> 24;
      this[offset + 1] = value >>> 16;
      this[offset + 2] = value >>> 8;
      this[offset + 3] = value & 255;
      return offset + 4;
    };
    Buffer4.prototype.writeBigInt64LE = defineBigIntMethod(function writeBigInt64LE(value, offset = 0) {
      return wrtBigUInt64LE(this, value, offset, -BigInt("0x8000000000000000"), BigInt("0x7fffffffffffffff"));
    });
    Buffer4.prototype.writeBigInt64BE = defineBigIntMethod(function writeBigInt64BE(value, offset = 0) {
      return wrtBigUInt64BE(this, value, offset, -BigInt("0x8000000000000000"), BigInt("0x7fffffffffffffff"));
    });
    function checkIEEE754(buf, value, offset, ext, max, min) {
      if (offset + ext > buf.length) throw new RangeError("Index out of range");
      if (offset < 0) throw new RangeError("Index out of range");
    }
    function writeFloat(buf, value, offset, littleEndian, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) {
        checkIEEE754(buf, value, offset, 4, 34028234663852886e22, -34028234663852886e22);
      }
      ieee754.write(buf, value, offset, littleEndian, 23, 4);
      return offset + 4;
    }
    Buffer4.prototype.writeFloatLE = function writeFloatLE(value, offset, noAssert) {
      return writeFloat(this, value, offset, true, noAssert);
    };
    Buffer4.prototype.writeFloatBE = function writeFloatBE(value, offset, noAssert) {
      return writeFloat(this, value, offset, false, noAssert);
    };
    function writeDouble(buf, value, offset, littleEndian, noAssert) {
      value = +value;
      offset = offset >>> 0;
      if (!noAssert) {
        checkIEEE754(buf, value, offset, 8, 17976931348623157e292, -17976931348623157e292);
      }
      ieee754.write(buf, value, offset, littleEndian, 52, 8);
      return offset + 8;
    }
    Buffer4.prototype.writeDoubleLE = function writeDoubleLE(value, offset, noAssert) {
      return writeDouble(this, value, offset, true, noAssert);
    };
    Buffer4.prototype.writeDoubleBE = function writeDoubleBE(value, offset, noAssert) {
      return writeDouble(this, value, offset, false, noAssert);
    };
    Buffer4.prototype.copy = function copy(target, targetStart, start, end) {
      if (!Buffer4.isBuffer(target)) throw new TypeError("argument should be a Buffer");
      if (!start) start = 0;
      if (!end && end !== 0) end = this.length;
      if (targetStart >= target.length) targetStart = target.length;
      if (!targetStart) targetStart = 0;
      if (end > 0 && end < start) end = start;
      if (end === start) return 0;
      if (target.length === 0 || this.length === 0) return 0;
      if (targetStart < 0) {
        throw new RangeError("targetStart out of bounds");
      }
      if (start < 0 || start >= this.length) throw new RangeError("Index out of range");
      if (end < 0) throw new RangeError("sourceEnd out of bounds");
      if (end > this.length) end = this.length;
      if (target.length - targetStart < end - start) {
        end = target.length - targetStart + start;
      }
      const len = end - start;
      if (this === target && typeof Uint8Array.prototype.copyWithin === "function") {
        this.copyWithin(targetStart, start, end);
      } else {
        Uint8Array.prototype.set.call(
          target,
          this.subarray(start, end),
          targetStart
        );
      }
      return len;
    };
    Buffer4.prototype.fill = function fill(val, start, end, encoding) {
      if (typeof val === "string") {
        if (typeof start === "string") {
          encoding = start;
          start = 0;
          end = this.length;
        } else if (typeof end === "string") {
          encoding = end;
          end = this.length;
        }
        if (encoding !== void 0 && typeof encoding !== "string") {
          throw new TypeError("encoding must be a string");
        }
        if (typeof encoding === "string" && !Buffer4.isEncoding(encoding)) {
          throw new TypeError("Unknown encoding: " + encoding);
        }
        if (val.length === 1) {
          const code2 = val.charCodeAt(0);
          if (encoding === "utf8" && code2 < 128 || encoding === "latin1") {
            val = code2;
          }
        }
      } else if (typeof val === "number") {
        val = val & 255;
      } else if (typeof val === "boolean") {
        val = Number(val);
      }
      if (start < 0 || this.length < start || this.length < end) {
        throw new RangeError("Out of range index");
      }
      if (end <= start) {
        return this;
      }
      start = start >>> 0;
      end = end === void 0 ? this.length : end >>> 0;
      if (!val) val = 0;
      let i;
      if (typeof val === "number") {
        for (i = start; i < end; ++i) {
          this[i] = val;
        }
      } else {
        const bytes = Buffer4.isBuffer(val) ? val : Buffer4.from(val, encoding);
        const len = bytes.length;
        if (len === 0) {
          throw new TypeError('The value "' + val + '" is invalid for argument "value"');
        }
        for (i = 0; i < end - start; ++i) {
          this[i + start] = bytes[i % len];
        }
      }
      return this;
    };
    var errors = {};
    function E(sym, getMessage, Base) {
      errors[sym] = class NodeError extends Base {
        constructor() {
          super();
          Object.defineProperty(this, "message", {
            value: getMessage.apply(this, arguments),
            writable: true,
            configurable: true
          });
          this.name = `${this.name} [${sym}]`;
          this.stack;
          delete this.name;
        }
        get code() {
          return sym;
        }
        set code(value) {
          Object.defineProperty(this, "code", {
            configurable: true,
            enumerable: true,
            value,
            writable: true
          });
        }
        toString() {
          return `${this.name} [${sym}]: ${this.message}`;
        }
      };
    }
    E(
      "ERR_BUFFER_OUT_OF_BOUNDS",
      function(name) {
        if (name) {
          return `${name} is outside of buffer bounds`;
        }
        return "Attempt to access memory outside buffer bounds";
      },
      RangeError
    );
    E(
      "ERR_INVALID_ARG_TYPE",
      function(name, actual) {
        return `The "${name}" argument must be of type number. Received type ${typeof actual}`;
      },
      TypeError
    );
    E(
      "ERR_OUT_OF_RANGE",
      function(str, range, input) {
        let msg = `The value of "${str}" is out of range.`;
        let received = input;
        if (Number.isInteger(input) && Math.abs(input) > 2 ** 32) {
          received = addNumericalSeparator(String(input));
        } else if (typeof input === "bigint") {
          received = String(input);
          if (input > BigInt(2) ** BigInt(32) || input < -(BigInt(2) ** BigInt(32))) {
            received = addNumericalSeparator(received);
          }
          received += "n";
        }
        msg += ` It must be ${range}. Received ${received}`;
        return msg;
      },
      RangeError
    );
    function addNumericalSeparator(val) {
      let res = "";
      let i = val.length;
      const start = val[0] === "-" ? 1 : 0;
      for (; i >= start + 4; i -= 3) {
        res = `_${val.slice(i - 3, i)}${res}`;
      }
      return `${val.slice(0, i)}${res}`;
    }
    function checkBounds(buf, offset, byteLength2) {
      validateNumber(offset, "offset");
      if (buf[offset] === void 0 || buf[offset + byteLength2] === void 0) {
        boundsError(offset, buf.length - (byteLength2 + 1));
      }
    }
    function checkIntBI(value, min, max, buf, offset, byteLength2) {
      if (value > max || value < min) {
        const n = typeof min === "bigint" ? "n" : "";
        let range;
        if (byteLength2 > 3) {
          if (min === 0 || min === BigInt(0)) {
            range = `>= 0${n} and < 2${n} ** ${(byteLength2 + 1) * 8}${n}`;
          } else {
            range = `>= -(2${n} ** ${(byteLength2 + 1) * 8 - 1}${n}) and < 2 ** ${(byteLength2 + 1) * 8 - 1}${n}`;
          }
        } else {
          range = `>= ${min}${n} and <= ${max}${n}`;
        }
        throw new errors.ERR_OUT_OF_RANGE("value", range, value);
      }
      checkBounds(buf, offset, byteLength2);
    }
    function validateNumber(value, name) {
      if (typeof value !== "number") {
        throw new errors.ERR_INVALID_ARG_TYPE(name, "number", value);
      }
    }
    function boundsError(value, length, type) {
      if (Math.floor(value) !== value) {
        validateNumber(value, type);
        throw new errors.ERR_OUT_OF_RANGE(type || "offset", "an integer", value);
      }
      if (length < 0) {
        throw new errors.ERR_BUFFER_OUT_OF_BOUNDS();
      }
      throw new errors.ERR_OUT_OF_RANGE(
        type || "offset",
        `>= ${type ? 1 : 0} and <= ${length}`,
        value
      );
    }
    var INVALID_BASE64_RE = /[^+/0-9A-Za-z-_]/g;
    function base64clean(str) {
      str = str.split("=")[0];
      str = str.trim().replace(INVALID_BASE64_RE, "");
      if (str.length < 2) return "";
      while (str.length % 4 !== 0) {
        str = str + "=";
      }
      return str;
    }
    function utf8ToBytes(string, units) {
      units = units || Infinity;
      let codePoint;
      const length = string.length;
      let leadSurrogate = null;
      const bytes = [];
      for (let i = 0; i < length; ++i) {
        codePoint = string.charCodeAt(i);
        if (codePoint > 55295 && codePoint < 57344) {
          if (!leadSurrogate) {
            if (codePoint > 56319) {
              if ((units -= 3) > -1) bytes.push(239, 191, 189);
              continue;
            } else if (i + 1 === length) {
              if ((units -= 3) > -1) bytes.push(239, 191, 189);
              continue;
            }
            leadSurrogate = codePoint;
            continue;
          }
          if (codePoint < 56320) {
            if ((units -= 3) > -1) bytes.push(239, 191, 189);
            leadSurrogate = codePoint;
            continue;
          }
          codePoint = (leadSurrogate - 55296 << 10 | codePoint - 56320) + 65536;
        } else if (leadSurrogate) {
          if ((units -= 3) > -1) bytes.push(239, 191, 189);
        }
        leadSurrogate = null;
        if (codePoint < 128) {
          if ((units -= 1) < 0) break;
          bytes.push(codePoint);
        } else if (codePoint < 2048) {
          if ((units -= 2) < 0) break;
          bytes.push(
            codePoint >> 6 | 192,
            codePoint & 63 | 128
          );
        } else if (codePoint < 65536) {
          if ((units -= 3) < 0) break;
          bytes.push(
            codePoint >> 12 | 224,
            codePoint >> 6 & 63 | 128,
            codePoint & 63 | 128
          );
        } else if (codePoint < 1114112) {
          if ((units -= 4) < 0) break;
          bytes.push(
            codePoint >> 18 | 240,
            codePoint >> 12 & 63 | 128,
            codePoint >> 6 & 63 | 128,
            codePoint & 63 | 128
          );
        } else {
          throw new Error("Invalid code point");
        }
      }
      return bytes;
    }
    function asciiToBytes(str) {
      const byteArray = [];
      for (let i = 0; i < str.length; ++i) {
        byteArray.push(str.charCodeAt(i) & 255);
      }
      return byteArray;
    }
    function utf16leToBytes(str, units) {
      let c, hi, lo;
      const byteArray = [];
      for (let i = 0; i < str.length; ++i) {
        if ((units -= 2) < 0) break;
        c = str.charCodeAt(i);
        hi = c >> 8;
        lo = c % 256;
        byteArray.push(lo);
        byteArray.push(hi);
      }
      return byteArray;
    }
    function base64ToBytes(str) {
      return base64.toByteArray(base64clean(str));
    }
    function blitBuffer(src, dst, offset, length) {
      let i;
      for (i = 0; i < length; ++i) {
        if (i + offset >= dst.length || i >= src.length) break;
        dst[i + offset] = src[i];
      }
      return i;
    }
    function isInstance(obj, type) {
      return obj instanceof type || obj != null && obj.constructor != null && obj.constructor.name != null && obj.constructor.name === type.name;
    }
    function numberIsNaN(obj) {
      return obj !== obj;
    }
    var hexSliceLookupTable = function() {
      const alphabet = "0123456789abcdef";
      const table = new Array(256);
      for (let i = 0; i < 16; ++i) {
        const i16 = i * 16;
        for (let j = 0; j < 16; ++j) {
          table[i16 + j] = alphabet[i] + alphabet[j];
        }
      }
      return table;
    }();
    function defineBigIntMethod(fn) {
      return typeof BigInt === "undefined" ? BufferBigIntNotDefined : fn;
    }
    function BufferBigIntNotDefined() {
      throw new Error("BigInt not supported");
    }
  }
});

// src/worker.ts
import init, { OuterMachine, WebUnpredictables } from "./defcpu_web.js";

// src/line-number.mjs
function findInstructionFromOffset(state2, from) {
  let node = state2.head.next;
  while (node && node.statement.range.end < from) {
    node = node.next;
  }
  if (node) {
    const { statement } = node;
    const sectionStart = statement.section.programHeader.p_vaddr;
    const start = sectionStart + statement.address;
    return start;
  } else {
    return void 0;
  }
}
function findInstruction(state2, address) {
  for (const section of state2.sections) {
    const segment = section.programHeader;
    if (address >= segment.p_vaddr && address < Math.ceil((segment.p_vaddr + segment.p_memsz) / 4096) * 4096) {
      const localAddress = address - segment.p_vaddr;
      let node = section.head.next;
      while (node) {
        if (node.statement.length > 0 && node.statement.address >= localAddress)
          return { section, instr: node.statement };
        node = node.next;
      }
      return { section, instr: null };
    }
  }
  return { section: null, instr: null };
}
function linePosition(state2, address) {
  let { instr: crashedInstr, section: crashedSection } = findInstruction(
    state2,
    address
  );
  let pos = "on";
  let errLine = null;
  if (crashedInstr === null) {
    if (crashedSection !== null) {
      pos = "after";
      state2.iterate((instr2, line2) => {
        if (instr2.section === crashedSection) errLine = line2;
      });
    }
  } else {
    state2.iterate((instr2, line2) => {
      if (errLine) return;
      if (instr2 == crashedInstr) errLine = line2;
    });
  }
  return errLine === null ? null : { pos, errLine };
}

// ../node_modules/@defasm/core/parser.js
var code;
var comment;
var currRange;
var line;
var match;
var prevRange;
var token;
var currSyntax = null;
var parentRange = null;
function setSyntax(syntax) {
  currSyntax = syntax;
}
function startAbsRange() {
  return parentRange = currRange.abs();
}
var tokenizer = /(["'])(\\(.|\n|$)|[^\\])*?(\1|$)|>>|<<|\|\||&&|>=|<=|<>|==|!=|[\w.$]+|[\S\n]/g;
function loadCode(source, index = 0) {
  tokenizer.lastIndex = index;
  code = source;
  line = (source.slice(0, index).match(/\n/g) || []).length + 1;
  next = defaultNext;
  parentRange = currRange = new Range(index, 0);
  match = 1;
  next();
}
var defaultNext = () => {
  prevRange = currRange;
  if (!match)
    return "\n";
  comment = false;
  match = tokenizer.exec(code);
  if (match) {
    token = match[0];
    currRange = new RelativeRange(parentRange, match.index, token.length);
    if (token == (currSyntax.intel ? ";" : "#")) {
      comment = true;
      token = ";";
      tokenizer.lastIndex = code.indexOf("\n", tokenizer.lastIndex);
      if (tokenizer.lastIndex < 0)
        tokenizer.lastIndex = code.length;
      currRange.length = tokenizer.lastIndex - match.index;
    }
  } else {
    token = "\n";
    currRange = new RelativeRange(parentRange, code.length, 1);
  }
  line += (token.match(/\n/g) || []).length;
  return token;
};
var next = defaultNext;
function ungetToken() {
  let t = token, p = currRange, oldNext = next;
  currRange = prevRange;
  next = () => token = (next = oldNext, currRange = p, t);
}
function setToken(tok, range = currRange) {
  token = tok || "\n";
  currRange = range;
}
var Range = class _Range {
  constructor(start = 0, length = 0) {
    if (start < 0 || length < 0)
      throw `Invalid range ${start} to ${start + length}`;
    this._start = start;
    this.length = length;
  }
  /** @param {Number} pos */
  includes(pos) {
    return this.end >= pos && pos >= this.start;
  }
  /** @param {Range} end */
  until(end) {
    return new _Range(this.start, end.end - this.start);
  }
  /** @param {string} text */
  slice(text) {
    return text.slice(this.start, this.end);
  }
  get start() {
    return this._start;
  }
  set start(val) {
    this._start = val;
  }
  get end() {
    return this.start + this.length;
  }
};
var RelativeRange = class _RelativeRange extends Range {
  constructor(parent, start, length) {
    super(start - parent.start, length);
    this.parent = parent;
  }
  get start() {
    return this.parent.start + this._start;
  }
  set start(val) {
    this._start = val - this.parent.start;
  }
  abs() {
    return new Range(this.start, this.length);
  }
  until(end) {
    return new _RelativeRange(this.parent, this.start, end.end - this.start);
  }
};
var ASMError = class {
  /**
   * @param {string} message The message this error holds
   * @param {Range} range The range of this error
   */
  constructor(message, range = currRange) {
    this.message = message;
    this.range = range;
  }
};

// ../node_modules/@defasm/core/relocations.js
var RelocEntry = class {
  /**
   * 
   * @param {Object} config
   * @param {number} config.offset
   * @param {number} config.sizeReduction
   * @param {import("./shuntingYard").IdentifierValue} config.value
   * @param {number} config.size
   * @param {boolean} config.signed
   * @param {boolean} config.pcRelative
   * @param {boolean} config.functionAddr
   */
  constructor({ offset, sizeReduction, value, size, signed, pcRelative, functionAddr }) {
    this.offset = offset;
    value = value.flatten();
    this.addend = value.addend - sizeReduction;
    if (pcRelative)
      this.addend += BigInt(offset);
    this.symbol = value.symbol;
    this.size = size;
    this.signed = signed;
    this.pcRelative = pcRelative;
    this.functionAddr = functionAddr;
  }
};

// ../node_modules/@defasm/core/utils.js
function inferSize(value, signed = true) {
  if (signed) {
    if (value < 0n)
      value = ~value;
    return value < 0x80n ? 8 : value < 0x8000n ? 16 : value < 0x80000000n ? 32 : 64;
  } else {
    if (value < 0n)
      value = -2n * value - 1n;
    return value < 0x100n ? 8 : value < 0x10000n ? 16 : value < 0x100000000n ? 32 : 64;
  }
}

// ../node_modules/@defasm/core/statement.js
var totalStatements = 0;
var StatementNode = class {
  /** @param {Statement?} statement */
  constructor(statement = null) {
    this.statement = statement;
    this.next = null;
  }
  /**
   * @param {Number} pos
   * @returns {StatementNode?} */
  find(pos) {
    if (this.statement && this.statement.range.includes(pos))
      return this;
    return this.next?.find(pos);
  }
  length() {
    let node = this, length = 0;
    while (node) {
      if (node.statement)
        length += node.statement.length;
      node = node.next;
    }
    return length;
  }
  dump() {
    let output, i = 0, node = this;
    try {
      output = Buffer.alloc(this.length());
    } catch (e) {
      output = new Uint8Array(this.length());
    }
    while (node) {
      if (node.statement) {
        output.set(node.statement.bytes.subarray(0, node.statement.length), i);
        i += node.statement.length;
      }
      node = node.next;
    }
    return output;
  }
  /** Select the instruction range that is affected by a given range
   * @param {Range} range
   * @param {boolean} update
   * @param {Number} sourceLength
   * @returns {InstructionRange}
   */
  getAffectedArea(range, update = false, sourceLength = 0) {
    let node = this;
    let head = this, last = null, tail = null;
    let changeOffset = sourceLength - range.length;
    while (node) {
      let instr2 = node.statement;
      if (instr2) {
        if (instr2.range.end < range.start)
          head = node;
        else if (instr2.range.start <= range.end) {
          last = node;
          if (update) {
            if (instr2.range.end >= range.end)
              instr2.range.length += changeOffset;
            instr2.remove();
          }
        } else {
          if (tail === null)
            tail = node;
          if (update)
            instr2.range.start += changeOffset;
        }
      }
      node = node.next;
    }
    if (update) {
      if (last) {
        range.start = Math.min(range.start, head.next.statement.range.start);
        range.length = last.statement.range.end - range.start;
      } else if (tail)
        range.length = tail.statement.range.start - range.start - 1;
      else
        range.length = sourceLength;
    }
    return { head, prev: head, tail };
  }
};
var Statement = class {
  /**
   * @param {Object} config
   * @param {Number} config.addr
   * @param {Number} config.maxSize
   * @param {Range} config.range
   * @param {ASMError?} config.error
   * @param {Section} config.section
   * @param {import("./parser.js").Syntax} config.syntax */
  constructor({ addr: addr2 = 0, maxSize = 0, range = new Range(), error = null, section = currSection, syntax = currSyntax } = {}) {
    this.id = totalStatements++;
    this.error = error;
    this.range = range;
    this.bytes = new Uint8Array(maxSize);
    this.syntax = syntax;
    this.address = addr2;
    this.section = section;
    this.sectionNode = new StatementNode(this);
    this.removed = true;
    this.clear();
  }
  clear() {
    this.length = 0;
    this.relocations = [];
    this.lineEnds = [];
  }
  /** @param {BigInt|Number} byte */
  genByte(byte) {
    this.bytes[this.length++] = Number(byte);
  }
  /**
   * @typedef {Object} ValueConfig
   * @property {number} size
   * @property {boolean} signed
   * @property {boolean} sizeRelative
   * @property {boolean} functionAddr
   * @property {number?} dispMul
   */
  /**
   * @param {import("./shuntingYard.js").IdentifierValue} value
   * @param {ValueConfig} config
   */
  genValue(value, {
    size,
    signed = false,
    sizeRelative = false,
    functionAddr = false,
    dispMul = null
  } = {}) {
    let sizeReduction = sizeRelative ? BigInt(this.length + size / 8) : 0n;
    let num = 0n;
    if (value.isRelocatable()) {
      if (size >= 128)
        throw new ASMError("Can't do 16 byte relocations", value.range);
      this.relocations.push({
        offset: this.length,
        sizeReduction,
        value,
        signed: signed && !value.pcRelative && size == 32,
        size,
        pcRelative: value.pcRelative,
        functionAddr: functionAddr && value.section == pseudoSections.UND
      });
    } else {
      num = value.addend - sizeReduction;
      if (dispMul !== null) {
        let shrunkValue = num / BigInt(dispMul);
        if (num % BigInt(dispMul) == 0 && inferSize(shrunkValue) == 8) {
          num = shrunkValue;
          size = 8;
        } else
          size = 32;
      }
    }
    for (const lineEnd of value.lineEnds)
      this.lineEnds.push(this.length + Math.min(lineEnd, size / 8));
    do {
      this.genByte(num & 0xffn);
      num >>= 8n;
    } while (size -= 8);
  }
  remove() {
    this.removed = true;
  }
};

// ../node_modules/@defasm/core/bitfield.js
function createBitfieldClass(fieldNames) {
  let prototype = {};
  for (let i = 0; i < fieldNames.length; i++) {
    let fieldValue = 1 << i;
    Object.defineProperty(prototype, fieldNames[i], {
      get() {
        return (this.bits & fieldValue) != 0;
      },
      set(value) {
        if (value)
          this.bits |= fieldValue;
        else
          this.bits &= ~fieldValue;
        return value;
      }
    });
  }
  ;
  prototype.add = function(field) {
    this.bits |= field.bits;
  };
  return class {
    constructor() {
      this.bits = 0;
      Object.setPrototypeOf(this, prototype);
    }
  };
}

// ../node_modules/@defasm/core/operands.js
var OperandType = class {
  constructor(name, { hasSize = true, isMemory = false, isVector = false } = {}) {
    this.name = name;
    this.hasSize = hasSize;
    this.isMemory = isMemory;
    this.isVector = isVector;
  }
  toString() {
    return this.name;
  }
};
var OPT = Object.freeze({
  REG: new OperandType("General-purpose register"),
  // 8/64-bit - ax, bl, esi, r15, etc.
  VEC: new OperandType("Vector register", { isVector: true }),
  // 64/512-bit - %mm0 / %mm7, %xmm0 / %xmm15, %ymm0 / %ymm15, %zmm0 / %zmm15
  VMEM: new OperandType("Vector memory", { isMemory: true, isVector: true }),
  // e.g. (%xmm0)
  IMM: new OperandType("Immediate value"),
  // e.g. $20
  MASK: new OperandType("Mask register"),
  // 64-bit - %k0 / %k7
  REL: new OperandType("Relative address", { isMemory: true }),
  // memory that consists of only an address (may be converted to MEM)
  MEM: new OperandType("Memory operand", { isMemory: true }),
  // e.g. (%rax)
  ST: new OperandType("Floating-point stack register", { hasSize: false }),
  // 80-bit - %st(0) / %st(7)
  SEG: new OperandType("Segment register", { hasSize: false }),
  // 16-bit - %cs, %ds, %es, %fs, %gs, %ss
  IP: new OperandType("Instruction pointer register", { hasSize: false }),
  // only used in memory - %eip or %rip
  BND: new OperandType("Bound register", { hasSize: false }),
  // 128-bit - %bnd0 / %bnd3
  CTRL: new OperandType("Control register", { hasSize: false }),
  // 64-bit - %cr0, %cr2, %cr3, %cr4 and %cr8
  DBG: new OperandType("Debug register", { hasSize: false })
  // 64-bit - %dr0 / %dr7
});
var registers = Object.assign({}, ...[
  "al",
  "cl",
  "dl",
  "bl",
  "ah",
  "ch",
  "dh",
  "bh",
  "ax",
  "cx",
  "dx",
  "bx",
  "sp",
  "bp",
  "si",
  "di",
  "eax",
  "ecx",
  "edx",
  "ebx",
  "esp",
  "ebp",
  "esi",
  "edi",
  "rax",
  "rcx",
  "rdx",
  "rbx",
  "rsp",
  "rbp",
  "rsi",
  "rdi",
  "es",
  "cs",
  "ss",
  "ds",
  "fs",
  "gs",
  "st",
  "rip",
  "eip",
  "spl",
  "bpl",
  "sil",
  "dil"
].map((x, i) => ({ [x]: i })));
var suffixes = {
  b: 8,
  w: 16,
  l: 32,
  q: 64,
  t: 80,
  x: 128,
  y: 256,
  z: 512
};
var floatSuffixes = {
  s: 32,
  l: 64,
  t: 80
};
var floatIntSuffixes = {
  s: 16,
  l: 32,
  q: 64
};
var sizeHints = Object.freeze({
  byte: 8,
  word: 16,
  long: 32,
  dword: 32,
  far: 48,
  fword: 48,
  qword: 64,
  tbyte: 80,
  oword: 128,
  xmmword: 128,
  ymmword: 256,
  zmmword: 512
});
function isSizeHint(sizeHint) {
  return sizeHints.hasOwnProperty(sizeHint);
}
function nameRegister(name, size, syntax) {
  return `${syntax.prefix ? "%" : ""}${size == 32 ? "e" : "r"}` + name;
}
var PrefixEnum = createBitfieldClass([
  "REX",
  "NOREX",
  "LOCK",
  "REPNE",
  "REPE",
  "DATASIZE",
  "ADDRSIZE",
  "SEG0",
  "SEG1",
  "SEG2",
  "SEG3",
  "SEG4",
  "SEG5",
  "EVEX"
]);
var regParsePos;
var regSuffixes = {
  b: 8,
  w: 16,
  d: 32
};
function isRegister(reg, bitness = currBitness) {
  reg = reg.toLowerCase();
  if (registers.hasOwnProperty(reg)) {
    if (bitness == 64)
      return true;
    let regIndex = registers[reg];
    return regIndex < registers.rax || regIndex >= registers.es && regIndex <= registers.st;
  }
  if (bitness == 64 && reg[0] === "r") {
    reg = reg.slice(1);
    if (parseInt(reg) >= 0 && parseInt(reg) < 16 && (!isNaN(reg) || regSuffixes[reg[reg.length - 1]]))
      return true;
  } else {
    let max = bitness == 64 ? 32 : 8;
    if (reg.startsWith("mm") || reg.startsWith("dr")) reg = reg.slice(2), max = 8;
    else if (reg.startsWith("cr")) reg = reg.slice(2), max = bitness == 64 ? 9 : 8;
    else if (reg.startsWith("xmm") || reg.startsWith("ymm") || reg.startsWith("zmm")) reg = reg.slice(3);
    else if (reg.startsWith("bnd")) reg = reg.slice(3), max = 4;
    else if (reg[0] == "k") reg = reg.slice(1), max = 8;
    else return false;
    if (!isNaN(reg) && (reg = parseInt(reg), reg >= 0 && reg < max))
      return true;
  }
  return false;
}
function parseRegister(expectedType = null) {
  let regToken = (currSyntax.prefix ? next() : token).toLowerCase();
  let reg = registers[regToken];
  let size = 0, type = -1, prefs = new PrefixEnum();
  if (reg >= registers.al && reg <= (currBitness == 64 ? registers.rdi : registers.edi)) {
    type = OPT.REG;
    size = 8 << (reg >> 3);
    if (size == 8 && reg >= registers.ah && reg <= registers.bh)
      prefs.NOREX = true;
    reg &= 7;
  } else if (reg >= registers.es && reg <= registers.gs) {
    type = OPT.SEG;
    size = 32;
    reg -= registers.es;
  } else if (reg == registers.st) {
    type = OPT.ST;
    reg = 0;
    if (next() == "(") {
      reg = parseInt(next());
      if (isNaN(reg) || reg >= 8 || reg < 0 || next() != ")")
        throw new ASMError("Unknown register");
    } else
      ungetToken();
  } else if (currBitness == 64 && (reg == registers.rip || reg == registers.eip)) {
    if (expectedType === null || !expectedType.includes(OPT.IP))
      throw new ASMError(`Can't use ${nameRegister("ip", reg == registers.eip ? 32 : 64, currSyntax)} here`);
    type = OPT.IP;
    size = reg == registers.eip ? 32 : 64;
    reg = 0;
  } else if (currBitness == 64 && reg >= registers.spl && reg <= registers.dil) {
    type = OPT.REG;
    size = 8;
    prefs.REX = true;
    reg -= registers.spl - 4;
  } else if (currBitness == 64 && regToken[0] == "r") {
    reg = parseInt(regToken.slice(1));
    if (isNaN(reg) || reg < 0 || reg >= 16)
      throw new ASMError("Unknown register");
    type = OPT.REG;
    let regLastChar = regToken[regToken.length - 1];
    if (isNaN(regLastChar)) {
      size = regSuffixes[regLastChar];
      if (!size)
        throw new ASMError("Unknown register");
    } else
      size = 64;
  } else {
    let max = currBitness == 64 ? 32 : 8;
    if (token.startsWith("bnd")) reg = regToken.slice(3), type = OPT.BND, max = 4;
    else if (regToken[0] == "k") reg = regToken.slice(1), type = OPT.MASK, max = 8, size = NaN;
    else if (regToken.startsWith("dr")) reg = regToken.slice(2), type = OPT.DBG, max = 8;
    else if (regToken.startsWith("cr")) reg = regToken.slice(2), type = OPT.CTRL, max = currBitness == 64 ? 9 : 8;
    else {
      type = OPT.VEC;
      if (regToken.startsWith("mm")) reg = regToken.slice(2), size = 64, max = 8;
      else if (regToken.startsWith("xmm")) reg = regToken.slice(3), size = 128;
      else if (regToken.startsWith("ymm")) reg = regToken.slice(3), size = 256;
      else if (regToken.startsWith("zmm")) reg = regToken.slice(3), size = 512;
      else
        throw new ASMError("Unknown register");
    }
    if (isNaN(reg) || !(reg = parseInt(reg), reg >= 0 && reg < max))
      throw new ASMError("Unknown register");
  }
  if (expectedType != null && expectedType.indexOf(type) < 0)
    throw new ASMError("Invalid register");
  regParsePos = currRange;
  next();
  return { reg, type, size, prefs };
}
var Operand = class {
  /** @param {Statement} instr */
  constructor(instr2, expectRelative = false) {
    this.reg = this.reg2 = -1;
    this.shift = 0;
    this.value = null;
    this.type = null;
    this.size = NaN;
    this.prefs = new PrefixEnum();
    this.attemptedSizes = 0;
    this.attemptedUnsignedSizes = 0;
    this.startPos = currRange;
    let indirect = token == "*";
    if (indirect && !instr2.syntax.intel)
      next();
    let forceMemory = false;
    if (instr2.syntax.prefix && isRegister(token))
      throw new ASMError("Registers must be prefixed with '%'");
    if (instr2.syntax.prefix ? token == "%" : isRegister(token)) {
      const regData = parseRegister();
      this.endPos = regParsePos;
      if (regData.type === OPT.SEG && token == ":") {
        this.prefs[`SEG${regData.reg}`] = true;
        forceMemory = true;
        next();
      } else {
        Object.assign(this, regData);
        return;
      }
    }
    if (instr2.syntax.intel) {
      this.type = expectRelative ? OPT.REL : OPT.IMM;
      if (token != "[" && !forceMemory) {
        let mayBeMem = !expectRelative;
        if (token.toLowerCase() == "offset") {
          next();
          this.type = OPT.IMM;
          mayBeMem = false;
        }
        this.expression = new Expression(instr2);
        if (this.expression.hasSymbols && mayBeMem)
          this.type = OPT.MEM;
      }
      const hasBracket = token == "[";
      if (hasBracket || forceMemory) {
        this.type = OPT.MEM;
        if (hasBracket)
          next();
        let secExpr = new Expression(instr2, true);
        if (this.expression)
          this.expression.apply("+", secExpr);
        else
          this.expression = secExpr;
        this.ripRelative = this.expression.ripRelative;
        if (this.expression.vecSize) {
          this.size = this.expression.vecSize;
          this.type = OPT.VMEM;
        }
        if (hasBracket) {
          if (token != "]")
            throw new ASMError("Expected ']'");
          next();
        }
      }
    } else {
      if (token[0] == "$") {
        if (token.length > 1) {
          setToken(token.slice(1));
          currRange.start++;
        } else
          next();
        this.expression = new Expression(instr2);
        this.type = OPT.IMM;
      } else {
        this.type = OPT.MEM;
        this.expression = new Expression(instr2, true);
        if (this.expression.vecSize) {
          this.size = this.expression.vecSize;
          this.type = OPT.VMEM;
        }
        if (token != "(") {
          if (!indirect && expectRelative)
            this.type = OPT.REL;
        } else {
          let tempReg;
          if (instr2.syntax.prefix ? next() == "%" : isRegister(next())) {
            tempReg = parseRegister([OPT.REG, OPT.IP]);
            this.reg = tempReg.reg;
          } else if (token == ",") {
            this.reg = -1;
            tempReg = { type: -1, size: currBitness };
          } else
            throw new ASMError("Expected register");
          if (tempReg.size == 32 && currBitness == 64)
            this.prefs.ADDRSIZE = true;
          else if (tempReg.size != currBitness)
            throw new ASMError("Invalid register size", regParsePos);
          if (tempReg.type === OPT.IP)
            this.ripRelative = true;
          else if (token == ",") {
            if (instr2.syntax.prefix ? next() != "%" : !isRegister(next()))
              throw new ASMError("Expected register");
            tempReg = parseRegister([OPT.REG, OPT.VEC]);
            this.reg2 = tempReg.reg;
            if (tempReg.type === OPT.VEC) {
              this.type = OPT.VMEM;
              this.size = tempReg.size;
              if (tempReg.size < 128)
                throw new ASMError("Invalid register size", regParsePos);
            } else {
              if (this.reg2 == 4)
                throw new ASMError(`Memory index cannot be ${tempReg.size == 64 ? "R" : "E"}SP`, regParsePos);
              if (tempReg.size == 32 && currBitness == 64)
                this.prefs.ADDRSIZE = true;
              else if (tempReg.size != currBitness)
                throw new ASMError("Invalid register size", regParsePos);
            }
            if (token == ",") {
              this.shift = "1248".indexOf(next());
              if (this.shift < 0)
                throw new ASMError("Scale must be 1, 2, 4, or 8");
              next();
            }
          }
          if (token != ")")
            throw new ASMError("Expected ')'");
          next();
        }
      }
    }
    if (this.expression) {
      if (this.type === OPT.REL)
        this.expression.apply("-", new CurrentIP(instr2));
      if (!this.expression.hasSymbols)
        this.evaluate(instr2);
    }
    this.endPos = prevRange;
  }
  sizeAllowed(size, unsigned = false) {
    return size >= (unsigned ? this.unsignedSize : this.size) || this.sizeAvailable(size, unsigned);
  }
  sizeAvailable(size, unsigned = false) {
    return !((unsigned ? this.attemptedUnsignedSizes : this.attemptedSizes) & 1 << (size >> 4));
  }
  recordSizeUse(size, unsigned = false) {
    if (unsigned)
      this.attemptedUnsignedSizes |= 1 << (size >> 4);
    else
      this.attemptedSizes |= 1 << (size >> 4);
  }
  clearAttemptedSizes() {
    this.attemptedSizes = this.attemptedUnsignedSizes = 0;
  }
  evaluate(instr2, intelMemory = false) {
    this.value = this.expression.evaluate(instr2);
    if (intelMemory) {
      this.prefs.ADDRSIZE = false;
      let { regBase = null, regIndex = null, shift = 1 } = this.value.regData ?? {};
      if (regBase)
        this.reg = regBase.reg;
      if (regIndex)
        this.reg2 = regIndex.reg;
      if (currBitness == 64 && (regBase && regBase.size == 32 || regIndex && regIndex.size == 32))
        this.prefs.ADDRSIZE = true;
      this.shift = [1, 2, 4, 8].indexOf(shift);
      if (this.shift < 0)
        throw new ASMError("Scale must be 1, 2, 4, or 8", this.value.range);
      if (this.ripRelative && regIndex)
        throw new ASMError(`Can't use another register with ${nameRegister("ip", regBase.size, instr2.syntax)}`, this.value.range);
    }
    if ((this.reg & 7) == 5)
      this.value.addend = this.value.addend || 0n;
  }
};

// ../node_modules/@defasm/core/shuntingYard.js
var unaryTemps = {
  "+": (a) => a,
  "-": (a) => -a,
  "~": (a) => ~a,
  "!": (a) => !a
};
var operatorTemps = [
  {
    "*": (a, b) => a * b,
    "/": (a, b) => a / (b || 1n),
    "%": (a, b) => a % (b || 1n),
    "<<": (a, b) => a << b,
    ">>": (a, b) => a >> b
  },
  {
    "|": (a, b) => a | b,
    "&": (a, b) => a & b,
    "^": (a, b) => a ^ b,
    "!": (a, b) => a | ~b
  },
  {
    "+": (a, b) => a + b,
    "-": (a, b) => a - b
  },
  {
    "==": (a, b) => a == b ? -1n : 0n,
    "<>": (a, b) => a != b ? -1n : 0n,
    "!=": (a, b) => a != b ? -1n : 0n,
    "<": (a, b) => a < b ? -1n : 0n,
    ">": (a, b) => a > b ? -1n : 0n,
    ">=": (a, b) => a >= b ? -1n : 0n,
    "<=": (a, b) => a <= b ? -1n : 0n
  },
  { "&&": (a, b) => a && b ? 1n : 0n },
  { "||": (a, b) => a || b ? 1n : 0n }
];
var operators = {};
var unaries = {};
for (let i = 0; i < operatorTemps.length; i++)
  for (const op of Object.keys(operatorTemps[i]))
    operators[op] = { func: operatorTemps[i][op], prec: i };
for (const op of Object.keys(unaryTemps))
  unaries[op] = { func: unaryTemps[op] };
var stringEscapeSeqs = {
  "a": 7,
  "b": 8,
  "e": 27,
  "f": 12,
  "n": 10,
  "r": 13,
  "t": 9,
  "v": 11
};
var encoder = new TextEncoder();
function readString(string) {
  if (string.length < 2 || string[string.length - 1] != string[0])
    throw new ASMError("Incomplete string");
  const lineEnds = [];
  let output = [];
  let matches = string.slice(1, -1).match(/(\\(?:x[0-9a-f]{1,2}|[0-7]{1,3}|u[0-9a-f]{1,8}|(.|\n)?))|\n|[^\\\n]+/ig);
  if (matches)
    for (let x of matches) {
      if (x[0] == "\\") {
        x = x.slice(1);
        if (x == "")
          throw new ASMError("Incomplete string");
        if (x.match(/x[0-9a-f]{1,2}/i))
          output.push(parseInt(x.slice(1), 16));
        else if (x.match(/u[0-9a-f]{1,8}/i))
          output.push(...encoder.encode(String.fromCodePoint(parseInt(x.slice(1), 16))));
        else if (x.match(/[0-7]{1,3}/))
          output.push(parseInt(x, 8) & 255);
        else if (stringEscapeSeqs.hasOwnProperty(x))
          output.push(stringEscapeSeqs[x]);
        else if (x != "\n")
          output.push(...encoder.encode(x));
      } else
        output.push(...encoder.encode(x));
      if (x == "\n")
        lineEnds.push(output.length);
    }
  return { bytes: new Uint8Array(output), lineEnds };
}
function scanIdentifier(id, intel) {
  if (id[0].match(/[a-z_.$]/i))
    return "symbol";
  if (id[0].match(/[^0-9]/))
    return null;
  if (id.match(/^([0-9]+|0x[0-9a-f]+|0o[0-7]+|0b[01]+)$/i) || intel && id.match(/^([0-9][0-9a-f]*)h$/i))
    return "number";
  return null;
}
function parseIdentifier(instr2) {
  let value = 0n, startRange = currRange;
  try {
    if (token === "\n")
      throw new ASMError("Expected value, got none");
    if (token[0] === "'") {
      let { bytes, lineEnds } = readString(token), i = bytes.length;
      while (i--) {
        value <<= 8n;
        value += BigInt(bytes[i]);
      }
      next();
      return new Identifier(instr2, value, startRange, lineEnds);
    }
    if (instr2.syntax.prefix ? token == "%" : isRegister(token))
      return new RegisterIdentifier(instr2, parseRegister([OPT.REG, OPT.IP, OPT.VEC]), regParsePos);
    const idType = scanIdentifier(token, instr2.syntax.intel);
    if (idType == "symbol") {
      const name = token;
      next();
      return new SymbolIdentifier(instr2, name, startRange);
    }
    if (idType === null)
      throw new ASMError("Invalid number");
    let mainToken = token;
    if (token[token.length - 1].toLowerCase() == "h")
      mainToken = "0x" + token.slice(0, -1);
    value = BigInt(mainToken);
    next();
    return new Identifier(instr2, value, startRange);
  } catch (e) {
    if (e.range === void 0)
      throw new ASMError(e);
    throw e;
  }
}
var IdentifierValue = class _IdentifierValue {
  /**
   * @param {Object} config
   * @param {BigInt} config.addend 
   * @property {import('./symbols.js').Symbol} config.symbol
   * @property {Section} config.section
   * @property {Range} config.range
   * @property {RegData} config.regData
   * @property {Number[]} lineEnds */
  constructor({ addend = null, symbol = null, section = pseudoSections.UND, range, regData = null, pcRelative = false, lineEnds = [] } = {}) {
    this.addend = addend;
    this.symbol = symbol;
    this.section = section;
    this.range = range;
    this.regData = regData;
    this.pcRelative = pcRelative;
    this.lineEnds = lineEnds;
  }
  isRelocatable() {
    return this.symbol && this.section != pseudoSections.ABS || this.pcRelative;
  }
  flatten() {
    let val = this, addend = this.addend;
    while (val.symbol && (val.section == pseudoSections.ABS || val.symbol.value.symbol && !val.symbol.bind) && val.symbol.value !== val) {
      val = val.symbol.value;
      addend += val.addend;
    }
    return new _IdentifierValue({
      ...val,
      addend
    });
  }
  absoluteValue() {
    let val = this, total = this.addend;
    let passed = /* @__PURE__ */ new Set([val]);
    while (val.symbol && !passed.has(val.symbol.value)) {
      val = val.symbol.value;
      total += val.addend;
      passed.add(val);
    }
    return total;
  }
  /**
   * @param {Statement} instr
   * @param {IdentifierValue} op1
   * @param {string} func
   * @param {IdentifierValue} op2
   */
  apply(instr2, func, op, allowPCRelative = true) {
    this.range = this.range.until(op.range);
    if (this.section == pseudoSections.ABS && op.section == pseudoSections.ABS)
      ;
    else if (func == "+" && this.section == pseudoSections.ABS && !this.pcRelative) {
      this.section = op.section;
      this.symbol = op.symbol;
    } else if ((func == "+" || func == "-") && op.section == pseudoSections.ABS)
      ;
    else if (this.pcRelative || op.pcRelative)
      throw new ASMError("Bad operands", this.range);
    else if (func == "-" && this.section == op.section && (this.section != pseudoSections.UND && this.section != pseudoSections.COM || this.symbol == op.symbol)) {
      if (this.symbol)
        this.addend = this.absoluteValue();
      if (op.symbol)
        op.addend = op.absoluteValue();
      this.section = op.section = pseudoSections.ABS;
      this.symbol = op.symbol = null;
    } else if (func == "-" && allowPCRelative && op.section == instr2.section)
      this.pcRelative = true;
    else
      throw new ASMError("Bad operands", this.range);
    if (this.regData || op.regData) {
      if (func != "+" && func != "-" && func != "*" || func == "-" && op.regData)
        throw new ASMError("Bad operands", this.range);
      let regOp = this.regData ? this : op;
      let nonRegOp = this.regData ? op : this;
      if (!this.regData)
        this.regData = op.regData;
      else if (op.regData) {
        if (func == "*")
          throw new ASMError("Bad operands", this.range);
        if (this.regData.regIndex && op.regData.regIndex)
          throw new ASMError("Can't have multiple index registers", this.range);
        if ([this.regData, op.regData].some((data) => data.regBase && data.regIndex))
          throw new ASMError("Too many registers", this.range);
        if (this.regData.regBase && op.regData.regBase) {
          this.regData.regIndex = [this.regData.regBase, op.regData.regBase].find((reg) => reg.reg != 4);
          if (this.regData.regIndex === void 0)
            throw new ASMError(`Can't have both registers be ${instr2.syntax.prefix ? "%" : ""}rsp`, this.range);
          if (this.regData.regIndex == this.regData.regBase)
            this.regData.regBase = op.regData.regBase;
        } else if (op.regData.regIndex) {
          this.regData.regIndex = op.regData.regIndex;
          this.regData.shift = op.regData.shift;
        } else
          this.regData.regBase = op.regData.regBase;
      }
      if (func == "*") {
        if (nonRegOp.section != pseudoSections.ABS)
          throw new ASMError("Scale must be absolute", nonRegOp.range);
        if (regOp.regData.regIndex && regOp.regData.regBase)
          throw new ASMError("Can't scale both base and index registers", this.range);
        if (regOp.regData.regBase) {
          const scaled = regOp.regData.regBase;
          if (scaled.reg == 4)
            throw new ASMError(`Can't scale ${nameRegister("sp", scaled.size, instr2.syntax)}`, this.range);
          if (scaled.type === OPT.IP)
            throw new ASMError(`Can't scale ${nameRegister("ip", scaled.size, instr2.syntax)}`, this.range);
          this.regData.regIndex = scaled;
          this.regData.regBase = null;
        }
        this.regData.shift *= Number(nonRegOp.addend);
        this.addend = regOp.addend !== null ? nonRegOp.addend * regOp.addend : null;
      } else if (this.addend !== null || op.addend !== null)
        this.addend = operators[func].func(this.addend ?? 0n, op.addend ?? 0n);
    } else
      this.addend = operators[func].func(this.addend, op.addend);
    this.pcRelative = this.pcRelative || op.pcRelative;
    this.lineEnds = [...this.lineEnds, ...op.lineEnds].sort((a, b) => a - b);
  }
  /** Infer the size of this value
   * @param {boolean} signed Whether to treat this value as signed or not */
  inferSize(signed = true) {
    return inferSize(this.addend, signed);
  }
};
var Identifier = class {
  /**
   * @param {Statement} instr
   * @param {Number} value
   * @param {Range} range
   * @param {Number[]} lineEnds */
  constructor(instr2, value, range, lineEnds = []) {
    this.value = value;
    this.range = range;
    this.lineEnds = lineEnds;
  }
  /**
   * @param {Statement} instr
   * @returns {IdentifierValue} */
  getValue(instr2) {
    return new IdentifierValue({
      addend: this.value,
      section: pseudoSections.ABS,
      range: this.range,
      lineEnds: this.lineEnds
    });
  }
};
var SymbolIdentifier = class extends Identifier {
  /**
   * @param {Statement} instr
   * @param {string} name
   * @param {Range} range */
  constructor(instr2, name, range) {
    super(instr2, 0, range);
    this.name = name;
    this.isIP = name == (instr2.syntax.intel ? "$" : ".");
    if (this.isIP)
      instr2.ipRelative = true;
  }
  /**
   * @param {Statement} instr
   * @returns {IdentifierValue} */
  getValue(instr2) {
    if (this.isIP)
      return new IdentifierValue({
        addend: BigInt(instr2.address),
        symbol: (instr2.section.head?.statement ?? instr2).symbol,
        section: instr2.section,
        range: this.range
      });
    const symbol = symbols.get(this.name);
    if (symbol.statement && !symbol.statement.error) {
      if (instr2.symbol && checkSymbolRecursion(symbol))
        throw new ASMError(`Recursive definition`, this.range);
      let isAbs = symbol.value.section == pseudoSections.ABS;
      return new IdentifierValue({
        addend: isAbs ? symbol.value.addend : 0n,
        symbol: isAbs ? null : symbol,
        section: symbol.value.section,
        range: this.range
      });
    }
    return new IdentifierValue({
      addend: 0n,
      symbol,
      range: this.range
    });
  }
};
var RegisterIdentifier = class extends Identifier {
  /**
   * @param {Statement} instr
   * @param {import("./operands.js").Register} register
   * @param {Range} range */
  constructor(instr2, register, range) {
    super(instr2, 0, range);
    this.register = register;
  }
  getValue() {
    return new IdentifierValue({
      section: pseudoSections.ABS,
      range: this.range,
      regData: this.register.type === OPT.VEC ? {
        shift: 1,
        regBase: null,
        regIndex: this.register
      } : {
        shift: 1,
        regBase: this.register,
        regIndex: null
      }
    });
  }
};
var Expression = class {
  /** @param {Statement} instr */
  constructor(instr2, expectMemory = false, uses = null) {
    this.hasSymbols = false;
    this.vecSize = 0;
    this.ripRelative = false;
    this.stack = [];
    let opStack = [];
    let lastOp, lastWasNum = false;
    while (token != "," && token != "\n" && token != ";") {
      if (!lastWasNum && unaries.hasOwnProperty(token)) {
        opStack.push({ range: currRange, func: token, prec: -1, unary: true });
        next();
      } else if (operators.hasOwnProperty(token)) {
        if (!lastWasNum) {
          if (expectMemory && instr2.syntax.prefix && token == "%") {
            if (instr2.syntax.intel) {
              lastWasNum = true;
              this.stack.push(parseIdentifier(instr2));
              continue;
            }
            if (opStack.length > 0 && opStack[opStack.length - 1].bracket) {
              ungetToken();
              setToken("(");
              return;
            }
          }
          throw new ASMError("Missing left operand");
        }
        const op = { range: currRange, func: token, prec: operators[token].prec, unary: false };
        next();
        lastWasNum = false;
        while (lastOp = opStack[opStack.length - 1], lastOp && lastOp.prec <= op.prec && !lastOp.bracket)
          this.stack.push(opStack.pop());
        opStack.push(op);
      } else if (unaries.hasOwnProperty(token))
        throw new ASMError("Unary operator can't be used here");
      else if (token == "(") {
        if (lastWasNum) {
          if (expectMemory)
            break;
          throw new ASMError("Unexpected parenthesis");
        }
        opStack.push({ range: currRange, bracket: true });
        next();
      } else if (token == ")") {
        if (!lastWasNum)
          throw new ASMError("Missing right operand", opStack.length ? opStack[opStack.length - 1].range : currRange);
        while (lastOp = opStack[opStack.length - 1], lastOp && !lastOp.bracket)
          this.stack.push(opStack.pop());
        if (!lastOp || !lastOp.bracket)
          throw new ASMError("Mismatched parentheses");
        opStack.pop();
        lastWasNum = true;
        next();
      } else if (instr2.syntax.intel && (token == "[" || token == "]")) {
        if (!lastWasNum)
          throw new ASMError("Missing right operand", opStack.length ? opStack[opStack.length - 1].range : currRange);
        break;
      } else {
        if (lastWasNum)
          throw new ASMError("Unexpected value");
        lastWasNum = true;
        if (!instr2.syntax.prefix && isRegister(token)) {
          if (!expectMemory)
            throw new ASMError("Can't use registers in an expression");
          if (!instr2.syntax.intel && opStack.length > 0 && opStack[opStack.length - 1].bracket)
            break;
        }
        this.stack.push(parseIdentifier(instr2));
      }
    }
    if (this.stack.length == 0) {
      if (expectMemory) {
        ungetToken();
        setToken("(");
        return;
      } else
        throw new ASMError("Expected expression");
    }
    if (!lastWasNum)
      throw new ASMError("Missing right operand", opStack.length ? opStack[opStack.length - 1].range : currRange);
    while (opStack[0]) {
      if (opStack[opStack.length - 1].bracket)
        throw new ASMError("Mismatched parentheses", opStack[opStack.length - 1].range);
      this.stack.push(opStack.pop());
    }
    for (const id of this.stack) {
      if (id.register && id.register.type === OPT.VEC)
        this.vecSize = id.register.size;
      else if (id.register && id.register.type === OPT.IP)
        this.ripRelative = true;
      else if (id.name) {
        if (!id.isIP) {
          const symbol = referenceSymbol(instr2, id.name);
          if (uses !== null)
            uses.push(symbol);
        }
        this.hasSymbols = true;
      }
    }
  }
  /**
   * @param {Statement} instr
   * @param {boolean} allowPCRelative
   * @param {boolean} expectAbsolute
   * @returns {IdentifierValue} */
  evaluate(instr2, allowPCRelative = true, expectAbsolute = false) {
    if (this.stack.length == 0)
      return new IdentifierValue({ section: pseudoSections.ABS });
    let stack = [], len = 0;
    for (const op of this.stack) {
      const func = op.func;
      if (func) {
        if (op.unary) {
          if (func == "+")
            continue;
          const val = stack[len - 1], minusRelative = val.section == instr2.section && func == "-";
          if (val.regData || val.section != pseudoSections.ABS && !minusRelative || minusRelative && !allowPCRelative)
            throw new ASMError("Bad operand", val.range);
          if (minusRelative)
            val.pcRelative = true;
          val.addend = unaries[func].func(val.addend);
        } else {
          stack[len - 2].apply(instr2, func, stack.pop(), allowPCRelative);
          len--;
        }
      } else
        stack[len++] = op.getValue(instr2);
    }
    if (stack.length > 1)
      throw new ASMError("Invalid expression", stack[0].range);
    if (expectAbsolute) {
      if (stack[0].section != pseudoSections.ABS)
        throw new ASMError("Expected absolute expression", stack[0].range);
    }
    return stack[0];
  }
  /**
   * @param {string} func
   * @param {Expression} expr */
  apply(func, expr = null) {
    if (expr === null)
      this.stack.push({ func, unary: true });
    else if (expr.stack.length > 0) {
      this.stack.push(...expr.stack, { func, unary: false });
      this.hasSymbols = this.hasSymbols || expr.hasSymbols;
      this.vecSize = this.vecSize || expr.vecSize;
      this.ripRelative = this.ripRelative || expr.ripRelative;
    }
  }
};
function CurrentIP(instr2) {
  this.hasSymbols = true;
  this.pcRelative = false;
  this.stack = [new SymbolIdentifier(instr2, instr2.syntax.intel ? "$" : ".", currRange)];
}
CurrentIP.prototype = Object.create(Expression.prototype);
function checkSymbolRecursion(symbol, passed = /* @__PURE__ */ new Set()) {
  if (passed.has(symbol))
    return true;
  passed.add(symbol);
  for (const use of symbol.uses)
    if (checkSymbolRecursion(use, passed))
      return true;
  passed.delete(symbol);
  return false;
}

// ../node_modules/@defasm/core/symbols.js
var recompQueue = [];
function makeSymbol({ name, type = void 0, bind = void 0, uses = [], references = [], definitions = [] } = {}) {
  return {
    statement: null,
    name,
    references,
    definitions,
    uses,
    value: new IdentifierValue({ addend: 0n }),
    type,
    bind
  };
}
var symbols = /* @__PURE__ */ new Map();
var fileSymbols = [];
function loadSymbols(table, fileArr) {
  symbols = table;
  fileSymbols = fileArr;
}
function queueRecomp(instr2) {
  if (!instr2.wantsRecomp)
    recompQueue.push(instr2.sectionNode);
  instr2.wantsRecomp = true;
}
var SymbolDefinition = class extends Statement {
  /** @type {Symbol} */
  symbol;
  constructor({ name, opcodeRange = null, isLabel = false, compile = true, type = 0, bind = 0, ...config }) {
    if (opcodeRange === null)
      opcodeRange = config.range;
    super(config);
    let uses = [];
    if (isLabel)
      this.expression = new CurrentIP(this);
    else {
      if (compile) {
        next();
        this.expression = new Expression(this, false, uses);
      }
    }
    this.removed = false;
    if (symbols.has(name)) {
      this.symbol = symbols.get(name);
      this.symbol.definitions.push(this);
      if (this.symbol.statement) {
        this.error = new ASMError(`This ${isLabel ? "label" : "symbol"} already exists`, opcodeRange);
        this.duplicate = true;
        return;
      }
      this.symbol.uses = uses;
      this.duplicate = false;
    } else
      symbols.set(name, this.symbol = makeSymbol({ name, type, bind, uses, definitions: [this] }));
    if (compile) {
      this.compile();
      for (const ref of this.symbol.references)
        if (!ref.removed)
          queueRecomp(ref);
    }
  }
  // Re-evaluate the symbol. Return true if references to the symbol should be recompiled
  compile() {
    let originError = this.error;
    let originValue = this.symbol.value;
    let prevAbs = this.prevAbs;
    this.error = null;
    let value = void 0;
    try {
      value = this.symbol.value = this.expression.evaluate(this, false);
      this.symbol.statement = this;
      this.prevAbs = value.absoluteValue();
    } catch (e) {
      this.error = e;
    }
    return !(originError && this.error) && (!value || originValue.addend !== value.addend || originValue.section !== value.section || prevAbs !== this.prevAbs);
  }
  recompile() {
    if (this.duplicate && this.symbol.statement)
      return;
    this.duplicate = false;
    if (this.compile()) {
      this.symbol.statement = this;
      for (const ref of this.symbol.references)
        queueRecomp(ref);
    }
  }
  remove() {
    if (!this.duplicate) {
      let refs = this.symbol.references;
      if (refs.length > 0) {
        this.symbol.statement = null;
        this.symbol.uses = [];
        let newDef = this.symbol.definitions.find((def) => def.duplicate);
        if (newDef)
          newDef.recompile();
        else
          for (const instr2 of this.symbol.references)
            queueRecomp(instr2);
      } else
        symbols.delete(this.symbol.name);
    }
    super.remove();
  }
};
function getAlignment(x) {
  return x <= 1n ? 1n : x <= 2n ? 2n : x <= 4n ? 4n : x <= 8n ? 8n : 16n;
}
var CommSymbol = class extends SymbolDefinition {
  constructor({ name, opcodeRange = null, ...config }) {
    super({ ...config, compile: false, bind: SYM_BINDS.global, type: SYM_TYPES.object, name: token });
    next();
    if (token != ",")
      throw new ASMError("Expected ','");
    next();
    this.sizeExpr = new Expression(this);
    this.alignExpr = null;
    if (token == ",") {
      next();
      this.alignExpr = new Expression(this);
    }
    this.removed = true;
    this.compile();
    for (const ref of this.symbol.references)
      if (!ref.removed)
        queueRecomp(ref);
  }
  compile() {
    let prevErr = this.error;
    this.error = null;
    try {
      const sizeVal = this.sizeExpr.evaluate(this, false, true);
      if (sizeVal.addend < 0n)
        throw new ASMError("Size cannot be negative", sizeVal.range);
      this.symbol.size = sizeVal.addend;
      if (this.alignExpr)
        this.symbol.value = this.alignExpr.evaluate(this, false, true);
      else
        this.symbol.value = new IdentifierValue({ addend: getAlignment(this.symbol.size) });
      this.symbol.value.section = pseudoSections.COM;
      this.removed = false;
      return prevErr !== null;
    } catch (e) {
      this.error = e;
      return prevErr === null;
    }
  }
};
function referenceSymbol(instr2, name, defining = false) {
  let symbol;
  if (symbols.has(name)) {
    symbol = symbols.get(name);
    symbol.references.push(instr2);
    if (defining)
      symbol.definitions.push(instr2);
  } else
    symbols.set(name, symbol = makeSymbol({ name, references: [instr2], definitions: defining ? [instr2] : [] }));
  return symbol;
}

// ../node_modules/@defasm/core/sections.js
var sections = [];
function loadSections(table, range) {
  sections = table;
  for (const section of table)
    section.cursor = section.head.getAffectedArea(range);
}
var pseudoSections = {
  ABS: { name: "*ABS*", index: 65521 },
  UND: { name: "*UND*", index: 0 },
  COM: { name: "*COM*", index: 65522 }
};
var sectionFlags = {
  a: 2,
  // SHF_ALLOC
  e: 134217728,
  // SHF_EXCLUDE
  o: 64,
  // SHF_INFO_LINK
  w: 1,
  // SHF_WRITE
  x: 4,
  // SHF_EXECINSTR
  M: 16,
  // SHF_MERGE
  S: 32,
  // SHF_STRINGS
  G: 512,
  // SHF_GROUP
  T: 1024
  // SHF_TLS
};
var sectionTypes = {
  "progbits": 1,
  "nobits": 8,
  "note": 7,
  "init_array": 14,
  "fini_array": 15,
  "preinit_array": 16
};
var STT_SECTION = 3;
var SHT_DYNSYM = 11;
var SHT_DYNAMIC = 6;
var Section = class {
  /** @param {string} name */
  constructor(name) {
    this.name = name;
    this.cursor = null;
    this.persistent = name == ".text" || name == ".data" || name == ".bss";
    this.head = new StatementNode(new SymbolDefinition({ addr: 0, name, isLabel: true, type: STT_SECTION, section: this }));
    this.entryPoints = [];
    this.cursor = { head: this.head, prev: this.head };
    switch (name) {
      case ".text":
      case ".init":
      case ".fini":
        this.flags = sectionFlags.a | sectionFlags.x;
        break;
      case ".rodata":
      case ".dynsym":
      case ".dynamic":
        this.flags = sectionFlags.a;
        break;
      case ".data":
      case ".bss":
      case ".preinit_array":
      case ".init_array":
      case ".fini_array":
        this.flags = sectionFlags.a | sectionFlags.w;
        break;
      default:
        this.flags = 0;
    }
    switch (name) {
      case ".notes":
        this.type = sectionTypes.note;
        break;
      case ".bss":
        this.type = sectionTypes.nobits;
        break;
      case ".preinit_array":
        this.type = sectionTypes.preinit_array;
        break;
      case ".init_array":
        this.type = sectionTypes.init_array;
        break;
      case ".fini_array":
        this.type = sectionTypes.fini_array;
        break;
      case ".dynsym":
        this.type = SHT_DYNSYM;
        break;
      case ".dynamic":
        this.type = SHT_DYNAMIC;
        break;
      default:
        this.type = sectionTypes.progbits;
    }
    switch (name) {
      case ".fini_array":
      case ".init_array":
        this.entrySize = 8;
        break;
      case ".dynsym":
        this.entrySize = 24;
        break;
      case ".dynamic":
        this.entrySize = 16;
        break;
      default:
        this.entrySize = 0;
    }
  }
  getRelocations() {
    let node = this.head, relocations = [];
    while (node) {
      for (const reloc of node.statement.relocations)
        relocations.push(new RelocEntry({ ...reloc, offset: node.statement.address + reloc.offset }));
      node = node.next;
    }
    return relocations;
  }
};

// ../node_modules/@defasm/core/directives.js
var SYM_BINDS = {
  "local": 0,
  "global": 1,
  "weak": 2
};
var SYM_TYPES = {
  "no_type": 0,
  "object": 1,
  "function": 2,
  "tls_object": 6
};
var SYM_VISIBS = {
  "internal": 1,
  "hidden": 2,
  "protected": 3,
  "exported": 4,
  "singleton": 5,
  "eliminate": 6
};
var DIRECTIVE_BUFFER_SIZE = 15;
var directives = {
  equ: -1,
  set: -1,
  byte: 1,
  short: 2,
  word: 2,
  // .word = .short
  hword: 2,
  // .hword = .short
  value: 2,
  // .value = .short
  "2byte": 2,
  // .2byte = .short
  int: 3,
  long: 3,
  // .long = .int
  "4byte": 4,
  // .4byte = .int
  quad: 4,
  "8byte": 4,
  // .8byte = .quad
  octa: 5,
  float: 6,
  single: 6,
  // .single = .float
  double: 7,
  asciz: 8,
  ascii: 9,
  string: 9,
  // .string = .ascii
  intel_syntax: 10,
  att_syntax: 11,
  text: 12,
  data: 13,
  bss: 14,
  globl: 15,
  global: 15,
  weak: 16,
  size: 17,
  type: 18,
  hidden: 19,
  local: 20,
  section: 21,
  file: 22,
  comm: 23
};
var intelDirectives = {
  "%assign": -1,
  db: 0,
  dw: directives.word,
  dd: directives.long,
  dq: directives.quad,
  ".intel_syntax": directives.intel_syntax,
  ".att_syntax": directives.att_syntax,
  global: directives.global,
  section: directives.section,
  segment: directives.segment
};
function isDirective(directive, intel) {
  directive = directive.toLowerCase();
  return intel ? intelDirectives.hasOwnProperty(directive) : directive[0] == "." && directives.hasOwnProperty(directive.slice(1));
}
function makeDirective(config, dir) {
  dir = dir.toLowerCase();
  let dirs = currSyntax.intel ? intelDirectives : directives;
  if (!dirs.hasOwnProperty(dir))
    throw new ASMError("Unknown directive", config.range);
  let dirID = dirs[dir];
  switch (dirID) {
    case intelDirectives.db:
    case directives.byte:
    case directives.word:
    case directives.int:
    case directives.quad:
    case directives.octa:
    case directives.asciz:
    case directives.ascii:
      return new DataDirective(config, dirID);
    case directives.float:
      return new FloatDirective(config, 0);
    case directives.double:
      return new FloatDirective(config, 1);
    case directives.intel_syntax:
      return new SyntaxDirective(config, true);
    case directives.att_syntax:
      return new SyntaxDirective(config, false);
    case directives.section:
      return new SectionDirective(config);
    case directives.text:
      return new SectionDirective(config, sections[0]);
    case directives.data:
      return new SectionDirective(config, sections[1]);
    case directives.bss:
      return new SectionDirective(config, sections[2]);
    case directives.local:
      return new SymBindDirective(config, SYM_BINDS.local);
    case directives.globl:
      return new SymBindDirective(config, SYM_BINDS.global);
    case directives.weak:
      return new SymBindDirective(config, SYM_BINDS.weak);
    case directives.size:
      return new SymSizeDirective(config);
    case directives.type:
      return new SymTypeDirective(config);
    case directives.hidden:
      return new SymHiddenDirective(config);
    case directives.file:
      return new FileDirective(config);
    case directives.equ:
      let name = token, opcodeRange = currRange;
      if (!currSyntax.intel && next() !== ",")
        throw new ASMError("Expected ','");
      return new SymbolDefinition({ ...config, name, opcodeRange });
    case directives.comm:
      return new CommSymbol(config);
  }
}
var SectionDirective = class extends Statement {
  /** @param {Section} section */
  constructor(config, section = null) {
    let flags = 0, type = sectionTypes.progbits, attribRange = null;
    if (section === null) {
      let sectionName = "";
      while (token != "," && token != ";" && token != "\n") {
        sectionName += token;
        next();
      }
      if (sectionName == "")
        throw new ASMError("Expected section name");
      section = sections.find((x) => x.name == sectionName) ?? null;
      if (token == ",") {
        attribRange = currRange;
        flags = 0;
        for (const byte of readString(next()).bytes) {
          const char = String.fromCharCode(byte);
          if (!sectionFlags.hasOwnProperty(char))
            throw new ASMError(`Unknown flag '${char}'`);
          flags |= sectionFlags[char];
        }
        if (next() == ",") {
          if (next() != "@")
            throw new ASMError("Expected '@'");
          const sectionType = next();
          if (!sectionTypes.hasOwnProperty(sectionType))
            throw new ASMError("Unknown section type");
          type = sectionTypes[sectionType];
          next();
        }
        attribRange = attribRange.until(currRange);
      }
      if (section === null)
        sections.push(section = new Section(sectionName));
      if (section.persistent && attribRange)
        throw new ASMError(`Can't give attributes to ${section.name}`, attribRange);
    }
    super({ ...config, maxSize: 0, section });
    section.entryPoints.push(this);
    this.switchSection = true;
    this.sectionAttributes = attribRange ? { flags, type } : null;
    this.attribRange = attribRange;
    if (this.sectionAttributes)
      try {
        this.recompile();
      } catch (e) {
        this.error = e;
      }
  }
  recompile() {
    this.error = null;
    if (this.section.entryPoints.some((x) => x !== this && !x.removed && !x.error && x.sectionAttributes !== null))
      throw new ASMError("Attributes already set for this section", this.attribRange);
    this.section.flags = this.sectionAttributes.flags;
    this.section.type = this.sectionAttributes.type;
  }
  remove() {
    this.section.entryPoints.splice(this.section.entryPoints.indexOf(this), 1);
    if (this.section.entryPoints.length == 0) {
      if (!this.section.persistent) {
        this.section.head.statement.remove();
        sections.splice(sections.indexOf(this.section), 1);
      }
    } else if (this.sectionAttributes !== null) {
      const otherDefinition = this.section.entryPoints.find((entry) => entry.sectionAttributes !== null);
      if (otherDefinition)
        queueRecomp(otherDefinition);
      else
        this.section.flags = 0;
    }
  }
};
var SyntaxDirective = class extends Statement {
  constructor(config, intel) {
    const prevSyntax = currSyntax;
    setSyntax({ prefix: currSyntax.prefix, intel });
    const prefSpecifier = token.toLowerCase();
    let prefix = !intel;
    if (prefSpecifier == "prefix")
      prefix = true;
    else if (prefSpecifier == "noprefix")
      prefix = false;
    else if (prefSpecifier != "\n" && prefSpecifier != ";") {
      setSyntax(prevSyntax);
      throw new ASMError("Expected 'prefix' or 'noprefix'");
    }
    if (token != "\n" && token != ";")
      next();
    super({ ...config, maxSize: 0, syntax: { intel, prefix } });
    this.switchSyntax = true;
  }
};
var DataDirective = class extends Statement {
  constructor(config, dirID) {
    super({ ...config, maxSize: DIRECTIVE_BUFFER_SIZE });
    this.outline = null;
    this.floatPrec = 0;
    let appendNullByte = 0;
    try {
      switch (dirID) {
        case intelDirectives.db:
          this.compileValues(1, true);
          break;
        case directives.byte:
          this.compileValues(1);
          break;
        case directives.word:
          this.compileValues(2);
          break;
        case directives.int:
          this.compileValues(4);
          break;
        case directives.quad:
          this.compileValues(8);
          break;
        case directives.octa:
          this.compileValues(16);
          break;
        case directives.asciz:
          appendNullByte = 1;
        case directives.ascii:
          this.bytes = new Uint8Array();
          do {
            if (token[0] == '"') {
              const string = readString(token);
              this.append(string, string.bytes.length + appendNullByte);
            } else
              throw new ASMError("Expected string");
          } while (next() == "," && next());
          break;
      }
    } catch (e) {
      this.error = e;
      while (token != ";" && token != "\n")
        next();
    }
  }
  append({ bytes, lineEnds }, length = bytes.length) {
    const temp = new Uint8Array(this.length + length + 1);
    temp.set(this.bytes.subarray(0, this.length));
    temp.set(bytes, this.length);
    this.bytes = temp;
    for (const lineEnd of lineEnds)
      this.lineEnds.push(this.length + lineEnd);
    this.length += length;
  }
  compileValues(valSize, acceptStrings = false) {
    this.valSize = valSize;
    let expression, needsRecompilation = false;
    this.outline = [];
    try {
      do {
        if (token[0] === '"') {
          if (acceptStrings) {
            const string = readString(token);
            this.outline.push(string);
          } else
            throw new ASMError("Unexpected string");
          next();
        } else {
          expression = new Expression(this);
          if (expression.hasSymbols)
            needsRecompilation = true;
          this.outline.push({ expression });
        }
      } while (token === "," && next());
      this.removed = false;
      this.compile();
    } finally {
      if (!needsRecompilation)
        this.outline = null;
    }
  }
  compile() {
    let op, outlineLength = this.outline.length;
    const startAddr = this.address;
    for (let i = 0; i < outlineLength; i++) {
      op = this.outline[i];
      try {
        if (op.bytes)
          this.append(op);
        else {
          if (op.value === void 0 || op.expression.hasSymbols)
            op.value = op.expression.evaluate(this, true);
          this.genValue(op.value, { size: this.valSize * 8 });
        }
        this.address = startAddr + this.length;
      } catch (e) {
        this.error = e;
        outlineLength = i;
        i = -1;
        this.length = 0;
      }
    }
    this.address = startAddr;
  }
  recompile() {
    this.clear();
    this.error = null;
    this.compile();
  }
  genByte(byte) {
    super.genByte(byte);
    if (this.length == this.bytes.length) {
      let temp = new Uint8Array(this.bytes.length + DIRECTIVE_BUFFER_SIZE);
      temp.set(this.bytes);
      this.bytes = temp;
    }
  }
};
var FloatDirective = class extends Statement {
  constructor(config, precision) {
    super({ ...config });
    let values = [];
    do {
      if (isNaN(token))
        throw new ASMError("Expected number");
      if (token == "\n") {
        this.error = new ASMError("Expected number");
        break;
      }
      values.push(token);
    } while (next() == "," && next());
    this.bytes = new Uint8Array((precision > 0 ? new Float64Array(values) : new Float32Array(values)).buffer);
    this.length = this.bytes.length;
  }
};
var SymInfo = class extends Statement {
  addSymbol() {
    let name = token, range = currRange;
    if (scanIdentifier(name, this.syntax.intel) != "symbol")
      return false;
    next();
    if (token != "," && token != ";" && token != "\n") {
      ungetToken();
      setToken(name);
      return false;
    }
    const symbol = referenceSymbol(this, name, true);
    if (symbol.type == STT_SECTION)
      throw new ASMError("Can't modify section labels");
    this.symbols.push({ range, symbol });
    return true;
  }
  constructor(config, name, proceedings = true) {
    super({ ...config, maxSize: 0 });
    this.symbols = [];
    if (!this.addSymbol())
      throw new ASMError("Expected symbol name");
    this.infoName = name;
    this.setting = [name];
    while (true) {
      if (token != ",") {
        if (proceedings)
          throw new ASMError("Expected ','");
        break;
      }
      next();
      if (!this.addSymbol())
        break;
    }
  }
  compile() {
    this.removed = false;
    for (const { symbol, range } of this.symbols) {
      for (const info of this.setting)
        if (symbol.definitions.some((x) => x !== this && !x.removed && !x.error && x.setting?.includes(info)))
          throw new ASMError(`${this.infoName} already set for this symbol`, range);
      this.setInfo(symbol);
    }
  }
  recompile() {
    this.error = null;
    this.compile();
  }
  remove() {
    super.remove();
    for (const info of this.setting)
      for (const { symbol } of this.symbols) for (const def of symbol.definitions)
        if (!def.removed && def.setting?.includes(info))
          queueRecomp(def);
  }
};
var SymBindDirective = class extends SymInfo {
  constructor(config, bind) {
    super(config, "Binding", false);
    this.binding = bind;
    try {
      this.compile();
    } catch (e) {
      this.error = e;
    }
  }
  setInfo(symbol) {
    symbol.bind = this.binding;
  }
  remove() {
    super.remove();
    for (const { symbol } of this.symbols)
      symbol.bind = void 0;
  }
};
var SymSizeDirective = class extends SymInfo {
  constructor(config) {
    super(config, "Size");
    this.expression = new Expression(this);
    try {
      this.compile();
    } catch (e) {
      this.error = e;
    }
  }
  compile() {
    this.value = this.expression.evaluate(this, false, true);
    super.compile();
  }
  setInfo(symbol) {
    symbol.size = this.value.addend;
  }
  remove() {
    super.remove();
    for (const { symbol } of this.symbols)
      symbol.size = void 0;
  }
};
var SymTypeDirective = class extends SymInfo {
  constructor(config) {
    super(config, "Type");
    this.visib = void 0;
    if (token != "@")
      throw new ASMError("Expected '@'");
    let type = next().toLowerCase();
    if (!SYM_TYPES.hasOwnProperty(type))
      throw new ASMError("Unknown symbol type");
    this.type = SYM_TYPES[type];
    if (next() == ",") {
      this.setting.push("Visibility");
      if (next() != "@")
        throw new ASMError("Expected '@'");
      let visib = next().toLowerCase();
      if (!SYM_VISIBS.hasOwnProperty(visib))
        throw new ASMError("Unknown symbol visibility");
      this.visib = SYM_VISIBS[visib];
      next();
    }
    try {
      this.compile();
    } catch (e) {
      this.error = e;
    }
  }
  setInfo(symbol) {
    symbol.type = this.type;
    symbol.visibility = this.visib;
  }
  remove() {
    super.remove();
    for (const { symbol } of this.symbols) {
      symbol.type = void 0;
      symbol.visibility = void 0;
    }
  }
};
var SymHiddenDirective = class extends SymInfo {
  constructor(config) {
    super(config, "Visibility", false);
    try {
      this.compile();
    } catch (e) {
      this.error = e;
    }
  }
  setInfo(symbol) {
    symbol.visibility = SYM_VISIBS.hidden;
  }
  remove() {
    super.remove();
    for (const { symbol } of this.symbols)
      symbol.visibility = void 0;
  }
};
var decoder = new TextDecoder();
var FileDirective = class extends Statement {
  constructor(config) {
    super({ ...config, maxSize: 0 });
    try {
      this.filename = decoder.decode(readString(token).bytes);
    } catch (e) {
      throw new ASMError("Bad string");
    }
    next();
    fileSymbols.push(this.filename);
  }
  remove() {
    fileSymbols.splice(fileSymbols.indexOf(this.filename), 1);
  }
};

// ../node_modules/@defasm/core/operations.js
var REG_MOD = -1;
var REG_OP = -2;
var OPC = {
  r: OPT.REG,
  v: OPT.VEC,
  i: OPT.IMM,
  j: OPT.REL,
  m: OPT.MEM,
  s: OPT.SEG,
  f: OPT.ST,
  b: OPT.BND,
  k: OPT.MASK,
  c: OPT.CTRL,
  d: OPT.DBG,
  g: OPT.VMEM
};
var sizers = Object.assign({ f: 48 }, suffixes);
var opCatcherCache = {};
var SIZETYPE_IMPLICITENC = 1;
var EvexPermits = createBitfieldClass([
  "MASK",
  "ZEROING",
  "BROADCAST_32",
  "BROADCAST_64",
  "SAE",
  "ROUNDING",
  "FORCEW",
  "FORCE",
  "FORCE_MASK"
]);
function parseEvexPermits(string) {
  let permits = new EvexPermits();
  for (let c of string) {
    switch (c) {
      case "k":
        permits.MASK = true;
        break;
      case "K":
        permits.FORCE_MASK = permits.MASK = true;
        break;
      case "z":
        permits.ZEROING = true;
        break;
      case "b":
        permits.BROADCAST_32 = true;
        break;
      case "B":
        permits.BROADCAST_64 = true;
        break;
      case "s":
        permits.SAE = true;
        break;
      case "r":
        permits.ROUNDING = true;
        break;
      case "w":
        permits.FORCEW = true;
        break;
      case "f":
        permits.FORCE = true;
        break;
    }
  }
  return permits;
}
function getSizes(format) {
  let sizes = { list: [], def: void 0, defVex: void 0, memory: void 0 };
  for (let i = 0; i < format.length; i++) {
    let defaultSize = false, defaultVexSize = false, memorySize = false, size = 0, sizeChar = format[i];
    if (sizeChar == "$")
      size |= SIZETYPE_IMPLICITENC, sizeChar = format[++i];
    if (sizeChar == "#")
      defaultSize = true, sizeChar = format[++i];
    if (sizeChar == "~")
      defaultVexSize = true, sizeChar = format[++i];
    if (sizeChar == "|")
      memorySize = true, sizeChar = format[++i];
    if (sizeChar < "a")
      defaultSize = true, size |= sizers[sizeChar.toLowerCase()] | SIZETYPE_IMPLICITENC;
    else
      size |= sizers[sizeChar];
    if (memorySize)
      sizes.memory = size;
    else
      sizes.list.push(size);
    if (defaultSize)
      sizes.def = size;
    if (defaultVexSize)
      sizes.defVex = size;
  }
  return sizes;
}
var sizeLen = (x) => x == 32 ? 4n : x == 16 ? 2n : 1n;
var absolute = (x) => x < 0n ? ~x : x;
var OpCatcher = class {
  /**
   * Constructor
   * @param {string} format 
   */
  constructor(format) {
    opCatcherCache[format] = this;
    let i = 1;
    this.sizes = [];
    this.forceRM = format[0] == "^";
    this.vexOpImm = format[0] == "<";
    this.vexOp = this.vexOpImm || format[0] == ">";
    this.moffset = format[0] == "%";
    if (this.forceRM || this.vexOp || this.moffset)
      format = format.slice(1);
    this.carrySizeInference = format[0] != "*";
    if (!this.carrySizeInference)
      format = format.slice(1);
    let opType = format[0];
    this.acceptsMemory = "rvbkm".includes(opType);
    this.unsigned = opType == "i";
    this.type = OPC[opType.toLowerCase()];
    this.forceRM = this.forceRM || this.acceptsMemory || this.type === OPT.VMEM;
    this.carrySizeInference = this.carrySizeInference && this.type !== OPT.IMM && this.type !== OPT.MEM;
    this.implicitValue = null;
    if (format[1] == "_") {
      this.implicitValue = parseInt(format[2]);
      i = 3;
    }
    this.defSize = this.defVexSize = -1;
    if (format[i] == "!") {
      this.sizes = 0;
      this.hasByteSize = false;
    } else if (format[i] == "/") {
      this.sizes = -2;
      this.hasByteSize = false;
      this.sizeDivisor = +(format[i + 1] || 2);
    } else {
      let sizeData = getSizes(format.slice(i));
      this.sizes = sizeData.list;
      if (sizeData.def)
        this.defSize = this.defVexSize = sizeData.def;
      if (sizeData.defVex)
        this.defVexSize = sizeData.defVex;
      if (sizeData.memory)
        this.memorySize = sizeData.memory;
      this.hasByteSize = this.sizes.some((x) => (x & 8) === 8);
    }
    if (this.sizes.length == 0) {
      if (!this.type.hasSize)
        this.sizes = 0;
      else
        this.sizes = -1;
    }
  }
  /** Attempt to "catch" a given operand.
   * @param {Operand} operand
   * @param {number} prevSize
   * @param {boolean} isVex
   * @returns {number|null} The operand's corrected size on success, null on failure
   */
  catch(operand, prevSize, isVex) {
    let opSize = this.moffset ? operand.dispSize : this.unsigned ? operand.unsignedSize : operand.size;
    let rawSize, size = 0, found = false;
    let defSize = isVex ? this.defVexSize : this.defSize;
    if (isNaN(opSize)) {
      if (operand.type === OPT.MEM && this.memorySize)
        return this.memorySize;
      if (defSize > 0)
        return defSize;
      else if (this.moffset) {
        if (currBitness == 64 && operand.value.inferSize() == 64)
          opSize = 64;
        else
          return null;
      } else if (this.sizes == -2) {
        opSize = (prevSize & ~7) / this.sizeDivisor;
        if (operand.type.isVector && (opSize < 128 || this.sizeDivisor > 2))
          opSize = 128;
      } else
        opSize = prevSize & ~7;
    } else if (this.type === OPT.IMM && defSize > 0 && defSize < opSize)
      return defSize;
    if (operand.type === OPT.MEM && this.memorySize)
      return operand.size == this.memorySize ? this.memorySize : null;
    if (this.sizes == -1) {
      rawSize = prevSize & ~7;
      if (opSize == rawSize || operand.type === OPT.IMM && opSize < rawSize)
        return Math.max(0, prevSize);
      return null;
    }
    if (this.sizes == -2) {
      rawSize = (prevSize & ~7) / this.sizeDivisor;
      if (operand.type.isVector && (rawSize < 128 || this.sizeDivisor > 2))
        rawSize = 128;
      if (opSize == rawSize)
        return opSize | SIZETYPE_IMPLICITENC;
      return null;
    }
    if (this.sizes !== 0) {
      for (size of this.sizes) {
        if (size == 64 && currBitness == 32 && this.type == OPT.REG)
          continue;
        rawSize = size & ~7;
        if (opSize == rawSize || (this.type === OPT.IMM || this.type === OPT.REL) && opSize < rawSize) {
          found = true;
          break;
        }
      }
      if (!found)
        return null;
    }
    return size;
  }
};
var Operation = class {
  /**
   * Constructor
   * @param {string[]} format 
   */
  constructor(format) {
    this.vexBase = 0;
    this.evexPermits = null;
    this.actuallyNotVex = false;
    this.vexOnly = false;
    this.requireMask = false;
    this.requireBitness = null;
    this.forceVex = format[0][0] == "V";
    this.vexOnly = format[0][0] == "v";
    if ("vVwl!xX".includes(format[0][0])) {
      let specializers = format.shift();
      if (specializers.includes("w")) this.vexBase |= 32768;
      if (specializers.includes("l")) this.vexBase |= 1024;
      if (specializers.includes("!"))
        this.actuallyNotVex = true;
      if (specializers.includes("x")) this.requireBitness = 32;
      if (specializers.includes("X")) this.requireBitness = 64;
    }
    let [opcode, extension] = format.shift().split(".");
    let adderSeparator = opcode.indexOf("+");
    if (adderSeparator < 0)
      adderSeparator = opcode.indexOf("-");
    if (adderSeparator >= 0) {
      this.opDiff = parseInt(opcode.slice(adderSeparator));
      opcode = opcode.slice(0, adderSeparator);
    } else
      this.opDiff = 1;
    if (opcode.includes(")"))
      [this.prefix, this.code] = opcode.split(")").map((x) => parseInt(x, 16));
    else {
      this.code = parseInt(opcode, 16);
      this.prefix = null;
    }
    if (extension === void 0) {
      this.extension = REG_MOD;
      this.modExtension = null;
    } else {
      if (extension[0] == "o")
        this.extension = REG_OP;
      else
        this.extension = parseInt(extension[0]);
      this.modExtension = extension[1] ? parseInt(extension[1]) : null;
    }
    this.allVectors = false;
    this.relativeSizes = null;
    this.allowVex = this.forceVex || format.some((op) => op.includes(">"));
    this.maxSize = 0;
    this.vexOpCatchers = this.allowVex ? [] : null;
    this.opCatchers = [];
    if (format.length == 0)
      return;
    let opCatcher;
    for (let operand of format) {
      if (operand == ">")
        continue;
      if (operand[0] == "{") {
        this.fixedDispMul = null;
        let permitsString = operand.slice(1).replace(/T[0-9R]/g, (substr) => {
          let type = substr[1];
          if (type == "R")
            this.fixedDispMul = "R";
          else
            this.fixedDispMul = 1 << substr[1];
          return "";
        });
        this.evexPermits = parseEvexPermits(permitsString);
        if (this.evexPermits.FORCE)
          this.vexOnly = true;
        if (this.evexPermits.FORCE_MASK)
          this.requireMask = true;
        continue;
      }
      opCatcher = opCatcherCache[operand] || new OpCatcher(operand);
      if (opCatcher.type === OPT.REL) this.relativeSizes = opCatcher.sizes;
      if (!opCatcher.vexOp || this.forceVex) this.opCatchers.push(opCatcher);
      if (this.vexOpCatchers !== null) this.vexOpCatchers.push(opCatcher);
      if (Array.isArray(opCatcher.sizes)) {
        let had64 = false;
        for (let size of opCatcher.sizes) {
          if (size > this.maxSize)
            this.maxSize = size & ~7;
          if ((size & ~7) == 64)
            had64 = true;
          else if (had64 && (size & ~7) > 64)
            this.allVectors = true;
        }
      }
    }
    if (this.allowVex || this.forceVex) {
      this.vexBase |= 30720 | [15, 3896, 3898].indexOf(this.code >> 8) + 1 | [null, 102, 243, 242].indexOf(this.prefix) << 8;
    }
  }
  /**
   * Check if the given VEX data is appropriate for this operation
   * @param {VexData} vexInfo
   * @returns {boolean}
   */
  validateVEX(vexInfo) {
    if (vexInfo.needed) {
      if (this.actuallyNotVex || !this.allowVex)
        return false;
      if (vexInfo.evex) {
        if (this.evexPermits === null || !this.evexPermits.MASK && vexInfo.mask > 0 || !(this.evexPermits.BROADCAST_32 || this.evexPermits.BROADCAST_64) && vexInfo.broadcast !== null || !this.evexPermits.ROUNDING && vexInfo.round > 0 || !this.evexPermits.SAE && vexInfo.round === 0 || !this.evexPermits.ZEROING && vexInfo.zeroing)
          return false;
      } else if (this.evexPermits?.FORCE)
        vexInfo.evex = true;
    } else if (this.vexOnly || this.evexPermits?.FORCE)
      return false;
    if (this.evexPermits?.FORCE_MASK && vexInfo.mask == 0)
      return false;
    return true;
  }
  /**
   * Attempt to fit the operand list into the operation
   * @param {Operand[]} operands
   * @param {Instruction} instr
   * @param {VexData} vexInfo
   * @returns an object containing encoding data, or null if the operand
   * list didn't fit
   */
  fit(operands, instr2, vexInfo) {
    if (!this.validateVEX(vexInfo))
      return null;
    let adjustByteOp = false, overallSize = 0, rexw = false;
    if (this.relativeSizes) {
      if (!(operands.length == 1 && operands[0].type === OPT.REL))
        return null;
      operands[0].size = this.getRelSize(operands[0], instr2);
    }
    let opCatchers = vexInfo.needed ? this.vexOpCatchers : this.opCatchers;
    if (operands.length != opCatchers.length)
      return null;
    let correctedSizes = new Array(operands.length), size = -1, prevSize = -1, i, catcher;
    for (i = 0; i < operands.length; i++) {
      catcher = opCatchers[i];
      if (size > 0 || Array.isArray(catcher.sizes)) {
        size = catcher.catch(operands[i], size, vexInfo.needed, vexInfo.broadcast !== null);
        if (size === null)
          return null;
      }
      correctedSizes[i] = size;
      if (size >= 512 && !vexInfo.evex) {
        vexInfo.evex = true;
        if (!this.validateVEX(vexInfo))
          return null;
      }
      if (!catcher.carrySizeInference)
        size = prevSize;
      prevSize = size;
    }
    for (i = 0; i < operands.length; i++) {
      if (correctedSizes[i] < 0) {
        size = opCatchers[i].catch(operands[i], size, vexInfo.needed);
        if (size === null)
          return null;
        correctedSizes[i] = size;
      }
    }
    let reg = null, rm = null, vex = this.vexBase, imms = [], correctedOpcode = this.code, evexImm = null, relImm = null, moffs = null;
    let extendOp = false, unsigned = false, dispMul = null;
    let operand;
    for (i = 0; i < operands.length; i++) {
      catcher = opCatchers[i], operand = operands[i];
      size = correctedSizes[i];
      if (catcher.moffset)
        operand.dispSize = size & ~7;
      else {
        operand.size = size & ~7;
        if (operand.size != 0)
          operand.recordSizeUse(operand.size, catcher.unsigned);
      }
      if (catcher.unsigned)
        unsigned = true;
      if (operand.size == 64 && !(size & SIZETYPE_IMPLICITENC) && !this.allVectors)
        rexw = true;
      if (catcher.implicitValue === null) {
        if (operand.type === OPT.IMM)
          imms.push(operand);
        else if (catcher.type === OPT.REL) {
          relImm = operand;
          instr2.ipRelative = true;
        } else if (catcher.moffset)
          moffs = operand;
        else if (catcher.forceRM)
          rm = operand;
        else if (catcher.vexOp) {
          if (catcher.vexOpImm)
            evexImm = BigInt(operand.reg << 4);
          else
            vex = vex & ~30720 | (~operand.reg & 15) << 11;
          if (operand.reg >= 16)
            vex |= 524288;
        } else
          reg = operand;
        if (operand.type === OPT.VEC && operand.size == 64 && vexInfo.needed)
          throw new ASMError("Can't encode MMX with VEX prefix", operand.endPos);
      }
      if (!catcher.moffset && overallSize < (size & ~7) && !(size & SIZETYPE_IMPLICITENC))
        overallSize = size & ~7;
      if (size >= 16)
        adjustByteOp = adjustByteOp || catcher.hasByteSize;
    }
    if (this.extension == REG_OP) {
      correctedOpcode += reg.reg & 7;
      extendOp = reg.reg > 7;
      reg = null;
    } else if (this.extension != REG_MOD) {
      if (rm === null) {
        if (this.modExtension === null)
          rm = reg;
        else
          rm = { type: OPT.MEM, reg: this.modExtension, value: null };
      }
      reg = { reg: this.extension };
    }
    vexInfo.needed = vexInfo.needed || this.forceVex;
    if (vexInfo.needed) {
      if (this.allVectors)
        vex |= 256;
      if (vexInfo.evex) {
        vex |= 1024;
        if (vexInfo.zeroing)
          vex |= 8388608;
        if (vexInfo.round !== null) {
          if (overallSize !== this.maxSize)
            throw new ASMError("Invalid vector size for embedded rounding", vexInfo.roundingPos);
          if (vexInfo.round > 0)
            vexInfo.round--;
          vex |= vexInfo.round << 21 | 1048576;
        } else {
          let sizeId = [128, 256, 512].indexOf(overallSize);
          vex |= sizeId << 21;
          if (vexInfo.broadcast !== null) {
            let intendedSize = vexInfo.broadcastOperand.size;
            let baseSize = this.evexPermits.BROADCAST_32 ? 32 : 64;
            let broadcastSize = baseSize << vexInfo.broadcast;
            if (broadcastSize !== intendedSize)
              throw new ASMError("Invalid broadcast", vexInfo.broadcastPos);
            vex |= 1048576;
            dispMul = baseSize >> 3;
          } else if (this.opCatchers.some((x) => x.acceptsMemory && x.sizes == -2))
            dispMul = (overallSize >> 3) / this.opCatchers.find((x) => x.sizes == -2).sizeDivisor;
          else if (this.opCatchers[0].type == OPT.VEC && !this.opCatchers[0].carrySizeInference)
            dispMul = 16;
          else if (this.opCatchers.some((x) => x.memorySize))
            dispMul = this.opCatchers.find((x) => x.memorySize).memorySize >> 3;
          else if (this.fixedDispMul !== null) {
            if (this.fixedDispMul === "R") {
              let memory = operands.find((x) => x.type === OPT.MEM);
              if (memory)
                dispMul = memory.size >> 3;
            } else
              dispMul = this.fixedDispMul;
          } else
            dispMul = overallSize >> 3;
          if (operands.some((x) => x.type === OPT.VMEM))
            dispMul = null;
        }
        vex |= vexInfo.mask << 16;
        if (this.evexPermits.FORCEW)
          vex |= 32768;
        if (reg.reg >= 16)
          vex |= 16, reg.reg &= 15;
        if (rm.reg2 >= 16)
          vex |= 524288;
      } else if (overallSize == 256)
        vex |= 1024;
    } else {
      if (overallSize > 128) {
        for (let reg2 of operands)
          if (reg2.size > 128 && reg2.endPos)
            throw new ASMError("YMM/ZMM registers can't be encoded without VEX", reg2.endPos);
      }
      for (let reg2 of operands)
        if (reg2.type === OPT.VEC && reg2.reg >= 16 && reg2.endPos)
          throw new ASMError("Registers with ID >= 16 can't be encoded without EVEX", reg2.endPos);
    }
    if (adjustByteOp)
      correctedOpcode += this.opDiff;
    return {
      opcode: correctedOpcode,
      size: overallSize,
      rexw,
      prefix: vexInfo.needed ? null : this.allVectors && overallSize > 64 ? 102 : this.prefix,
      extendOp,
      /** @type {Operand} */
      reg,
      /** @type {Operand} */
      rm,
      vex: vexInfo.needed ? vex : null,
      evexImm,
      relImm,
      imms,
      unsigned,
      moffs,
      dispMul
    };
  }
  /**
   * Predict a fitting size for a given relative operand
   * @param {Operand} operand
   * @param {Instruction} instr
  */
  getRelSize(operand, instr2) {
    if (operand.value.isRelocatable())
      return Math.max(...this.relativeSizes);
    const target = operand.value.addend - BigInt((this.code > 255 ? 2 : 1) + (this.prefix !== null ? 1 : 0));
    if (this.relativeSizes.length == 1) {
      const size = this.relativeSizes[0];
      if (absolute(target - sizeLen(size)) >= 1n << BigInt(size - 1))
        throw new ASMError(`Can't fit offset in ${size >> 3} byte${size != 8 ? "s" : ""}`, operand.startPos.until(operand.endPos));
      return size;
    }
    let [small, large] = this.relativeSizes;
    let smallLen = sizeLen(small), largeLen = sizeLen(large) + (this.opDiff > 256 ? 1n : 0n);
    if (absolute(target - smallLen) >= 1n << BigInt(small - 1) || !operand.sizeAllowed(small, false)) {
      if (small != operand.size && operand.sizeAllowed(small, false)) {
        queueRecomp(instr2);
        return small;
      }
      if (absolute(target - largeLen) >= 1n << BigInt(large - 1))
        throw new ASMError(`Can't fit offset in ${large >> 3} bytes`, operand.startPos.until(operand.endPos));
      return large;
    }
    return small;
  }
  /**
   * Check if a list of operands has the right types for this operation
   * @param {Operand[]} operands 
   * @param {VexData} vexInfo 
   */
  matchTypes(operands, vexInfo) {
    if (vexInfo.mask == 0 && this.requireMask)
      return false;
    let opCatchers = vexInfo.needed ? this.vexOpCatchers : this.opCatchers;
    if (operands.length != opCatchers.length)
      return false;
    for (let i = 0; i < operands.length; i++) {
      const catcher = opCatchers[i], operand = operands[i];
      if (
        // Check that the types match
        operand.type != catcher.type && !(operand.type === OPT.MEM && catcher.acceptsMemory) || // In case of implicit operands, check that the values match
        catcher.implicitValue !== null && catcher.implicitValue !== (operand.type === OPT.IMM ? Number(operand.value.addend) : operand.reg) || // Super special case: if the operand is of type moffset,
        // make sure it is only an offset
        catcher.moffset && (operand.reg >= 0 || operand.reg2 >= 0)
      )
        return false;
    }
    return true;
  }
};

// ../node_modules/@defasm/core/mnemonicList.js
var mnemonicList_default = `
aaa:x 37
aad
x D50A
x D5 ib

aam
x D40A
x D4 ib

aas:x 3F

adcx:66)0F38F6 r Rlq

addpd:66)0F58 v >V Vxyz {kzrBw
addps:0F58 v >V Vxyz {kzrb
addsd:F2)0F58 v >V Vx {kzrw
addss:F3)0F58 v >V Vx {kzr

addsubpd:66)0FD0 v >V Vxy
addsubps:F2)0FD0 v >V Vxy

adox:F3)0F38F6 r Rlq
aesdec:66)0F38DE v >V Vxyz {
aesdeclast:66)0F38DF v >V Vxyz {
aesenc:66)0F38DC v >V Vxyz {
aesenclast:66)0F38DD v >V Vxyz {
aesimc:66)0F38DB v Vx >
aeskeygenassist:66)0F3ADF ib v Vx >
andn:V 0F38F2 r >Rlq R

andpd:66)0F54 v >V Vxyz {kzBw
andps:0F54 v >V Vxyz {kzb

andnpd:66)0F55 v >V Vxyz {kzBw
andnps:0F55 v >V Vxyz {kzb

bextr:V 0F38F7 >Rlq r R

blendpd:66)0F3A0D ib v >V Vxy
blendps:66)0F3A0C ib v >V Vxy

blendvpd
66)0F3815 V_0x v V
v 66)0F3A4B <Vxy v >V V

blendvps
66)0F3814 V_0x v V
v 66)0F3A4A <Vxy v >V V

blsi:V 0F38F3.3 r >Rlq
blsmsk:V 0F38F3.2 r >Rlq
blsr:V 0F38F3.1 r >Rlq
bndcl:F3)0F1A rQ B
bndcn:F2)0F1B rQ B
bndcu:F2)0F1A rQ B
bndldx:0F1A m B
bndmk:F3)0F1B m B

bndmov
66)0F1A b B
66)0F1B B b

bndstx:0F1B B m

bound:x 62 m Rwl

bsf:0FBC r Rwlq
bsr:0FBD r Rwlq
bswap:0FC8.o Rlq

bt
0FA3 Rwlq r
0FBA.4 iB rwlq

btc
0FBB Rwlq r
0FBA.7 iB rwlq

btr
0FB3 Rwlq r
0FBA.6 iB rwlq

bts
0FAB Rwlq r
0FBA.5 iB rwlq

bzhi:V 0F38F5 >Rlq r R

call
E8 jl
x FF.2 rwL
X FF.2 rQ
FF.3 mf

callw:x E8 jw

cbtw/cbw:66)98
cltd/cdq:99
cltq/cdqe:X 48)98
clac:0F01CA
clc:F8
cld:FC
cldemote:0F1C.0 m
clflush:0FAE.7 m
clflushopt:66)0FAE.7 m
cli:FA
clrssbsy:F3)0FAE.6 m
clts:0F06
clwb:66)0FAE.6 m
cmc:F5

cmppd
66)0FC2 ib v >V Vxy
66)0FC2 ib v >Vxyz K {kBsfw

cmpps
0FC2 ib v >V Vxy
0FC2 ib v >Vxyz K {kbsf

cmps{bwlq:A6

cmpsd
F2)0FC2 ib v >V Vx
F2)0FC2 ib v|Q >Vx K {ksfw

cmpss
F3)0FC2 ib v >V Vx
F3)0FC2 ib v|l >Vx K {ksf

cmpxchg:0FB0 Rbwlq r
cmpxchg8b:0FC7.1 m
cmpxchg16b:0FC7.1 m#q

comisd:66)0F2F v Vx > {sw
comiss:0F2F v Vx > {s

cpuid:0FA2

crc32
F2)0F38F0 rbwl RL
F2)0F38F0 rbq Rq

cvtdq2pd:F3)0FE6 v/ Vxyz > {kzb
cvtdq2ps:0F5B v Vxyz > {kzbr
cvtpd2dq:F2)0FE6 v#xy~z V/ > {kzBrw
cvtpd2pi:66)0F2D vX VQ
cvtpd2ps:66)0F5A v#xy~z V/ > {kzBrw
cvtpi2pd:66)0F2A vQ Vx
cvtpi2ps:0F2A vQ Vx
cvtps2dq:66)0F5B v Vxyz > {kzbr
cvtps2pd:0F5A v/ Vxyz > {kzbs
cvtps2pi:0F2D vX VQ
cvtsd2si:F2)0F2D v#x Rlq > {r
cvtsd2ss:F2)0F5A vX >Vx Vx {kzrw
cvtsi2sd:F2)0F2A rlq >Vx Vx {r
cvtsi2ss:F3)0F2A rlq >Vx Vx {r
cvtss2sd:F3)0F5A v >Vx Vx {kzs
cvtss2si:F3)0F2D v#x Rlq > {r
cvttpd2dq:66)0FE6 v#xy~z V/ > {kzBsw
cvttpd2pi:66)0F2C vX VQ
cvttps2dq:F3)0F5B v Vxyz > {kzbs
cvttps2pi:0F2C vX VQ
cvttsd2si:F2)0F2C v#x Rlq > {s
cvttss2si:F3)0F2C v#x Rlq > {s

cqto/cqo:X 48)99
cwtd/cwd:66)99
cwtl/cwde:98

daa:x 27
das:x 2F

dec
x 48.o Rwl
FE.1 rbwlq

div:F6.6 rbwlq

divpd:66)0F5E v >V Vxyz {kzBwr
divps:0F5E v >V Vxyz {kzbr
divsd:F2)0F5E v >V Vx {kzwr
divss:F3)0F5E v >V Vx {kzr

dppd:66)0F3A41 ib v >V Vx
dpps:66)0F3A40 ib v >V Vxy

emms:0F77
endbr32:F3)0F1EFB
endbr64:F3)0F1EFA
enter:C8 iW ib
extractps:66)0F3A17 ib Vx rL > {

f2xm1:D9F0
fabs:D9E1
fbld:DF.4 m
fbstp:DF.6 m
fchs:D9E0
fclex:9BDBE2
fcmovb:DA.0 F F_0
fcmove:DA.1 F F_0
fcmovbe:DA.2 F F_0
fcmovu:DA.3 F F_0
fcmovnb:DB.0 F F_0
fcmovne:DB.1 F F_0
fcmovnbe:DB.2 F F_0
fcmovnu:DB.3 F F_0
fcompp:DED9
fcomi:DB.6 F F_0
fcomip:DF.6 F F_0
fcos:D9FF
fdecstp:D9F6
ffree:DD.0 F

fild
DF.0 mW
DB.0 ml
DF.5 m$q

fincstp:D9F7
finit:9BDBE3

fist
DF.2 mW
DB.2 ml

fistp
DF.3 mW
DB.3 ml
DF.7 m$q

fisttp
DF.1 mW
DB.1 ml
DD.1 m$q

fld
D9.0 ml
DD.0 m$q
DB.5 mt
D9.0 F

fld1:D9E8
fldl2t:D9E9
fldl2e:D9EA
fldpi:D9EB
fldlg2:D9EC
fldln2:D9ED
fldz:D9EE
fldcw:D9.5 m
fldenv:D9.4 m
fnclex:DBE2
fninit:DBE3
fnop:D9D0
fnsave:DD.6 m
fnstcw:D9.7 m
fnstenv:D9.6 m

fnstsw
DD.7 m
DFE0 R_0W

fpatan:D9F3
fprem:D9F8
fprem1:D9F5
fptan:D9F2
frndint:D9FC
frstor:DD.4 m
fsave:9BDD.6 m
fscale:D9FD
fsin:D9FE
fsincos:D9FB
fsqrt:D9FA

fst
D9.2 ml
DD.2 m$q
DD.2 F

fstcw:9BD9.7 m
fstenv:9BD9.6 m

fstp
D9.3 ml
DD.3 m$q
DD.3 F

fstpt:DB.7 m

fstsw
9BDD.7 m
9BDFE0 R_0W

ftst:D9E4

fucom
DD.4 F
DDE1

fucomp
DD.5 F
DDE9

fucompp:DAE9
fucomi:DB.5 F F_0
fucomip:DF.5 F F_0
fwait:#wait
fxam:D9E5

fxch
D9.1 F
D9C9

fxrstor:0FAE.1 m
fxrstor64:X 0FAE.1 m#q
fxsave:0FAE.0 m
fxsave64:X 0FAE.0 m#q
fxtract:D9F4
fyl2x:D9F1
fyl2xp1:D9F9

gf2p8affineinvqb:w 66)0F3ACF ib v >V Vxyz {kzBw
gf2p8affineqb:w 66)0F3ACE ib v >V Vxyz {kzBw
gf2p8mulb:66)0F38CF v >V Vxyz {kz

haddpd:66)0F7C v >V Vxy
haddps:F2)0F7C v >V Vxy

hlt:F4

hsubpd:66)0F7D v >V Vxy
hsubps:F2)0F7D v >V Vxy

idiv:F6.7 rbwlq

imul
F6.5 rbwlq
0FAF r Rwlq
6B Ib r Rwlq
69 iw rw Rw
69 il r Rlq

in
E4 ib R_0bwl
EC R_2W R_0bwl

inc
x 40.o Rwl
FE.0 rbwlq

incsspd:F3)0FAE.5 Rl
incsspq:F3)0FAE.5 Rq
ins{bwl:6C
insertps:66)0F3A21 ib v >V Vx {

int
CC i_3b
CD ib

int1:F1
int3:CC
into:x CE
invd:0F08
invlpg:0F01.7 m
invpcid:66)0F3882 m RQ
iret{wLq:CF

jmp
EB-2 jbl
x FF.4 rwL
X FF.4 rQ
FF.5 mf

jmpw:x E9 jw

jcxz:x 67)E3 jb
jecxz
x E3 jb
X 67)E3 jb

jrcxz:X E3 jb

kaddb:Vl 66)0F4A ^K >K K
kaddw:Vl 0F4A ^K >K K
kaddd:Vlw 66)0F4A ^K >K K
kaddq:Vlw 0F4A ^K >K K

kandb:Vl 66)0F41 ^K >K K
kandw:Vl 0F41 ^K >K K
kandd:Vlw 66)0F41 ^K >K K
kandq:Vlw 0F41 ^K >K K

kandnb:Vl 66)0F42 ^K >K K
kandnw:Vl 0F42 ^K >K K
kandnd:Vlw 66)0F42 ^K >K K
kandnq:Vlw 0F42 ^K >K K

kmovb
V 66)0F90 k K >
V 66)0F91 K m >
V 66)0F92 ^Rl K >
V 66)0F93 ^K Rl >

kmovw
V 0F90 k K >
V 0F91 K m >
V 0F92 ^Rl K >
V 0F93 ^K Rl >

kmovd
Vw 66)0F90 k K >
Vw 66)0F91 K m >
V F2)0F92 ^Rl K >
V F2)0F93 ^K Rl >

kmovq
Vw 0F90 k K >
Vw 0F91 K m >
V F2)0F92 ^Rq K >
V F2)0F93 ^K Rq >

knotb:V 66)0F44 ^K K >
knotw:V 0F44 ^K K >
knotd:Vw 66)0F44 ^K K >
knotq:Vw 0F44 ^K K >

korb:Vl 66)0F45 ^K >K K
korw:Vl 0F45 ^K >K K
kord:Vlw 66)0F45 ^K >K K
korq:Vlw 0F45 ^K >K K

kortestb:V 66)0F98 ^K K >
kortestw:V 0F98 ^K K >
kortestd:Vw 66)0F98 ^K K >
kortestq:Vw 0F98 ^K K >

kshiftlb:V 66)0F3A32 iB ^K K >
kshiftlw:Vw 66)0F3A32 iB ^K K >
kshiftld:V 66)0F3A33 iB ^K K >
kshiftlq:Vw 66)0F3A33 iB ^K K >

kshiftrb:V 66)0F3A30 iB ^K K >
kshiftrw:Vw 66)0F3A30 iB ^K K >
kshiftrd:V 66)0F3A31 iB ^K K >
kshiftrq:Vw 66)0F3A31 iB ^K K >


ktestb:V 66)0F99 ^K K >
ktestw:V 0F99 ^K K >
ktestd:Vw 66)0F99 ^K K >
ktestq:Vw 0F99 ^K K >

kunpckbw:Vl 66)0F4B ^K >K K
kunpckdq:Vlw 0F4B ^K >K K
kunpckwd:Vl 0F4B ^K >K K

kxnorb:Vl 66)0F46 ^K >K K
kxnorw:Vl 0F46 ^K >K K
kxnord:Vlw 66)0F46 ^K >K K
kxnorq:Vlw 0F46 ^K >K K

kxorb:Vl 66)0F47 ^K >K K
kxorw:Vl 0F47 ^K >K K
kxord:Vlw 66)0F47 ^K >K K
kxorq:Vlw 0F47 ^K >K K

lahf:9F
lar:0F02 rW Rwlq
lcall/:FF.3 m

lds:x C5 m Rwl
lss:0FB2 m Rwl
les:x C4 m Rwl
lfs:0FB4 m Rwl
lgs:0FB5 m Rwl

lddqu:F2)0FF0 m Vxy >
ldmxcsr:0FAE.2 m >
lea:8D m Rwlq
leave:C9
lfence:0FAEE8
lgdt:0F01.2 m
lidt:0F01.3 m
ljmp/:FF.5 m
lldt:0F00.2 rW
lmsw:0F01.6 rW
lods{bwlq:AC
loop:E2 jb
loope:E1 jb
loopne:E0 jb

lret
CB
CA i$w

lsl:0F03 rW Rwlq
ltr:0F00.3 rW
lzcnt:F3)0FBD r Rwlq

maskmovdqu:66)0FF7 ^Vx V >
maskmovq:0FF7 ^VQ V

maxpd:66)0F5F v >V Vxyz {kzBsw
maxps:0F5F v >V Vxyz {kzbs
maxsd:F2)0F5F v >V Vx {kzsw
maxss:F3)0F5F v >V Vx {kzs

mfence:0FAEF0

minpd:66)0F5D v >V Vxyz {kzBsw
minps:0F5D v >V Vxyz {kzbs
minsd:F2)0F5D v >V Vx {kzsw
minss:F3)0F5D v >V Vx {kzs

monitor:0F01C8

mov
X A0 %mlq R_0bwlq
X A2 R_0bwlq %mlq
88 Rbwlq r
8A r Rbwlq
X C7.0 Il Rq
X C7.0 iL mq
B0+8.o i Rbwlq
C6.0 i rbwl
8C s ^RwlQ
8C s mW
8E ^RWlQ s
8E mW s
X 0F20 C ^RQ
X 0F21 D ^RQ
X 0F22 ^RQ C
X 0F23 ^RQ D
x 0F20 C ^RL
x 0F21 D ^RL
x 0F22 ^RL C
x 0F23 ^RL D

movabs/
X A0 %mlQ R_0bwlq
X A2 R_0bwlq %mlQ
X B8.o i Rq

movapd
66)0F28 v Vxyz > {kzw
66)0F29 Vxyz v > {kzw

movaps
0F28 v Vxyz > {kz
0F29 Vxyz v > {kz

movbe
0F38F0 m Rwlq
0F38F1 Rwlq m

movd
0F6E rL VQ
0F7E VQ rL
66)0F6E rL Vx > {
66)0F7E Vx rL > {

movddup:F2)0F12 v Vxyz > {kzw
movdiri:0F38F9 Rlq m
movdir64b:66)0F38F8 m RQ

movdqa
66)0F6F v Vxy >
66)0F7F Vxy v >

movdqa32
66)0F6F v Vxyz > {kzf
66)0F7F Vxyz v > {kzf

movdqa64
66)0F6F v Vxyz > {kzfw
66)0F7F Vxyz v > {kzfw

movdqu
F3)0F6F v Vxy >
F3)0F7F Vxy v >

movdqu8
F2)0F6F v Vxyz > {kzf
F2)0F7F Vxyz v > {kzf

movdqu16
F2)0F6F v Vxyz > {kzfw
F2)0F7F Vxyz v > {kzfw

movdqu32
F3)0F6F v Vxyz > {kzf
F3)0F7F Vxyz v > {kzf

movdqu64
F3)0F6F v Vxyz > {kzfw
F3)0F7F Vxyz v > {kzfw

movdq2q:F2)0FD6 ^Vx VQ
movhlps:0F12 ^Vx >V V {

movhpd
66)0F16 m >V Vx {w
66)0F17 Vx m > {w

movhps
0F16 m >V Vx {
0F17 Vx m > {

movlhps:0F16 ^Vx >V V {

movlpd
66)0F12 m >V Vx {w
66)0F13 Vx m > {w

movlps
0F12 m >V Vx {
0F13 Vx m > {

movmskpd:66)0F50 ^Vxy R! >
movmskps:0F50 ^Vxy R! >

movntdqa:66)0F382A m Vxyz > {
movntdq:66)0FE7 Vxyz m > {
movnti:0FC3 Rlq m

movntpd:66)0F2B Vxyz m > {w
movntps:0F2B Vxyz m > {

movntq:0FE7 VQ m

movq
0F6E ^R Vq
0F7E Vq ^R
66)0F6E ^R#q VX > {
66)0F7E VX ^R#q > {
0F6F vQ V
0F7F VQ v
F3)0F7E v Vx > {w
66)0FD6 Vx v > {w

movq2dq:F3)0FD6 ^VQ Vx
movs{bwlq:A4

movsd
F2)0F10 ^Vx >V V {kzw
F2)0F10 m Vx > {kzw
F2)0F11 Vx m > {kw

movshdup:F3)0F16 v Vxyz > {kz

movsldup:F3)0F12 v Vxy > {kz

movss
F3)0F10 ^Vx >V V {kz
F3)0F10 m Vx > {kz
F3)0F11 Vx m > {k

movsbw/:0FBE rB Rw
movsbl/:0FBE rB Rl
movsbq/:0FBE rB Rq
movswl/:0FBF rW Rl
movswq/:0FBF rW Rq
movslq/:63 rL Rq
movsxd/:63 rL Rq
/movsxd:63 rL Rwlq
movsx:0FBE rb$w Rwlq

movupd
66)0F10 v Vxyz > {kzw
66)0F11 Vxyz v > {kzw

movups
0F10 v Vxyz > {kz
0F11 Vxyz v > {kz

movzbw/:0FB6 rB Rw
movzbl/:0FB6 rB Rl
movzwl/:0FB7 rW Rl
movzx:0FB6 rb$w Rwlq
mpsadbw:66)0F3A42 ib v >V Vxy
mul:F6.4 rbwlq

mulpd:66)0F59 v >V Vxyz {kzBrw
mulps:0F59 v >V Vxyz {kzbr
mulsd:F2)0F59 v >V Vx {kzrw
mulss:F3)0F59 v >V Vx {kzr

mulx:V F2)0F38F6 r >Rlq R
mwait:0F01C9

neg:F6.3 rbwlq

nop
90
0F1F.0 rwL

not:F6.2 rbwlq

orpd:66)0F56 v >V Vxyz {kzBw
orps:0F56 v >V Vxyz {kzb

out
E6 R_0bwl ib
EE R_0bwl R_2W

outs{bwl:6E

pabsb:0F381C v Vqxyz > {kz
pabsd:0F381E v Vqxyz > {kzb
pabsq:66)0F381F v Vxyz > {kzBwf
pabsw:0F381D v Vqxyz > {kz

packssdw:0F6B v >V Vqxyz {kzb
packsswb:0F63 v >V Vqxyz {kz
packusdw:66)0F382B v >V Vxyz {kzb
packuswb:0F67 v >V Vqxyz {kz

paddb:0FFC v >V Vqxyz {kz
paddd:0FFE v >V Vqxyz {kbz
paddq:0FD4 v >V Vqxyz {kBzw
paddw:0FFD v >V Vqxyz {kz

paddsb:0FEC v >V Vqxyz {kz
paddsw:0FED v >V Vqxyz {kz
paddusb:0FDC v >V Vqxyz {kz
paddusw:0FDD v >V Vqxyz {kz

palignr:0F3A0F ib v >V Vqxyz {kz

pand:0FDB v >V Vqxy
pandd:66)0FDB v >V Vxyz {kzbf
pandq:66)0FDB v >V Vxyz {kzBwf

pandn:0FDF v >V Vqxy
pandnd:66)0FDF v >V Vxyz {kzbf
pandnq:66)0FDF v >V Vxyz {kzBwf

pause:F3)90

pavgb:0FE0 v >V Vqxyz {kz
pavgw:0FE3 v >V Vqxyz {kz

pblendvb
66)0F3810 V_0x v V
v 66)0F3A4C <Vxy v >V V

pblendw:66)0F3A0E ib v >V Vxy
pclmulqdq:66)0F3A44 ib v >V Vxyz {

pcmpeqb
0F74 v >V Vqxy
66)0F74 v >Vxyz K {kf

pcmpeqd
0F76 v >V Vqxy
66)0F76 v >Vxyz K {kbf

pcmpeqw
0F75 v >V Vqxy
66)0F75 v >Vxyz K {kf

pcmpeqq
66)0F3829 v >V Vxy
66)0F3829 v >Vxyz K {kBwf

pcmpestri:66)0F3A61 ib v Vx >

pcmpestrm:66)0F3A60 ib v Vx >

pcmpgtb
0F64 v >V Vqxy
66)0F64 v >Vxyz K {kf

pcmpgtd
0F66 v >V Vqxy
66)0F66 v >Vxyz K {kbf

pcmpgtq
66)0F3837 v >V Vxy
66)0F3837 v >Vxyz K {kBwf

pcmpgtw
0F65 v >V Vqxy
66)0F65 v >Vxyz K {kf

pcmpistri:66)0F3A63 ib v Vx >
pcmpistrm:66)0F3A62 ib v Vx >

pdep:V F2)0F38F5 r >Rlq R
pext:V F3)0F38F5 r >Rlq R

pextrb:66)0F3A14 ib Vx r! > {
pextrd:66)0F3A16 ib Vx rL > {

pextrw
0FC5 ib ^Vqx R! > {
66)0F3A15 ib Vx m > {

pextrq:66)0F3A16 ib Vx r#q > {

phaddw:0F3801 v >V Vqxy
phaddd:0F3802 v >V Vqxy
phaddsw:0F3803 v >V Vqxy

phminposuw:66)0F3841 v Vx >

phsubd:0F3806 v >V Vqxy
phsubsw:0F3807 v >V Vqxy
phsubw:0F3805 v >V Vqxy

pinsrb:66)0F3A20 ib rL >Vx Vx {
pinsrd:66)0F3A22 ib rL >Vx Vx {
pinsrq:66)0F3A22 ib r#q >Vx Vx {
pinsrw:0FC4 ib *rL >V Vqx {

pmaddubsw:0F3804 v >V Vqxyz {kz
pmaddwd:0FF5 v >V Vqxyz {kz

pmaxsb:66)0F383C v >V Vxyz {kz
pmaxsd:66)0F383D v >V Vxyz {kzb
pmaxsq:66)0F383D v >V Vxyz {kzBwf
pmaxsw:0FEE v >V Vqxyz {kz

pmaxub:0FDE v >V Vqxyz {kz
pmaxud:66)0F383F v >V Vxyz {kzb
pmaxuq:66)0F383F v >V Vxyz {kzBwf
pmaxuw:66)0F383E v >V Vxyz {kz

pminsb:66)0F3838 v >V Vxyz {kz
pminsw:0FEA v >V Vqxyz {kz
pminsq:66)0F3839 v >V Vxyz {kzBwf
pminsd:66)0F3839 v >V Vxyz {kzb

pminub:0FDA v >V Vqxyz {kz
pminud:66)0F383B v >V Vxyz {kzb
pminuq:66)0F383B v >V Vxyz {kzBwf
pminuw:66)0F383A v >V Vxyz {kz

pmovmskb:0FD7 ^Vqxy R! >
pmovsxbw:66)0F3820 v/ Vxyz > {kz
pmovsxbd:66)0F3821 v/4 Vxyz > {kz
pmovsxbq:66)0F3822 v/8 Vxyz > {kz
pmovsxwd:66)0F3823 v/ Vxyz > {kz
pmovsxwq:66)0F3824 v/4 Vxyz > {kz
pmovsxdq:66)0F3825 v/ Vxyz > {kz

pmovzxbw:66)0F3830 v/ Vxyz > {kz
pmovzxbd:66)0F3831 v/4 Vxyz > {kz
pmovzxbq:66)0F3832 v/8 Vxyz > {kz
pmovzxwd:66)0F3833 v/ Vxyz > {kz
pmovzxwq:66)0F3834 v/4 Vxyz > {kz
pmovzxdq:66)0F3835 v/ Vxyz > {kz

pmuldq:66)0F3828 v >V Vxyz {kzBw

pmulhrsw:0F380B v >V Vqxyz {kz
pmulhuw:0FE4 v >V Vqxyz {kz
pmulhw:0FE5 v >V Vqxyz {kz

pmulld:66)0F3840 v >V Vxyz {kzb
pmullq:66)0F3840 v >V Vxyz {kzBfw
pmullw:0FD5 v >V Vqxyz {kz
pmuludq:0FF4 v >V Vqxyz {kzBw

pop
X 58.o RwQ
x 58.o Rwl
X 8F.0 mwQ
x 8F.0 mwL
x 07 s_0
x 17 s_2
x 1F s_3
0FA1 s_4
0FA9 s_5

popa:x 61
popad:#popa

popcnt:F3)0FB8 r Rwlq

popf:9D
popfq:#popf
popfw:66)9D

por:0FEB v >V Vqxy
pord:66)0FEB v >Vxyz V {kzbf
porq:66)0FEB v >Vxyz V {kzBwf

prefetcht0:0F18.1 m
prefetcht1:0F18.2 m
prefetcht2:0F18.3 m
prefetchnta:0F18.0 m
prefetchw:0F0D.1 m

psadbw:0FF6 v >V Vqxyz {

pshufb:0F3800 v >V Vqxyz {kz
pshufd:66)0F70 ib v Vxyz > {kzb
pshufhw:F3)0F70 ib v Vxyz > {kz
pshuflw:F2)0F70 ib v Vxyz > {kz
pshufw:0F70 ib v VQ

psignb:0F3808 v >V Vqxy
psignd:0F380A v >V Vqxy
psignw:0F3809 v >V Vqxy

pslldq
66)0F73.7 ib Vxy >V
66)0F73.7 ib v >Vxyz {f

pslld
0FF2 vQ VQ
66)0FF2 *vX >V Vxyz {kz
0F72.6 ib Vqxy >V
66)0F72.6 ib v >Vxyz {kzbf

psllq
0FF3 vQ VQ
66)0FF3 *vX >V Vxyz {kzw
0F73.6 ib Vqxy >V
66)0F73.6 ib v >Vxyz {kzBfw

psllw
0FF1 vQ VQ
66)0FF1 *vX >V Vxyz {kz
0F71.6 ib Vqxy >V
66)0F71.6 ib v >Vxyz {kzf

psrad
0FE2 vQ VQ
66)0FE2 *vX >V Vxyz {kz
0F72.4 ib Vqxy >V
66)0F72.4 ib v >Vxyz {kzbf

psraq
66)0FE2 *vX >V Vxyz {kzwf
66)0F72.4 ib v >Vxyz {kzBfw

psraw
0FE1 vQ VQ
66)0FE1 *vX >V Vxyz {kz
0F71.4 ib Vqxy >V
66)0F71.4 ib v >Vxyz {kzf

psrldq
66)0F73.3 ib Vxy >V
66)0F73.3 ib v >Vxyz {f

psrld
0FD2 vQ VQ
66)0FD2 *vX >V Vxyz {kz
0F72.2 ib Vqxy >V
66)0F72.2 ib v >Vxyz {kzbf

psrlq
0FD3 vQ VQ
66)0FD3 *vX >V Vxyz {kzw
0F73.2 ib Vqxy >V
66)0F73.2 ib v >Vxyz {kzBfw

psrlw
0FD1 vQ VQ
66)0FD1 *vX >V Vxyz {kz
0F71.2 ib Vqxy >V
66)0F71.2 ib v >Vxyz {kzf

psubb:0FF8 v >V Vqxyz {kz
psubd:0FFA v >V Vqxyz {kzb
psubq:0FFB v >V Vqxyz {kzBw
psubw:0FF9 v >V Vqxyz {kz

psubsb:0FE8 v >V Vqxyz {kz
psubsw:0FE9 v >V Vqxyz {kz
psubusb:0FD8 v >V Vqxyz {kz
psubusw:0FD9 v >V Vqxyz {kz

ptest:66)0F3817 v Vxy >
ptwrite:F3)0FAE.4 rlq

punpckhbw:0F68 v >V Vqxyz {kz
punpckhwd:0F69 v >V Vqxyz {kz
punpckhdq:0F6A v >V Vqxyz {kzb
punpckhqdq:66)0F6D v >V Vxyz {kzBw

punpcklbw:0F60 v >V Vqxyz {kz
punpcklwd:0F61 v >V Vqxyz {kz
punpckldq:0F62 v >V Vqxyz {kzb
punpcklqdq:66)0F6C v >V Vxyz {kzBw

push
X 50.o RwQ
x 50.o Rwl
6A-2 Ibl
FF.6 mwQ
x 06 s_0
x 0E s_1
x 16 s_2
x 1E s_3
0FA0 s_4
0FA8 s_5

pusha:x 60
pushad:#pusha

pushf{wQ}:9C
pushw:66)6A-2 Ib$w

pxor:0FEF v >V Vqxy
pxord:66)0FEF v >Vxyz V {kzbf
pxorq:66)0FEF v >Vxyz V {kzBfw

rcpps:0F53 v Vxy >
rcpss:F3)0F53 v >V Vx

rdfsbase:F3)0FAE.0 Rlq
rdgsbase:F3)0FAE.1 Rlq
rdmsr:0F32
rdpid:F3)0FC7.7 RQ
rdpkru:0F01EE
rdpmc:0F33
rdrand:0FC7.6 Rwlq
rdseed:0FC7.7 Rwlq
rdsspd:F3)0F1E.1 Rl
rdsspq:F3)0F1E.1 Rq
rdtsc:0F31
rdtscp:0F01F9

ret
C3
C2 i$w

rorx:V F2)0F3AF0 ib r Rlq

roundpd:66)0F3A09 ib v Vxy >
roundps:66)0F3A08 ib v Vxy >
roundsd:66)0F3A0B ib v >V Vx
roundss:66)0F3A0A ib v >V Vx

rsm:0FAA

rsqrtps:0F52 v Vxy >
rsqrtss:F3)0F52 v >V Vx

rstorssp:F3)0F01.5 m

sahf:9E
sal:#shl
sarx:V F3)0F38F7 >Rlq r R
saveprevssp:F3)0F01EA
scas{bwlq:AE
setssbsy:F3)0F01E8
sfence:0FAEF8
sgdt:0F01.0 m
sha1rnds4:0F3ACC ib v Vx
sha1nexte:0F38C8 v Vx
sha1msg1:0F38C9 v Vx
sha1msg2:0F38CA v Vx
sha256rnds2:0F38CB V_0x v V
sha256msg1:0F38CC v Vx
sha256msg2:0F38CD v Vx

shld
0FA4 ib Rwlq r
0FA5 R_1b Rwlq r

shlx:V 66)0F38F7 >Rlq r R

shrd
0FAC ib Rwlq r
0FAD R_1b Rwlq r

shrx:V F2)0F38F7 >Rlq r R

shufpd:66)0FC6 ib v >V Vxyz {kzBw
shufps:0FC6 ib v >V Vxyz {kzb

sidt:0F01.1 m

sldt
0F00.0 Rwl$q
0F00.0 mW

smsw
0F01.4 Rwlq
0F01.4 mWL

sqrtpd:66)0F51 v Vxyz > {kzBrw
sqrtps:0F51 v Vxyz > {kzbr
sqrtsd:F2)0F51 v >V Vx {kzrw
sqrtss:F3)0F51 v >V Vx {kzr

stac:0F01CB
stc:F9
std:FD
sti:FB
stmxcsr:0FAE.3 m >
stos{bwlq:AA

str
0F00.1 RwL$q
0F00.1 mW

subpd:66)0F5C v >V Vxyz {kzrBw
subps:0F5C v >V Vxyz {kzrb
subsd:F2)0F5C v >V Vx {kzrw
subss:F3)0F5C v >V Vx {kzr

swapgs:X 0F01F8
syscall:X 0F05
sysenter:0F34
sysexit{Lq:0F35
sysret{Lq:X 0F07

test
A8 i R_0bwl
A9 iL R_0q
F6.0 i rbwl
F7.0 iL rq
84 Rbwlq r

tpause:66)0FAE.6 Rl
tzcnt:F3)0FBC r Rwlq

ucomisd:66)0F2E v Vx > {sw
ucomiss:0F2E v Vx > {s

ud0:0FFF rL R
ud1:0FB9 rL R
ud2:0F0B

umonitor
67F3)0FAE.6 Rl
F3)0FAE.6 RQ

umwait:F2)0FAE.6 Rl

unpckhpd:66)0F15 v >V Vxyz {kzBw
unpckhps:0F15 v >V Vxyz {kzb
unpcklpd:66)0F14 v >V Vxyz {kzBw
unpcklps:0F14 v >V Vxyz {kzb

valignd:66)0F3A03 ib v >Vxyz V {kzbf
valignq:66)0F3A03 ib v >Vxyz V {kzBfw

vblendmpd:66)0F3865 v >V Vxyz {kzBfw
vblendmps:66)0F3865 v >V Vxyz {kzbf

vbroadcastss:66)0F3818 vx|l Vxyz > {kz
vbroadcastsd:66)0F3819 vx|Q Vyz > {kzw

vbroadcastf128:66)0F381A m Vy >
vbroadcastf32x2:66)0F3819 vx|Q Vyz > {kzf
vbroadcastf32x4:66)0F381A m|x Vyz > {kzf
vbroadcastf64x2:66)0F381A m|x Vyz > {kzwf
vbroadcastf32x8:66)0F381B m|y Vz > {kzf
vbroadcastf64x4:66)0F381B m|y Vz > {kzfw

vbroadcasti128:66)0F385A m Vy >
vbroadcasti32x2:66)0F3859 vx|Q Vxyz > {kzf
vbroadcasti32x4:66)0F385A m|x Vyz > {kzf
vbroadcasti64x2:66)0F385A m|x Vyz > {kzfw
vbroadcasti32x8:66)0F385B m|y Vz > {kzf
vbroadcasti64x4:66)0F385B m|y Vz > {kzfw

vcompresspd:66)0F388A Vxyz v > {kzwfT3
vcompressps:66)0F388A Vxyz v > {kzfT2

vcvtne2ps2bf16:F2)0F3872 v >V Vxyz {kzbf
vcvtneps2bf16:F3)0F3872 v#xy~z V/ > {kzbf

vcvtpd2qq:66)0F7B v Vxyz > {kzBwrf
vcvtpd2udq:0F79 v#xy~z V/ > {kzBwrf
vcvtpd2uqq:66)0F79 v Vxyz > {kzBwrf
vcvtph2ps:66)0F3813 v/ Vxyz > {kzs
vcvtps2ph:66)0F3A1D ib Vxyz v/ > {kzs
vcvtps2udq:0F79 v Vxyz > {kzbrf
vcvtps2qq:66)0F7B v/ Vxyz > {kzbrf
vcvtps2uqq:66)0F79 v/ Vxyz > {kzbrf
vcvtqq2pd:F3)0FE6 v Vxyz > {kzBrfw
vcvtqq2ps:0F5B v#xy~z V/ > {kzBrfw
vcvtsd2usi:F2)0F79 v#x Rlq > {rfT3
vcvtss2usi:F3)0F79 v#x Rlq > {rfT2
vcvttpd2qq:66)0F7A v Vxyz > {kzBwsf
vcvttpd2udq:0F78 v#xy~z V/ > {kzBwsf
vcvttpd2uqq:66)0F78 v Vxyz > {kzBwsf
vcvttps2udq:0F78 v Vxyz > {kzbsf
vcvttps2qq:66)0F7A v/ Vxyz > {kzbsf
vcvttps2uqq:66)0F78 v/ Vxyz > {kzbsf
vcvttsd2usi:F2)0F78 v#x Rlq > {sfT3
vcvttss2usi:F3)0F78 v#x Rlq > {sfT2
vcvtudq2pd:F3)0F7A v/ Vxyz > {kzbf
vcvtudq2ps:F2)0F7A v Vxyz > {kzbrf
vcvtuqq2pd:F3)0F7A v Vxyz > {kzBrfw
vcvtuqq2ps:F2)0F7A v#xy~z V/ > {kzBfrw
vcvtusi2sd:F2)0F7B rlq >Vx V {rfTR
vcvtusi2ss:F3)0F7B rlq >Vx V {rfTR

vdbpsadbw:66)0F3A42 ib v >Vxyz V {kzf
vdpbf16ps:F3)0F3852 v >Vxyz V {kzf

vexpandpd:66)0F3888 v Vxyz > {kzwfT3
vexpandps:66)0F3888 v Vxyz > {kzfT2

verr:! 0F00.4 rW
verw:! 0F00.5 rW

vextractf128:66)0F3A19 ib Vy vX >
vextractf32x4:66)0F3A19 ib Vyz vx|x > {kzf
vextractf64x2:66)0F3A19 ib Vyz vx|x > {kzfw
vextractf32x8:66)0F3A1B ib Vz vy|y > {kzf
vextractf64x4:66)0F3A1B ib Vz vy|y > {kzfw

vextracti128:66)0F3A39 ib Vy vX >
vextracti32x4:66)0F3A39 ib Vyz vx|x > {kzf
vextracti64x2:66)0F3A39 ib Vyz vx|x > {kzfw
vextracti32x8:66)0F3A3B ib Vz vy|y > {kzf
vextracti64x4:66)0F3A3B ib Vz vy|y > {kzfw

vfixupimmpd:66)0F3A54 ib v >Vxyz V {kzBsfw
vfixupimmps:66)0F3A54 ib v >Vxyz V {kzbsf
vfixupimmsd:66)0F3A55 ib v|Q >Vx V {kzsfw
vfixupimmss:66)0F3A55 ib v|l >Vx V {kzsf

vfpclasspd:66)0F3A66 iB vxyz K > {kBfw
vfpclassps:66)0F3A66 iB vxyz K > {kbf
vfpclasssd:66)0F3A67 ib v#x K > {kfwT3
vfpclassss:66)0F3A67 ib v#x K > {kfT2

vgatherdpd
vw 66)0F3892 >Vxy *Gx V
66)0F3892 G/ Vxyz > {Kfw

vgatherdps
66)0F3892 >Vxy G V
66)0F3892 Gxyz V > {Kf

vgatherqpd
vw 66)0F3893 >Vxy G V
66)0F3893 Gxyz V > {Kfw

vgatherqps
66)0F3893 >Vx Gxy Vx
66)0F3893 Gxyz V/ > {Kf

vgetexppd:66)0F3842 v Vxyz > {kzBsfw
vgetexpps:66)0F3842 v Vxyz > {kzbsf
vgetexpsd:66)0F3843 vx|q >Vx V > {kzsfw
vgetexpss:66)0F3843 vx|l >Vx V > {kzsf

vgetmantpd:66)0F3A26 ib v Vxyz > {kzBsfw
vgetmantps:66)0F3A26 ib v Vxyz > {kzbsf
vgetmantsd:66)0F3A27 ib vx|q >Vx V {kzsfw
vgetmantss:66)0F3A27 ib vx|l >Vx V {kzsf

vinsertf128:66)0F3A18 ib vX >Vy V
vinsertf32x4:66)0F3A18 ib vx|x >Vyz V {kzf
vinsertf64x2:66)0F3A18 ib vx|x >Vyz V {kzfw
vinsertf32x8:66)0F3A1A ib vy|y >Vz V {kzf
vinsertf64x4:66)0F3A1A ib vy|y >Vz V {kzfw

vinserti128:66)0F3A38 ib vX >Vy V
vinserti32x4:66)0F3A38 ib vx|x >Vyz V {kzf
vinserti64x2:66)0F3A38 ib vx|x >Vyz V {kzfw
vinserti32x8:66)0F3A3A ib vy|y >Vz V {kzf
vinserti64x4:66)0F3A3A ib vy|y >Vz V {kzfw

vmaskmovpd
66)0F382D m >Vxy V
66)0F382F Vxy >V m

vmaskmovps
66)0F382C m >Vxy V
66)0F382E Vxy >V m

vp2intersectd:F2)0F3868 v >Vxyz K {bf
vp2intersectq:F2)0F3868 v >Vxyz K {Bfw

vpblendd:66)0F3A02 ib v >Vxy V

vpblendmb:66)0F3866 v >Vxyz V {kzf
vpblendmd:66)0F3864 v >Vxyz V {kzbf
vpblendmq:66)0F3864 v >Vxyz V {kzBfw
vpblendmw:66)0F3866 v >Vxyz V {kzfw

vpbroadcastb
66)0F3878 vx|b Vxyz > {kz
66)0F387A ^R! Vxyz > {kzf

vpbroadcastd
66)0F3858 vx|l Vxyz > {kz
66)0F387C ^Rl Vxyz > {kzf

vpbroadcastq
66)0F3859 vx|Q Vxyz > {kzw
66)0F387C ^Rq Vxyz > {kzf

vpbroadcastw
66)0F3879 vx|w Vxyz > {kz
66)0F387B ^R! Vxyz > {kzf

vpbroadcastmb2q:F3)0F382A ^K Vxyz > {wf
vpbroadcastmw2d:F3)0F383A ^K Vxyz > {f

vpcmpb:66)0F3A3F ib v >Vxyz K {kf
vpcmpd:66)0F3A1F ib v >Vxyz K {kbf
vpcmpq:66)0F3A1F ib v >Vxyz K {kBfw
vpcmpw:66)0F3A3F ib v >Vxyz K {kfw

vpcmpub:66)0F3A3E ib v >Vxyz K {kf
vpcmpud:66)0F3A1E ib v >Vxyz K {kbf
vpcmpuq:66)0F3A1E ib v >Vxyz K {kBfw
vpcmpuw:66)0F3A3E ib v >Vxyz K {kfw

vpcompressb
66)0F3863 Vxyz ^V > {kzf
66)0F3863 Vxyz m|b > {kf

vpcompressw
66)0F3863 Vxyz ^V > {kzfw
66)0F3863 Vxyz m|w > {kfw

vpcompressd:66)0F388B Vxyz v|l > {kzf
vpcompressq:66)0F388B Vxyz v|Q > {kzfw

vpconflictd:66)0F38C4 v Vxyz > {kzbf
vpconflictq:66)0F38C4 v Vxyz > {kzBfw

vpdpbusd:66)0F3850 v >Vxyz V {kzbf
vpdpbusds:66)0F3851 v >Vxyz V {kzbf
vpdpwssd:66)0F3852 v >Vxyz V {kzbf
vpdpwssds:66)0F3853 v >Vxyz V {kzbf

vperm2f128:66)0F3A06 ib v >Vy V
vperm2i128:66)0F3A46 ib v >Vy V

vpermb:66)0F388D v >Vxyz V {kzf
vpermd:66)0F3836 v >Vyz V {kzb
vpermw:66)0F388D v >Vxyz V {kzwf

vpermq
vw 66)0F3A00 ib v Vyz > {kzB
66)0F3836 v >Vyz V {kzBfw

vpermi2b:66)0F3875 v >Vxyz V {kzf
vpermi2d:66)0F3876 v >Vxyz V {kzbf
vpermi2q:66)0F3876 v >Vxyz V {kzBfw
vpermi2w:66)0F3875 v >Vxyz V {kzfw

vpermi2pd:66)0F3877 v >Vxyz V {kzBfw
vpermi2ps:66)0F3877 v >Vxyz V {kzbf

vpermilpd
66)0F380D v >Vxyz V {kzBw
66)0F3A05 ib v Vxyz > {kzBw

vpermilps
66)0F380C v >Vxyz V {kzb
66)0F3A04 ib v Vxyz > {kzb

vpermpd
vw 66)0F3A01 ib v Vyz > {kzB
66)0F3816 v >Vyz V {kzBwf

vpermps:66)0F3816 v >Vyz V {kzb

vpermt2b:66)0F387D v >Vxyz V {kzf
vpermt2d:66)0F387E v >Vxyz V {kzbf
vpermt2q:66)0F387E v >Vxyz V {kzBfw
vpermt2w:66)0F387D v >Vxyz V {kzfw

vpermt2pd:66)0F387F v >Vxyz V {kzBfw
vpermt2ps:66)0F387F v >Vxyz V {kzbf

vpexpandb:66)0F3862 v|b Vxyz > {kzf
vpexpandd:66)0F3889 v|l Vxyz > {kzf
vpexpandq:66)0F3889 v|Q Vxyz > {kzfw
vpexpandw:66)0F3862 v|w Vxyz > {kzfw

vpgatherdd
66)0F3890 >Vxy G V
66)0F3890 Gxyz V > {Kf

vpgatherdq
vw 66)0F3890 >Vxy *Gx V
66)0F3890 G/ Vxyz > {Kfw

vpgatherqd
66)0F3891 >Vx *Gxy V
66)0F3891 Gxyz V/ > {Kf

vpgatherqq
vw 66)0F3891 >Vxy G V
66)0F3891 Gxyz V > {Kfw

vplzcntd:66)0F3844 v Vxyz > {kzbf
vplzcntq:66)0F3844 v Vxyz > {kzBwf

vpmadd52huq:66)0F38B5 v >Vxyz V {kzBwf
vpmadd52luq:66)0F38B4 v >Vxyz V {kzBwf

vpmaskmovd
66)0F388C m >Vxy V
66)0F388E Vxy >V m

vpmaskmovq
vw 66)0F388C m >Vxy V
vw 66)0F388E Vxy >V m

vpmovb2m:F3)0F3829 ^Vxyz K > {f
vpmovd2m:F3)0F3839 ^Vxyz K > {f
vpmovq2m:F3)0F3839 ^Vxyz K > {fw
vpmovw2m:F3)0F3829 ^Vxyz K > {fw

vpmovdb:F3)0F3831 Vxyz v/4 > {kzf
vpmovdw:F3)0F3833 Vxyz v/ > {kzf
vpmovqb:F3)0F3832 Vxyz v/8 > {kzf
vpmovqd:F3)0F3835 Vxyz v/ > {kzf
vpmovqw:F3)0F3834 Vxyz v/4 > {kzf
vpmovwb:F3)0F3830 Vxyz v/ > {kzf

vpmovsdb:F3)0F3821 Vxyz v/4 > {kzf
vpmovsdw:F3)0F3823 Vxyz v/ > {kzf
vpmovsqb:F3)0F3822 Vxyz v/8 > {kzf
vpmovsqd:F3)0F3825 Vxyz v/ > {kzf
vpmovsqw:F3)0F3824 Vxyz v/4 > {kzf
vpmovswb:F3)0F3820 Vxyz v/ > {kzf

vpmovusdb:F3)0F3811 Vxyz v/4 > {kzf
vpmovusdw:F3)0F3813 Vxyz v/ > {kzf
vpmovusqb:F3)0F3812 Vxyz v/8 > {kzf
vpmovusqd:F3)0F3815 Vxyz v/ > {kzf
vpmovusqw:F3)0F3814 Vxyz v/4 > {kzf
vpmovuswb:F3)0F3810 Vxyz v/ > {kzf

vpmovm2b:F3)0F3828 ^K Vxyz > {f
vpmovm2d:F3)0F3838 ^K Vxyz > {f
vpmovm2q:F3)0F3838 ^K Vxyz > {fw
vpmovm2w:F3)0F3828 ^K Vxyz > {fw

vpmultishiftqb:66)0F3883 v >Vxyz V {kzBfw

vpopcntb:66)0F3854 v Vxyz > {kzf
vpopcntd:66)0F3855 v Vxyz > {kzbf
vpopcntw:66)0F3854 v Vxyz > {kzfw
vpopcntq:66)0F3855 v Vxyz > {kzBfw

vprold:66)0F72.1 ib v >Vxyz {kzbf
vprolq:66)0F72.1 ib v >Vxyz {kzBfw

vprolvd:66)0F3815 v >Vxyz V {kzbf
vprolvq:66)0F3815 v >Vxyz V {kzBfw

vprord:66)0F72.0 ib v >Vxyz {kzbf
vprorq:66)0F72.0 ib v >Vxyz {kzBfw

vprorvd:66)0F3814 v >Vxyz V {kzbf
vprorvq:66)0F3814 v >Vxyz V {kzBfw

vpscatterdd:66)0F38A0 Vxyz G > {Kf
vpscatterdq:66)0F38A0 Vxyz G/ > {Kfw
vpscatterqd:66)0F38A1 V/ Gxyz > {Kf
vpscatterqq:66)0F38A1 Vxyz G > {Kfw

vpshldd:66)0F3A71 ib v >Vxyz V {kzbf
vpshldq:66)0F3A71 ib v >Vxyz V {kzBfw
vpshldw:66)0F3A70 ib v >Vxyz V {kzfw

vpshldvd:66)0F3871 v >Vxyz V {kzbf
vpshldvq:66)0F3871 v >Vxyz V {kzBfw
vpshldvw:66)0F3870 v >Vxyz V {kzfw

vpshrdd:66)0F3A73 ib v >Vxyz V {kzbf
vpshrdq:66)0F3A73 ib v >Vxyz V {kzBfw
vpshrdw:66)0F3A72 ib v >Vxyz V {kzfw

vpshrdvd:66)0F3873 v >Vxyz V {kzbf
vpshrdvq:66)0F3873 v >Vxyz V {kzBfw
vpshrdvw:66)0F3872 v >Vxyz V {kzfw

vpshufbitqmb:66)0F388F v >Vxyz K {kf

vpsllvd:66)0F3847 v >Vxyz V {kzb
vpsllvq:vw 66)0F3847 v >Vxyz V {kzB
vpsllvw:66)0F3812 v >Vxyz V {kzfw

vpsravd:66)0F3846 v >Vxyz V {kzb
vpsravq:66)0F3846 v >Vxyz V {kzBfw
vpsravw:66)0F3811 v >Vxyz V {kzfw

vpsrlvd:66)0F3845 v >Vxyz V {kzb
vpsrlvq:vw 66)0F3845 v >Vxyz V {kzB
vpsrlvw:66)0F3810 v >Vxyz V {kzfw

vpternlogd:66)0F3A25 ib v >Vxyz V {kzbf
vpternlogq:66)0F3A25 ib v >Vxyz V {kzBfw

vptestmb:66)0F3826 v >Vxyz K {kf
vptestmd:66)0F3827 v >Vxyz K {kbf
vptestmq:66)0F3827 v >Vxyz K {kBfw
vptestmw:66)0F3826 v >Vxyz K {kfw

vptestnmb:F3)0F3826 v >Vxyz K {kf
vptestnmd:F3)0F3827 v >Vxyz K {kbf
vptestnmq:F3)0F3827 v >Vxyz K {kBfw
vptestnmw:F3)0F3826 v >Vxyz K {kfw

vrangepd:66)0F3A50 ib v >Vxyz V {kzBsfw
vrangeps:66)0F3A50 ib v >Vxyz V {kzbsf
vrangesd:66)0F3A51 ib v|Q >Vx V {kzsfw
vrangess:66)0F3A51 ib v|l >Vx V {kzsf

vrcp14pd:66)0F384C v Vxyz > {kzBfw
vrcp14ps:66)0F384C v Vxyz > {kzbf
vrcp14sd:66)0F384D v|Q >Vx V {kzfw
vrcp14ss:66)0F384D v|l >Vx V {kzf

vreducepd:66)0F3A56 ib v Vxyz > {kzBsfw
vreduceps:66)0F3A56 ib v Vxyz > {kzbsf
vreducesd:66)0F3A57 ib v|Q >Vx V {kzsfw
vreducess:66)0F3A57 ib v|l >Vx V {kzsf

vrndscalepd:66)0F3A09 ib v Vxyz > {kzBsfw
vrndscaleps:66)0F3A08 ib v Vxyz > {kzbsf
vrndscalesd:66)0F3A0B ib v|Q >Vx V {kzsfw
vrndscaless:66)0F3A0A ib v|l >Vx V {kzsf

vrsqrt14pd:66)0F384E v Vxyz > {kzBfw
vrsqrt14ps:66)0F384E v Vxyz > {kzbf
vrsqrt14sd:66)0F384F v|Q >Vx V {kzfw
vrsqrt14ss:66)0F384F v|l >Vx V {kzf

vscalefpd:66)0F382C v >Vxyz V {kzBrfw
vscalefps:66)0F382C v >Vxyz V {kzbrf
vscalefsd:66)0F382D v|Q >Vx V {kzrfw
vscalefss:66)0F382D v|l >Vx V {kzrf

vscatterdpd:66)0F38A2 Vxyz G/ > {Kfw
vscatterdps:66)0F38A2 Vxyz G > {Kf
vscatterqpd:66)0F38A3 Vxyz G > {Kfw
vscatterqps:66)0F38A3 V/ Gxyz > {Kf

vshuff32x4:66)0F3A23 ib v >Vyz V {kzbf
vshuff64x2:66)0F3A23 ib v >Vyz V {kzBfw

vshufi32x4:66)0F3A43 ib v >Vyz V {kzbf
vshufi64x2:66)0F3A43 ib v >Vyz V {kzBfw

vtestpd:66)0F380F v Vxy >
vtestps:66)0F380E v Vxy >

vzeroall:vl 0F77 >
vzeroupper:0F77 >

wait:9B
wbinvd:0F09
wbnoinvd:F3)0F09
wrfsbase:F3)0FAE.2 Rlq
wrgsbase:F3)0FAE.3 Rlq
wrmsr:0F30
wrpkru:0F01EF
wrssd:0F38F6 Rl m
wrssq:0F38F6 Rq m
wrussd:66)0F38F5 Rl m
wrussq:66)0F38F5 Rq m
xabort:C6F8 ib
xadd:0FC0 Rbwlq r
xbegin:C7F8 jl

xchg
90 R_0Q R_0
87C0 R_0l R_0
90.o R_0wlq R
90.o Rwlq R_0
86 Rbwlq r
86 r Rbwlq

xend:0F01D5
xgetbv:0F01D0
xlat:D7

xorpd:66)0F57 v >V Vxyz {kzBw
xorps:0F57 v >V Vxyz {kzb

xrstor:0FAE.5 m
xrstor64:X 0FAE.5 m#q
xrstors:0FC7.3 m
xrstors64:X 0FC7.3 m#q
xsave:0FAE.4 m
xsave64:X 0FAE.4 m#q
xsavec:0FC7.4 m
xsavec64:X 0FC7.4 m#q
xsaveopt:0FAE.6 m
xsaveopt64:0FAE.6 m#q
xsaves:0FC7.5 m
xsaves64:X 0FC7.5 m#q
xsetbv:0F01D1
xtest:0F01D6
`;

// ../node_modules/@defasm/core/mnemonics.js
var lines;
var relativeMnemonics = [];
var mnemonics = {};
var intelDifferences = {};
var intelInvalids = [];
var attInvalids = [];
mnemonicList_default.match(/.*:.*(?=\n)|.[^]*?(?=\n\n)/g).forEach((x) => {
  lines = x.split(/[\n:]/);
  let name = lines.shift();
  if (name.includes("{")) {
    let suffixes2;
    [name, suffixes2] = name.split("{");
    let opcode = parseInt(lines[0].match(/[0-9a-f]+/i)[0], 16);
    let higherOpcode = (opcode + (suffixes2.includes("b") ? 1 : 0)).toString(16);
    for (let suffix of suffixes2) {
      let fullName = name + suffix.toLowerCase();
      if (suffix <= "Z") {
        mnemonics[name] = lines;
        mnemonics[fullName] = ["#" + name];
      } else {
        switch (suffix.toLowerCase()) {
          case "b":
            mnemonics[fullName] = lines;
            break;
          case "w":
            mnemonics[fullName] = ["66)" + higherOpcode];
            break;
          case "l":
            mnemonics[fullName] = [higherOpcode];
            intelDifferences[name + "d"] = [higherOpcode];
            intelInvalids.push(fullName);
            break;
          case "q":
            mnemonics[fullName] = ["X 48)" + higherOpcode];
            break;
        }
      }
    }
  } else {
    if (name.includes("/")) {
      let intelName;
      [name, intelName] = name.split("/");
      if (name) {
        if (intelName)
          intelDifferences[intelName] = lines;
        intelInvalids.push(name);
      } else {
        name = intelName;
        if (intelInvalids.includes(name)) {
          intelInvalids.splice(intelInvalids.indexOf(name), 1);
          intelDifferences[name] = lines;
          return;
        }
        attInvalids.push(name);
      }
    }
    mnemonics[name] = lines;
    if (lines[0].includes("j"))
      relativeMnemonics.push(name);
  }
});
var hex = (num) => num.toString(16);
var arithmeticMnemonics = "add or adc sbb and sub xor cmp".split(" ");
arithmeticMnemonics.forEach((name, i) => {
  let opBase = i * 8;
  mnemonics[name] = [
    hex(opBase + 4) + " i R_0bw",
    "83." + i + " Ib rwlq",
    hex(opBase + 5) + " iL R_0l",
    "80." + i + " i rbwl",
    hex(opBase + 5) + " iL R_0q",
    "81." + i + " IL rq",
    hex(opBase) + " Rbwlq r",
    hex(opBase + 2) + " r Rbwlq"
  ];
});
var shiftMnemonics = `rol ror rcl rcr shl shr  sar`.split(" ");
shiftMnemonics.forEach((name, i) => {
  if (name)
    mnemonics[name] = [
      "D0." + i + " rbwlq",
      "D0." + i + " i_1B rbwlq",
      "D2." + i + " R_1b rbwlq",
      "C0." + i + " iB rbwlq"
    ];
});
var conditionals = `o
no
b c nae
ae nb nc
e z
ne nz
be na
a nbe
s
ns
p pe
np po
l nge
ge nl
le ng
g nle`.split("\n");
conditionals.forEach((names, i) => {
  names = names.split(" ");
  let firstName = names.shift();
  mnemonics["j" + firstName] = [hex(112 + i) + "+3856 jbl"];
  mnemonics["j" + firstName + "w"] = ["x " + hex(112 + i + 3856) + " jw"];
  relativeMnemonics.push("j" + firstName);
  relativeMnemonics.push("j" + firstName + "w");
  mnemonics["cmov" + firstName] = [hex(3904 + i) + " r Rwlq"];
  mnemonics["set" + firstName] = [hex(3984 + i) + ".0 rB"];
  names.forEach((name) => {
    mnemonics["j" + name] = ["#j" + firstName];
    mnemonics["j" + name + "w"] = ["#j" + firstName + "w"];
    relativeMnemonics.push("j" + name);
    relativeMnemonics.push("j" + name + "w");
    mnemonics["cmov" + name] = ["#cmov" + firstName];
    mnemonics["set" + name] = ["#set" + firstName];
  });
});
var fpuArithMnemonics = "add mul com comp sub subr div divr";
fpuArithMnemonics.split(" ").forEach((name, i) => {
  let list = ["D8." + i + " ml", "DC." + i + " m$q"];
  mnemonics["fi" + name] = ["DA." + i + " ml", "DE." + i + " m$w"];
  if (i == 2 || i == 3) list.push("D8." + i + " F", hex(55489 + i * 8));
  else {
    list.push("D8." + i + " F F_0");
    list.push("DC." + i + " F_0 F");
    mnemonics["f" + name + "p"] = ["DE." + i + " F_0 F", hex(57025 + i * 8)];
  }
  mnemonics["f" + name] = list;
});
var vfmOps = ["add", "sub"];
var vfmDirs = ["132", "213", "231"];
var vfmTypes = ["pd", "ps", "sd", "ss"];
var vfmPrefs = ["vfm", "vfnm"];
vfmDirs.forEach((dir, dirI) => vfmOps.forEach((op, opI) => vfmTypes.forEach((type, typeI) => {
  vfmPrefs.forEach((pref, prefI) => mnemonics[pref + op + dir + type] = [
    (typeI % 2 ? "" : "vw ") + "66)" + hex(997528 + 16 * dirI + 4 * prefI + 2 * opI + (typeI >> 1)) + " v >Vx" + (typeI < 2 ? "yz" : "") + " V {kzr" + ["B", "b", "", ""][typeI]
  ]);
  if (typeI < 2) {
    mnemonics["vfm" + op + vfmOps[1 - opI] + dir + type] = [
      (typeI % 2 ? "" : "vw ") + "66)" + hex(997526 + 16 * dirI + opI) + " v >Vxyz V {kzr" + "Bb"[typeI]
    ];
  }
})));
function isMnemonic(mnemonic, intel) {
  if (mnemonics.hasOwnProperty(mnemonic))
    return !(intel ? intelInvalids : attInvalids).includes(mnemonic);
  return intel && intelDifferences.hasOwnProperty(mnemonic);
}
var MnemonicInterpretation = class {
  /** 
   * Constructor
   * @param {string} raw
   * @param {Operation[]} operations
   * @param {Number | null | undefined} size
   * @param {boolean} isVex
   */
  constructor(raw, operations, size, isVex) {
    this.raw = raw;
    this.operations = operations;
    this.relative = relativeMnemonics.includes(raw);
    this.size = size;
    this.vex = isVex && !operations[0].actuallyNotVex || operations[0].forceVex;
  }
};
function addMnemonicInterpretation(list, raw, intel, size, isVex, bitness) {
  if (!isMnemonic(raw, intel))
    return;
  const operations = getOperations(raw, intel).filter(
    (x) => (isVex ? (x.allowVex || x.actuallyNotVex) && !x.forceVex : !x.vexOnly) && (x.requireBitness === null || x.requireBitness === bitness)
  );
  if (operations.length == 0)
    return;
  list.push(new MnemonicInterpretation(raw, operations, size, isVex));
}
function fetchMnemonic(mnemonic, intel, expectSuffix = !intel, bitness = currBitness) {
  mnemonic = mnemonic.toLowerCase();
  if (mnemonic.startsWith("vv"))
    return [];
  let isVex = mnemonic[0] == "v";
  let possibleOpcodes = isVex ? [mnemonic, mnemonic.slice(1)] : [mnemonic];
  let interps = [];
  for (const raw of possibleOpcodes) {
    addMnemonicInterpretation(interps, raw, intel, void 0, isVex, bitness);
    if (expectSuffix) {
      const suffixArray = raw[0] == "f" ? raw[1] == "i" ? floatIntSuffixes : floatSuffixes : suffixes;
      const suffixLetter = raw[raw.length - 1];
      let size = suffixArray[suffixLetter];
      if (bitness == 32 && suffixLetter == "q")
        size = null;
      addMnemonicInterpretation(
        interps,
        raw.slice(0, -1),
        intel,
        size,
        isVex,
        bitness
      );
    }
  }
  return interps;
}
function getOperations(opcode, intel) {
  if (intel) {
    if (intelDifferences.hasOwnProperty(opcode)) {
      if (mnemonics.hasOwnProperty(opcode))
        return [...extractMnemonic(intelDifferences, opcode), ...getOperations(opcode, false)];
      return extractMnemonic(intelDifferences, opcode);
    } else if (intelInvalids.includes(opcode))
      return [];
  } else if (attInvalids.includes(opcode))
    return [];
  if (!mnemonics.hasOwnProperty(opcode))
    return [];
  return extractMnemonic(mnemonics, opcode);
}
function extractMnemonic(database, opcode) {
  let operations = database[opcode];
  if (typeof operations[0] == "string") {
    if (operations[0][0] == "#")
      return database[opcode] = extractMnemonic(database, operations[0].slice(1));
    return database[opcode] = operations.map((line2) => new Operation(line2.split(" ")));
  }
  return operations;
}

// ../node_modules/@defasm/core/instructions.js
var MAX_INSTR_SIZE = 15;
var prefixes = Object.freeze({
  lock: "LOCK",
  repne: "REPNE",
  repnz: "REPNE",
  rep: "REPE",
  repe: "REPE",
  repz: "REPE",
  data16: "DATASIZE",
  addr32: "ADDRSIZE",
  evex: "EVEX"
});
var SHORT_DISP = 48;
function parseRoundingMode(vexInfo) {
  let roundingName = "", roundStart = currRange;
  vexInfo.evex = true;
  while (next() != "}") {
    if (token == "\n")
      throw new ASMError("Expected '}'");
    roundingName += token;
  }
  vexInfo.round = ["sae", "rn-sae", "rd-sae", "ru-sae", "rz-sae"].indexOf(roundingName);
  vexInfo.roundingPos = new Range(roundStart.start, currRange.end - roundStart.start);
  if (vexInfo.round < 0)
    throw new ASMError("Invalid rounding mode", vexInfo.roundingPos);
}
function explainNoMatch(interps, operands, vexInfo) {
  let minOperandCount = Infinity, maxOperandCount = 0;
  let firstOrderPossible = false, secondOrderPossible = false;
  let requiresMask = false;
  for (const interp of interps)
    for (const operation of interp.operations) {
      let opCount = (interp.vex ? operation.vexOpCatchers : operation.opCatchers).length;
      if (opCount > maxOperandCount)
        maxOperandCount = opCount;
      if (opCount < minOperandCount)
        minOperandCount = opCount;
      if (opCount == operands.length && operation.requireMask)
        requiresMask = true;
      vexInfo.needed = interp.vex;
      firstOrderPossible = firstOrderPossible || operation.matchTypes(operands, vexInfo);
      operands.reverse();
      secondOrderPossible = secondOrderPossible || operation.matchTypes(operands, vexInfo);
      operands.reverse();
    }
  if (operands.length < minOperandCount)
    return "Not enough operands";
  if (operands.length > maxOperandCount)
    return "Too many operands";
  if (!firstOrderPossible && secondOrderPossible)
    return "Wrong operand order";
  if (vexInfo.mask == 0 && requiresMask)
    return "Must use a mask for this instruction";
  return "Wrong operand type" + (operands.length == 1 ? "" : "s");
}
var Instruction = class extends Statement {
  constructor({ name, ...config }) {
    super({ ...config, maxSize: MAX_INSTR_SIZE });
    this.opcode = name.toLowerCase();
    this.opcodeRange = new RelativeRange(config.range, config.range.start, config.range.length);
    this.interpret();
  }
  // Generate Instruction.outline
  interpret() {
    let opcode = this.opcode, operand = null, evexPrefixRange;
    let vexInfo = {
      needed: false,
      evex: false,
      mask: 0,
      zeroing: false,
      round: null,
      broadcast: null
    };
    let memoryOperand = null;
    this.needsRecompilation = false;
    let operands = [];
    this.prefsToGen = new PrefixEnum();
    while (prefixes.hasOwnProperty(opcode)) {
      this.prefsToGen[prefixes[opcode]] = true;
      if (opcode == "evex")
        evexPrefixRange = this.opcodeRange;
      this.opcodeRange = new RelativeRange(this.range, currRange.start, currRange.length);
      opcode = token.toLowerCase();
      if (opcode === ";" || opcode === "\n")
        throw new ASMError("Expected opcode", this.opcodeRange);
      next();
    }
    this.opcode = opcode;
    let interps = fetchMnemonic(opcode, this.syntax.intel);
    if (interps.length == 0)
      throw new ASMError("Unknown opcode", this.opcodeRange);
    interps = interps.filter((interp) => interp.size !== null);
    if (interps.length == 0)
      throw new ASMError(
        "Invalid opcode suffix",
        new RelativeRange(this.range, this.opcodeRange.end - 1, 1)
      );
    if (this.prefsToGen.EVEX) {
      interps = interps.filter((interp) => interp.operations.some((op) => op.evexPermits !== null));
      if (interps.length == 0)
        throw new ASMError("No EVEX encoding exists for this instruction", evexPrefixRange);
    }
    const expectRelative = interps.some((interp) => interp.relative);
    if (!this.syntax.intel && token == "{") {
      parseRoundingMode(vexInfo);
      if (next() != ",")
        throw new ASMError("Expected ','");
      next();
    }
    while (token != ";" && token != "\n") {
      let sizePtrRange = currRange, enforcedSize = null;
      if (this.syntax.intel) {
        if (token == "{") {
          parseRoundingMode(vexInfo);
          next();
          break;
        }
        let sizePtr = token;
        if (isSizeHint(sizePtr.toLowerCase())) {
          let following = next();
          if (following.toLowerCase() == "ptr") {
            sizePtrRange = sizePtrRange.until(currRange);
            enforcedSize = sizeHints[sizePtr.toLowerCase()];
            if (",;\n{:".includes(next())) {
              ungetToken();
              setToken(following);
            }
          } else {
            if (",;\n{:".includes(following)) {
              ungetToken();
              setToken(sizePtr);
            } else
              enforcedSize = sizeHints[sizePtr.toLowerCase()];
          }
        }
      }
      operand = new Operand(this, expectRelative);
      if (operand.expression && operand.expression.hasSymbols)
        this.needsRecompilation = true;
      operands.push(operand);
      if (operand.reg >= 16 || operand.reg2 >= 16 || operand.size == 512)
        vexInfo.evex = true;
      if (operand.type.isMemory) {
        memoryOperand = operand;
        if (enforcedSize)
          operand.size = enforcedSize;
      } else if (enforcedSize)
        throw new ASMError("Size hints work only for memory operands", sizePtrRange);
      while (token == "{") {
        vexInfo.evex = true;
        if (this.syntax.prefix ? next() == "%" : next()[0] == "k") {
          vexInfo.mask = parseRegister([OPT.MASK]).reg;
          if ((vexInfo.mask & 7) == 0)
            throw new ASMError(`Can't use ${this.syntax.prefix ? "%" : ""}k0 as writemask`, regParsePos);
        } else if (token == "z")
          vexInfo.zeroing = true, next();
        else if (operand.type === OPT.MEM) {
          vexInfo.broadcastOperand = operand;
          vexInfo.broadcast = ["1to2", "1to4", "1to8", "1to16"].indexOf(token) + 1;
          if (vexInfo.broadcast == 0)
            throw new ASMError("Invalid broadcast mode");
          vexInfo.broadcastPos = currRange;
          next();
        } else
          throw new ASMError("Invalid decorator");
        if (token != "}")
          throw new ASMError("Expected '}'");
        next();
      }
      if (token != ",")
        break;
      next();
    }
    this.operandStartPos = operands.length > 0 ? operands[0].startPos : this.opcodeRange;
    if (this.syntax.intel && !(operands.length == 2 && operands[0].type === OPT.IMM && operands[1].type === OPT.IMM))
      operands.reverse();
    if (memoryOperand && vexInfo.round !== null)
      throw new ASMError("Embedded rounding can only be used on reg-reg", vexInfo.roundingPos);
    if (memoryOperand && this.prefsToGen.ADDRSIZE)
      memoryOperand.dispSize = 32;
    if (this.prefsToGen.EVEX)
      vexInfo.evex = true;
    let matchingInterps = [];
    for (const interp of interps) {
      vexInfo.needed = interp.vex;
      const matchingOps = interp.operations.filter((operation) => operation.matchTypes(operands, vexInfo));
      if (matchingOps.length > 0)
        matchingInterps.push({ ...interp, operations: matchingOps });
    }
    if (matchingInterps.length == 0)
      throw new ASMError(explainNoMatch(interps, operands, vexInfo), this.operandStartPos.until(currRange));
    this.outline = { operands, memoryOperand, interps: matchingInterps, vexInfo };
    this.endPos = currRange;
    this.removed = false;
    try {
      this.compile();
    } catch (e) {
      this.error = e;
      this.clear();
    }
    if (!this.needsRecompilation && !this.ipRelative)
      this.outline = void 0;
  }
  compile() {
    if (this.outline === void 0)
      throw ASMError("Critical error: unneeded recompilation");
    let { operands, memoryOperand, interps, vexInfo } = this.outline;
    let prefsToGen = this.prefsToGen;
    this.clear();
    if (memoryOperand)
      memoryOperand.evaluate(this, this.syntax.intel);
    for (let i = 0; i < operands.length; i++) {
      const op2 = operands[i];
      prefsToGen.add(op2.prefs);
      if (op2.type === OPT.IMM) {
        if (op2.expression.hasSymbols)
          op2.evaluate(this);
        if (op2.value.isRelocatable()) {
          const firstInterp = interps[0];
          if (firstInterp.operations.length == 1) {
            let operation = firstInterp.operations[0];
            op2.size = Math.max(
              ...(firstInterp.vex ? operation.vexOpCatchers : operation.opCatchers)[i].sizes
            ) & ~7;
          } else {
            op2.size = 8;
            relocationSizeLoop:
              for (const interp of interps) for (const operation of interp.operations) {
                const sizes = (interp.vex ? operation.vexOpCatchers : operation.opCatchers)[i].sizes;
                if (sizes.length != 1 || (sizes[0] & ~7) != 8) {
                  op2.size = operands.length > 2 ? operands[2].size : operands[1].size;
                  if (op2.size > 32)
                    op2.size = 32;
                  break relocationSizeLoop;
                }
              }
          }
          op2.unsignedSize = op2.size;
        } else if (!op2.expression.hasSymbols) {
          op2.size = op2.value.inferSize(true);
          op2.unsignedSize = op2.value.inferSize(false);
        } else {
          let max = op2.value.inferSize();
          for (let size = 8; size <= max; size *= 2) {
            if ((size != op2.size || op2.size == max) && op2.sizeAllowed(size)) {
              op2.size = size;
              op2.recordSizeUse(size);
              if (size < max)
                queueRecomp(this);
              break;
            }
          }
          max = op2.value.inferSize(false);
          for (let size = 8; size <= max; size *= 2) {
            if ((size != op2.unsignedSize || op2.unsignedSize == max) && op2.sizeAllowed(size, true)) {
              op2.unsignedSize = size;
              op2.recordSizeUse(size, true);
              if (size < max)
                queueRecomp(this);
              break;
            }
          }
        }
      }
    }
    let op, found = false, rexVal = 64, memOpSize = memoryOperand?.size;
    interpLoop:
      for (const interp of interps) {
        if (memoryOperand)
          memoryOperand.size = interp.size || memOpSize;
        vexInfo.needed = interp.vex;
        for (const operation of interp.operations) {
          op = operation.fit(operands, this, vexInfo);
          if (op !== null) {
            found = true;
            break interpLoop;
          }
        }
      }
    if (!found) {
      if (memoryOperand && isNaN(memoryOperand.size))
        throw new ASMError("Ambiguous memory size", memoryOperand.startPos.until(memoryOperand.endPos));
      throw new ASMError(
        "Wrong operand size" + (operands.length == 1 ? "" : "s"),
        this.operandStartPos.until(this.endPos)
      );
    }
    if (op.rexw)
      rexVal |= 8, prefsToGen.REX = true;
    let modRM = null, sib = null;
    if (op.extendOp)
      rexVal |= 1, prefsToGen.REX = true;
    else if (op.rm !== null) {
      let extraRex;
      [extraRex, modRM, sib] = this.makeModRM(op.rm, op.reg);
      if (extraRex !== 0)
        rexVal |= extraRex, prefsToGen.REX = true;
    }
    if (prefsToGen.REX && prefsToGen.NOREX)
      throw new ASMError("Can't encode high 8-bit register", operands[0].startPos.until(currRange));
    let opcode = op.opcode;
    if (op.size == 16)
      prefsToGen.DATASIZE = true;
    if (prefsToGen.SEG0) this.genByte(38);
    if (prefsToGen.SEG1) this.genByte(46);
    if (prefsToGen.SEG2) this.genByte(54);
    if (prefsToGen.SEG3) this.genByte(62);
    if (prefsToGen.SEG4) this.genByte(100);
    if (prefsToGen.SEG5) this.genByte(101);
    if (prefsToGen.ADDRSIZE) this.genByte(103);
    if (prefsToGen.DATASIZE) this.genByte(102);
    if (prefsToGen.LOCK) this.genByte(240);
    if (prefsToGen.REPNE) this.genByte(242);
    if (prefsToGen.REPE) this.genByte(243);
    if (op.prefix !== null) {
      if (op.prefix > 255)
        this.genByte(op.prefix >> 8);
      this.genByte(op.prefix);
    }
    if (op.vex !== null)
      makeVexPrefix(op.vex, rexVal, vexInfo.evex).map((x) => this.genByte(x));
    else {
      if (prefsToGen.REX)
        this.genByte(rexVal);
      if (opcode > 65535)
        this.genByte(opcode >> 16);
      if (opcode > 255)
        this.genByte(opcode >> 8);
    }
    this.genByte(opcode);
    if (modRM !== null)
      this.genByte(modRM);
    if (sib !== null)
      this.genByte(sib);
    if (op.rm?.value?.addend != null) {
      let sizeRelative = false, value = op.rm.value;
      if (op.rm.ripRelative && op.rm.value.section != pseudoSections.ABS && !op.rm.value.pcRelative) {
        for (let imm of op.imms)
          value.addend -= BigInt(imm.size >> 3);
        sizeRelative = true;
        value.apply(this, "-", new IdentifierValue({
          addend: BigInt(this.address),
          symbol: (this.section.head.statement ?? instr).symbol,
          section: this.section,
          range: value.range
        }));
        this.ipRelative = true;
      }
      this.genValue(value, {
        size: op.rm.dispSize || 32,
        signed: true,
        sizeRelative,
        dispMul: op.dispMul
      });
    }
    if (op.relImm !== null)
      this.genValue(op.relImm.value, { size: op.relImm.size, sizeRelative: true, functionAddr: true });
    else if (op.evexImm !== null)
      this.genByte(op.evexImm);
    else if (op.moffs !== null)
      this.genValue(op.moffs.value, { size: op.moffs.dispSize, signed: true });
    else for (const imm of op.imms)
      this.genValue(imm.value, { size: imm.size, signed: !op.unsigned });
  }
  /** Generate the ModRM byte
   * @param {Operand} rm
   * @param {Operand} r */
  makeModRM(rm, r) {
    let modrm = 0, rex = 0, sib = null;
    let rmReg = rm.reg, rmReg2 = rm.reg2, rReg = r.reg;
    if (rReg >= 8) {
      rex |= 4;
      rReg &= 7;
    }
    modrm |= rReg << 3;
    if (rm.ripRelative) {
      rm.value.addend = rm.value.addend || 0n;
      return [rex, modrm | 5, null];
    }
    if (!rm.type.isMemory)
      modrm |= 192;
    else if (rmReg >= 0) {
      if (rm.value.addend != null) {
        this.determineDispSize(rm, 8, 32);
        if (rm.dispSize == 8)
          modrm |= 64;
        else
          modrm |= 128;
      }
    } else {
      rmReg = 5;
      if (currBitness == 64 && rmReg2 < 0)
        rmReg2 = 4;
      rm.value.addend ||= 0n;
    }
    rex |= rmReg >> 3;
    rmReg &= 7;
    modrm |= rmReg2 < 0 ? rmReg : 4;
    if ((modrm & 192) != 192 && (modrm & 7) == 4) {
      if (rmReg2 < 0)
        rmReg2 = 4;
      else if (rmReg2 >= 8) {
        rex |= 2;
        rmReg2 &= 7;
      }
      sib = rm.shift << 6 | rmReg2 << 3 | rmReg;
    }
    return [rex, modrm, sib];
  }
  /** Determine whether to shorten a memory operand's displacement if possible,
   * and queue for recompilation as necessary
   * @param {Operand} operand The memory operand to determine
   * @param {Number} shortSize The possible short size
   * @param {Number} longSize The default long size */
  determineDispSize(operand, shortSize, longSize) {
    if (!operand.value.isRelocatable() && operand.value.inferSize() <= shortSize && (operand.dispSize == shortSize || operand.sizeAvailable(SHORT_DISP))) {
      operand.dispSize = shortSize;
      operand.recordSizeUse(SHORT_DISP);
    } else if (!operand.value.isRelocatable() && operand.expression && operand.expression.hasSymbols && operand.dispSize != shortSize && operand.sizeAvailable(SHORT_DISP)) {
      operand.dispSize = shortSize;
      operand.recordSizeUse(SHORT_DISP);
      queueRecomp(this);
    } else
      operand.dispSize = longSize;
  }
  recompile() {
    this.error = null;
    try {
      for (const op of this.outline.operands)
        if (op.expression && op.expression.hasSymbols)
          op.value = op.expression.evaluate(this);
      this.compile();
    } catch (e) {
      this.clear();
      throw e;
    }
  }
};
function makeVexPrefix(vex, rex, isEvex) {
  if (isEvex)
    vex ^= 524304;
  let vex1 = vex & 255, vex2 = vex >> 8, vex3 = vex >> 16;
  vex1 |= (~rex & 7) << 5;
  vex2 |= (rex & 8) << 4;
  if (isEvex)
    return [98, vex1, vex2, vex3];
  if ((vex1 & 127) == 97 && (vex2 & 128) == 0)
    return [197, vex2 | vex1 & 128];
  return [196, vex1, vex2];
}

// ../node_modules/@defasm/core/compiler.js
var prevNode = null;
var currSection = null;
var currBitness;
var addr = 0;
function setSection(section) {
  currSection = section;
  const prevInstr = section.cursor.prev.statement;
  return prevInstr.address + prevInstr.length;
}
function addInstruction(instr2, seekEnd = true) {
  if (instr2.section !== currSection)
    instr2.address = setSection(instr2.section);
  prevNode = prevNode.next = new StatementNode(instr2);
  currSection.cursor.prev = currSection.cursor.prev.next = instr2.sectionNode;
  setSyntax(instr2.syntax);
  if (seekEnd && token != "\n" && token != ";") {
    instr2.error = new ASMError("Expected end of line");
    while (token != "\n" && token != ";")
      next();
  }
  addr = instr2.address + instr2.length;
  instr2.range.length = (seekEnd ? currRange.start : currRange.end) - instr2.range.start;
}
var AssemblyState = class {
  /** @param {AssemblyConfig} */
  constructor({
    syntax = {
      intel: false,
      prefix: true
    },
    writableText = false,
    bitness = 64
  } = {}) {
    this.defaultSyntax = syntax;
    this.bitness = bitness;
    this.symbols = /* @__PURE__ */ new Map();
    this.fileSymbols = [];
    setSyntax(syntax);
    loadSymbols(this.symbols, this.fileSymbols);
    currBitness = bitness;
    this.sections = [
      new Section(".text"),
      new Section(".data"),
      new Section(".bss")
    ];
    if (writableText)
      this.sections[0].flags |= sectionFlags.w;
    this.head = new StatementNode();
    this.source = "";
    this.compiledRange = new Range();
    this.errors = [];
  }
  /** Compile Assembly from source code into machine code 
   * @param {string} source
  */
  compile(source, {
    haltOnError = false,
    range: replacementRange = new Range(0, this.source.length),
    doSecondPass = true
  } = {}) {
    this.source = /* If the given range is outside the current
    code's span, fill the in-between with newlines */
    this.source.slice(0, replacementRange.start).padEnd(replacementRange.start, "\n") + source + this.source.slice(replacementRange.end);
    loadSymbols(this.symbols, this.fileSymbols);
    loadSections(this.sections, replacementRange);
    currBitness = this.bitness;
    let { head, tail } = this.head.getAffectedArea(replacementRange, true, source.length);
    setSyntax(head.statement ? head.statement.syntax : this.defaultSyntax);
    addr = setSection(head.statement ? head.statement.section : this.sections[0]);
    loadCode(this.source, replacementRange.start);
    prevNode = head;
    while (match) {
      let range = startAbsRange();
      try {
        if (token != "\n" && token != ";") {
          let name = token;
          next();
          if (token == ":")
            addInstruction(new SymbolDefinition({ addr, name, range, isLabel: true }), false);
          else if (token == "=" || currSyntax.intel && token.toLowerCase() == "equ")
            addInstruction(new SymbolDefinition({ addr, name, range }));
          else {
            let isDir = false;
            if (currSyntax.intel) {
              if (name[0] == "%") {
                isDir = true;
                name += token.toLowerCase();
                next();
              } else
                isDir = isDirective(name, true);
            } else
              isDir = name[0] == ".";
            if (isDir)
              addInstruction(makeDirective({ addr, range }, currSyntax.intel ? name : name.slice(1)));
            else if (currSyntax.intel && isDirective(token, true)) {
              addInstruction(new SymbolDefinition({ addr, name, range, isLabel: true }), false);
              name = token;
              range = startAbsRange();
              next();
              addInstruction(makeDirective({ addr, range }, name));
            } else
              addInstruction(new Instruction({ addr, name, range }));
          }
        }
      } catch (error) {
        while (token != "\n" && token != ";")
          next();
        if (haltOnError && !(doSecondPass && error.range))
          throw `Error on line ${line}: ${error.message}`;
        if (!error.range)
          console.error(`Error on line ${line}:
`, error);
        else
          addInstruction(new Statement({ addr, range, error }), !comment);
      }
      if (comment)
        addInstruction(new Statement({ addr, range: startAbsRange() }), false);
      next();
      if (currRange.end > replacementRange.end)
        break;
    }
    for (const section of sections) {
      let node = section.cursor.tail;
      while (node && node.statement.range.start < currRange.start)
        node = node.next;
      section.cursor.tail = node;
    }
    while (tail && tail.statement.range.start < currRange.start) {
      tail.statement.remove();
      tail = tail.next;
    }
    if (tail && currSection !== tail.statement.section && !tail.statement.switchSection) {
      let tailSection = tail.statement.section;
      let node = tailSection.cursor.tail;
      currSection.cursor.prev.next = node;
      while (node && !node.statement.switchSection) {
        node.statement.section = currSection;
        if (node.statement.ipRelative)
          queueRecomp(node.statement);
        currSection.cursor.prev = node;
        node = node.next;
      }
      tailSection.cursor.tail = node;
    }
    for (const section of sections) {
      let prev = section.cursor.prev;
      prev.next = section.cursor.tail;
      if (prev.next)
        recompQueue.push(prev);
    }
    prevNode.next = tail;
    if (tail) {
      let instr2 = tail.statement;
      if ((currSyntax.prefix != instr2.syntax.prefix || currSyntax.intel != instr2.syntax.intel) && !instr2.switchSyntax) {
        let nextSyntaxChange = tail;
        while (nextSyntaxChange.next && !nextSyntaxChange.next.switchSyntax)
          nextSyntaxChange = nextSyntaxChange.next;
        const recompStart = prevNode.statement ? prevNode.statement.range.end : 0;
        const recompEnd = nextSyntaxChange.statement.range.end;
        const recompRange = new Range(recompStart, recompEnd - recompStart);
        this.compile(recompRange.slice(this.source), {
          haltOnError,
          range: recompRange,
          doSecondPass: false
        });
      }
    }
    this.compiledRange = replacementRange.until(prevRange);
    if (doSecondPass)
      this.secondPass(haltOnError);
  }
  // Run the second pass on the instruction list
  secondPass(haltOnError = false) {
    addr = 0;
    let node;
    loadSymbols(this.symbols);
    symbols.forEach((symbol, name) => {
      symbol.references = symbol.references.filter((instr2) => !instr2.removed);
      symbol.definitions = symbol.definitions.filter((instr2) => !instr2.removed);
      if ((symbol.statement === null || symbol.statement.error) && symbol.references.length == 0 && symbol.definitions.length == 0)
        symbols.delete(name);
    });
    recompQueue.sort((a, b) => a.statement.address - b.statement.address);
    while (node = recompQueue.shift()) {
      addr = node.statement.address;
      do {
        let instr2 = node.statement;
        if (instr2) {
          instr2.address = addr;
          if ((instr2.wantsRecomp || instr2.ipRelative) && !instr2.removed) {
            try {
              instr2.wantsRecomp = false;
              instr2.recompile();
            } catch (e) {
              instr2.error = e;
              if (instr2.symbol)
                for (const ref of instr2.symbol.references)
                  queueRecomp(ref);
            }
          }
          addr = instr2.address + instr2.length;
        }
        node = node.next;
      } while (node && node.statement.address != addr);
    }
    this.errors = [];
    const reportedErrors = [];
    this.iterate((instr2, line2) => {
      if (instr2.outline && instr2.outline.operands)
        for (let op of instr2.outline.operands)
          op.clearAttemptedSizes();
      const error = instr2.error;
      if (error) {
        this.errors.push(error);
        if (!error.range) {
          console.error(`Error on line ${line2}:
`, error);
          error.range = new RelativeRange(instr2.range, instr2.range.start, instr2.range.length);
        }
        reportedErrors.push({ line: line2, error });
      }
    });
    if (haltOnError && reportedErrors.length > 0)
      throw reportedErrors.map(({ error, line: line2 }) => {
        const linePart = `Error on line ${line2}: `;
        return linePart + (error.range.parent ?? error.range).slice(this.source) + "\n" + " ".repeat(linePart.length + (error.range.parent ? error.range.start - error.range.parent.start : 0)) + "^ " + error.message;
      }).join("\n\n");
  }
  line(line2) {
    if (line2-- < 1)
      throw "Invalid line";
    let start = 0;
    while (line2) {
      start = this.source.indexOf("\n", start) + 1;
      if (start == 0)
        return new Range(this.source.length + line2, 0);
      line2--;
    }
    let end = this.source.indexOf("\n", start);
    if (end < 0)
      end = this.source.length;
    return new Range(start, end - start);
  }
  /**
   * @callback instrCallback
   * @param {Statement} instr
   * @param {Number} line
  */
  /** @param {instrCallback} func */
  iterate(func) {
    let line2 = 1, nextLine = 0, node = this.head.next;
    while (nextLine != Infinity) {
      nextLine = this.source.indexOf("\n", nextLine) + 1 || Infinity;
      while (node && node.statement.range.end < nextLine) {
        func(node.statement, line2);
        node = node.next;
      }
      line2++;
    }
  }
  /**
   * @callback lineCallback
   * @param {Uint8Array} bytes
   * @param {Number} line
  */
  /** @param {lineCallback} func */
  bytesPerLine(func) {
    let lineQueue = [];
    let line2 = 1, nextLine = 0, node = this.head.next;
    while (nextLine != Infinity) {
      let bytes = new Uint8Array();
      let addToBytes = () => {
        if (lineQueue.length > 0) {
          const line3 = lineQueue.shift();
          if (line3.length > 0) {
            let newBytes = new Uint8Array(bytes.length + line3.length);
            newBytes.set(bytes);
            newBytes.set(line3, bytes.length);
            bytes = newBytes;
          }
        }
      };
      nextLine = this.source.indexOf("\n", nextLine) + 1 || Infinity;
      addToBytes();
      while (node && node.statement.range.start < nextLine) {
        let instr2 = node.statement, prevEnd = 0;
        for (const end of [...instr2.lineEnds, instr2.length]) {
          if (end <= instr2.length)
            lineQueue.push(instr2.bytes.subarray(prevEnd, end));
          prevEnd = end;
        }
        addToBytes();
        node = node.next;
      }
      func(bytes, line2);
      line2++;
    }
  }
};

// ../defasm/cli-browser/elf.js
var import_buffer = __toESM(require_buffer(), 1);
function header(fields) {
  const result = class {
    #fields;
    /** @param {T} config */
    constructor(config) {
      for (const name of Object.keys(fields))
        this[name] = config[name] ?? 0;
      this.#fields = fields;
    }
    dump(wordSize) {
      const data = import_buffer.Buffer.alloc(result.size(wordSize));
      for (const name of Object.keys(this.#fields)) {
        let [offset, size] = fields[name], number = this[name];
        if (Array.isArray(offset)) offset = offset[wordSize >> 6];
        if (Array.isArray(size)) size = size[wordSize >> 6];
        switch (size) {
          case 1:
            data.writeInt8(number, offset);
            break;
          case 2:
            data.writeInt16LE(number, offset);
            break;
          case 4:
            data.writeInt32LE(number, offset);
            break;
          case 8:
            data.writeBigInt64LE(BigInt(number), offset);
            break;
        }
      }
      return data;
    }
    static size(wordSize) {
      let length = 0;
      for (let [offset, size] of Object.values(fields)) {
        if (Array.isArray(offset)) offset = offset[wordSize >> 6];
        if (Array.isArray(size)) size = size[wordSize >> 6];
        length = Math.max(length, offset + size);
      }
      return length;
    }
  };
  return result;
}
var ELFHeader = header({
  /** 0x7F followed by ELF(45 4c 46) in ASCII; these four bytes constitute the magic number. */
  EI_MAG: [0, 4],
  /** This byte is set to either 1 or 2 to signify 32- or 64-bit format, respectively. */
  EI_CLASS: [4, 1],
  /** This byte is set to either 1 or 2 to signify little or big endianness, respectively. This affects interpretation of multi-byte fields starting with offset 0x10. */
  EI_DATA: [5, 1],
  /** Set to 1 for the original and current version of ELF. */
  EI_VERSION: [6, 1],
  /** Identifies the target operating system ABI. It is often set to 0 regardless of the target platform. */
  EI_OSABI: [7, 1],
  /** Further specifies the ABI version. Its interpretation depends on the target ABI. Linux kernel (after at least 2.6) has no definition of it, so it is ignored for statically-linked executables. */
  EI_ABIVERSION: [8, 1],
  /** currently unused, should be filled with zeros. */
  EI_PAD: [9, 7],
  /** Identifies object file type. */
  e_type: [16, 2],
  /** Specifies target instruction set architecture. */
  e_machine: [18, 2],
  /** Set to 1 for the original version of ELF. */
  e_version: [20, 4],
  /** This is the memory address of the entry point from where the process starts executing. */
  e_entry: [24, [4, 8]],
  /** Points to the start of the program header table. */
  e_phoff: [[28, 32], [4, 8]],
  /** Points to the start of the section header table. */
  e_shoff: [[32, 40], [4, 8]],
  /** Interpretation of this field depends on the target architecture. */
  e_flags: [[36, 48], 4],
  /** Contains the size of this header. */
  e_ehsize: [[40, 52], 2],
  /** Contains the size of a program header table entry. */
  e_phentsize: [[42, 54], 2],
  /** Contains the number of entries in the program header table. */
  e_phnum: [[44, 56], 2],
  /** Contains the size of a section header table entry. */
  e_shentsize: [[46, 58], 2],
  /** Contains the number of entries in the section header table. */
  e_shnum: [[48, 60], 2],
  /** Contains index of the section header table entry that contains the section names. */
  e_shstrndx: [[50, 62], 2]
});
var ProgramHeader = header({
  /** Identifies the type of the segment. */
  p_type: [0, 4],
  /** Segment-dependent flags. */
  p_flags: [[24, 4], 4],
  /** Offset of the segment in the file image. */
  p_offset: [[4, 8], [4, 8]],
  /** Virtual address of the segment in memory. */
  p_vaddr: [[8, 16], [4, 8]],
  /** On systems where physical address is relevant, reserved for segment's physical address. */
  p_paddr: [[12, 24], [4, 8]],
  /** Size in bytes of the segment in the file image. May be 0. */
  p_filesz: [[16, 32], [4, 8]],
  /** Size in bytes of the segment in memory. May be 0. */
  p_memsz: [[20, 40], [4, 8]],
  /** 0 and 1 specify no alignment. Otherwise should be a positive, integral power of 2, with p_vaddr equating p_offset modulus p_align. */
  p_align: [[28, 48], [4, 8]]
});
var SectionHeader = header({
  /** An offset to a string in the .shstrtab section that represents the name of this section. */
  sh_name: [0, 4],
  /** Identifies the type of this header. */
  sh_type: [4, 4],
  /** Identifies the attributes of the section. */
  sh_flags: [8, [4, 8]],
  /** Virtual address of the section in memory, for sections that are loaded. */
  sh_addr: [[12, 16], [4, 8]],
  /** Offset of the section in the file image. */
  sh_offset: [[16, 24], [4, 8]],
  /** Size in bytes of the section in the file image. May be 0. */
  sh_size: [[20, 32], [4, 8]],
  /** Contains the section index of an associated section. This field is used for several purposes, depending on the type of section. */
  sh_link: [[24, 40], 4],
  /** Contains extra information about the section. This field is used for several purposes, depending on the type of section. */
  sh_info: [[28, 44], 4],
  /** Contains the required alignment of the section. This field must be a power of two. */
  sh_addralign: [[32, 48], [4, 8]],
  /** Contains the size, in bytes, of each entry, for sections that contain fixed-size entries. Otherwise, this field contains zero. */
  sh_entsize: [[36, 56], [4, 8]]
});

// ../defasm/cli-browser/files.js
var import_buffer2 = __toESM(require_buffer(), 1);
var OutputBuffer = class {
  chunks = [];
  length = 0;
  write(buffer, position) {
    this.chunks.push({ buffer, position });
    this.length = Math.max(this.length, position + buffer.length);
  }
  toUint8Array() {
    const arr = new Uint8Array(this.length);
    for (const { buffer, position } of this.chunks) {
      arr.set(buffer, position);
    }
    return arr;
  }
};
function createExecutable(state2) {
  const outBuffer = new OutputBuffer();
  let entryPoint = 0, entrySection = state2.sections.find((section) => section.name == ".text");
  let programHeaders = [], fileOffset = Math.ceil(ELFHeader.size(state2.bitness) / 4096) * 4096, memoryOffset = 4194304;
  let sections2 = [...state2.sections];
  let commonSymbols = [];
  state2.symbols.forEach((symbol, name) => {
    if (name == "_start" && symbol.bind == 1) {
      entryPoint = Number(symbol.value.absoluteValue());
      entrySection = symbol.value.section;
    }
    if (symbol.value.section == pseudoSections.UND)
      throw `Can't assemble executable: unknown symbol ${symbol.name}`;
    if (symbol.value.section == pseudoSections.COM)
      commonSymbols.push(symbol);
  });
  for (const section of sections2) {
    const data = section.head.dump();
    outBuffer.write(data, fileOffset);
    const header2 = new ProgramHeader({
      p_type: 1,
      p_flags: (section.flags & sectionFlags.a ? 4 : 0) | (section.flags & sectionFlags.w ? 2 : 0) | (section.flags & sectionFlags.x ? 1 : 0),
      p_offset: fileOffset,
      p_vaddr: memoryOffset,
      p_paddr: memoryOffset,
      p_filesz: data.length,
      p_memsz: data.length
    });
    programHeaders.push(header2);
    if (section == entrySection)
      entryPoint += memoryOffset;
    section.programHeader = header2;
    header2.section = section;
    let length = data.length || 1;
    fileOffset = Math.ceil((fileOffset + length) / 4096) * 4096;
    memoryOffset = Math.ceil((memoryOffset + length) / 4096) * 4096;
  }
  const bss = sections2.find((section) => section.name == ".bss").programHeader;
  if (commonSymbols.length > 0) {
    let sectionSize = bss.p_memsz;
    for (const symbol of commonSymbols) {
      const alignment = Number(symbol.value.addend) || 1;
      sectionSize = Math.ceil(sectionSize / alignment) * alignment;
      symbol.address = sectionSize;
      sectionSize += Number(symbol.size);
    }
    bss.p_memsz = sectionSize;
  }
  outBuffer.write(new ELFHeader({
    EI_MAG: 1179403647,
    EI_CLASS: state2.bitness >> 5,
    EI_DATA: 1,
    EI_VERSION: 1,
    EI_OSABI: 0,
    e_type: 2,
    // ET_EXEC
    e_machine: state2.bitness === 64 ? 62 : 3,
    e_version: 1,
    e_entry: entryPoint,
    e_phoff: fileOffset,
    e_ehsize: ELFHeader.size(state2.bitness),
    e_phentsize: ProgramHeader.size(state2.bitness),
    e_phnum: programHeaders.length
  }).dump(state2.bitness), 0);
  for (const header2 of programHeaders) {
    outBuffer.write(header2.dump(state2.bitness), fileOffset);
    fileOffset += ProgramHeader.size(state2.bitness);
  }
  for (const section of state2.sections) for (const reloc of section.getRelocations()) {
    const offset = section.programHeader.p_vaddr + reloc.offset;
    const buffer = import_buffer2.Buffer.alloc(reloc.size / 8);
    let value = reloc.addend + (reloc.symbol.value.section == pseudoSections.COM ? BigInt(reloc.symbol.address + bss.p_vaddr) : reloc.symbol?.value ? reloc.symbol.value.absoluteValue() + BigInt(reloc.symbol.value.section.programHeader.p_vaddr) : 0n);
    if (reloc.pcRelative)
      value -= BigInt(offset);
    let bigInt = reloc.size == 64;
    value = value & (1n << BigInt(reloc.size)) - 1n;
    buffer[`write${bigInt ? "Big" : ""}${reloc.signed ? "" : "U"}Int${reloc.size}${reloc.size > 8 ? "LE" : ""}`](bigInt ? value : Number(value));
    outBuffer.write(buffer, section.programHeader.p_offset + reloc.offset);
  }
  return outBuffer.toUint8Array();
}

// src/worker.ts
var wasmReady = false;
init().then(() => {
  wasmReady = true;
});
var om;
var state;
var breakpoints = /* @__PURE__ */ new Set();
var running = false;
globalThis.addEventListener("message", (fullMsg) => {
  const msg = fullMsg.data;
  switch (msg.type) {
    case "run":
      startRunningCode(msg);
      break;
    case "poll-status":
      if (!om || !state) {
        console.warn("Poll while not running");
        return;
      }
      postMessageFromWorker({
        type: "status",
        status: getStatus(om, state)
      });
      break;
    case "halt":
      if (!om) {
        console.warn("Halt while not running");
        return;
      }
      om.free();
      om = void 0;
      state = void 0;
      break;
    case "pause":
      running = false;
      break;
    case "continue":
      running = true;
      continueRunningCode(true);
      break;
    case "single-step":
      singleStep();
      break;
    case "set-breakpoints":
      setBreakpoints(msg.breakpointFroms);
      break;
    default:
      msg;
      console.error("Unrecognized message type", msg);
      break;
  }
});
function postMessageFromWorker(msg) {
  globalThis.postMessage(msg);
}
function setBreakpoints(breakpointFroms) {
  if (!state) return;
  breakpoints = /* @__PURE__ */ new Set();
  for (const from of breakpointFroms) {
    const addr2 = findInstructionFromOffset(state, from);
    if (addr2 === void 0) continue;
    breakpoints.add(BigInt(addr2));
  }
}
function arrToString(arr) {
  const maxLen = 128 * 1024;
  if (arr.length > maxLen) {
    arr = arr.slice(0, maxLen);
  }
  return new TextDecoder("utf-8").decode(arr);
}
function getStatus(om2, state2) {
  const rip = om2.get_rip();
  const linePos = rip < Number.MAX_SAFE_INTEGER ? linePosition(state2, Number(rip)) : null;
  return {
    stdout: arrToString(om2.get_stdout()),
    stderr: arrToString(om2.get_stderr()),
    registersStr: arrToString(om2.get_registers_str()),
    linePos,
    fullStepCount: om2.get_full_step_count()
  };
}
function startRunningCode(data) {
  try {
    running = true;
    const ds = data.state;
    state = new AssemblyState();
    state.compile(ds.doc, { haltOnError: true });
    const elf = createExecutable(state);
    if (!wasmReady) {
      throw new Error("Wasm module not yet loaded");
    }
    setBreakpoints(ds.breakpoints);
    if (om) {
      om.free();
    }
    om = OuterMachine.init(
      elf,
      // argv
      getArgv(ds.inputConfig),
      // envp
      getEnvp(ds.inputConfig),
      // TODO-seed: proper seed
      getInitUnp(ds.inputConfig)
    );
    setTimeout(() => continueRunningCode(false), 0);
  } catch (e) {
    postMessageFromWorker({
      type: "error",
      error: `Error when running: ${e}`
    });
  }
}
function getArgv(ic) {
  let argv;
  try {
    argv = JSON.parse(ic.argv);
  } catch (e) {
    throw new Error("Invalid argv JSON.");
  }
  if (!Array.isArray(argv) || argv.some((e) => typeof e !== "string")) {
    throw new Error("Argv should be a JSON array of strings.");
  }
  argv.unshift(ic.arg0);
  return argv;
}
function getEnvp(ic) {
  let envp;
  try {
    envp = JSON.parse(ic.envp);
  } catch (e) {
    throw new Error("Invalid envp JSON.");
  }
  if (!Array.isArray(envp) || envp.some((e) => typeof e !== "string")) {
    throw new Error("Envp should be a JSON array of strings.");
  }
  return envp;
}
function getInitUnp(ic) {
  if (!ic.useFixed) {
    return WebUnpredictables.from_random_seed(BigInt(ic.randomSeed));
  }
  return WebUnpredictables.from_fixed(
    ic.vdsoPtr,
    ic.rand16,
    ic.execfnPtr,
    ic.platformOffset
  );
}
function checkDone() {
  if (!om || !state) return;
  if (om.is_done()) {
    running = false;
    postMessageFromWorker({
      type: "done",
      status: getStatus(om, state)
    });
    return true;
  }
}
function checkBreakpoint() {
  if (!om || !state) return;
  if (breakpoints.has(om.get_rip())) {
    running = false;
    postMessageFromWorker({
      type: "pause",
      status: getStatus(om, state)
    });
    return true;
  }
}
function singleStep() {
  if (!om || !state) return;
  if (checkDone()) return;
  om.step();
  if (checkDone()) return;
  postMessageFromWorker({
    type: "pause",
    status: getStatus(om, state)
  });
}
function continueRunningCode(firstAfterContinue) {
  if (!om) return;
  if (!running) {
    return;
  }
  try {
    if (checkDone()) return;
    if (!firstAfterContinue) {
      if (checkBreakpoint()) return;
    }
    for (let i = 0; i < 65536; i++) {
      om.step();
      if (checkDone()) return;
      if (checkBreakpoint()) return;
    }
    setTimeout(() => continueRunningCode(false), 0);
  } catch (e) {
    postMessageFromWorker({
      type: "error",
      error: `Error when running: ${e}`
    });
  }
}
/*! Bundled license information:

ieee754/index.js:
  (*! ieee754. BSD-3-Clause License. Feross Aboukhadijeh <https://feross.org/opensource> *)

buffer/index.js:
  (*!
   * The buffer module from node.js, for the browser.
   *
   * @author   Feross Aboukhadijeh <https://feross.org>
   * @license  MIT
   *)
*/
//# sourceMappingURL=worker.js.map
