import { Layout } from "components/layout/Layout";
import type { AppProps } from "next/app";
import GlobalStyles from "styles/globalStyles";
import "../styles/globals.css";
import { ApiProvider } from "@reduxjs/toolkit/query/react";
import { baseApi } from "api";

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <GlobalStyles />
      <Layout>
        <ApiProvider api={baseApi}>
          <Component {...pageProps} />
        </ApiProvider>
      </Layout>
    </>
  );
}

export default App;
