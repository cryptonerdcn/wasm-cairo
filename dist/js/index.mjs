const maxWorkers = 1
const worker = new Worker('/js/worker.cjs')

worker.onmessage = function (e) {
    console.log("onmessage");
    document.getElementById("sierra_program").value = e.data;
    document.getElementById("Compile").disabled = false;
};

// Run cairo-rs through our proxy function.
window.ClickFunc = () => {
    //get textarea cairo_program's value
    let cairo_program = document.getElementById("cairo_program").value;
    console.log("clicked!");
    // disable compile button
    document.getElementById("Compile").disabled = true;
    worker.postMessage({
        data: cairo_program,
        functionToRun: "compileCairoProgram"
    });
    worker.onmessage = function(e) {
        document.getElementById("sierra_program").value = e.data;
        document.getElementById("Compile").disabled = false;
    };
}

window.runFunc = () => {
    //get textarea cairo_program's value
    let cairo_program = document.getElementById("cairo_program").value;
    document.getElementById("Run").disabled = true;
    worker.postMessage({
        data: cairo_program,
        functionToRun: "runCairoProgram"
    });
    worker.onmessage = function(e) {
        document.getElementById("run_result").value = e.data;
        document.getElementById("Run").disabled = false;
    };
}