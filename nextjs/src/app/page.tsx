import { loadUsers } from "@/lib"
import Link from "next/link"

export default async function Home() {
	var users = await loadUsers()

	return (
		<main>
			<h1>Hello {users.myself.first_name}!</h1>
			<ul>
				{users.friends.map(user =>
					<li key={user.id}>
						<Link href={`/chats/${user.id}`}>{user.first_name}</Link>
					</li>)}
			</ul>
		</main>
	)
}
