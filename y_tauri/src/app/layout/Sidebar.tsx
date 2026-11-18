import { NavLink } from "react-router";
import { routeGroups } from "../routes.ts";
import styles from "./Sidebar.module.css";

export default function Sidebar() {
  return (
    <nav className={styles.sidebar}>
      <div className={styles.brand}>Tauri 2 学习案例</div>
      {routeGroups.map((group) => (
        <div key={group.title} className={styles.group}>
          <div className={styles.groupTitle}>{group.title}</div>
          {group.routes.map(({ path, label }) => (
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
