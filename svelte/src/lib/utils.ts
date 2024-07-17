export type MaybePromise<T> = T | Promise<T>

export function delay<T>(value: T, milliseconds: number) {
	return new Promise<T>((resolve) => setTimeout(() => resolve(value), milliseconds))
}
