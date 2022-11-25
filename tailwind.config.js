/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.{ts,svelte,html}",
  ],
  theme: {
    colors: {
      black: '#141414',
      dark: '#1F1F1F',
      white: '#F5F5F5',
      light: '#EBEBEB'
    }
  }
}
