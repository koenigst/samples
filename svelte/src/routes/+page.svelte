<script lang="ts">
	const { data } = $props()
</script>

{#await data.users}
	<span>Loading...</span>
{:then users}
	<h1>Hello {users.myself.first_name}!</h1>
	<ul>
		{#each users.friends as user}
			<li>
				<a href="/chats/{user.id}">{user.first_name}</a>
			</li>
		{/each}
	</ul>
{/await}
