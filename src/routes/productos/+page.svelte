<script lang="ts">
	import ModalNewProductForm from '$lib/components/ModalNewProductForm.svelte';
	import type { Product } from '$lib/interfaces/product';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { onDestroy } from 'svelte';
	import { List } from 'svelte-virtual';
	import { fade, fly, slide } from 'svelte/transition';
	import MinusIcon from '~icons/memory/minus';
	import PlusIcon from '~icons/memory/plus';

	let products: Product[] = [];
	let filteredProducts: Product[] = [];
	let product: Product | null = null;
	let selectedIndex: number = -1;

	let query = '';

	const fetchProducts = async () =>
		await invoke<Product[]>('get_products').then((result) => (products = result));

	const select = (index: number) => {
		product = filteredProducts[index];
		selectedIndex = index;
	};

	const save = async (e: any) => {
		if (!product) {
			return;
		}

		const formData = new FormData(e.target);

		const { name, description, barcode, cost, price, stock } = Object.fromEntries(formData);

		invoke('save_product', {
			product: {
				...product,
				name,
				description,
				barcode,
				cost: Number(cost),
				price: Number(price),
				stock: Number(stock)
			}
		}).then(() => fetchProducts());
	};

	const remove = () => {};

	const setNumberValue = (e: any) => {
		if (isNaN(e.target.value)) {
			e.target.value = 0;
		}

		e.target.value = parseFloat(e.target.value || 0).toFixed(2) || 0;
	};

	fetchProducts();

	const modalStore = getModalStore();

	const newProduct = () => {
		new Promise<boolean>((resolve) => {
			const modal: ModalSettings = {
				type: 'component',
				title: 'Nuevo producto',
				component: {
					ref: ModalNewProductForm
				},
				response: (r: boolean) => {
					resolve(r);
				}
			};
			modalStore.trigger(modal);
		}).then(async (response: any) => {
			if (response) {
				const { name, description, barcode, cost, price, stock } = response;

				await invoke('create_product', {
					product: { name, description, barcode, cost, price, stock }
				});

				fetchProducts();
			}
		});
	};

	$: {
		filteredProducts = products.filter(
			(product) =>
				product.barcode.toLowerCase().includes(query.toLowerCase()) ||
				product.name.toLowerCase().includes(query.toLowerCase())
		);
	}
</script>

<div class="flex gap-x-8">
	<div class="space-y-2">
		<div class="flex gap-x-2">
			<input class="input w-full" bind:value={query} />
			<button class="btn px-4 variant-ghost flex gap-x-0.5" on:click={newProduct}>
				<PlusIcon /> Nuevo producto
			</button>
		</div>

		<div class="border border-surface-700 rounded overflow-hidden h-[80vh] w-[36rem]">
			<List itemCount={filteredProducts.length} itemSize={32}>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					slot="item"
					let:index
					let:style
					{style}
					class="item flex items-center group hover:bg-primary-300 cursor-pointer"
					class:selected={selectedIndex === index}
					on:click={() => select(index)}
				>
					<span
						class="barcode h-full w-40 flex items-center justify-center font-medium bg-primary-200 border-e border-e-surface-700"
					>
						{filteredProducts[index].barcode}
					</span>
					<span class="ps-2 font-medium">
						{filteredProducts[index].name}
					</span>
				</div>
			</List>
		</div>
	</div>

	{#if product !== null}
		<div in:fly={{ x: -80, opacity: -1, duration: 150 }} out:fade={{ duration: 75 }}>
			<form on:submit|preventDefault={save} class="space-y-2">
				<div class="space-y-2 w-[32rem] card h-fit p-4">
					<label class="label">
						<span> Código de barras </span>
						<input class="input" name="barcode" value={product.barcode} />
					</label>

					<label class="label">
						<span>Nombre</span>
						<input class="input" name="name" value={product.name} />
					</label>

					<label class="label">
						<span>Descripción</span>
						<input class="input" name="description" value={product.description} />
					</label>

					<div class="flex gap-x-3">
						<!-- svelte-ignore a11y-label-has-associated-control -->
						<label class="label">
							<span>Precio de costo</span>
							<div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
								<div class="input-group-shim">$</div>
								<input
									class="w-full"
									name="cost"
									value={product.cost.toFixed(2)}
									on:change={setNumberValue}
								/>
							</div>
						</label>

						<!-- svelte-ignore a11y-label-has-associated-control -->
						<label class="label">
							<span>Precio de venta</span>
							<div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
								<div class="input-group-shim">$</div>
								<input
									class="w-full"
									name="price"
									value={product.price.toFixed(2)}
									on:change={setNumberValue}
								/>
							</div>
						</label>
					</div>

					<label class="label">
						<span>Stock</span>
						<div class="flex gap-x-1 items-center">
							<input
								class="input block w-24"
								type="number"
								name="stock"
								min="0"
								bind:value={product.stock}
							/>
							<div class="flex flex-col gap-0.5">
								<button
									class="h-5 w-5 p-0 btn variant-filled-success border border-success-700"
									on:click={() => product.stock++}
								>
									<PlusIcon class="text-sm" />
								</button>
								<button
									class="h-5 w-5 p-0 btn variant-filled-error border border-error-700"
									disabled={product.stock <= 0}
									on:click={() => product.stock--}
								>
									<MinusIcon class="text-sm" />
								</button>
							</div>
						</div>
					</label>
				</div>
				<div class="w-full flex justify-between text-sm">
					<button class="btn variant-ghost" on:click={(product = null)}> Cerrar </button>
					<div>
						<button class="ks-btn ks-btn-success" type="submit"> Guardar </button>
						<button class="ks-btn ks-btn-error variant-filled-error" type="button">
							Eliminar
						</button>
					</div>
				</div>
			</form>
		</div>
	{/if}
</div>

<style lang="postcss">
	.barcode {
		@apply group-hover:bg-primary-300;
	}

	.selected {
		@apply bg-primary-300;
	}
</style>
