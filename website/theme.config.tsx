import React from "react";
import { DocsThemeConfig } from "nextra-theme-docs";

const appName = "Elebox";

const description =
  "Lightweight personal electronic parts inventory management software.";

const title =
  "Elebox - Personal Electronic Parts Inventory Management Software";

const config: DocsThemeConfig = {
  logo: <span>Elebox</span>,
  project: {
    link: "https://github.com/ziteh/elebox",
  },
  docsRepositoryBase: "https://github.com/ziteh/elebox/tree/main/website",
  head: (
    <>
      <meta name="theme-color" content="#121212" />
      <meta httpEquiv="Content-Language" content="en" />
      <meta name="description" content={description} />
      <meta property="og:description" content={description} />
      <meta property="og:title" content={title} />
      <meta property="og:type" content="website" />
      <meta property="og:site_name" content={appName} />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={description} />
      <meta name="apple-mobile-web-app-title" content={appName} />
      <link rel="icon" href="/favicon.png" type="image/png" />
      {/* Responsive  */}
      <meta name="viewport" content="initial-scale=1, width=device-width" />
      {/* Robot font */}
      <link rel="preconnect" href="https://fonts.googleapis.com" />
      <link
        rel="preconnect"
        href="https://fonts.gstatic.com"
        crossOrigin="anonymous"
      />
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
    defaultTheme: "dark",
  },
  darkMode: false,
  // i18n: [
  //   {
  //     locale: "en",
  //     text: "English",
  //   },
  //   {
  //     locale: "zh",
  //     text: "繁體中文",
  //   },
  // ],
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
