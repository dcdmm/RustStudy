import type { ComponentType } from "react";
import HomePage from "@/features/home/HomePage.tsx";
import CommandPage from "@/features/command/CommandPage.tsx";
import ErrorHandlingPage from "@/features/error-handling/ErrorHandlingPage.tsx";
import StatePage from "@/features/state/StatePage.tsx";
import ChannelPage from "@/features/channel/ChannelPage.tsx";
import EventPage from "@/features/event/EventPage.tsx";
import WindowPage from "@/features/window/WindowPage.tsx";
import PluginPage from "@/features/plugin/PluginPage.tsx";
import FilesPage from "@/features/files/FilesPage.tsx";
import CpuPage from "@/features/cpu/CpuPage.tsx";
import IoPage from "@/features/io/IoPage.tsx";

export type PageRoute = {
  path: string;
  label: string;
  Component: ComponentType;
};

export type RouteGroup = {
  title: string;
  routes: PageRoute[];
};

export const routeGroups: RouteGroup[] = [
  {
    title: "开始",
    routes: [{ path: "/", label: "首页", Component: HomePage }],
  },
  {
    title: "基础",
    routes: [
      { path: "/command", label: "1. Command", Component: CommandPage },
      { path: "/error-handling", label: "2. 错误处理", Component: ErrorHandlingPage },
      { path: "/state", label: "3. 状态管理", Component: StatePage },
    ],
  },
  {
    title: "通信",
    routes: [
      { path: "/channel", label: "4. Channel", Component: ChannelPage },
      { path: "/event", label: "5. 事件", Component: EventPage },
    ],
  },
  {
    title: "进阶",
    routes: [
      { path: "/window", label: "6. 窗口与权限", Component: WindowPage },
      { path: "/plugin", label: "7. 插件", Component: PluginPage },
      { path: "/files", label: "8. 文件与数据", Component: FilesPage },
    ],
  },
  {
    title: "性能",
    routes: [
      { path: "/cpu", label: "9. CPU密集: rayon", Component: CpuPage },
      { path: "/io", label: "10. IO密集: tokio", Component: IoPage },
    ],
  },
];
