/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{html,js}',
  ],
  theme: {
    extend: {
      colors: {
        primary: '#1D4ED8', // Azul principal
        secondary: '#9333EA', // Púrpura secundario
        accent: '#F59E0B', // Amarillo acento
        neutral: '#374151', // Gris neutro
        base: '#FFFFFF', // Blanco base
        info: '#3B82F6', // Azul información
        success: '#10B981', // Verde éxito
        warning: '#FBBF24', // Amarillo advertencia
        danger: '#EF4444', // Rojo peligro
      },
    },
  },
  plugins: [],
}
