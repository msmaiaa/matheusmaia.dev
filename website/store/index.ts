import create from "zustand";

type UserData = {
  username: string;
};

type UserState = {
  data?: UserData;
  loggedIn: boolean;
  isLoading: boolean;
  setUser: (data: UserData) => void;
  setLoggedIn: (logged: boolean) => void;
  setIsLoading: (loading: boolean) => void;
};

export const useStore = create<UserState>((set) => ({
  data: undefined,
  loggedIn: false,
  isLoading: true,
  setUser(data) {
    set(() => ({ data }));
  },
  setLoggedIn(loggedIn) {
    set(() => ({ loggedIn }));
  },
  setIsLoading(isLoading) {
    set(() => ({ isLoading }));
  },
}));
