import { useStore } from "store";

export const useAuth = () => {
  const { isLoggedIn, isLoading, data } = useStore((state) => ({
    isLoggedIn: state.loggedIn,
    data: state.data,
    isLoading: state.isLoading,
  }));

  return {
    loggedIn: isLoggedIn && data,
    isLoading,
  };
};
