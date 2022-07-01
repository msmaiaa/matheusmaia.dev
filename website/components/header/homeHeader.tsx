import tw from "twin.macro";

const $Name = tw.h1`
	text-gray-200
	text-2xl
`;

const $Description = tw.h4`
	text-gray-500
	text-base
`;

const $Container = tw.div`
	flex
	flex-col
	mb-6
	items-center
`;

export const HomeHeader = () => {
  return (
    <$Container>
      <$Name>Matheus Maia</$Name>
      <$Description>Software developer</$Description>
    </$Container>
  );
};
