/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      'gothic': ["'DotGothic16'"]
    },
    extend: {
      keyframes: {
        growout: {
          '0%': { transform: 'scale(0)' },
          '80%' : {transform: 'scale(1.1)'},
          '100%': { transform: 'scale(1)' },
        }
      }
    },
  },
  plugins: [],
}