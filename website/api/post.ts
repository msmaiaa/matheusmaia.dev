import { baseApi } from "api";
import {
  BlogPost,
  CreatePostInput,
  PaginatedResponse,
  SearchPostQuery,
} from "types";

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
    createPost: builder.mutation<void, CreatePostInput>({
      query: (body) => {
        return {
          url: "/post",
          method: "POST",
          body,
        };
      },
    }),
  }),
});

export const { useGetBlogPostsMutation, useCreatePostMutation } = PostService;
