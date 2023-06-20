importScripts("/pkg/wasm-cairo.js")
const { greet, compileCairoProgram, runCairoProgram } = wasm_bindgen;

(async () => {
    await wasm_bindgen("/pkg/wasm-cairo_bg.wasm")

    console.log(greet("dozo"))
})();

onmessage = function (e) {
    const { data, functionToRun } = e.data;
    wasm_bindgen("/pkg/wasm-cairo_bg.wasm").then(() => {
        let result;
        switch (functionToRun) {
            case "runCairoProgram":
                result = runCairoProgram(data);
                break;
            case "compileCairoProgram":
                result = compileCairoProgram(data);
                break;
            default:
                console.error(`Unexpected function: ${functionToRun}`);
                return;
        }
        console.log("text: " + result)
        postMessage(result);
    });
}