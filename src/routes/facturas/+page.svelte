<script lang="ts">
	import { invoke } from "@tauri-apps/api";

	let invoices: any[] = [];

	invoke<any>('get_invoices').then((result) => (invoices = result));
</script>

<div>
  <pre>
    {JSON.stringify(invoices, null, 2)}
  </pre>
</div>
