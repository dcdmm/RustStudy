import type { ReactNode } from "react";
import styles from "./PageHeader.module.css";

type PageHeaderProps = {
  title: string;
  description: ReactNode;
  // 本页面对应的源码文件,学习时对照着看
  sources?: string[];
};

// 页面顶部: 标题 + 说明 + 对应的源码文件
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
