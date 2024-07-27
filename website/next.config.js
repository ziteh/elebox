const withNextra = require("nextra")({
  theme: "nextra-theme-docs",
  themeConfig: "./theme.config.tsx",
});

module.exports = withNextra({
  output: "export",
  // i18n: {
  //   locales: ["en", "zh"],
  //   defaultLocale: "en",
  // },
  assetPrefix: "./",
  basePath: "https://github.com/ziteh/elebox",
  images: {
    loader: "akamai",
    path: "",
    unoptimized: true,
    remotePatterns: [
      {
        protocol: "https",
        hostname: "i.imgur.com",
        port: "",
        pathname: "/**",
      },
    ],
  },
});
