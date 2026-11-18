import type { ReactNode } from "react";
import styles from "./PageHeader.module.css";

type PageHeaderProps = {
  title: string;
  description: ReactNode;
  sources?: string[];
};

export default function PageHeader({ title, description, sources = [] }: PageHeaderProps) {
  return (
    <header className={styles.header}>
      <h1 className={styles.title}>{title}</h1>
      <p className={styles.description}>{description}</p>
      {sources.length > 0 && (
        <p className={styles.sources}>
          源码:
          {sources.map((source) => (
            <code key={source}>{source}</code>
          ))}
        </p>
      )}
    </header>
  );
}
