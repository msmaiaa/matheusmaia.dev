import { useGetBlogPostsQuery } from "api/post";
import React, { useEffect } from "react";
import tw from "twin.macro";
import styled from "styled-components";
import { BlogPost } from "types";
import { format } from "date-fns";

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

const Blog = () => {
  const { data: posts } = useGetBlogPostsQuery();
  useEffect(() => {
    console.log(posts);
  }, [posts]);
  return (
    <div className="flex justify-center">
      <$InnerContainer>
        <$PostsArea>
          {posts?.data.map((post) => (
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
