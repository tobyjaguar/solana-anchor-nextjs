# Solana NextJS and Anchor Project

## Steps to setup a project manually

### Init project with Anchor

- anchor init <project-name>
- cd into project
- anchor build (check the installation)

### Install NextJS Manually

- run `yarn add next@latest react@latest react-dom@latest`
- add scripts
- add app/layout.tsx
- add app/page.tsx
- linter will complain
- run `yarn dev` (installs typoescript automagically)

### Install Tailwind

- `yarn add tailwindcss @tailwindcss/postcss postcss`
- add postcss.config.mjs
- import global.css into the root layout

### Clone and Run

- clone the repo
- change the project name at: 
  - programs/project-name
  - programs/src/lib.rs
  - programs/Cargo.toml
- run yarn
- run anchor build

