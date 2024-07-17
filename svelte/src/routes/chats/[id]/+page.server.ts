import { appendChatMessage, ensureChat, getChat } from '$lib/server'
import { loadUser } from '$lib'
import type { Actions, PageServerLoadEvent, RequestEvent } from './$types'

export async function load(event: PageServerLoadEvent) {
	const id = parseInt(event.params.id)
	return {
		friend: await loadUser(event.fetch, id),
		messages: getChat(id),
	}
}

async function send(event: RequestEvent) {
	const id = parseInt(event.params.id)
	const data = await event.request.formData()
	const message = data.get('message')?.valueOf() as string
	await ensureChat(id)
	await appendChatMessage(id, message)
}

export const actions: Actions = {
	default: send,
}
