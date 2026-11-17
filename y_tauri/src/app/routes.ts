import type { ComponentType } from "react";
import HomePage from "@/features/home/HomePage.tsx";
import CommandPage from "@/features/command/CommandPage.tsx";
import ErrorHandlingPage from "@/features/error-handling/ErrorHandlingPage.tsx";
import StatePage from "@/features/state/StatePage.tsx";
import ChannelPage from "@/features/channel/ChannelPage.tsx";
import EventPage from "@/features/event/EventPage.tsx";
import WindowPage from "@/features/window/WindowPage.tsx";
import PluginPage from "@/features/plugin/PluginPage.tsx";

export type PageRoute = {
  path: string;
  // 侧边栏菜单中显示的名字
  label: string;
  Component: ComponentType;
};

export type RouteGroup = {
  title: string;
  routes: PageRoute[];
};

// 页面配置: 路由(router.ts)和左侧菜单(layout/Sidebar.tsx)都根据它生成
// 新增页面只需要在这里加一项
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
    ],
  },
];
