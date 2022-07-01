import { Socials } from "components/socials";
import type { NextPage } from "next";
import tw from "twin.macro";

const Container = tw.div`
	flex
	flex-col
`;

const Home: NextPage = () => {
  return (
    <Container>
      <Socials />
    </Container>
  );
};

export default Home;
