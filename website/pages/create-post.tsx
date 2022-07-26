import tw from "twin.macro";
import { $Input } from "components/ui/input";
import { $Button } from "components/ui/button";
import { useGetTagsQuery } from "api/tags";
import Select, { StylesConfig } from "react-select";
import { Controller, useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import dynamic from "next/dynamic";
import "react-markdown-editor-lite/lib/index.css";
import MarkdownIt from "markdown-it";
import { useCreatePostMutation } from "api/post";
import { CreatePostInput } from "types";

const mdParser = new MarkdownIt();
const MdEditor = dynamic(() => import("react-markdown-editor-lite"), {
  ssr: false,
});

const $InnerContainer = tw.div`
	flex
	w-1/2
`;

const customSelectStyles: StylesConfig = {
  control: (base) => ({
    ...base,
    backgroundColor: "#0d1117",
    border: "2px solid #30363d",
  }),
  menu: (provided) => ({
    ...provided,
    backgroundColor: "#0d1117",
  }),
  option: (base) => ({
    ...base,
    backgroundColor: "#0d1117",
    color: "#c9d1d9",
    "&:hover": {
      backgroundColor: "rgb(40, 46, 51)",
    },
  }),
};

const createPostSchema = z.object({
  title: z.string(),
  tags: z
    .array(
      z.object({
        value: z.number(),
        label: z.string(),
      })
    )
    .optional(),
  published: z.boolean().default(true),
  content: z.string(),
});

type CreatePostSchema = z.infer<typeof createPostSchema>;

const CreatePost = () => {
  const { data: tags } = useGetTagsQuery();
  const [createPost] = useCreatePostMutation();
  const { register, control, watch, setValue, handleSubmit } =
    useForm<CreatePostSchema>({
      resolver: zodResolver(createPostSchema),
    });

  const formTags = watch("tags");
  const formContent = watch("content");

  const handleMdChange = ({ text }: any) => {
    setValue("content", text);
  };

  const onSubmit = async (data: CreatePostSchema) => {
    let payload: CreatePostInput = {
      content: data.content,
      title: data.title,
    };
    if (data.tags) {
      payload.tags = data.tags.map((tag) => tag.value);
    }
    try {
      await createPost(payload).unwrap();

      //	TODO: use a toast library?
      alert("created!");
    } catch (e) {
      alert(e);
    }
  };

  return (
    <div className="flex justify-center">
      <$InnerContainer>
        <form
          className="flex flex-col w-full"
          onSubmit={handleSubmit(onSubmit)}
        >
          <$Input placeholder="Title" tw="h-9 pl-2" {...register("title")} />
          <Controller
            control={control}
            name="tags"
            render={({ field: { onChange, ref } }) => (
              <Select
                placeholder="Select the tags"
                ref={ref}
                value={formTags}
                onChange={(val) => onChange(val)}
                className="mt-2"
                styles={customSelectStyles}
                isMulti
                options={tags?.map((tag) => {
                  return {
                    value: tag.id,
                    label: tag.name,
                  };
                })}
              />
            )}
          />
          <MdEditor
            style={{ height: "500px" }}
            className="mt-4 bg-bg_primary"
            value={formContent}
            renderHTML={(text) => mdParser.render(text)}
            onChange={handleMdChange}
          />
          <$Button className="mt-4" type="submit">
            Create post
          </$Button>
        </form>
      </$InnerContainer>
    </div>
  );
};

export default CreatePost;
