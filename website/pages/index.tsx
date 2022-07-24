import { HomeHeader } from "components/header/homeHeader";
import { Socials } from "components/socials";
import { $Button } from "components/ui/button";
import { useAuth } from "hooks/auth";
import type { NextPage } from "next";
import Link from "next/link";
import tw from "twin.macro";

const $Container = tw.div`
	flex
	flex-col
	items-center
`;

const Home: NextPage = () => {
  const { isLoggedIn } = useAuth();

  return (
    <>
      <$Container>
        <HomeHeader />
        <Socials />
      </$Container>
      {isLoggedIn && (
        <div className="flex items-center mt-24 flex-col">
          <p className="text-white">hello, Matheus</p>
          <Link href="/blog">
            <$Button tw="w-48 mt-2">blog posts here</$Button>
          </Link>
        </div>
      )}
    </>
  );
};

export default Home;
