'use server'

import { delay } from '@/lib/utils'

const chats = new Map<number, string[]>()

export async function ensureChat(userIndex: number) {
	if (!chats.get(userIndex)) {
		chats.set(userIndex, [])
	}

	await delay<void>(undefined, 200)
}

export async function appendChatMessage(userIndex: number, message: string) {
	chats.get(userIndex)?.push(message)
	return await getChat(userIndex)
}

export async function getChat(userIndex: number) {
	const messages = chats.get(userIndex) ?? []

	const lastMessages = messages.length < 10 ? messages : messages.slice(-10)

	return await delay(lastMessages, 200)
}

export async function getChats() {
	return await delay([...chats.keys()], 200)
}
