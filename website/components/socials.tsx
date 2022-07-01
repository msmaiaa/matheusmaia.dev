import tw from "twin.macro";
import { LinkedinIcon } from "./icons/linkedin";
import React from "react";
import { IconComponent } from "types/icon";
import { GithubIcon } from "./icons/github";
import { SteamIcon } from "./icons/steam";

type SocialMediaButtonProps = {
  name: string;
  icon: IconComponent;
  url: string;
};

const socials: SocialMediaButtonProps[] = [
  {
    name: "Linkedin",
    icon: LinkedinIcon,
    url: "https://www.linkedin.com/in/matheus-santos-488093178/",
  },
  {
    name: "Github",
    icon: GithubIcon,
    url: "https://www.github.com/msmaiaa",
  },
  {
    name: "Steam",
    icon: SteamIcon,
    url: "https://steamcommunity.com/profiles/76561197993987883/",
  },
];

const SocialMediaIcon = ({ name, icon, url }: SocialMediaButtonProps) => {
  let $StyledIcon = tw(icon)`
		hover:cursor-pointer
		not-last:mr-2
		mb-2
		w-6
		h-6
	`;
  return (
    <$StyledIcon
      aria-label={name}
      onClick={() => window.open(url, "_blank")}
      fill="#fff"
    />
  );
};

const $Socials = tw.div`
	flex
	flex-wrap
	justify-center
	w-80
`;
export const Socials = () => {
  return (
    <$Socials>
      {socials.map((social) => (
        <SocialMediaIcon {...social} key={social.name} />
      ))}
    </$Socials>
  );
};
