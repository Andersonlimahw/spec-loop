import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

const base = "/spec-loop";

export default defineConfig({
  site: "https://andersonlimahw.github.io",
  base,
  integrations: [
    starlight({
      title: "SpecLoop",
      description: "Spec-driven QA loops for AI-native teams",
      favicon: "/favicon.svg",
      logo: {
        src: "./src/assets/logo.png",
        alt: "SpecLoop",
      },
      locales: {
        root: {
          label: "English",
          lang: "en",
        },
      },
      customCss: ["./src/styles/starlight.css"],
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/Andersonlimahw/spec-loop",
        },
        {
          icon: "external",
          label: "Lemon",
          href: "https://lemon.dev.br",
        },
      ],
      sidebar: [
        {
          label: "Overview",
          items: [{ label: "Start Here", link: "/" }],
        },
        {
          label: "Part I - Mental Model",
          items: [
            { slug: "01-what-is-specloop", label: "01 - What Is SpecLoop" },
            { slug: "02-core-loop", label: "02 - The Core Loop" },
            { slug: "03-agents-and-roles", label: "03 - Agents And Roles" },
            { slug: "04-browser-and-runners", label: "04 - Browser And Runners" },
          ],
        },
        {
          label: "Part II - Using It",
          items: [
            { slug: "05-specs-and-evals", label: "05 - Specs And Evals" },
            { slug: "06-cli-and-workflows", label: "06 - CLI And Workflows" },
            { slug: "07-examples", label: "07 - Examples" },
            { slug: "08-roadmap-and-open-core", label: "08 - Open-Core Roadmap" },
          ],
        },
      ],
    }),
  ],
});
