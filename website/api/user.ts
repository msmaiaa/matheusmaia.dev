import { baseApi } from "api";

const UserService = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getCurrentUser: builder.mutation<{ username: string }, void>({
      query: () => ({
        url: "/auth/me",
        method: "get",
      }),
    }),
  }),
});

export const { useGetCurrentUserMutation } = UserService;
