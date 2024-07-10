import type { RequestEvent, SubmitFunction } from '@sveltejs/kit'

export type FetchFunction = RequestEvent['fetch']

export type SubmitEvent = Parameters<SubmitFunction>[0]
export type SubmittedEvent = Parameters<Exclude<Awaited<ReturnType<SubmitFunction>>, void>>[0]
