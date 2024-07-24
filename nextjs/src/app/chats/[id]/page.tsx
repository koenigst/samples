import { getChat, loadUser } from "@/lib"
import { Messages } from "./messages"

export default async function Chat({ params: { id } }: { params: { id: number } }) {
	const [friend, chat] = await Promise.all([(loadUser(id)), (getChat(id))])

	return (
		<main>
			<h1>Chat with {friend.first_name}</h1>

			<Messages id={id} initial={chat} />
		</main>
	)
}
