import "../styles/globals.css";
import type { AppProps } from "next/app";
import GlobalStyles from "styles/globalStyles";

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <GlobalStyles />
      <Component {...pageProps} />
    </>
  );
}

export default App;
