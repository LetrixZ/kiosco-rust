<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	// import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	import '../theme.postcss';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';

	import { page } from '$app/stores';
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
