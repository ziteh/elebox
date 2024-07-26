import React from "react";
import { DocsThemeConfig } from "nextra-theme-docs";

const config: DocsThemeConfig = {
  logo: <span>Elebox</span>,
  project: {
    link: "https://github.com/ziteh/elebox",
  },
  docsRepositoryBase: "https://github.com/ziteh/elebox/tree/main/website",
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
    placeholder: "search",
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
