/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/src/**/*.{html,js,rs}",
    "./frontend/src/**/*.{html,js,rs}",
    "./leptail/src/**/*.{html,js,rs}",
    "./tailwind-utils/src/**/*.{html,js,rs}",
    "./theme-moonlight/src/**/*.{html,js,rs}",
  ],
  theme: {
    extend: {
      colors: {
        clifford: "#da373d", 
      },
    },
  },
  safelist: [ 
    {
      pattern: /bg-(slate|cyan|fuchsia|sky|emerald|amber|rose)-(100|200|300|600|700)/,
      variants: ['dark'],
    },
    {
      pattern: /ring-(slate|cyan|fuchsia|sky|emerald|amber|rose)-(200|800)/,
      variants: ['hover', 'dark:hover'],
    },
  ],
  // darkMode: 'class',
};
