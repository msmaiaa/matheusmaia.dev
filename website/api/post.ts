import { baseApi } from "api";
import { BlogPost, PaginatedResponse } from "types";

const PostService = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getBlogPosts: builder.query<PaginatedResponse<BlogPost[]>, void>({
      query: () => ({
        url: "/post/blog_page",
      }),
    }),
  }),
});

export const { useGetBlogPostsQuery } = PostService;
