function toggleDetails(key) {
    var details = document.getElementById('details-' + key);
    if (details.style.display === 'none' || details.style.display === '') {
        details.style.display = 'block';
    } else {
        details.style.display = 'none';
    }
}

function toggleIncluded(key) {
    var included = document.getElementById('included-' + key);
    var mandatory = document.getElementById('mandatory-' + key);
    var optional = document.getElementById('optional-' + key);

    if (included.checked) {
        mandatory.disabled = false;
        optional.disabled = false;
        mandatory.checked = false;
        optional.checked = false;
    } else {
        mandatory.disabled = true;
        optional.disabled = true;
        mandatory.checked = false;
        optional.checked = false;
    }
}

function toggleMandatoryOptional(key) {
    var mandatory = document.getElementById('mandatory-' + key);
    var optional = document.getElementById('optional-' + key);

    if (mandatory.checked && optional.checked) {
        // If both are checked, uncheck the one that was just clicked
        if (event.target.id === 'mandatory-' + key) {
            optional.checked = false;
        } else {
            mandatory.checked = false;
        }
    }
}