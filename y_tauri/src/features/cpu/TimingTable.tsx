import type { Timing } from "./api.ts";
import styles from "./TimingTable.module.css";

export default function TimingTable({ timings }: { timings: Timing[] }) {
  const baseline = timings[0]?.ms ?? 0;
  const slowest = Math.max(...timings.map((timing) => timing.ms));

  return (
    <table className="table">
      <thead>
        <tr>
          <th>方式</th>
          <th>耗时</th>
          <th className="num">加速比</th>
        </tr>
      </thead>
      <tbody>
        {timings.map((timing) => (
          <tr key={timing.label}>
            <td>{timing.label}</td>
            <td>
              <div className={styles.cell}>
                <div className={styles.track}>
                  <div className={styles.bar} style={{ width: `${(timing.ms / slowest) * 100}%` }} />
                </div>
                <span className={styles.ms}>{formatMs(timing.ms)}</span>
              </div>
            </td>
            <td className="num">{(baseline / timing.ms).toFixed(1)}x</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function formatMs(ms: number): string {
  return ms < 10 ? `${ms.toFixed(1)} ms` : `${Math.round(ms).toLocaleString()} ms`;
}
