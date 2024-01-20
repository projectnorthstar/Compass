# 🧭 Compass: Utilities for using a northstar headset on the web! 
### ( ⚠️ extremely WIP )

## Purpose:

The web (through WebXR) provides a great cross platform way to develop for XR. This library provides necessary tooling in order for Northstar based headsets to access WebGL libraries through WebXR. 

In it's current form the goals are as follows: 

1) Send Leap Motion hand tracking data to the browser via websockets.
2) Provide a auto-windowing method and distortion method in-browser. (Auto-Windowing is only possible in Chromium based browsers until more browsers support the screen API)
3) Provide a simple to use Polyfill or Web Extension to allow the use of Northstar in WebGL Libraries like Three.js, Babylon.JS, PlayCanvas, WonderLand Engine and more.
