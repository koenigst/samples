<script lang="ts">
	import { enhance } from '$app/forms'
	import type { SubmitEvent } from '$lib'
	import { FormEnhancer } from '$lib'

	const { data } = $props()

	let messageForm: HTMLFormElement

	let submittedMessages: string[] = $state([])

	const formEnhancer = new FormEnhancer(() => messageForm, { submit: appendMessage })

	const sending = formEnhancer.submitting

	function appendMessage(event: SubmitEvent) {
		const value = event.formData.get('message')
		const message = typeof value === 'string' ? value : ''
		submittedMessages = [...submittedMessages, message]
	}
</script>

<h1>Chat with {data.friend.first_name}</h1>

{#await data.messages}
	<span>Loading...</span>
{:then messages}
	{#each messages.concat(submittedMessages) as message}
		<p>{message}</p>
	{/each}
{/await}

<form bind:this={messageForm} method="POST" use:enhance={(e) => formEnhancer.submit(e)}>
	<!-- svelte-ignore a11y_autofocus -->
	<input name="message" type="text" autofocus />
	<button disabled={$sending}>Send</button>
</form>
