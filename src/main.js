import init, { run } from '../pkg/caniuse_rs.js';

try {
    // Restore theme if set
    let theme = localStorage.getItem("theme");
    if (typeof theme !== "string") {
        if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
            theme = "dark";
        } else {
            theme = "light";
        }
    }

    document.documentElement.dataset.theme = theme;
} catch (_e) {
    // If localStorage access is forbidden, do nothing
}

// Load main app
// TODO: Fallback to showing <noscript> tag if wasm is not available
async function main() {
    await init('/caniuse_rs_bg.wasm');
    run();
}
main()
