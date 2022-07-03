import Link from "next/link";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import tw from "twin.macro";
import { z } from "zod";
import { $Button } from "components/ui/button";
import { $Input } from "components/ui/input";
import { LoginInput } from "types";
import { useLoginMutation } from "api/auth";
import { useStore } from "store";
import { useAuth } from "hooks/auth";
import { useEffect } from "react";
import { useRouter } from "next/router";

const $FormInput = tw($Input)`
	w-2/3
	pl-4
	h-12
`;

const $Form = tw.form`
	flex	
	flex-col
	w-96
	border
	rounded-md
	border-border_primary
`;

const $ButtonArea = tw.div`
	flex
	mt-8
	mb-4
	justify-around
`;

const $InputArea = tw.div`
	mt-4
	flex
	flex-col
	items-center
`;

const loginSchema = z.object({
  username: z.string(),
  password: z.string(),
});

export const LoginForm = () => {
  const { loggedIn, isLoading } = useAuth();
  const router = useRouter();
  const { handleSubmit, register } = useForm<LoginInput>({
    resolver: zodResolver(loginSchema),
  });
  const [login] = useLoginMutation();
  const { setUser, setLoggedIn } = useStore((state) => ({
    setUser: state.setUser,
    setLoggedIn: state.setLoggedIn,
  }));

  useEffect(() => {
    if (!isLoading && loggedIn) {
      if (loggedIn) {
        router.push("/");
      }
    }
  }, [loggedIn, isLoading]);

  const onSubmit = (data: LoginInput) => {
    login(data)
      .unwrap()
      .then((data) => {
        setUser(data);
        setLoggedIn(true);
        localStorage.setItem("token", data.token);
      })
      .catch((error) => console.error(error));
  };

  return (
    <$Form onSubmit={handleSubmit(onSubmit)} autoComplete="off">
      <$InputArea>
        <$FormInput
          type="text"
          placeholder="username"
          {...register("username")}
          tw="mb-4"
        />
        <$FormInput
          type="text"
          placeholder="password"
          {...register("password")}
        />
      </$InputArea>
      <$ButtonArea>
        <$Button type="submit" tw="text-white">
          login
        </$Button>
        <Link href="/">
          <$Button>go back</$Button>
        </Link>
      </$ButtonArea>
    </$Form>
  );
};
