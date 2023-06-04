(()=>{var e,t,r,n,o,i={},a={};function c(e){var t=a[e];if(void 0!==t)return t.exports;var r=a[e]={id:e,loaded:!1,exports:{}};return i[e](r,r.exports,c),r.loaded=!0,r.exports}c.m=i,e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",n=e=>{e&&!e.d&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},c.a=(o,i,a)=>{var c;a&&((c=[]).d=1);var s,d,l,u=new Set,p=o.exports,h=new Promise(((e,t)=>{l=t,d=e}));h[t]=p,h[e]=e=>(c&&e(c),u.forEach(e),h.catch((e=>{}))),o.exports=h,i((o=>{var i;s=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var i=[];i.d=0,o.then((e=>{a[t]=e,n(i)}),(e=>{a[r]=e,n(i)}));var a={};return a[e]=e=>e(i),a}}var c={};return c[e]=e=>{},c[t]=o,c})))(o);var a=()=>s.map((e=>{if(e[r])throw e[r];return e[t]})),d=new Promise((t=>{(i=()=>t(a)).r=0;var r=e=>e!==c&&!u.has(e)&&(u.add(e),e&&!e.d&&(i.r++,e.push(i)));s.map((t=>t[e](r)))}));return i.r?d:a()}),(e=>(e?l(h[r]=e):d(p),n(c)))),c&&(c.d=0)},c.d=(e,t)=>{for(var r in t)c.o(t,r)&&!c.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},c.f={},c.e=e=>Promise.all(Object.keys(c.f).reduce(((t,r)=>(c.f[r](e,t),t)),[])),c.u=e=>e+".index.js",c.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),c.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),c.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),o={},c.l=(e,t,r,n)=>{if(o[e])o[e].push(t);else{var i,a;if(void 0!==r)for(var s=document.getElementsByTagName("script"),d=0;d<s.length;d++){var l=s[d];if(l.getAttribute("src")==e){i=l;break}}i||(a=!0,(i=document.createElement("script")).charset="utf-8",i.timeout=120,c.nc&&i.setAttribute("nonce",c.nc),i.src=e),o[e]=[t];var u=(t,r)=>{i.onerror=i.onload=null,clearTimeout(p);var n=o[e];if(delete o[e],i.parentNode&&i.parentNode.removeChild(i),n&&n.forEach((e=>e(r))),t)return t(r)},p=setTimeout(u.bind(null,void 0,{type:"timeout",target:i}),12e4);i.onerror=u.bind(null,i.onerror),i.onload=u.bind(null,i.onload),a&&document.head.appendChild(i)}},c.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},c.v=(e,t,r,n)=>{var o=fetch(c.p+""+r+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,n).then((t=>Object.assign(e,t.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)))},(()=>{var e;c.g.importScripts&&(e=c.g.location+"");var t=c.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");if(r.length)for(var n=r.length-1;n>-1&&!e;)e=r[n--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),c.p=e})(),(()=>{var e={179:0};c.f.j=(t,r)=>{var n=c.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var o=new Promise(((r,o)=>n=e[t]=[r,o]));r.push(n[2]=o);var i=c.p+c.u(t),a=new Error;c.l(i,(r=>{if(c.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var o=r&&("load"===r.type?"missing":r.type),i=r&&r.target&&r.target.src;a.message="Loading chunk "+t+" failed.\n("+o+": "+i+")",a.name="ChunkLoadError",a.type=o,a.request=i,n[1](a)}}),"chunk-"+t,t)}};var t=(t,r)=>{var n,o,[i,a,s]=r,d=0;if(i.some((t=>0!==e[t]))){for(n in a)c.o(a,n)&&(c.m[n]=a[n]);s&&s(c)}for(t&&t(r);d<i.length;d++)o=i[d],c.o(e,o)&&e[o]&&e[o][0](),e[o]=0},r=self.webpackChunk=self.webpackChunk||[];r.forEach(t.bind(null,0)),r.push=t.bind(null,r.push.bind(r))})(),c.e(235).then(c.bind(c,235)).then((e=>{const t=document.getElementById("canvas");function r(){t.width=window.innerWidth,t.height=window.innerHeight}window.addEventListener("resize",r,!1),r();let n=new e.FlockingApp;!function e(){n.draw(),window.requestAnimationFrame(e)}(),document.getElementById("triangleboid").checked=!0,document.getElementById("showgrid").checked=!1,document.getElementById("triangleboid").onchange=function(){n.show_boid_headings=this.checked},document.getElementById("showgrid").onchange=function(){n.show_qtree_grid=this.checked}})).catch(console.error)})();