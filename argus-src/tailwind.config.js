/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}'
  ],
  theme: {
    extend: {
      fontSize: {
      },
      colors: {
        'image-show-text-bg': 'var(--color-image-show-text-bg)',
        'background-soft': 'var(--color-background-soft)',
      },
    }
  },
  plugins: [],
  darkMode: 'class'
}
