<script lang="ts">
	import { page } from '$app/stores';
	import '../theme.postcss';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';
	
</script>

<nav class="bg-primary-200 border-b border-b-surface-700 shadow-sm flex">
	<a href="/" class:active={$page.route.id === '/'}> Venta </a>
	<a href="/productos" class:active={$page.route.id === '/productos'}> Listado de productos </a>
	<a href="/facturas" class:active={$page.route.id === '/facturas'}> Facturas </a>
</nav>

<div class="container mx-auto p-4">
	<slot />
</div>

<style lang="postcss">
	nav a {
		@apply text-lg font-medium uppercase px-3.5 py-0.5 border-b-transparent border-b-2 hover:border-b-surface-700;
	}

	a:active,
	a.active {
		@apply bg-primary-300 border-b-primary-700;
	}
</style>
