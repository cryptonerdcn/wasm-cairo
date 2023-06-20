const maxWorkers = 1
const worker = new Worker('/js/worker.cjs')

// Run cairo-rs through our proxy function.
window.ClickFunc = () => {
    //get textarea cairo_program's value
    const cairo_program = getActiveTextAreaValue();
    if (cairo_program == "" || cairo_program == null || cairo_program == undefined) {
        return;
    }
    console.log("clicked!");
    // disable compile button
    document.getElementById("Compile").disabled = true;
    worker.postMessage({
        data: cairo_program,
        replaceIds: document.getElementById("replace-ids").checked ,
        functionToRun: "compileCairoProgram"
    });
    worker.onmessage = function(e) {
        document.getElementById("sierra_program").value = e.data;
        document.getElementById("Compile").disabled = false;
    };
}

window.runFunc = () => {
    //get textarea cairo_program's value
    const cairo_program = getActiveTextAreaValue();
    if (cairo_program == "" || cairo_program == null || cairo_program == undefined) {
        return;
    }
    document.getElementById("Run").disabled = true;
    const gasValue = document.getElementById("available-gas").value;
    worker.postMessage({
        data: cairo_program,
        availableGas: gasValue == "" ? undefined : parseInt(gasValue),
        printFullMemory: document.getElementById("print-full-memory").checked,
        useDBGPrintHint: document.getElementById("use-cairo-debug-print").checked,
        functionToRun: "runCairoProgram"
    });
    worker.onmessage = function(e) {
        document.getElementById("run_result").value = e.data;
        document.getElementById("Run").disabled = false;
    };
}

const getActiveTextAreaValue = () => {
    // Select all textareas with the "active" class
    const textAreas = document.querySelectorAll('textarea.active');

    // Loop through the textareas and return the one with style.display not set to "none"
    for (var i = 0; i < textAreas.length; i++) {
        if (textAreas[i].style.display !== 'none') {
            return textAreas[i].value;
        }
    }

    // Return null if no such textarea is found
    return null;
}