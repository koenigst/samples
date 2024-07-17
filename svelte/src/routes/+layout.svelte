<script>
	export let data
</script>

<nav>
	{#each data.links as link}
		<a href={link.href}>{link.name}</a>
	{/each}
</nav>

<slot></slot>
