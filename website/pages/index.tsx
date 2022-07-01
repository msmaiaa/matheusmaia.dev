import type { NextPage } from "next";
import tw from "twin.macro";

const Container = tw.div`
	flex
	flex-col
`;

const Home: NextPage = () => {
  return (
    <Container>
      <div>aaaa</div>
      <div>aaaa</div>
      <div>aaaa</div>
      <div>aaaa</div>
    </Container>
  );
};

export default Home;
