<script>
	const { data, children } = $props()
</script>

<nav>
	{#each data.links as link}
		<a href={link.href}>{link.name}</a>
	{/each}
</nav>

{@render children()}
