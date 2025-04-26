//const openSelectorButton = document.getElementById('theme-toggle-button');
const openSelectorButton = document.getElementById("open-theme-selector-button");
const closeSelectorButton = document.getElementById("close-theme-selector-button");


const themeOverlay = document.getElementById('theme-overlay');
const themeRadios = document.querySelectorAll('input[name="theme"]');

const styleSheet = document.createElement('link');
styleSheet.rel = 'stylesheet';
document.head.appendChild(styleSheet);

function setTheme(themeName) {
    if (themeName === 'wave') {
        styleSheet.href = 'wave.css';
    } else if (themeName === 'dark') {
        styleSheet.href = 'dark.css';
    } else if (themeName === 'light') {
        styleSheet.href = 'light.css';
    } else {
        styleSheet.href = 'wave.css'; // Default to 'wave'
    }
    localStorage.setItem('theme', themeName);
}

openSelectorButton.addEventListener('click', () => {
    themeOverlay.classList.toggle('active'); // Show/hide overlay
});

closeSelectorButton.addEventListener('click', () => {
    themeOverlay.classList.toggle('active'); // Show/hide overlay
});

// Close the overlay on `Esc`
document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' || event.keyCode === 27) { // 'Escape' key or keyCode 27
        themeOverlay.classList.toggle('active'); // Show/hide overlay
    }
});


themeRadios.forEach(radio => {
    radio.addEventListener('change', () => {
        setTheme(radio.value);
        themeOverlay.classList.remove('active'); // Hide overlay after selection
    });
});

// Load saved theme on page load
const savedTheme = localStorage.getItem('theme');
if (savedTheme) {
    document.querySelector(`input[value="${savedTheme}"]`).checked = true; // Select the saved radio button
    setTheme(savedTheme);
} else {
    setTheme('wave'); // Default to 'wave'
}

// Make sure the mobile styles are loaded after the main styles in order to override some properties
const mobileStyleSheet = document.createElement('link');
mobileStyleSheet.rel = 'stylesheet';
mobileStyleSheet.href = 'mobile.css';
document.head.appendChild(mobileStyleSheet);
