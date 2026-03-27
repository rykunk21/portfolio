/** @type {import('tailwindcss').Config} */

module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        // Each scale maps to CSS vars written by apply_palette() in theme.rs.
        // Use these tokens in components instead of raw Tailwind colors.
        //
        //   primary   → blue-bell     (links, buttons, interactive)
        //   highlight → carrot-orange (CTAs, badges, active states)
        //   accent    → desert-sand   (warm contrast, decorative)
        //   neutral   → graphite      (text, backgrounds, dark chrome)
        //   surface   → pale-slate    (cards, panels, subtle backgrounds)

        primary: {
          50:  "var(--color-primary-50)",
          100: "var(--color-primary-100)",
          200: "var(--color-primary-200)",
          300: "var(--color-primary-300)",
          400: "var(--color-primary-400)",
          500: "var(--color-primary-500)",
          600: "var(--color-primary-600)",
          700: "var(--color-primary-700)",
          800: "var(--color-primary-800)",
          900: "var(--color-primary-900)",
          950: "var(--color-primary-950)",
        },
        highlight: {
          50:  "var(--color-highlight-50)",
          100: "var(--color-highlight-100)",
          200: "var(--color-highlight-200)",
          300: "var(--color-highlight-300)",
          400: "var(--color-highlight-400)",
          500: "var(--color-highlight-500)",
          600: "var(--color-highlight-600)",
          700: "var(--color-highlight-700)",
          800: "var(--color-highlight-800)",
          900: "var(--color-highlight-900)",
          950: "var(--color-highlight-950)",
        },
        accent: {
          50:  "var(--color-accent-50)",
          100: "var(--color-accent-100)",
          200: "var(--color-accent-200)",
          300: "var(--color-accent-300)",
          400: "var(--color-accent-400)",
          500: "var(--color-accent-500)",
          600: "var(--color-accent-600)",
          700: "var(--color-accent-700)",
          800: "var(--color-accent-800)",
          900: "var(--color-accent-900)",
          950: "var(--color-accent-950)",
        },
        neutral: {
          50:  "var(--color-neutral-50)",
          100: "var(--color-neutral-100)",
          200: "var(--color-neutral-200)",
          300: "var(--color-neutral-300)",
          400: "var(--color-neutral-400)",
          500: "var(--color-neutral-500)",
          600: "var(--color-neutral-600)",
          700: "var(--color-neutral-700)",
          800: "var(--color-neutral-800)",
          900: "var(--color-neutral-900)",
          950: "var(--color-neutral-950)",
        },
        surface: {
          50:  "var(--color-surface-50)",
          100: "var(--color-surface-100)",
          200: "var(--color-surface-200)",
          300: "var(--color-surface-300)",
          400: "var(--color-surface-400)",
          500: "var(--color-surface-500)",
          600: "var(--color-surface-600)",
          700: "var(--color-surface-700)",
          800: "var(--color-surface-800)",
          900: "var(--color-surface-900)",
          950: "var(--color-surface-950)",
        },
      },
    },
  },
  plugins: [],
}
