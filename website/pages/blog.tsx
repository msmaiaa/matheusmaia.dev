import { useGetBlogPostsMutation } from "api/post";
import React, { useCallback, useEffect, useState } from "react";
import tw from "twin.macro";
import styled from "styled-components";
import { BlogPost, SearchPostQuery } from "types";
import { format } from "date-fns";
import { $Input } from "components/ui/input";
import { $Button } from "components/ui/button";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import Link from "next/link";

const $PostsArea = tw.div`
	flex
	flex-col
	w-full
`;

const $PostContainer = styled.div`
  transition: all 0.1s;
  &:hover {
    cursor: pointer;
    border-color: rgb(110, 118, 129);
    color: #c9d1d9;
  }
  &:first-of-type {
    border-top-left-radius: 2px;
    border-top-right-radius: 2px;
  }
  &:last-of-type {
    border-bottom-left-radius: 2px;
    border-bottom-right-radius: 2px;
  }

  display: flex;
  align-items: center;
  width: 100%;
  border: 1px solid rgb(48, 51, 61);
  color: white;
  height: 42px;
`;

const Post: React.FC<{ post: BlogPost }> = ({ post }) => {
  return (
    <$PostContainer>
      <div className="border-r border-border_primary flex h-full items-center pl-2 pr-2">
        <img
          src={post.author.avatar_url}
          className="rounded-full w-6 h-6 mr-2"
        />
        <p>{post.author.username}</p>
      </div>
      <h1 className="ml-4 flex-grow">{post.title}</h1>
      <p className="mr-4">{format(new Date(post.created_at), "dd/MM/yyyy")}</p>
    </$PostContainer>
  );
};

const $InnerContainer = tw.div`
	flex
	flex-col
	w-1/2
`;

const searchSchema = z.object({
  title: z.string().optional(),
});

const Blog = () => {
  const { register, handleSubmit } = useForm<SearchPostQuery>({
    resolver: zodResolver(searchSchema),
  });
  const [posts, setPosts] = useState<BlogPost[]>([]);
  const [getPosts] = useGetBlogPostsMutation();

  const updatePosts = useCallback((args: SearchPostQuery) => {
    getPosts(args)
      .unwrap()
      .then((result) => {
        setPosts(result.data);
      });
  }, []);

  useEffect(() => {
    updatePosts({});
  }, []);

  const onSubmit = (data: SearchPostQuery) => {
    updatePosts(data);
  };
  return (
    <div className="flex justify-center">
      <$InnerContainer>
        <div className="flex w-full mb-4">
          <form className="flex-grow" onSubmit={handleSubmit(onSubmit)}>
            <$Input
              placeholder="Search by title"
              className="pl-2 h-8"
              {...register("title")}
            />
            <$Button className="ml-4 w-24">Search</$Button>
          </form>
          <Link href="/create-post">
            <$Button className="w-32">Create post</$Button>
          </Link>
        </div>
        <$PostsArea>
          {posts.map((post) => (
            <Post post={post} />
          ))}
        </$PostsArea>
        <p className="text-white w-full bg-gray-900 mt-4">
          pagination goes here
        </p>
      </$InnerContainer>
    </div>
  );
};

export default Blog;
