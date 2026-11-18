export type Attempt<T> = { ok: true; data: T } | { ok: false; error: string };

export async function attempt<T>(promise: Promise<T>): Promise<Attempt<T>> {
  try {
    return { ok: true, data: await promise };
  } catch (err) {
    return { ok: false, error: String(err) };
  }
}
