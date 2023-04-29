module.exports = {
    content: {
        // "./src/**/*.{html,js,rs}",
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        fontFamily: {
            'sans': ['"Open Sans"', 'sans-serif'],
        },
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("daisyui")
    ],
    daisyui: {
        themes: [
            "dracula",
        ],
    },
}