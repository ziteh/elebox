import React from "react";
import { DocsThemeConfig } from "nextra-theme-docs";

const config: DocsThemeConfig = {
  logo: <span>Elebox</span>,
  project: {
    link: "https://github.com/ziteh/elebox",
  },
  docsRepositoryBase: "https://github.com/ziteh/elebox/tree/main/website",
  head: (
    <>
      {/* Responsive  */}
      <meta name="viewport" content="initial-scale=1, width=device-width" />
      {/* Robot font */}
      <link rel="preconnect" href="https://fonts.googleapis.com" />
      <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
      <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap"
      />
      {/* Icon */}
      <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/icon?family=Material+Icons"
      />
    </>
  ),
  useNextSeoProps() {
    return {
      titleTemplate: "%s – Elebox",
    };
  },
  footer: {
    text: "",
  },
  nextThemes: {
    defaultTheme: "system",
  },
  i18n: [
    {
      locale: "en",
      text: "English",
    },
    // {
    //   locale: "zh",
    //   text: "繁體中文",
    // },
  ],
  search: {
    placeholder: "Search",
  },
  sidebar: {
    toggleButton: true,
    autoCollapse: true,
  },
  toc: {
    float: true,
    backToTop: true,
  },
  editLink: {
    component: null,
  },
  feedback: {
    content: null,
  },
  gitTimestamp: null,
};

export default config;
