'use server'

import { appendChatMessage, ensureChat } from "@/lib";

export async function send(id: number, message: string) {
	await ensureChat(id)
	return await appendChatMessage(id, message)
}
