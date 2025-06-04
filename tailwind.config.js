/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,rs}"],
  theme: {
    extend: {
      colors: {
        'highlight': {
          DEFAULT: "#FF5733",
          10: "#060201",
          20: "#23120B", 
          30: "#3C1C12",
          40: "#512316",
          50: "#662A1A",
          60: "#7C311E",
          70: "#933822",
          80: "#AB3F26",
          90: "#C34629",
          100: "#DB4D2D",
          110: "#F45431",
          120: "#FF6A45",
          130: "#FF8665",
          140: "#FF9F82",
          150: "#FFB59F",
          160: "#FFCABE",
        },
      },
    },
  },
  plugins: [],
}