const WEBSOCKET_HOST = window.location.hostname;
const WEBSOCKET_PORT = import.meta.env.WEBSOCKET_PORT || 8999;
const WEBSOCKET_ADDRESS = `ws://${window.location.hostname}:${WEBSOCKET_PORT}`;
// const WEBSOCKET_ADDRESS = `ws://joe-raspi.local:${WEBSOCKET_PORT}`;

const config = {
  WEBSOCKET_HOST,
  WEBSOCKET_PORT,
  WEBSOCKET_ADDRESS,
};

export default config;
