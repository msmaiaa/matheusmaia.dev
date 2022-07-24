import { useStore } from "store";

export const useAuth = () => {
  const { isLoggedIn, isLoading } = useStore((state) => ({
    isLoggedIn: state.loggedIn,
    isLoading: state.isLoading,
  }));

  return {
    isLoggedIn,
    isLoading,
  };
};
