import { delay } from '@/lib/utils'

const chats = new Map<number, string[]>()

export function ensureChat(userIndex: number) {
	if (!chats.get(userIndex)) {
		chats.set(userIndex, [])
	}

	return delay<void>(undefined, 200)
}

export function appendChatMessage(userIndex: number, message: string) {
	chats.get(userIndex)?.push(message)
	return delay<void>(undefined, 200)
}

export function getChat(userIndex: number) {
	const messages = chats.get(userIndex) ?? []

	const lastMessages = messages.length < 10 ? messages : messages.slice(-10)

	return delay(lastMessages, 200)
}

export function getChats() {
	return delay([...chats.keys()], 200)
}
