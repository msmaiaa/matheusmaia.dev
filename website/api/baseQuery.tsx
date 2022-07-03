import {
  BaseQueryFn,
  FetchArgs,
  fetchBaseQuery,
  FetchBaseQueryError,
} from "@reduxjs/toolkit/dist/query";

const baseQuery = fetchBaseQuery({
  baseUrl: process.env.NEXT_PUBLIC_API_URL,
  prepareHeaders: (headers) => {
    console.log(process.env.NEXT_PUBLIC_API_URL);
    let token = localStorage.getItem("token");
    if (token) {
      headers.set("Authorizaton", `Bearer ${token}`);
    }
    return headers;
  },
});

const apiBaseQuery: BaseQueryFn<
  string | FetchArgs,
  unknown,
  FetchBaseQueryError
> = async (args, api, extraOptions) => {
  const result = await baseQuery(args, api, extraOptions);
  return result;
};

export { apiBaseQuery };
