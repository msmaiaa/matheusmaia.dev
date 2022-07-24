import { Layout } from "components/layout/Layout";
import type { AppProps } from "next/app";
import GlobalStyles from "styles/globalStyles";
import "../styles/globals.css";
import { ApiProvider } from "@reduxjs/toolkit/query/react";
import { baseApi } from "api";
import { useGetCurrentUserMutation } from "api/user";
import React, { ReactNode, useEffect } from "react";
import { useStore } from "store";

const AuthHandler: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [getUser] = useGetCurrentUserMutation();
  const { setLoggedIn, setIsLoading } = useStore((state) => ({
    setLoggedIn: state.setLoggedIn,
    setIsLoading: state.setIsLoading,
  }));

  useEffect(() => {
    getUser()
      .unwrap()
      .then(() => {
        setLoggedIn(true);
      })
      .catch(() => localStorage.removeItem("token"));

    setIsLoading(false);
  }, []);
  return <>{children}</>;
};

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <GlobalStyles />
      <Layout>
        <ApiProvider api={baseApi}>
          <AuthHandler>
            <Component {...pageProps} />
          </AuthHandler>
        </ApiProvider>
      </Layout>
    </>
  );
}

export default App;
