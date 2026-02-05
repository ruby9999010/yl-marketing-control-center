/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'macos-bg': '#1C1C1E',
        'macos-separator': 'rgba(255, 255, 255, 0.1)',
        'macos-blue': '#0A84FF',
        'macos-green': '#30D158',
        'macos-red': '#FF453A',
        'macos-yellow': '#FFD60A',
        'macos-text-primary': 'rgba(255, 255, 255, 0.85)',
        'macos-text-secondary': 'rgba(255, 255, 255, 0.55)',
        'macos-text-tertiary': 'rgba(255, 255, 255, 0.25)',
      },
      borderRadius: {
        'macos': '8px',
        'macos-lg': '12px',
      },
      boxShadow: {
        'macos': '0 4px 16px rgba(0, 0, 0, 0.3)',
        'macos-lg': '0 8px 32px rgba(0, 0, 0, 0.4)',
      },
    },
  },
  plugins: [],
}
