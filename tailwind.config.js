const colors = require('tailwindcss/colors')

module.exports = {
  content: [
    "./src/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {
        colors: {},
        borderWidth: {
            '3': '3px',
        }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
