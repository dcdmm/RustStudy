import type { ReactNode } from "react";
import styles from "./Card.module.css";

type CardProps = {
  title: string;
  children: ReactNode;
};

// 内容卡片,页面中的每个小示例放在一个Card中
export default function Card({ title, children }: CardProps) {
  return (
    <section className={styles.card}>
      <h2 className={styles.title}>{title}</h2>
      {children}
    </section>
  );
}
