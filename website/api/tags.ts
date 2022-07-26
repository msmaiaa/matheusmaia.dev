import { baseApi } from "api";
import { Tag } from "types";

const TagService = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getTags: builder.query<Tag[], void>({
      query: () => ({
        url: "/tag",
      }),
    }),
  }),
});

export const { useGetTagsQuery } = TagService;
