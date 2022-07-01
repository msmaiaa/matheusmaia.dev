import "../styles/globals.css";
import type { AppProps } from "next/app";
import GlobalStyles from "styles/globalStyles";
import { Layout } from "components/layout/Layout";

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <GlobalStyles />
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </>
  );
}

export default App;
