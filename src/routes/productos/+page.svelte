<script lang="ts">
	import ModalNewProductForm from '$lib/components/ModalNewProductForm.svelte';
	import type { Product } from '$lib/interfaces/product';
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { List } from 'svelte-virtual';
	import MinusIcon from '~icons/memory/minus';
	import PlusIcon from '~icons/memory/plus';

	let products: Product[] = [];
	let selectedProduct: Product | null = null;
	let selectedIndex: number = -1;

	const fetchProducts = async () =>
		await invoke<Product[]>('get_products').then((result) => (products = result));

	const selectProduct = (index: number) => {
		selectedProduct = products[index];
		selectedIndex = index;
	};

	const save = async (e: any) => {
		if (!selectedProduct) {
			return;
		}

		const formData = new FormData(e.target);

		const { name, description, barcode, cost, price, stock } = Object.fromEntries(formData);

		invoke('save_product', {
			product: {
				...selectedProduct,
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
</script>

<div class="flex gap-x-8">
	<div class="space-y-2">
		<button class="btn w-full text-xl variant-ghost flex gap-x-2" on:click={newProduct}>
			<PlusIcon /> Nuevo producto
		</button>
		<div class="border border-surface-700 rounded overflow-hidden h-[80vh] w-[36rem]">
			<List itemCount={products.length} itemSize={32}>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					slot="item"
					let:index
					let:style
					{style}
					class="item flex items-center group hover:bg-primary-300 cursor-pointer"
					class:selected={selectedIndex === index}
					on:click={() => selectProduct(index)}
				>
					<span
						class="barcode h-full w-40 flex items-center justify-center font-medium bg-primary-200 border-e border-e-surface-700"
					>
						{products[index].barcode}
					</span>
					<span class="ps-2 font-medium">
						{products[index].name}
					</span>
				</div>
			</List>
		</div>
	</div>

	{#if selectedProduct !== null}
		<div>
			<form on:submit|preventDefault={save} class="space-y-2">
				<div class="space-y-2 w-[32rem] card h-fit p-4">
					<label class="label">
						<span> Código de barras </span>
						<input class="input" name="barcode" value={selectedProduct.barcode} />
					</label>

					<label class="label">
						<span>Nombre</span>
						<input class="input" name="name" value={selectedProduct.name} />
					</label>

					<label class="label">
						<span>Descripción</span>
						<input class="input" name="description" value={selectedProduct.description} />
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
									value={selectedProduct.cost.toFixed(2)}
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
									value={selectedProduct.price.toFixed(2)}
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
								bind:value={selectedProduct.stock}
							/>
							<div class="flex flex-col gap-0.5">
								<button
									class="h-5 w-5 p-0 btn variant-filled-success border border-success-700"
									on:click={() => selectedProduct.stock++}
								>
									<PlusIcon class="text-sm" />
								</button>
								<button
									class="h-5 w-5 p-0 btn variant-filled-error border border-error-700"
									disabled={selectedProduct.stock <= 0}
									on:click={() => selectedProduct.stock--}
								>
									<MinusIcon class="text-sm" />
								</button>
							</div>
						</div>
					</label>
				</div>
				<div class="ms-auto w-fit text-sm">
					<button class="ks-btn ks-btn-success" type="submit"> Guardar </button>
					<button class="ks-btn ks-btn-error variant-filled-error" type="button"> Eliminar </button>
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
