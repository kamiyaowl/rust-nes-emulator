/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".index.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/rust_nes_emulator_wasm_bg.wasm": function() {
/******/ 			return {
/******/ 				"./rust_nes_emulator_wasm_bg.js": {
/******/ 					"__wbg_log_c758cfb2ddae4233": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/rust_nes_emulator_wasm_bg.js"].exports["__wbg_log_c758cfb2ddae4233"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/rust_nes_emulator_wasm_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["./pkg/rust_nes_emulator_wasm_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/rust_nes_emulator_wasm_bg.wasm":"9baa343ee04477b466ee"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("async function main() {\n  const { memory } = await __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm_bg */ \"./pkg/rust_nes_emulator_wasm_bg.wasm\"));\n  const {\n    WasmEmulator,\n    KeyEvent,\n    get_screen_width,\n    get_screen_height,\n    get_num_of_colors\n  } = await Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm.js */ \"./pkg/rust_nes_emulator_wasm.js\"));\n  const SCREEN_WIDTH = get_screen_width();\n  const SCREEN_HEIGHT = get_screen_height();\n  const NUM_OF_COLORS = get_num_of_colors(); // rust上での扱い、imageDataはalphaもある\n  const emu = new WasmEmulator();\n  emu.reset();\n  const rustBuf = new Uint8Array(memory.buffer);\n  const fbBasePtr = emu.get_fb_ptr();\n\n  function draw() {\n    const canvas = document.getElementById(\"fb\");\n    const ctx = canvas.getContext(\"2d\");\n    const imageData = ctx.getImageData(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);\n    for (let j = 0; j < SCREEN_HEIGHT; j++) {\n      for (let i = 0; i < SCREEN_WIDTH; i++) {\n        const imageDataPtr = j * (SCREEN_WIDTH * 4) + i * 4;\n        const rustDataPtr =\n          fbBasePtr + j * (SCREEN_WIDTH * NUM_OF_COLORS) + i * NUM_OF_COLORS;\n        imageData.data[imageDataPtr + 0] = rustBuf[rustDataPtr + 0]; // red\n        imageData.data[imageDataPtr + 1] = rustBuf[rustDataPtr + 1]; // green\n        imageData.data[imageDataPtr + 2] = rustBuf[rustDataPtr + 2]; // blue\n        imageData.data[imageDataPtr + 3] = 255; //alpha\n      }\n    }\n    ctx.putImageData(imageData, 0, 0);\n  }\n\n  // FPS制御とか\n  const emulateFps = 60;\n  const emulateInterval = 1000.0 / emulateFps;\n  let isEmulateEnable = false;\n\n  // Animation Frame Firedには依存せずに実行する\n  function emulate_loop() {\n    const start = new Date().getTime();\n    if (isEmulateEnable) {\n      emu.step_line();\n    }\n    const elapsed = ((new Date().getTime()) - start);\n    const diffTime = emulateInterval - elapsed;\n    // めちゃはやだったら待たせるし、間に合ってなければ即\n    const sleepTime = diffTime < 0 ? 0 : diffTime;\n    setTimeout(emulate_loop, sleepTime);\n  }\n  // Animation Frame Firedに同期してcanvasだけ書き換える\n  function draw_loop() {\n    if (isEmulateEnable) {\n      draw();\n    }\n    requestAnimationFrame(draw_loop);\n  }\n  emulate_loop();\n  draw_loop();\n\n  function release_key(key) {\n    if (isEmulateEnable) {\n      switch (key) {\n        case \"j\":\n          emu.update_key(KeyEvent.ReleaseA);\n          break;\n        case \"k\":\n          emu.update_key(KeyEvent.ReleaseB);\n          break;\n        case \"u\":\n          emu.update_key(KeyEvent.ReleaseSelect);\n          break;\n        case \"i\":\n          emu.update_key(KeyEvent.ReleaseStart);\n          break;\n        case \"w\":\n          emu.update_key(KeyEvent.ReleaseUp);\n          break;\n        case \"s\":\n          emu.update_key(KeyEvent.ReleaseDown);\n          break;\n        case \"a\":\n          emu.update_key(KeyEvent.ReleaseLeft);\n          break;\n        case \"d\":\n          emu.update_key(KeyEvent.ReleaseRight);\n          break;\n      }\n    }\n  }\n  function press_key(key) {\n    if (isEmulateEnable) {\n      switch (key) {\n        case \"j\":\n          emu.update_key(KeyEvent.PressA);\n          break;\n        case \"k\":\n          emu.update_key(KeyEvent.PressB);\n          break;\n        case \"u\":\n          emu.update_key(KeyEvent.PressSelect);\n          break;\n        case \"i\":\n          emu.update_key(KeyEvent.PressStart);\n          break;\n        case \"w\":\n          emu.update_key(KeyEvent.PressUp);\n          break;\n        case \"s\":\n          emu.update_key(KeyEvent.PressDown);\n          break;\n        case \"a\":\n          emu.update_key(KeyEvent.PressLeft);\n          break;\n        case \"d\":\n          emu.update_key(KeyEvent.PressRight);\n          break;\n      }\n    }\n  }\n\n  ELEMENT.locale(\"ja\", ELEMENT.lang.ja);\n  const app = new Vue({\n    el: \"#app\",\n    data: {\n      navbarVisible: true,\n      loadRomVisible: false,\n      keyconfigVisible: false,\n      gamepadVisible: false,\n      keyconfig: [\n        { key: \"A\", info: \"Left\" },\n        { key: \"W\", info: \"Up\" },\n        { key: \"S\", info: \"Down\" },\n        { key: \"D\", info: \"Right\" },\n        { key: \"J\", info: \"A\" },\n        { key: \"K\", info: \"B\" },\n        { key: \"U\", info: \"Select\" },\n        { key: \"I\", info: \"Start\" }\n      ]\n    },\n    methods: {\n      romSelect(e) {\n        if (e.target.files.length == 0) return;\n        const reader = new FileReader();\n        reader.onload = file => {\n          const arrayBuf = file.target.result;\n          const src = new Uint8Array(arrayBuf);\n          // stop emulate\n          isEmulateEnable = false;\n          // cassette load\n          if (!emu.load(src)) {\n            // error notify\n            this.$notify({\n              title: \"Load ROM Error\"\n            });\n            return;\n          }\n          // read success notify\n          const h = this.$createElement;\n          this.$notify({\n            title: \"Load ROM Success\",\n            message: h(\"i\", { style: \"color: teal\" }, e.target.files[0].name)\n          });\n          // start emulate\n          emu.reset();\n          isEmulateEnable = true;\n        };\n        // あとはcallbackで\n        reader.readAsArrayBuffer(e.target.files[0]);\n      },\n      reset() {\n        // emulate start時のみ\n        if (isEmulateEnable) {\n          isEmulateEnable = false;\n          emu.reset();\n          // notify\n          this.$notify({\n            title: \"Emulator Reset\"\n          });\n          // start\n          isEmulateEnable = true;\n        }\n      },\n      press_key(key) {\n        console.log(\"press\", key);\n        press_key(key);\n      },\n      release_key(key) {\n        console.log(\"release\", key);\n        release_key(key);\n      }\n    },\n    mounted() {\n      window.addEventListener(\"keyup\", e => {\n        release_key(e.key);\n      });\n      window.addEventListener(\"keydown\", e => {\n        press_key(e.key);\n      });\n    }\n  });\n}\n\nmain();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

/******/ });