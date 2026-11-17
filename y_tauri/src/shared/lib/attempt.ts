// 把Promise转换为不会reject的结果对象: 成功时{ ok: true, data },失败时{ ok: false, error }
// 适合作为useActionState的返回值,在界面中根据ok分别显示结果或错误
export type Attempt<T> = { ok: true; data: T } | { ok: false; error: string };

export async function attempt<T>(promise: Promise<T>): Promise<Attempt<T>> {
  try {
    return { ok: true, data: await promise };
  } catch (err) {
    return { ok: false, error: String(err) };
  }
}
