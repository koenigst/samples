import { loadUsers } from '$lib'
import type { PageLoadEvent } from './$types'

export function load(event: PageLoadEvent) {
	return {
		users: loadUsers(event.fetch),
	}
}
