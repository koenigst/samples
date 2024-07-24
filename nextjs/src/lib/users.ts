export type User = {
	id: number
	first_name: string
}

export type Users = {
	myself: User
	friends: User[]
}

export async function loadUser(index: number) {
	const response = await fetch(`https://reqres.in/api/users/${index}`)
	const content = await response.json()
	return <User>content.data
}

export async function loadUsers() {
	const response = await fetch('https://reqres.in/api/users?delay=1')
	const content = await response.json()
	const [myself, ...friends] = <User[]>content.data
	return <Users>{
		myself,
		friends,
	}
}
