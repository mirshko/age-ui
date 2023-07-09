/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "src/**/*.{ts,js,svelte}"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
