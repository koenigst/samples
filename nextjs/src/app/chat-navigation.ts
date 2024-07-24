import { getChats, loadUsers } from '@/lib'

export type LinkData = {
	href: string
	name: string
}

export async function loadChatNavigation() {
	const chatsPromise = getChats()
	const usersPromise = loadUsers()

	const chats = await chatsPromise
	const users = await usersPromise

	return chats
		.map((c) => users.friends.find((f) => f.id === c) ?? { id: c, first_name: undefined })
		.map((u) => <LinkData>{ href: `/chats/${u.id}`, name: u.first_name ?? `chat ${u.id}` })
}
