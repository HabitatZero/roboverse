var GZ3D = GZ3D || {
  REVISION: "1",
};

const globalEmitter = new EventEmitter2({ verboseMemoryLeak: true });

// Assuming all mobile devices are touch devices.
const isTouchDevice = /Mobi/.test(navigator.userAgent);
