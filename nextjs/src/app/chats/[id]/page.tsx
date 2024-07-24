import { getChat, loadUser } from "@/lib"
import { FormEvent, useState } from "react"

'use client'

enum Status {
	Empty,
	Editing,
	Submitting
}

function Messages({ initial }: { initial: string[] }) {

	const [submitted, setSubmitted] = useState<string[]>([])
	const [edited, setEdited] = useState('')
	const [status, setStatus] = useState(Status.Empty)

	async function handleSubmit(e: FormEvent) {

	}

	async function handleMessageChange(e: FormEvent) {

	}

	const messages = [...initial, ...submitted]

	return (
		<>
			{messages.map(message => <p>{message}</p>)}

			<form method="POST" onSubmit={handleSubmit}>
				<input name="message" type="text" autoFocus value={edited} onChange={handleMessageChange} />
				<button disabled={status === Status.Submitting}>Send</button>
			</form>
		</>
	)
}

export default async function Chat({ id }: { id: number }) {
	const [friend, chat] = await Promise.all([(loadUser(id)), (getChat(id))])

	return (
		<main>
			<h1>Chat with {friend.first_name}</h1>

			<Messages initial={chat} />
		</main>
	)
}
