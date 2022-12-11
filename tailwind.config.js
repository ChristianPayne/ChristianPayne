/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.{ts,svelte,html}",
  ],
  theme: {
    colors: {
      light: {
        background: '#F5F5F5',
        foreground: '#D6D6D6',
        accent: '#B8B8B8',
        text: '#141414',
      },
      dark: {
        background: '#141414',
        foreground: '#333333',
        accent: '#525252',
        text: '#F5F5F5',
      }
    }
  }
}
