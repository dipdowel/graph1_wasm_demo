const themeSelect = document.getElementById('theme-select');
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

themeSelect.addEventListener('change', function() {
    setTheme(this.value);
});

// Load saved theme on page load
const savedTheme = localStorage.getItem('theme');
if (savedTheme) {
    themeSelect.value = savedTheme;
    setTheme(savedTheme);
} else {
    setTheme('wave'); // Default to wave if no theme is saved
}