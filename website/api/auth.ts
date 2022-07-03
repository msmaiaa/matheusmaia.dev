import { baseApi } from "api";
import { LoginInput, LoginOutput } from "types";

const AuthService = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    login: builder.mutation<LoginOutput, LoginInput>({
      query: (data) => ({
        url: "/auth/login",
        method: "POST",
        body: { ...data },
      }),
    }),
  }),
});

export const { useLoginMutation } = AuthService;
