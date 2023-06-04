module.exports = {
    content: {
        // "./src/**/*.{html,js,rs}",
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        fontFamily: {
            'sans': ['Figtree', 'Noto Sans JP', 'Noto Sans'],
        },
        extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("daisyui")
    ],
    daisyui: {
        themes: [
            "night",
        ],
        // darkTheme: "dark",
        // base: true,
        // styled: true,
        // utils: true,
        // rtl: false,
        // logs: true,
        // prefix: "",
    },
}