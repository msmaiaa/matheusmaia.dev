export type LoginInput = {
  username: string;
  password: string;
};

export type LoginOutput = {
  username: string;
  token: string;
};

export type PaginatedResponse<T> = {
  data: T;
  total: number;
};

export type BlogPost = {
  id: number;
  content: string;
  title: string;
  slug: string;
  published: string;
  author_id: string;
  created_at: string;
  updated_at: string;
  author: {
    id: string;
    username: string;
    avatar_url: string;
  };
};

export type SearchPostQuery = {
  title?: string;
};

export type Tag = {
  id: number;
  name: string;
};

export type CreatePostInput = {
  title: string;
  content: string;
  tags?: number[];
};
