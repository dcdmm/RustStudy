import { Outlet } from "react-router";
import Sidebar from "./Sidebar.tsx";
import StatusBar from "./StatusBar.tsx";
import styles from "./MainLayout.module.css";

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
