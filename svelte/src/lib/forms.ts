import type { SubmitEvent, SubmittedEvent } from '$lib/extracted-types'
import type { MaybePromise } from '$lib/utils'
import { derived, writable } from 'svelte/store'

export class FormEnhancer {
	private submittingWritable = writable(false)
	public submitting = derived(this.submittingWritable, ($s) => $s)

	constructor(
		private formAccessor: () => HTMLFormElement,
		private callbacks: {
			submit?: (event: SubmitEvent) => MaybePromise<void>
			submitted?: (event: SubmittedEvent) => MaybePromise<void>
		},
	) {}

	public async submit(event: SubmitEvent) {
		this.submittingWritable.set(true)
		if (this.callbacks.submit) {
			await this.callbacks.submit(event)
		}
		this.formAccessor().reset()

		return (e: SubmittedEvent) => this.submitted(e)
	}

	private async submitted(event: SubmittedEvent) {
		try {
			if (this.callbacks.submitted) {
				await this.callbacks.submitted(event)
			}
			await event.update({
				reset: false,
				invalidateAll: event.result.type !== 'success',
			})
		} finally {
			this.submittingWritable.set(false)
		}
	}
}
