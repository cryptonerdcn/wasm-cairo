// JavaScript for sidebar navigation
function openPage(evt, pageName) {
    let i, pageContent, sidebarItem;
    pageContent = document.getElementsByClassName("page-content");
    for (i = 0; i < pageContent.length; i++) {
        pageContent[i].style.display = "none";
    }
    sidebarItem = document.getElementsByClassName("sidebar-item");
    for (i = 0; i < sidebarItem.length; i++) {
        sidebarItem[i].className = sidebarItem[i].className.replace(" active", "");
    }
    document.getElementById(pageName).style.display = "block";
    evt.currentTarget.className += " active";
}

// JavaScript for tab navigation
function openTab(evt, tabName) {
    let i, tabcontent, tablinks;
    tabcontent = document.getElementsByClassName("tabcontent");
    for (i = 0; i < tabcontent.length; i++) {
        tabcontent[i].style.display = "none";
    }
    tablinks = document.getElementsByClassName("tablink");
    for (i = 0; i < tablinks.length; i++) {
        tablinks[i].className = tablinks[i].className.replace(" active", "");
    }
    document.getElementById(tabName).style.display = "block";
    evt.currentTarget.className += " active";
}

// Get the element with id="defaultOpen" and click on it
document.getElementsByClassName("sidebar-item")[0].click();
document.getElementsByClassName("tablink")[0].click();

// Use the 'DOMContentLoaded' event to ensure the DOM is fully loaded before trying to add event listeners.
document.addEventListener('DOMContentLoaded', function() {

    document.getElementById('open-file-button').addEventListener('click', function() {
        // Simulate a click on the hidden file input when the button is clicked
        document.getElementById('file-input').click();
    });

    document.getElementById('file-input').addEventListener('change', function(e) {
        // When a file is selected, load its content into the textarea
        var file = e.target.files[0];
        if (file) {
            var reader = new FileReader();
            reader.onload = function(e) {
                // document.getElementById("cairo_program").value = e.target.result;
                const area = getActiveTextArea()
                if (area) {
                    area.value = e.target.result;
                }
            }
            reader.readAsText(file);
        }
    });

});

// Activate the default tab content
document.getElementById("cairo_program").style.display = "block";

Array.from(document.querySelectorAll(".tab-item")).forEach(function(tab) {
    tab.addEventListener("click", function() {
        // Remove .active class from all tabs
        Array.from(document.querySelectorAll(".tab-item")).forEach(function(tab) {
            tab.classList.remove("active");
        });

        // Add .active class to clicked tab
        this.classList.add("active");

        // Hide all textareas
        Array.from(document.querySelectorAll(".codeEditor")).forEach(function(editor) {
            editor.style.display = "none";
        });

        // Show the textarea associated with the clicked tab
        document.getElementById("cairo_program").style.display = "block";
        document.getElementById("cairo_program").classList.add("active");
    });
});

document.getElementById("new-tab").addEventListener("click", function() {
    // Create new tab
    var newTab = document.createElement("button");
    newTab.textContent = "New File"; 
    newTab.className = "tab-item";

    // Create new textarea for the new tab
    var newTextArea = document.createElement("textarea");
    newTextArea.className = "codeEditor";

    // Add the new tab before the plus button
    this.parentNode.insertBefore(newTab, this);

    // Add the new textarea to the tabs content
    document.querySelector(".tabs-content").appendChild(newTextArea);
    
    // Attach event to new tab
    newTab.addEventListener("click", function() {
        // Hide all textareas
        Array.from(document.querySelectorAll(".codeEditor")).forEach(function(editor) {
            editor.style.display = "none";
            editor.classList.remove("active");
        });

        Array.from(document.querySelectorAll(".tab-item")).forEach(function(tab) {
            tab.classList.remove("active");
        });

        // Add .active class to clicked tab
        this.classList.add("active");

        // Show the textarea associated with the clicked tab
        newTextArea.style.display = "block";
        newTextArea.classList.add("active");
    });
});


const getActiveTextArea = () => {
    // Select all textareas with the "active" class
    const textAreas = document.querySelectorAll('textarea.active');

    // Loop through the textareas and return the one with style.display not set to "none"
    for (var i = 0; i < textAreas.length; i++) {
        if (textAreas[i].style.display !== 'none') {
            return textAreas[i];
        }
    }

    // Return null if no such textarea is found
    return null;
}

window.saveFileFunc = async () => {
    const textarea = document.querySelector('.codeEditor.active');
    if(textarea.value == "") {
        return;
    }
    const options = {
        suggestedName: 'astro.cairo',
        types: [{
            description: 'Cairo File',
            accept: { 'text/plain': ['.cairo'] },
        }],
    };
    const fileHandle = await window.showSaveFilePicker(options);
    const writable = await fileHandle.createWritable();
    await writable.write(textarea.value);
    await writable.close();
    alert("File has been saved.");
}

// Attach the function to the save button
document.getElementById("save-file-button").addEventListener("click", saveFileFunc);

// Save the compiled file
window.saveCompiledFileFunc = async () => {
    const textarea = document.getElementById('sierra_program');
    if(textarea.value == "") {
        return;
    }
    const options = {
        suggestedName: 'astro_compiled.json',
        types: [{
            description: 'JSON File',
            accept: { 'text/plain': ['.json'] },
        }],
    };
    const fileHandle = await window.showSaveFilePicker(options);
    const writable = await fileHandle.createWritable();
    await writable.write(textarea.value);
    await writable.close();
    alert("File has been saved.");
}

// Attach the function to the save button
document.getElementById("save-compiled-file-button").addEventListener("click", saveCompiledFileFunc);

// Load the default cairo program
window.addEventListener('DOMContentLoaded', (event) => {
    fetch('HelloStarknetAstro.cairo')
        .then(response => response.text())
        .then(data => {
            document.getElementById('cairo_program').value = data;
        })
        .catch((error) => {
            console.error('Error:', error);
        });
});