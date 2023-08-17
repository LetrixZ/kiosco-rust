<script lang="ts">
	// Props
	/** Exposes parent props to this component. */
	export let parent: any;

	// Stores
	import { getModalStore } from '@skeletonlabs/skeleton';
	import MinusIcon from '~icons/memory/minus';
	import PlusIcon from '~icons/memory/plus';
	import RequiredIcon from '~icons/mdi/asterisk';

	let productForm: HTMLFormElement;

	const modalStore = getModalStore();

	// Form Data
	const formData = {
		barcode: '',
		name: '',
		description: '',
		cost: 0,
		price: 0,
		stock: 0
	};

	// We've created a custom submit function to pass the response and close the modal.
	const formSubmit = () => {
		const formData = new FormData(productForm);
		const { name, description, barcode, cost, price, stock } = Object.fromEntries(formData);

		if ($modalStore[0].response) {
			$modalStore[0].response({
				name,
				description,
				barcode,
				cost: Number(cost),
				price: Number(price),
				stock: Number(stock)
			});
		}

		modalStore.close();
	};

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';
	const cForm = 'border border-surface-500 p-4 space-y-2 rounded-container-token';

	const setNumberValue = (e: any) => {
		if (isNaN(e.target.value)) {
			e.target.value = 0;
		}

		e.target.value = parseFloat(e.target.value || 0).toFixed(2) || 0;
	};

	$: valid = formData.barcode.length && formData.name.length;
</script>

<!-- @component This example creates a simple form modal. -->

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
		<header class={cHeader}>{$modalStore[0].title ?? '(title missing)'}</header>
		<form class="modal-form {cForm}" bind:this={productForm}>
			<label class="label">
				<span>Código de barras <RequiredIcon class="inline mb-2.5 text-[8px] text-red-700" /></span>
				<input class="input" name="barcode" type="text" bind:value={formData.barcode} />
			</label>

			<label class="label">
				<span>Nombre <RequiredIcon class="inline mb-2.5 text-[8px] text-red-700" /></span>
				<input class="input" name="name" type="text" bind:value={formData.name} />
			</label>

			<label class="label">
				<span>Descripción</span>
				<input class="input" name="description" type="text" bind:value={formData.description} />
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
							value={formData.cost.toFixed(2)}
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
							value={formData.price.toFixed(2)}
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
						bind:value={formData.stock}
					/>
					<div class="flex flex-col gap-0.5">
						<button
							class="h-5 w-5 p-0 btn variant-filled-success border border-success-700"
							on:click={() => formData.stock++}
						>
							<PlusIcon class="text-sm" />
						</button>
						<button
							class="h-5 w-5 p-0 btn variant-filled-error border border-error-700"
							disabled={formData.stock <= 0}
							on:click={() => formData.stock--}
						>
							<MinusIcon class="text-sm" />
						</button>
					</div>
				</div>
			</label>
		</form>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
        <button class="btn {parent.buttonPositive}" on:click={formSubmit} disabled={!valid}>Crear</button>
    </footer>
	</div>
{/if}
