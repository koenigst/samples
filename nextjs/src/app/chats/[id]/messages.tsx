'use client'

import { useOptimistic } from "react";
import { useFormStatus } from "react-dom";
import { send } from "./actions";

export function Messages({ id, initial }: { id: number, initial: string[]; }) {

	const [messages, addMessage] = useOptimistic<string[], string>(initial, (all, added) => [...all, added])
	const { pending } = useFormStatus()

	async function handleSubmit(data: FormData) {
		const message = data.get('message')
		if (typeof message === 'string') {
			addMessage(message)
			await send(id, message)
		}
	}

	return (
		<>
			{messages.map((message, key) => <p key={key}>{message}</p>)}

			<form action={handleSubmit}>
				<input name="message" type="text" autoFocus />
				<button disabled={pending}>Send</button>
			</form>
		</>
	);
}
