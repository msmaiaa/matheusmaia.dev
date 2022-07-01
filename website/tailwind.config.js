/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')

module.exports = {
  content: ["./pages/**/*.{tsx, ts}", "./components/**/*.{tsx, ts}"],
  theme: {
    extend: {
			colors: {
				bg_primary: "#0d1117"
			}
		},
  },
  plugins: [plugin(({addVariant}) => {
		addVariant('not-last', '&:not(:last-child)')
	})],
}
