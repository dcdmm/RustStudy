import { Outlet } from "react-router";
import Sidebar from "./Sidebar.tsx";
import StatusBar from "./StatusBar.tsx";
import styles from "./MainLayout.module.css";

// 主布局: 左侧菜单 + 右侧内容区 + 底部状态栏
// 切换页面时只有<Outlet />处的内容会变,菜单和状态栏保持不动
export default function MainLayout() {
  return (
    <div className={styles.layout}>
      <Sidebar />
      <main className={styles.content}>
        <div className={styles.inner}>
          <Outlet />
        </div>
      </main>
      <StatusBar />
    </div>
  );
}
