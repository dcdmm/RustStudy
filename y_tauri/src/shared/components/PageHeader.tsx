import type { ReactNode } from "react";
import styles from "./PageHeader.module.css";

type PageHeaderProps = {
  title: string;
  description: ReactNode;
};

export default function PageHeader({ title, description }: PageHeaderProps) {
  return (
    <header className={styles.header}>
      <h1 className={styles.title}>{title}</h1>
      <p className={styles.description}>{description}</p>
    </header>
  );
}
