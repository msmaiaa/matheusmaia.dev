import { baseApi } from "api";
import { BlogPost, PaginatedResponse, SearchPostQuery } from "types";

const PostService = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getBlogPosts: builder.mutation<
      PaginatedResponse<BlogPost[]>,
      SearchPostQuery
    >({
      query: (params) => {
        return {
          url: "/post/blog_page",
          params: params.title ? params : {},
        };
      },
    }),
  }),
});

export const { useGetBlogPostsMutation } = PostService;
