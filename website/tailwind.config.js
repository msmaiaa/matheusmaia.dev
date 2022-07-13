/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')

module.exports = {
  content: ["./pages/**/*.{tsx, ts}", "./components/**/*.{tsx, ts}"],
  theme: {
    extend: {
			colors: {
				bg_primary: "#0d1117",
				border_primary: "#30363d",
				btn_bg: "#21262d",
				btn_text: "#c9d1d9",
				btn_border: "rgb(240 246 252 / 10%)",
				btn_hover_bg: "rgb(40, 46, 51)",
				btn_hover_border: "rgb(110, 118, 129)"
			}
		},
  },
  plugins: [plugin(({addVariant}) => {
		addVariant('not-last', '&:not(:last-child)')
	})],
}
