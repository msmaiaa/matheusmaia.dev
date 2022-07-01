import { ReactNode } from "react";
import tw from "twin.macro";

const $PageWrapper = tw.div`
	min-h-screen
	bg-bg_primary
	pt-16
`;

const $PageLayout = tw.div`
	flex
	flex-col
	items-center
`;

type Props = {
  children: ReactNode;
};

export const Layout = ({ children }: Props) => (
  <$PageWrapper>
    <$PageLayout>
      <main>{children}</main>
    </$PageLayout>
  </$PageWrapper>
);
