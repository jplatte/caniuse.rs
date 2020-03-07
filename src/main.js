import init, { run } from '../pkg/caniuse_rs.js';

let theme;
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

// Load main app
// TODO: Fallback to showing <noscript> tag if wasm is not available
async function main() {
    await init('/caniuse_rs.wasm');
    run();
}
main()
