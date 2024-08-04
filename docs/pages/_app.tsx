import * as React from "react";
import { ThemeProvider, createTheme, CssBaseline } from "@mui/material";

const App = ({ Component, pageProps }) => {
  const muiTheme = createTheme({
    palette: {
      mode: "dark",
    },
  });

  return (
    <ThemeProvider theme={muiTheme}>
      <CssBaseline />
      <Component {...pageProps} />
    </ThemeProvider>
  );
};

export default App;
