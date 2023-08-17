<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import CloseIcon from '~icons/ep/close-bold';
	import MinusIcon from '~icons/memory/minus';
	import PlusIcon from '~icons/memory/plus';

	let lines: any[] = [];
	let query = '';
	let inputQuery: HTMLInputElement;
	let results: any[] = [];

	let showQuantiyModal = false;
	let quantityModalInput: HTMLInputElement | null = null;

	let focusTimeout: any;

	const focus = () => {
		if (focusTimeout) {
			clearTimeout(focusTimeout);
		}

		focusTimeout = setTimeout(() => {
			inputQuery.focus();
		}, 50);
	};

	const decrease = (i: number) => {
		lines[i].unidades--;
		focus();
	};

	const increase = (i: number) => {
		lines[i].unidades++;
		focus();
	};

	const remove = (i: number) => {
		lines = lines.filter((_, idx) => idx !== i);
		focus();
	};

	$: {
		if (query.length > 0) {
			invoke<any[]>('search', { query }).then((result) => (results = result));
		} else {
			results = [];
		}
	}

	const add = (producto: any) => {
		const lineaExistente = lines.find((linea) => linea.producto.name === producto.name);

		if (lineaExistente) {
			lineaExistente.unidades++;
			lines = lines;
		} else {
			lines = [...lines, { producto, unidades: 1 }];
		}

		focus();
		query = '';
	};

	const submitBar = () => {
		if (results.length == 1) {
			add(results[0]);
		}
	};

	const itemClick = (item: any, e: any) => {
		e.target.blur();
		add(item);
	};

	const itemKeyDown = (item: any, e: KeyboardEvent) => {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			add(item);
		}
	};

	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === '*') {
			e.preventDefault();

			if (showQuantiyModal) {
				showQuantiyModal = false;
			} else if (lines.length) {
				showQuantiyModal = true;

				setTimeout(
					() =>
						quantityModalInput &&
						(quantityModalInput.value = lines.at(-1).unidades) &&
						quantityModalInput.select(),
					0
				);
			}
		} else if (e.key === 'Escape') {
			if (showQuantiyModal) {
				showQuantiyModal = false;
			}
		} else if (showQuantiyModal) {
			const value = parseInt(quantityModalInput!.value) || 0;

			if (e.key === '+') {
				e.preventDefault();

				changeQuantity(value + 1);
			} else if (e.key === '-') {
				e.preventDefault();

				if (value > 1) {
					changeQuantity(value - 1);
				}
			}
		} else if (/^[a-z0-9]$/i.test(e.key)) {
			if (showQuantiyModal) {
				quantityModalInput?.focus();
			} else {
				inputQuery.focus();
			}
		}
	};

	const quantitySubmit = (e: any) => {
		const formData = new FormData(e.target);

		const { quantity } = Object.fromEntries(formData);
		const unidades = parseInt(quantity.toString()) || 1;

		if (unidades <= 0) {
			return;
		}

		showQuantiyModal = false;
	};

	const changeQuantity = (value: any) => {
		let input = '1';

		if (typeof value === 'object') {
			input = value.target.value;
		} else {
			input = value;
			quantityModalInput!.value = input.toString();
		}

		const unidades = parseInt(input.toString());

		if (isNaN(unidades) || unidades <= 0) {
			return;
		}

		lines.at(-1) && (lines.at(-1).unidades = unidades);

		lines = [...lines];
	};

	$: {
		if (showQuantiyModal && quantityModalInput) {
			quantityModalInput.focus();
		}
	}

	onMount(() => inputQuery.focus());

	const createInvoice = async () => {
		const invoice = {
			total: lines.reduce((acc, line) => acc + line.producto.price * line.unidades, 0),
			lines: lines.map((line) => ({
				name: line.producto.name,
				quantity: line.unidades,
				price: line.producto.price,
				product: line.producto
			}))
		};

		await invoke('create_invoice', { invoice });

		lines = [];
	};

	invoke('get_invoices').then(console.log);
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="space-y-2">
	{#if showQuantiyModal}
		<form class="p-2 card w-fit" on:submit={quantitySubmit}>
			<label class="label">
				<p class="font-semibold">Unidades de la última línea</p>
				<input
					class="input w-full"
					bind:this={quantityModalInput}
					placeholder="Unidades"
					type="number"
					name="quantity"
					on:input={changeQuantity}
				/>
			</label>
		</form>
	{/if}

	<form class="relative" on:submit|preventDefault={submitBar}>
		<input
			class="input text-3xl"
			bind:this={inputQuery}
			bind:value={query}
			spellcheck="false"
			type="search"
		/>
	</form>

	{#if results.length}
		<div
			in:slide={{ duration: 150 }}
			out:slide={{ duration: 75 }}
			class="my-4 rounded border border-surface-700 overflow-hidden"
		>
			<div class="max-h-96 overflow-y-auto">
				{#each results as item, i}
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
					<div
						tabindex="0"
						class="select-none item text-2xl flex justify-between cursor-pointer px-3 py-1 outline-none border-b border-surface-700 rounded-none"
						on:click|self={(e) => itemClick(item, e)}
						on:keydown={(e) => itemKeyDown(item, e)}
					>
						<span class="pointer-events-none">
							<b class="font-semibold">
								{item.name} | {item.barcode}
							</b>
							{#if item.description}
								({item.description})
							{/if}
						</span>
						<span class="pointer-events-none">
							${item.price.toFixed(2)}
						</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<div class="my-4">
		<div class="flex font-medium text-lg border-b-2 border-b-surface-900 py-1">
			<span class="w-[2%]" />
			<span class="w-[60%]">Producto</span>
			<span class="w-[15%] text-center">Precio unitario</span>
			<span class="w-[10%] text-center">Unidades</span>
			<span class="w-[15%] text-center">Precio total</span>
			<span class="w-[2%]" />
		</div>

		<div class="flex flex-col font-medium text-2xl">
			{#each lines as linea, i}
				<div
					class="linea py-2 flex items-center border-b border-b-surface-700 hover:bg-primary-50"
					class:selected={showQuantiyModal && i === lines.length - 1}
				>
					<span class="w-[2%] text-center">{i + 1}</span>
					<span class="w-[60%] font-semibold">{linea.producto.name}</span>
					<span class="w-[15%] text-center">${linea.producto.price.toFixed(2)}</span>
					<span class="w-[10%] text-center flex justify-center gap-x-2">
						<button
							class="h-8 w-8 p-0 btn variant-filled-error border border-error-700"
							disabled={linea.unidades <= 1}
							on:click={() => decrease(i)}
						>
							<MinusIcon class="text-xl" />
						</button>
						<span class="w-12 font-medium">
							{linea.unidades}
						</span>
						<button
							class="h-8 w-8 p-0 btn variant-filled-success border border-success-700"
							on:click={() => increase(i)}
						>
							<PlusIcon class="text-xl" />
						</button>
					</span>
					<span class="w-[15%] text-center font-semibold">
						${(linea.producto.price * linea.unidades).toFixed(2)}
					</span>
					<span class="w-[2%]">
						<button class="h-6 w-6 p-0 btn variant-filled-error" on:click={() => remove(i)}>
							<CloseIcon />
						</button>
					</span>
				</div>
			{/each}
		</div>

		<div class="flex justify-between text-5xl font-bold pt-2 px-3">
			<span>Precio total</span>
			<span>
				${lines
					.reduce((acc, linea) => (acc += linea.producto.price * linea.unidades), 0)
					.toFixed(2)}
			</span>
		</div>
	</div>

	<div class="w-full flex justify-end">
		<button
			class="btn variant-filled-secondary text-lg"
			on:click={createInvoice}
			disabled={!lines.length}
		>
			Finalizar
		</button>
	</div>
</div>

<style lang="postcss">
	.linea:last-of-type {
		@apply border-b-2;
	}

	.linea.selected {
		@apply bg-primary-300;
	}
</style>
