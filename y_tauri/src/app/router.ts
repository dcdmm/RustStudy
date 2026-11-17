import { createBrowserRouter, redirect } from "react-router";
import MainLayout from "./layout/MainLayout.tsx";
import { routeGroups } from "./routes.ts";

// 所有页面都嵌套在MainLayout中,渲染到MainLayout的<Outlet />位置
export const router = createBrowserRouter([
  {
    path: "/",
    Component: MainLayout,
    children: [
      ...routeGroups
        .flatMap((group) => group.routes)
        .map(({ path, Component }) => (path === "/" ? { index: true, Component } : { path, Component })),
      // 未知路径回到首页
      { path: "*", loader: () => redirect("/") },
    ],
  },
]);
