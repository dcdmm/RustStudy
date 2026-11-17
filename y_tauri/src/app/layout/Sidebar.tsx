import { NavLink } from "react-router";
import { routeGroups } from "../routes.ts";
import styles from "./Sidebar.module.css";

// 左侧菜单,根据app/routes.ts中的配置生成
export default function Sidebar() {
  return (
    <nav className={styles.sidebar}>
      <div className={styles.brand}>Tauri 2 学习案例</div>
      {routeGroups.map((group) => (
        <div key={group.title} className={styles.group}>
          <div className={styles.groupTitle}>{group.title}</div>
          {group.routes.map(({ path, label }) => (
            // NavLink: 当前路由匹配时isActive为true,用来高亮菜单项
            // end: 精确匹配,否则"/"会匹配所有路径
            <NavLink
              key={path}
              to={path}
              end
              className={({ isActive }) => (isActive ? `${styles.link} ${styles.active}` : styles.link)}
            >
              {label}
            </NavLink>
          ))}
        </div>
      ))}
    </nav>
  );
}
