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
                document.getElementById("cairo_program").value = e.target.result;
            }
            reader.readAsText(file);
        }
    });

});

