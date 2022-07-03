import { HomeHeader } from "components/header/homeHeader";
import { Socials } from "components/socials";
import type { NextPage } from "next";
import Link from "next/link";
import tw from "twin.macro";

const $Container = tw.div`
	flex
	flex-col
	items-center
`;

const Home: NextPage = () => {
  return (
    <$Container>
      <HomeHeader />
      <Socials />
    </$Container>
  );
};

export default Home;
