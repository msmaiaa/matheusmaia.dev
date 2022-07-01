import { Layout } from "components/layout/Layout";
import type { AppProps } from "next/app";
import GlobalStyles from "styles/globalStyles";
import "../styles/globals.css";

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
