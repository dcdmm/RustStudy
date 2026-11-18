import { createBrowserRouter, redirect } from "react-router";
import MainLayout from "./layout/MainLayout.tsx";
import { routeGroups } from "./routes.ts";

export const router = createBrowserRouter([
  {
    path: "/",
    Component: MainLayout,
    children: [
      ...routeGroups
        .flatMap((group) => group.routes)
        .map(({ path, Component }) => (path === "/" ? { index: true, Component } : { path, Component })),
      { path: "*", loader: () => redirect("/") },
    ],
  },
]);
