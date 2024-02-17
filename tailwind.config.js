/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/src/**/*.{html,js,rs}",
    "./frontend/src/**/*.{html,js,rs}",
    "./leptail/src/**/*.{html,js,rs}",
    "./tailwind-utils/src/**/*.{html,js,rs}",
    "./theme-gradiance/src/**/*.{html,js,rs}",
    "./theme-moonlight/src/**/*.{html,js,rs}",
  ],
  theme: {
    extend: {
      colors: {
        fuchsia: {
          50: "#FCF1FE",
          100: "#F9E3FD",
          200: "#F1BDF9",
          300: "#E993F6",
          400: "#D946EF",
          500: "#D227EC",
          600: "#C213DD",
          700: "#AD11C5",
          800: "#880D9B",
          900: "#630A71",
          950: "#460750"
        }
      }
    },
  },
  safelist: [ 
    {
      pattern: /(bg|from|to|via|ring)-(slate|cyan|fuchsia|violet|sky|indigo|blue|emerald|amber|rose)-(50|100|200|300|400|500|600|700|800|900|950)/,
      variants: ['dark', 'hover', 'dark:hover'],
    },
    {
      pattern: /(bg|text)-gradient-to-(t|tr|r|br|b|bl|l|tl)/,
      variants: ['dark', 'hover', 'dark:hover'],
    },  
    // {
    //   pattern: /bg-(slate|cyan|fuchsia|sky|emerald|amber|rose)-(100|200|300|600|700)/,
    //   variants: ['dark'],
    // },
    // {
    //   pattern: /ring-(slate|cyan|fuchsia|sky|emerald|amber|rose)-(200|800)/,
    //   variants: ['hover', 'dark:hover'],
    // },  
  ],
  // darkMode: 'class',
};
