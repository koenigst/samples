import { chatsDependencyKey, getChats } from '$lib/server'
import { loadUsers } from '$lib'
import type { LayoutServerLoadEvent } from './$types'

export type LinkData = {
	href: string
	name: string
}

export async function load({ depends, fetch }: LayoutServerLoadEvent) {
	depends(chatsDependencyKey)

	const chatsPromise = getChats()
	const usersPromise = loadUsers(fetch)

	const chats = await chatsPromise
	const users = await usersPromise

	const mappedChats = chats
		.map((c) => users.friends.find((f) => f.id === c) ?? { id: c, first_name: undefined })
		.map((u) => <LinkData>{ href: `/chats/${u.id}`, name: u.first_name ?? `chat ${u.id}` })

	return {
		links: <LinkData[]>[{ href: '/', name: 'friends' }, ...mappedChats],
	}
}
