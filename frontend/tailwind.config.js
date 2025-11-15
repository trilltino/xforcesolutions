/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  safelist: [
    // Layout
    'container', 'mx-auto', 'px-4', 'py-8', 'min-h-screen',
    // Flexbox & Grid
    'flex', 'flex-col', 'flex-shrink-0', 'items-center', 'items-start', 'justify-between', 'justify-center', 
    'space-x-2', 'space-y-2', 'space-y-1', 'space-y-4', 'space-y-6',
    'grid', 'md:grid-cols-2', 'lg:grid-cols-3', 'gap-4', 'gap-6', 'gap-8',
    // Spacing
    'mb-1', 'mb-2', 'mb-3', 'mb-4', 'mb-6', 'mb-8', 'mb-12', 'mb-16',
    'mt-1', 'mt-2', 'ml-1', 'ml-2', 'ml-4', 'mr-2', 'mr-4',
    'p-2', 'p-4', 'p-6', 'p-8', 'px-4', 'px-6', 'py-2', 'py-3', 'py-4', 'py-8',
    'h-6', 'h-8', 'h-12', 'h-16', 'h-48', 'h-[300px]', 'h-[400px]', 'h-[500px]', 'md:h-[600px]',
    'w-4', 'w-6', 'w-8', 'w-10', 'w-12', 'w-48', 'w-64', 'w-full', 'w-[calc(100%-2rem)]', 'h-[calc(100%-2rem)]',
    // Typography
    'text-sm', 'text-lg', 'text-xl', 'text-2xl', 'text-3xl', 'text-4xl', 'text-5xl', 'md:text-7xl', 'md:text-5xl', 'md:text-2xl',
    'font-bold', 'font-medium', 'font-semibold', 'font-black', 'font-heading', 'font-nav', 'font-sans',
    'text-center', 'text-left', 'text-white', 'text-gray-300', 'text-gray-400', 'text-gray-600', 'text-gray-700', 'text-gray-900',
    'text-red-400', 'text-green-400', 'text-transparent', 'bg-clip-text',
    'dark:text-white', 'dark:text-gray-300', 'dark:text-gray-400', 'dark:text-gray-600', 'dark:text-gray-700', 'dark:text-gray-900',
    // Colors & Backgrounds
    'bg-black', 'bg-white', 'bg-gray-50', 'bg-gray-100', 'bg-gray-200', 'bg-gray-800',
    'bg-black/95', 'bg-white/95', 'bg-red-100', 'bg-red-900/30', 'bg-red-900/50', 'bg-green-900/50',
    'dark:bg-black', 'dark:bg-gray-800',
    // Hover states
    'hover:text-red-400', 'hover:text-white', 'hover:text-gray-900', 
    'hover:bg-red-900/30', 'hover:bg-red-100', 'hover:bg-gray-200', 'hover:bg-gray-700', 'hover:bg-gray-800', 'hover:bg-primary-700',
    'dark:hover:text-red-400', 'dark:hover:text-white', 'dark:hover:bg-red-900/30', 'dark:hover:bg-gray-700', 'dark:hover:bg-gray-800',
    'hover:border-red-600', 'hover:border-primary-500', 'hover:shadow-lg', 'hover:shadow-red-600/20', 'dark:hover:shadow-lg', 'dark:hover:shadow-red-600/20',
    'hover:scale-110', 'group-hover:scale-110', 'group-hover:text-red-400', 'dark:group-hover:text-red-400',
    // Borders
    'border', 'border-2', 'border-4', 'border-b', 'border-t', 
    'border-red-600', 'border-red-900', 'border-green-600', 'border-gray-300', 'border-gray-700', 'border-primary-500',
    'dark:border-red-900', 'dark:border-gray-300', 'dark:border-gray-700', 'dark:border-primary-500',
    'border-red-900/30', 'border-gray-300/30', 'dark:border-red-900/30',
    'rounded-lg', 'rounded', 'rounded-full',
    // Positioning
    'fixed', 'absolute', 'relative', 'inset-0', 'inset-2', 'inset-4', 'top-0', 'left-0', 'right-0', 'bottom-0',
    'z-50', 'z-10',
    // Effects
    'backdrop-blur-sm', 'shadow-lg', 'shadow-xl', 'shadow-2xl',
    'transition-all', 'transition-colors', 'transition-transform', 'duration-200', 'duration-300',
    'opacity-50', 'opacity-80', 'scale-110',
    // Gradients
    'bg-gradient-to-r', 'bg-gradient-to-br', 'bg-gradient-to-t',
    'from-black/90', 'from-black/50', 'from-black', 'via-black/50', 'to-transparent',
    'from-primary-500', 'to-primary-700', 'from-white', 'to-gray-300', 'from-gray-900', 'to-gray-700',
    'dark:from-white', 'dark:to-gray-300', 'dark:border-primary-500',
    // Display
    'hidden', 'md:hidden', 'block', 'inline-block', 'md:flex',
    // Object fit
    'object-cover', 'object-contain',
    // Overflow
    'overflow-hidden',
    // Max width
    'max-w-2xl', 'max-w-4xl', 'max-w-6xl', 'max-w-7xl',
    // Interactive
    'cursor-pointer', 'cursor-not-allowed', 'focus:outline-none', 'focus:ring-2', 'focus:ring-primary-500', 'focus:border-primary-500',
    'dark:focus:border-primary-500', 'disabled:opacity-50', 'disabled:cursor-not-allowed',
    // Aria states
    'aria-[current]:text-red-400', 'dark:aria-[current]:text-red-400', 'aria-[current]:bg-red-900/20', 'dark:aria-[current]:bg-red-900/20',
    // Animations
    'animate-fade-in', 'animate-spin',
    // Primary colors
    'bg-primary-500', 'bg-primary-600', 'bg-primary-700', 'text-primary-500',
    // Custom utilities (from tailwind.css)
    'reflective-header', 'glossy-white', 'glossy-dark',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        heading: ['"Playfair Display"', 'serif'],
        nav: ['Montserrat', 'sans-serif'],
        mono: ['"JetBrains Mono"', 'monospace'],
      },
      colors: {
        primary: {
          50: '#fef2f2',
          100: '#fee2e2',
          200: '#fecaca',
          300: '#fca5a5',
          400: '#f87171',
          500: '#ef4444',
          600: '#dc2626',
          700: '#b91c1c',
          800: '#991b1b',
          900: '#7f1d1d',
        },
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-down': 'slideDown 0.3s ease-out',
        'scale-up': 'scaleUp 0.2s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideDown: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        scaleUp: {
          '0%': { transform: 'scale(0.95)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
      },
      backdropBlur: {
        xs: '2px',
      },
    },
  },
  plugins: [],
}
