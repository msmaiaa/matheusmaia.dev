/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./pages/**/*.{tsx, ts}", "./components/**/*.{tsx, ts}"],
  theme: {
    extend: {
			colors: {
				bg_primary: "#0d1117"
			}
		},
  },
  plugins: [],
}
