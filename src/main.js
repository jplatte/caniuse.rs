import init, { run } from '../pkg/caniuse_rs.js';

var theme;
try {
    // Restore theme if set
    theme = localStorage.getItem("theme");
} catch (_e) {
    // If localStorage access is forbidden, do nothing
}

if (typeof theme !== "string") {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        theme = "dark";
    } else {
        theme = "light";
    }
}

document.documentElement.dataset.theme = theme;

if ('WebAssembly' in window) {
    var noscript_tag = document.body.querySelector("noscript");
    noscript_tag.parentElement.removeChild(noscript_tag);

    // Load main app
    async function main() {
        await init('/caniuse_rs.wasm');
        run();
    }
    main()
} else {
    // With scripts enabled, the noscript tag's contents are not interpreted as
    // HTML. As such, no matter what combination of tags and text it contains,
    // when this code is executed, the whole content will just be one text node.
    document.body.innerHTML = document.body.querySelector("noscript").innerText;
}
