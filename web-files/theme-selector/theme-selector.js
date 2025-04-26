//const openSelectorButton = document.getElementById('theme-toggle-button');
const openSelectorButton = document.getElementById("open-theme-selector-button");
const closeSelectorButton = document.getElementById("close-theme-selector-button");


const themeOverlay = document.getElementById('theme-overlay');
const themeRadios = document.querySelectorAll('input[name="theme"]');

const mainCss = document.createElement('link');
mainCss.rel = 'stylesheet';
document.head.appendChild(mainCss);

const prismCss = document.createElement('link');
prismCss.rel = 'stylesheet';
document.head.appendChild(prismCss);

function setTheme(themeName) {
    if (themeName === 'wave') {
        mainCss.href = 'wave.css';
        prismCss.href = 'prism-tomorrow.css';
    } else if (themeName === 'dark') {
        mainCss.href = 'dark.css';
        prismCss.href = 'prism-tomorrow.dark.css';
    } else if (themeName === 'light') {
        mainCss.href = 'light.css';
        prismCss.href = 'prism-tomorrow.light.css';
    } else {
        mainCss.href = 'wave.css'; // Default to 'wave'
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

// Make sure the theme selector styles are loaded after the main styles in order to override some properties
const themeSelectorStyleSheet = document.createElement('link');
themeSelectorStyleSheet.rel = 'stylesheet';
themeSelectorStyleSheet.href = 'theme-selector/theme-selector.css';
document.head.appendChild(themeSelectorStyleSheet);

